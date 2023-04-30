use shar_search::SharBinarySearch;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn std(v: &[i64], val: i64) -> Result<usize, usize> {
    v.bl_binary_search(&val)
}

fn bl(v: &[i64], val: i64) -> Result<usize, usize> {
    v.bl_binary_search(&val)
}

fn test_all<G>(mut f: G, limit: i64)
where
    G: FnMut(&[i64], i64) -> Result<usize, usize>,
{
    let mut sorted = vec![];
    let found = f(&sorted, -1);
    assert_eq!(found, Err(0));

    for i in 0..limit {
        sorted.push(i);
        let _found = f(&sorted, -1);
        // assert_eq!(found, Err(0));

        for j in 0..=i {
            let _found = f(&sorted, j);
            // assert_eq!(found, Ok(j as usize));
        }

        let _found = f(&sorted, i + 1);
        // assert_eq!(found, Ok(sorted.len()));
    }
}

pub fn all(c: &mut Criterion) {
    let mut group = c.benchmark_group("all");

    group.bench_function("std", |b| b.iter(|| test_all(std, black_box(64))));
    group.bench_function("bl", |b| b.iter(|| test_all(bl, black_box(64))));
}

pub fn half(c: &mut Criterion) {
    let mut group = c.benchmark_group("half");

    group.bench_function("std_1", |b| b.iter(|| test_all(std, black_box(1))));
    group.bench_function("std_2", |b| b.iter(|| test_all(std, black_box(2))));
    group.bench_function("std_3", |b| b.iter(|| test_all(std, black_box(3))));
    group.bench_function("std_4", |b| b.iter(|| test_all(std, black_box(4))));
    group.bench_function("std_32", |b| b.iter(|| test_all(std, black_box(32))));
    group.bench_function("std_128", |b| b.iter(|| test_all(std, black_box(128))));
    group.bench_function("std_256", |b| b.iter(|| test_all(std, black_box(256))));
    group.bench_function("std_512", |b| b.iter(|| test_all(std, black_box(512))));
    group.bench_function("std_1024", |b| b.iter(|| test_all(std, black_box(1024))));

    group.bench_function("bl_1", |b| b.iter(|| test_all(bl, black_box(1))));
    group.bench_function("bl_2", |b| b.iter(|| test_all(bl, black_box(2))));
    group.bench_function("bl_3", |b| b.iter(|| test_all(bl, black_box(3))));
    group.bench_function("bl_4", |b| b.iter(|| test_all(bl, black_box(4))));
    group.bench_function("bl_32", |b| b.iter(|| test_all(bl, black_box(32))));
    group.bench_function("bl_128", |b| b.iter(|| test_all(bl, black_box(128))));
    group.bench_function("bl_256", |b| b.iter(|| test_all(bl, black_box(256))));
    group.bench_function("bl_512", |b| b.iter(|| test_all(bl, black_box(512))));
    group.bench_function("bl_1024", |b| b.iter(|| test_all(bl, black_box(1024))));
}

criterion_group!(benches, all, half);
criterion_main!(benches);
