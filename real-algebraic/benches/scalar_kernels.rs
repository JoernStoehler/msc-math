//! Criterion benches for the scalar kernels we care about.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use num_rational::BigRational;
use num_traits::Signed;
use real_algebraic::{dot, solve_square, Algebraic, OrderedField, TanPiFifth};

type TanPiFifthField = Algebraic<TanPiFifth>;

fn benchmark_scalar_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar_add");

    let f_left = 1.25f64;
    let f_right = 2.75f64;
    group.bench_function(BenchmarkId::new("f64", "pair"), |b| {
        b.iter(|| black_box(f_left) + black_box(f_right))
    });

    let q_left = BigRational::from_frac(5, 4);
    let q_right = BigRational::from_frac(11, 4);
    group.bench_function(BenchmarkId::new("BigRational", "pair"), |b| {
        b.iter(|| black_box(q_left.clone()) + black_box(q_right.clone()))
    });

    let t = TanPiFifthField::generator();
    let a = TanPiFifthField::from_i64(1) + t.clone();
    let b_val = TanPiFifthField::from_frac(3, 2) - t;
    group.bench_function(BenchmarkId::new("TanPiFifthField", "pair"), |b| {
        b.iter(|| black_box(a.clone()) + black_box(b_val.clone()))
    });

    group.finish();
}

fn benchmark_scalar_sub(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar_sub");

    let f_left = 1.25f64;
    let f_right = 2.75f64;
    group.bench_function(BenchmarkId::new("f64", "pair"), |b| {
        b.iter(|| black_box(f_left) - black_box(f_right))
    });

    let q_left = BigRational::from_frac(5, 4);
    let q_right = BigRational::from_frac(11, 4);
    group.bench_function(BenchmarkId::new("BigRational", "pair"), |b| {
        b.iter(|| black_box(q_left.clone()) - black_box(q_right.clone()))
    });

    let t = TanPiFifthField::generator();
    let a = TanPiFifthField::from_i64(1) + t.clone();
    let b_val = TanPiFifthField::from_frac(3, 2) - t;
    group.bench_function(BenchmarkId::new("TanPiFifthField", "pair"), |b| {
        b.iter(|| black_box(a.clone()) - black_box(b_val.clone()))
    });

    group.finish();
}

fn benchmark_scalar_mul(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar_mul");

    let f_left = 1.25f64;
    let f_right = 2.75f64;
    group.bench_function(BenchmarkId::new("f64", "pair"), |b| {
        b.iter(|| black_box(f_left) * black_box(f_right))
    });

    let q_left = BigRational::from_frac(5, 4);
    let q_right = BigRational::from_frac(11, 4);
    group.bench_function(BenchmarkId::new("BigRational", "pair"), |b| {
        b.iter(|| black_box(q_left.clone()) * black_box(q_right.clone()))
    });

    let t = TanPiFifthField::generator();
    let a = TanPiFifthField::from_i64(1) + t.clone();
    let b_val = TanPiFifthField::from_frac(3, 2) - t;
    group.bench_function(BenchmarkId::new("TanPiFifthField", "pair"), |b| {
        b.iter(|| black_box(a.clone()) * black_box(b_val.clone()))
    });

    group.finish();
}

fn benchmark_scalar_div(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar_div");

    let f_left = 1.25f64;
    let f_right = 2.75f64;
    group.bench_function(BenchmarkId::new("f64", "pair"), |b| {
        b.iter(|| black_box(f_left) / black_box(f_right))
    });

    let q_left = BigRational::from_frac(5, 4);
    let q_right = BigRational::from_frac(11, 4);
    group.bench_function(BenchmarkId::new("BigRational", "pair"), |b| {
        b.iter(|| black_box(q_left.clone()) / black_box(q_right.clone()))
    });

    let t = TanPiFifthField::generator();
    let a = TanPiFifthField::from_i64(1) + t.clone();
    let b_val = TanPiFifthField::from_i64(2) + t;
    group.bench_function(BenchmarkId::new("TanPiFifthField", "pair"), |b| {
        b.iter(|| black_box(a.clone()) / black_box(b_val.clone()))
    });

    group.finish();
}

