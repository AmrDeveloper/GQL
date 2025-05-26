use std::hint::black_box;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use gitql_parser::tokenizer::Tokenizer;

const QUERY_100_CHAR: &str = "SELECT name, COUNT(name) FROM commits GROUP BY name, author_email ORDER BY commit_num DESC LIMIT 100";

fn tokenizer_100_char_benchmark(c: &mut Criterion) {
    c.bench_function("Tokenizer 100 Char", |b| {
        b.iter(|| Tokenizer::tokenize(black_box(QUERY_100_CHAR.to_owned())))
    });
}

fn tokenizer_100k_char_benchmark(c: &mut Criterion) {
    let query_100k_char = QUERY_100_CHAR.repeat(100_000 / 100);
    c.bench_function("Tokenizer 100K Char", |b| {
        b.iter(|| Tokenizer::tokenize(black_box(query_100k_char.to_owned())))
    });
}

fn tokenizer_1m_char_benchmark(c: &mut Criterion) {
    let query_100k_char = QUERY_100_CHAR.repeat(1_000_000 / 100);
    c.bench_function("Tokenizer 1M Char", |b| {
        b.iter(|| Tokenizer::tokenize(black_box(query_100k_char.to_owned())))
    });
}

fn tokenizer_10m_char_benchmark(c: &mut Criterion) {
    let query_100k_char = QUERY_100_CHAR.repeat(10_000_000 / 100);
    c.bench_function("Tokenizer 10M Char", |b| {
        b.iter(|| Tokenizer::tokenize(black_box(query_100k_char.to_owned())))
    });
}

criterion_group! {
   name = benches;
   config = Criterion::default().significance_level(0.1).sample_size(10);
   targets =
   // Tokenizer
   tokenizer_100_char_benchmark,
   tokenizer_100k_char_benchmark,
   tokenizer_1m_char_benchmark,
   tokenizer_10m_char_benchmark
}

criterion_main!(benches);
