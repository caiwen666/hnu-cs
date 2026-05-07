use std::alloc::{alloc_zeroed, Layout};
use std::hint::black_box;
use std::time::{Duration, Instant};

/// 以 64 字节步长顺序扫过缓冲区，测量近似读带宽 (GB/s)。
fn sequential_read(buf: &[u8], measure: Duration) -> f64 {
    let size = buf.len();
    let lines = size.div_ceil(64);
    let bytes_per_pass = lines * 64;

    // 预热
    for _ in 0..2 {
        for line in 0..lines {
            let i = (line * 64).min(size.saturating_sub(1));
            black_box(buf[i]);
        }
    }

    let start = Instant::now();
    let mut passes: u64 = 0;
    while start.elapsed() < measure {
        for line in 0..lines {
            let i = (line * 64).min(size.saturating_sub(1));
            black_box(buf[i]);
        }
        passes += 1;
    }

    let elapsed = start.elapsed().as_secs_f64().max(1e-9);
    let total_bytes = bytes_per_pass as f64 * passes as f64;
    total_bytes / elapsed / (1024.0 * 1024.0 * 1024.0)
}

/// 取中位数
fn median(sorted: &[f64]) -> f64 {
    let n = sorted.len();
    if n.is_multiple_of(2) {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    } else {
        sorted[n / 2]
    }
}

/// 取上下四分位数和中位数
/// 
/// 返回的三个数字依次增大
fn quartiles_hinges(sorted: &[f64]) -> (f64, f64, f64) {
    let n = sorted.len();
    match n {
        0 => (0.0, 0.0, 0.0),
        1 => (sorted[0], sorted[0], sorted[0]),
        2 => (sorted[0], (sorted[0] + sorted[1]) / 2.0, sorted[1]),
        _ => {
            let mid = n / 2;
            let (lower, upper) = if n.is_multiple_of(2) {
                (&sorted[..mid], &sorted[mid..])
            } else {
                (&sorted[..mid], &sorted[mid + 1..])
            };
            let q1 = median(lower);
            let q2 = median(sorted);
            let q3 = median(upper);
            (q1, q2, q3)
        }
    }
}

struct RobustStats {
    /// 剔除掉离群值后的均值
    mean_inlier: f64,
    /// 全样本中位数
    median_all: f64,
    /// 离群值个数
    removed: usize,
    total: usize,
    min_raw: f64,
    max_raw: f64,
}

/// 计算比较符合统计学意义的数据
fn robust_bandwidth_stats(mut samples: Vec<f64>) -> RobustStats {
    let total = samples.len();
    
    let mut sorted_all = samples.clone();
    sorted_all.sort_by(|a, b| a.total_cmp(b));
    let median_all = median(&sorted_all);
    let min_raw = sorted_all[0];
    let max_raw = sorted_all[total - 1];

    let k = 1.5;
    let (q1, _q2, q3) = quartiles_hinges(&sorted_all);
    let iqr = (q3 - q1).max(0.0);
    // 上下四分位数 ± k * IQR 内的样本认为是没问题的，其余的认为是离群
    let lo = q1 - k * iqr;
    let hi = q3 + k * iqr;
    // 剔除离群值
    samples.retain(|&x| x >= lo && x <= hi);

    let removed = total - samples.len();
    let mean_inlier = if samples.is_empty() {
        // 如果全被剔除，则用全样本中位数兜底
        median_all
    } else {
        samples.iter().sum::<f64>() / samples.len() as f64
    };

    RobustStats {
        mean_inlier,
        median_all,
        removed,
        total,
        min_raw,
        max_raw,
    }
}

fn main() {
    // 绑定当前线程到指定 CPU 核心，cpu id 从 0 开始
    // 应该是和 AIDA64 上面写的一致
    let core: u32 = 15;
    let mask = 1usize << core;
    use windows_sys::Win32::System::Threading::{GetCurrentThread, SetThreadAffinityMask};
    let prev = unsafe { SetThreadAffinityMask(GetCurrentThread(), mask) };
    if prev == 0 {
        println!("{:#?}", std::io::Error::last_os_error());
    }

    // 工作集大小
    let sizes: &[usize] = &[
        4 * 1024,
        8 * 1024,
        16 * 1024,
        32 * 1024,
        48 * 1024,
        64 * 1024,
        96 * 1024,
        128 * 1024,
        192 * 1024,
        256 * 1024,
        384 * 1024,
        512 * 1024,
        768 * 1024,
        1024 * 1024,
        1536 * 1024,
        2 * 1024 * 1024,
        3 * 1024 * 1024,
        4 * 1024 * 1024,
        6 * 1024 * 1024,
        8 * 1024 * 1024,
        12 * 1024 * 1024,
        16 * 1024 * 1024,
        24 * 1024 * 1024,
        32 * 1024 * 1024,
        48 * 1024 * 1024,
        64 * 1024 * 1024,
    ];

    println!(
        "{:>6}  {:>8}  {:>8}  {:>10}",
        "工作集",
        "均值",
        "中位数",
        "[min,max]",
    );
    println!("{:-<72}", "");

    let mut chart_points: Vec<(f64, f64)> = Vec::with_capacity(sizes.len());

    for &size in sizes {
        // 直接 4kb 对齐，确保缓存行都是工作集内容
        let layout = Layout::from_size_align(size, 4096).unwrap();
        let buf = unsafe { std::slice::from_raw_parts_mut(alloc_zeroed(layout), size) };
        // 写入一些随机数据，避免全零被优化或合并
        for (i, b) in buf.iter_mut().enumerate() {
            *b = (i as u8).wrapping_mul(17).wrapping_add(3);
        }
        let mut runs = Vec::new();
        // 反复测量 20 次
        for _ in 0..20 {
            // 每次读 200ms
            runs.push(sequential_read(buf, Duration::from_millis(300)));
        }
        let st = robust_bandwidth_stats(runs);
        chart_points.push((size as f64, st.mean_inlier));

        let size_str = if size >= 1024 * 1024 {
            format!("{:>6.1} MiB", size as f64 / (1024.0 * 1024.0))
        } else if size >= 1024 {
            format!("{:>6.1} KiB", size as f64 / 1024.0)
        } else {
            format!("{:>6} B", size)
        };

        let outlier_note = if st.removed > 0 {
            format!("(剔{}/{}次)", st.removed, st.total)
        } else {
            String::new()
        };

        println!(
            "{size_str}  {:>8.2} GiB/s  {:>6.2}  {:>6.1}-{:<6.1} {}",
            st.mean_inlier,
            st.median_all,
            st.min_raw,
            st.max_raw,
            outlier_note
        );

        // 这里理论上要释放内存，但是我就不释放，泄露就泄露
    }
}
