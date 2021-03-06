use criterion::{black_box, criterion_group, criterion_main, Criterion};
use procedural_generation::*;
use std::time::Duration;

fn perlin_gen(width: usize, height: usize) {
    Generator::new()
        .with_size(width, height)
        .spawn_perlin(|value| {
            if value > 0.66 {
                2
            } else if value > 0.33 {
                1
            } else {
                0
            }
        });
}

fn room_gen(width: usize, height: usize) {
    let size = Size::new((10, 10), (100, 100));
    Generator::new()
        .with_size(width, height)
        .spawn_rooms(1, 1000, &size);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample-size-example");
    group.warm_up_time(Duration::from_secs(1));
    group.sample_size(10);
    group.bench_function("perlin_gen 1000 1000", |b| b.iter(|| perlin_gen(black_box(1000), black_box(1000))));
    group.bench_function("room_gen 1000 1000", |b| b.iter(|| room_gen(black_box(1000), black_box(1000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
