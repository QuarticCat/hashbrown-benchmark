use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hashbrown::HashMap;
use rand::prelude::*;

fn hashbrown_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();

    for num_entries_exp in 10..28 {
        let num_entries = 1 << num_entries_exp;

        let mut map = HashMap::<u64, u64>::new();
        map.reserve(num_entries);
        for _ in 0..num_entries {
            map.insert(rng.gen(), rng.gen());
        }

        let query_keys = (0..1_000_000).map(|_| rng.gen()).collect::<Vec<u64>>();
        c.bench_function(&format!("num_entries: 1 << {num_entries_exp}"), |b| {
            b.iter(|| {
                for key in &query_keys {
                    black_box(map.get(key));
                }
            })
        });
    }
}

criterion_group!(benches, hashbrown_benchmark);
criterion_main!(benches);