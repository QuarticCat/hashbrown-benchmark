use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hashbrown::HashMap;
use rand::prelude::*;

const RANGE: std::ops::RangeInclusive<u64> = u64::MIN..=u64::MAX;

fn random_key_bench(c: &mut Criterion) {
    let mut rng = thread_rng();

    for num_entries_exp in 13..28 {
        let num_entries = 1 << num_entries_exp;

        let mut map = HashMap::<u64, u64>::new();
        map.reserve(num_entries);
        for _ in 0..num_entries {
            map.insert(rng.gen_range(RANGE), rng.gen_range(RANGE));
        }

        let query_keys = (0..1_000_000)
            .map(|_| rng.gen_range(RANGE))
            .collect::<Vec<u64>>();
        c.bench_function(&format!("[random] 2^{num_entries_exp} entries"), |b| {
            b.iter(|| {
                for key in &query_keys {
                    black_box(map.get(key));
                }
            })
        });
    }
}

fn existing_key_bench(c: &mut Criterion) {
    let mut rng = thread_rng();

    for num_entries_exp in 13..28 {
        let num_entries = 1 << num_entries_exp;

        let mut map = HashMap::<u64, u64>::new();
        map.reserve(num_entries);
        for _ in 0..num_entries {
            map.insert(rng.gen_range(RANGE), rng.gen_range(RANGE));
        }

        let existing_keys = map.keys().copied().collect::<Vec<_>>();
        let query_keys = (0..1_000_000)
            .map(|_| existing_keys[rng.gen_range(0..map.len())])
            .collect::<Vec<u64>>();
        c.bench_function(&format!("[exist] 2^{num_entries_exp} entries"), |b| {
            b.iter(|| {
                for key in &query_keys {
                    black_box(map.get(key));
                }
            })
        });
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::new(10, 0));
    targets = random_key_bench, existing_key_bench
}
criterion_main!(benches);
