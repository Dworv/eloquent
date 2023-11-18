use std::io::Read;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sim::*;

fn criterion_benchmark(c: &mut Criterion) {
    let layout = Layout::from_order(Key::all());
    let speeds = Speeds::init();
    let mut text = String::new();
    std::fs::File::open("../text/text.txt").unwrap().read_to_string(&mut text).unwrap();
    let text = preprocess_str(&text);
    
    c.bench_function("sim regular", |b| b.iter(|| sim(black_box(&layout), black_box(&speeds), black_box(&text))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);