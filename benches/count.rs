//! Criterion benchmarks for the counting core.
//!
//! Run with `cargo bench`. The text is generated with a fixed seed, so every
//! run measures the same input.

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;
use wfreq::core::{count_words, filter_by_length, top_n};

/// Builds `words` words of text from a 5,000-word vocabulary.
///
/// Word choice is skewed so a few words are very common and most are rare,
/// like real text. Some words are capitalized or followed by a comma so the
/// cleaning and lowercasing code is exercised too.
fn corpus(words: usize) -> String {
    const VOCAB: usize = 5_000;
    let mut state: u64 = 42;
    let mut text = String::with_capacity(words * 9);
    for i in 0..words {
        // 64-bit LCG (Knuth's MMIX constants): fast, deterministic, no extra crate
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let r = (state >> 11) as f64 / (1u64 << 53) as f64;
        let idx = ((r * r * r) * VOCAB as f64) as usize;
        if i % 13 == 0 {
            text.push_str("Word");
        } else {
            text.push_str("word");
        }
        text.push_str(&idx.to_string());
        text.push_str(if i % 17 == 0 { ", " } else { " " });
    }
    text
}

fn bench_count_words(c: &mut Criterion) {
    let mut group = c.benchmark_group("count_words");
    for words in [10_000, 100_000, 1_000_000] {
        let text = corpus(words);
        group.throughput(Throughput::Bytes(text.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(words), &text, |b, text| {
            b.iter(|| count_words(black_box(text), false))
        });
    }
    group.finish();
}

fn bench_rank_and_filter(c: &mut Criterion) {
    let counts = count_words(&corpus(1_000_000), false);

    c.bench_function("top_n/10 of 1M words", |b| {
        b.iter(|| top_n(black_box(&counts), 10))
    });

    c.bench_function("filter_by_length/3..=8 of 1M words", |b| {
        b.iter_batched(
            || counts.clone(),
            |mut map| filter_by_length(&mut map, 3, Some(8)),
            criterion::BatchSize::LargeInput,
        )
    });
}

criterion_group!(benches, bench_count_words, bench_rank_and_filter);
criterion_main!(benches);
