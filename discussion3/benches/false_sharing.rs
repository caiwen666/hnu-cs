use criterion::{Criterion, criterion_group, criterion_main};
use discussion3::{test_false_sharing, test_false_sharing2, test_false_sharing3};

fn bench_test_false_sharing(c: &mut Criterion) {
    let mut group = c.benchmark_group("false_sharing");
    group.sample_size(10);
    group.bench_function("test_false_sharing", |b| {
        b.iter(test_false_sharing);
    });
    group.bench_function("test_false_sharing2", |b| {
        b.iter(test_false_sharing2);
    });
    group.bench_function("test_false_sharing3", |b| {
        b.iter(test_false_sharing3);
    });
    group.finish();
}

criterion_group!(benches, bench_test_false_sharing);
criterion_main!(benches);
