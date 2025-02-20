use stinpse::Parser;
use criterion::{criterion_group, criterion_main, Criterion};

const BENCH_INPUT: &str = "ls -l -a -h -b -c -d 'ls -l' \"ls -l\"";

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parser", |b| b.iter(|| Parser::parse(BENCH_INPUT)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