fn benchmark_sign(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar_sign");

    let f_value = 1.25f64;
    group.bench_function(BenchmarkId::new("f64", "value"), |b| {
        b.iter(|| black_box(f_value).partial_cmp(&0.0).unwrap())
    });

    let q_value = BigRational::from_frac(5, 4);
    group.bench_function(BenchmarkId::new("BigRational", "value"), |b| {
        b.iter(|| black_box(q_value.clone()).signum())
    });

    let t = TanPiFifthField::generator();
    let value = TanPiFifthField::from_i64(1) + t;
    group.bench_function(BenchmarkId::new("TanPiFifthField", "value"), |b| {
        b.iter(|| black_box(value.clone()).sign())
    });

    group.finish();
}

fn benchmark_dot4(c: &mut Criterion) {
    let mut group = c.benchmark_group("dot4");

    let f_left = [1.0f64, 2.0, 3.0, 4.0];
    let f_right = [4.0f64, 3.0, 2.0, 1.0];
    group.bench_function(BenchmarkId::new("f64", "dim4"), |b| {
        b.iter(|| {
            black_box(f_left[0]) * black_box(f_right[0])
                + black_box(f_left[1]) * black_box(f_right[1])
                + black_box(f_left[2]) * black_box(f_right[2])
                + black_box(f_left[3]) * black_box(f_right[3])
        })
    });

    let q_left = [
        BigRational::from_i64(1),
        BigRational::from_i64(2),
        BigRational::from_i64(3),
        BigRational::from_i64(4),
    ];
    let q_right = [
        BigRational::from_i64(4),
        BigRational::from_i64(3),
        BigRational::from_i64(2),
        BigRational::from_i64(1),
    ];
    group.bench_function(BenchmarkId::new("BigRational", "dim4"), |b| {
        b.iter(|| dot(black_box(&q_left), black_box(&q_right)))
    });

    let t = TanPiFifthField::generator();
    let a = [
        TanPiFifthField::from_i64(1),
        t.clone(),
        TanPiFifthField::from_i64(2),
        TanPiFifthField::from_i64(3),
    ];
    let b_val = [
        TanPiFifthField::from_i64(4),
        TanPiFifthField::from_i64(3),
        t,
        TanPiFifthField::from_i64(1),
    ];
    group.bench_function(BenchmarkId::new("TanPiFifthField", "dim4"), |b| {
        b.iter(|| dot(black_box(&a), black_box(&b_val)))
    });

    group.finish();
}

fn benchmark_solve2(c: &mut Criterion) {
    let mut group = c.benchmark_group("solve2");

    let q_matrix = [
        [BigRational::from_i64(2), BigRational::from_i64(1)],
        [BigRational::from_i64(1), BigRational::from_i64(1)],
    ];
    let q_rhs = [BigRational::from_i64(1), BigRational::from_i64(0)];
    group.bench_function(BenchmarkId::new("BigRational", "2x2"), |b| {
        b.iter(|| solve_square(black_box(&q_matrix), black_box(&q_rhs)))
    });

    let t = TanPiFifthField::generator();
    let a_matrix = [
        [TanPiFifthField::one(), t.clone()],
        [t, TanPiFifthField::from_i64(3)],
    ];
    let a_rhs = [TanPiFifthField::from_i64(1), TanPiFifthField::from_i64(0)];
    group.bench_function(BenchmarkId::new("TanPiFifthField", "2x2"), |b| {
        b.iter(|| solve_square(black_box(&a_matrix), black_box(&a_rhs)))
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_scalar_add,
    benchmark_scalar_sub,
    benchmark_scalar_mul,
    benchmark_scalar_div,
    benchmark_sign,
    benchmark_dot4,
    benchmark_solve2
);
criterion_main!(benches);
