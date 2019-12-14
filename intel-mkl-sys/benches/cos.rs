#[macro_use]
extern crate criterion;

use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    for &n in &[100, 1000, 10000] {
        c.bench_function(&format!("cos{}", n), |b| {
            let in_ = vec![0.0_f64; n];
            let mut out = vec![0.0_f64; n];
            b.iter(|| {
                for i in 0..n {
                    out[i] = in_[i].cos();
                }
            })
        });

        c.bench_function(&format!("vcos{}", n), |b| {
            let in_ = vec![0.0_f64; n];
            let mut out = vec![0.0_f64; n];
            b.iter(|| unsafe {
                intel_mkl_sys::vdCos(n as i32, in_.as_ptr(), out.as_mut_ptr());
            })
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
