use crossbeam_utils::CachePadded;
use std::sync::atomic::{AtomicU32, Ordering};

const ITER_PER_THREAD: u64 = 100_000_000;

#[repr(align(64))]
struct Aligned64(AtomicU32);

/// 将线程固定到某个核心
fn pin_worker_thread(core_index: usize) {
    let Some(ids) = core_affinity::get_core_ids() else {
        return;
    };
    if ids.is_empty() {
        return;
    }
    let id = ids[core_index % ids.len()];
    let _ = core_affinity::set_for_current(id);
}

/// 两颗计数器各自用 `CachePadded` 隔开，通常落到不同 cache line，无伪共享。
pub fn test_false_sharing() {
    let mut arr = [CachePadded::new(AtomicU32::new(0)),
        CachePadded::new(AtomicU32::new(0))];
    let (left, right) = arr.split_at_mut(1);
    let a = &mut left[0];
    let b = &mut right[0];
    std::thread::scope(|s| {
        s.spawn(|| {
            pin_worker_thread(0);
            for _ in 0..ITER_PER_THREAD {
                a.fetch_add(1, Ordering::Relaxed);
            }
        });
        s.spawn(|| {
            pin_worker_thread(1);
            for _ in 0..ITER_PER_THREAD {
                b.fetch_add(1, Ordering::Relaxed);
            }
        });
    });
}

/// 同上，但计数器在同一结构体内相邻，易触发伪共享（具体差多少与 CPU / 绑核有关）。
pub fn test_false_sharing2() {
    let mut arr = [AtomicU32::new(0), AtomicU32::new(0)];
    let (left, right) = arr.split_at_mut(1);
    let a = &mut left[0];
    let b = &mut right[0];
    std::thread::scope(|s| {
        s.spawn(|| {
            pin_worker_thread(0);
            for _ in 0..ITER_PER_THREAD {
                a.fetch_add(1, Ordering::Relaxed);
            }
        });
        s.spawn(|| {
            pin_worker_thread(1);
            for _ in 0..ITER_PER_THREAD {
                b.fetch_add(1, Ordering::Relaxed);
            }
        });
    });
}

pub fn test_false_sharing3() {
    let mut arr = [Aligned64(AtomicU32::new(0)), Aligned64(AtomicU32::new(0))];
    let (left, right) = arr.split_at_mut(1);
    let a = &mut left[0];
    let b = &mut right[0];
    std::thread::scope(|s| {
        s.spawn(|| {
            pin_worker_thread(0);
            for _ in 0..ITER_PER_THREAD {
                a.0.fetch_add(1, Ordering::Relaxed);
            }
        });
        s.spawn(|| {
            pin_worker_thread(1);
            for _ in 0..ITER_PER_THREAD {
                b.0.fetch_add(1, Ordering::Relaxed);
            }
        });
    });
}
