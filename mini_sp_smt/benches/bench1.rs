use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mini_sp_smt::*;
use z3_sys::*;

#[inline]
fn cfg_bench() -> ConfigZ3 {
    ConfigZ3::new()
}

#[inline]
fn ctx_bench() -> ContextZ3 {
    ContextZ3::new(&ConfigZ3::new())
}

#[inline]
fn and_bench() -> Z3_ast {
    let ctx = ContextZ3::new(&ConfigZ3::new());
    let boolsort = BoolSortZ3::new(&ctx);

    let bool1 = BoolZ3::new(&ctx, true);
    let bool2 = BoolZ3::new(&ctx,false);

    let x1 = BoolVarZ3::new(&ctx, &boolsort, "x1");
    let x2 = BoolVarZ3::new(&ctx, &boolsort, "x2");

    ANDZ3::new(&ctx, vec!(x1, x2, bool1, bool2))
}

pub fn criterion_benchmark_cfg(c: &mut Criterion) {
    c.bench_function("ConfigZ3", |b| b.iter(|| cfg_bench()));
}

pub fn criterion_benchmark_ctx(c: &mut Criterion) {
    c.bench_function("ContextZ3", |b| b.iter(|| ctx_bench()));
}

pub fn criterion_benchmark_and(c: &mut Criterion) {
    c.bench_function("ANDZ3", |b| b.iter(|| and_bench()));
}



criterion_group!(benches, criterion_benchmark_cfg, criterion_benchmark_ctx, criterion_benchmark_and);
// criterion_group!(benches, criterion_benchmark_ctx);
criterion_main!(benches);