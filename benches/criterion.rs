extern crate criterion;

use criterion::{criterion_group, criterion_main, Criterion};
use jld_common::run;

fn jld_common_benchmark(c: &mut Criterion) {
    c.bench_function("jld-common", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                match run() {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error running jld-common: {:?}", e);
                    }
                }
            }
        })
    });
}

criterion_group!(benches, jld_common_benchmark);
criterion_main!(benches);
