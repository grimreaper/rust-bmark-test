use bmark;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_simple_one_char_palindrome(c: &mut Criterion) {
    c.bench_function("one char one iterator palindrome", |b| {
        b.iter(|| bmark::simple_palindrome("a"))
    });
}

pub fn criterion_dual_one_char_palindrome(c: &mut Criterion) {
    c.bench_function("one char dual iterator palindrome", |b| {
        b.iter(|| bmark::dual_iterator_palindrome("a"))
    });
}

pub fn criterion_compare_simple_vs_dual(c: &mut Criterion) {
    let mut group = c.benchmark_group("comparison one char");
    group.bench_function("one char simple iterator palindrome)", |b| {
        b.iter(|| bmark::simple_palindrome("a"))
    });
    group.bench_function("one char dual iterator palindrome)", |b| {
        b.iter(|| bmark::dual_iterator_palindrome("a"))
    });
    group.finish();
}

criterion_group!(
    benches,
    criterion_simple_one_char_palindrome,
    criterion_dual_one_char_palindrome,
    criterion_compare_simple_vs_dual
);
// criterion_group!(benches, criterion_dual_one_char_palindrome);
criterion_main!(benches);
