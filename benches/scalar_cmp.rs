use bitmap::*;
use bitvec::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let vec = (0..2049).map(|x| (x * x + x) % 10).collect::<Vec<_>>();

    c.bench_function("scalar_eq_bitmap", |b| {
        b.iter(|| scalar_eq_bitmap(black_box(&vec), 0).len())
    });
    c.bench_function("scalar_eq_bitmap1", |b| {
        b.iter(|| scalar_eq_bitmap1(black_box(&vec), 0).len())
    });

    c.bench_function("scalar_eq_bool", |b| {
        b.iter(|| scalar_eq_bool(black_box(&vec), 0).len())
    });

    c.bench_function("bv_scalar_eq_loop", |b| {
        b.iter(|| bv_scalar_eq_loop(black_box(&vec), 0).len())
    });

    c.bench_function("bv_scalar_eq_collect", |b| {
        b.iter(|| bv_scalar_eq_collect(black_box(&vec), 0).len())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
