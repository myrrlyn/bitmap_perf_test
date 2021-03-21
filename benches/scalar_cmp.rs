use bitmap::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};


fn criterion_benchmark(c: &mut Criterion) {
    let vec = (0..2049).map(|x| (x*x + x) % 10).collect::<Vec<_>>();

    c.bench_function("scalar_eq_bitmap", |b| b.iter(|| scalar_eq_bitmap(black_box(&vec), 0)));

    c.bench_function("scalar_eq_bool", |b| b.iter(|| scalar_eq_bool(black_box(&vec), 0)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
