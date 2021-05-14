use criterion::{criterion_group, criterion_main, Criterion};

const OUTPUT: &str = include_str!("../assets/cargo-expand.txt");

fn bench_marking(c: &mut Criterion) {
    c.bench_function("get_markers", |b| {
        b.iter(|| {
            yew_ansi::get_markers(OUTPUT).for_each(|m| {
                criterion::black_box(m);
            })
        })
    });
}

criterion_group!(benches, bench_marking);
criterion_main!(benches);
