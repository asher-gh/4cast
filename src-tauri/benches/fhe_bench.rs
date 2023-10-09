use criterion::{criterion_group, criterion_main, Criterion};
use tfhe::{generate_keys, Config, ConfigBuilder};

// TODO: bench: key pair generation
// TODO: bench: how does vector encryption scale
// TODO: bench: mean calculation

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("key generation", |b| {
        b.iter(|| {
            let config: Config = ConfigBuilder::all_disabled()
                .enable_default_integers()
                .build();

            generate_keys(config);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
