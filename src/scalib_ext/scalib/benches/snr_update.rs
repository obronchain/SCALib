use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use ndarray::{Array1, Array2};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use scalib::snr;

fn bench_snr_update(c: &mut Criterion) {
    let mut group = c.benchmark_group("snr_update");
    let nc = 256;
    let np = 8;
    let n = 1000;
    for ns in [250000,500000,1000000,2000000].iter() {
        let mut snr = snr::SNR::new(nc, *ns, np);
        let x = Array2::<i16>::random((n, *ns), Uniform::new(0, 10000));
        let y = Array2::<u16>::random((n, np), Uniform::new(0, nc as u16));

        group.bench_with_input(BenchmarkId::new("snr_update", ns), ns, |b, ns| {
            b.iter(|| {
                snr.update(x.view(), y.view());
            })
        });
    }
    group.finish();
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = bench_snr_update
}
criterion_main!(benches);
