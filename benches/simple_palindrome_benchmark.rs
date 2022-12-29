use ::function_name::named;
use bmark;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::Throughput;
use rand::distributions::Alphanumeric;
use rand::distributions::DistString;

#[named]
pub fn one_charcter_string(c: &mut Criterion) {
    let mut group = c.benchmark_group(function_name!().replace("_", " "));
    group.bench_function("simple iterator palindrome", |b| {
        b.iter(|| bmark::simple_palindrome("a"))
    });
    group.bench_function("dual iterator palindrome", |b| {
        b.iter(|| bmark::dual_iterator_palindrome("a"))
    });
    group.finish();
}

#[named]
pub fn random_not_likely_palindrome_very_long_string(c: &mut Criterion) {
    let random_as_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 1600);
    let my_random = random_as_string.as_str();

    let mut group = c.benchmark_group(function_name!().replace("_", " "));
    group.bench_function("single iterator simple", |b| {
        b.iter(|| bmark::simple_palindrome(my_random))
    });
    group.bench_function("dual iterator palindrome", |b| {
        b.iter(|| bmark::dual_iterator_palindrome(my_random))
    });
    group.finish();
}

#[named]
pub fn random_not_likely_palindrome_differing_random_strings(c: &mut Criterion) {
    static KB: usize = 1024;

    let mut group = c.benchmark_group(function_name!().replace("_", " "));
    for size in [KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        let random_as_string = Alphanumeric.sample_string(&mut rand::thread_rng(), size.to_owned());
        let my_random = random_as_string.as_str();
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| my_random);
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    one_charcter_string,
    random_not_likely_palindrome_very_long_string,
    random_not_likely_palindrome_differing_random_strings
);
// criterion_group!(benches, criterion_dual_one_char_palindrome);
criterion_main!(benches);
