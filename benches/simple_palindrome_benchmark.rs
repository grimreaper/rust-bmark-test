use bmark;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::distributions::{Alphanumeric, DistString};

pub fn one_charcter_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("one character string");
    group.bench_function("simple iterator palindrome", |b| {
        b.iter(|| bmark::simple_palindrome("a"))
    });
    group.bench_function("dual iterator palindrome", |b| {
        b.iter(|| bmark::dual_iterator_palindrome("a"))
    });
    group.finish();
}

pub fn random_not_likely_palindrome_very_long_string(c: &mut Criterion) {
    let string = Alphanumeric
        .sample_string(&mut rand::thread_rng(), 1600)
        .to_owned()
        .as_str();

    let mut group = c.benchmark_group("random not likely palinddrome very long string");
    group.bench_function("single iterator simple", |b| {
        b.iter(|| bmark::simple_palindrome(string))
    });
    group.bench_function("dual iterator palindrome", |b| {
        b.iter(|| bmark::dual_iterator_palindrome(string))
    });
    group.finish();
}

criterion_group!(
    benches,
    one_charcter_string,
    random_not_likely_palindrome_very_long_string,
);
// criterion_group!(benches, criterion_dual_one_char_palindrome);
criterion_main!(benches);
