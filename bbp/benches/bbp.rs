use bbp::chudnovsky;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rug::Integer;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("chudovsky", |b| {
        b.iter(|| chudnovsky(black_box(Integer::from(0x20))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
