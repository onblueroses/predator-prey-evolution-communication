use criterion::{Criterion, criterion_group, criterion_main};

fn bench_placeholder(c: &mut Criterion) {
    c.bench_function("placeholder", |b| {
        b.iter(|| {
            // Benchmarks will be added as modules are implemented
            2 + 2
        });
    });
}

criterion_group!(benches, bench_placeholder);
criterion_main!(benches);
