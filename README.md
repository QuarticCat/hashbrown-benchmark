# Hashbrown Benchmark

Check the impact of hashmap sizes on the performance of hashmap lookups.

## Run

```console
$ cargo bench -- --quiet
```

## Results

![results-graph](results.svg)

<details>
  <summary>Click me</summary>

```text
num_entries: 1 << 10    time:   [4.0161 ms 4.0287 ms 4.0427 ms]

num_entries: 1 << 11    time:   [4.1842 ms 4.2042 ms 4.2285 ms]

num_entries: 1 << 12    time:   [4.0224 ms 4.0319 ms 4.0444 ms]

num_entries: 1 << 13    time:   [4.1605 ms 4.1757 ms 4.1927 ms]

num_entries: 1 << 14    time:   [4.2499 ms 4.2630 ms 4.2770 ms]

num_entries: 1 << 15    time:   [4.3286 ms 4.3443 ms 4.3626 ms]

num_entries: 1 << 16    time:   [4.5513 ms 4.6267 ms 4.7110 ms]

num_entries: 1 << 17    time:   [4.6607 ms 4.7779 ms 4.9147 ms]

num_entries: 1 << 18    time:   [6.4940 ms 6.6921 ms 6.8998 ms]

num_entries: 1 << 19    time:   [6.2598 ms 6.3494 ms 6.4397 ms]

num_entries: 1 << 20    time:   [8.0513 ms 8.1612 ms 8.2774 ms]

num_entries: 1 << 21    time:   [8.7615 ms 8.8270 ms 8.9022 ms]

num_entries: 1 << 22    time:   [9.5370 ms 9.6113 ms 9.6966 ms]

num_entries: 1 << 23    time:   [18.071 ms 18.252 ms 18.438 ms]

num_entries: 1 << 24    time:   [23.677 ms 23.781 ms 23.890 ms]

num_entries: 1 << 25    time:   [25.239 ms 25.319 ms 25.411 ms]

num_entries: 1 << 26    time:   [26.435 ms 26.581 ms 26.747 ms]

num_entries: 1 << 27    time:   [26.209 ms 26.255 ms 26.316 ms]
```

</details>

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](/LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE-MIT](/LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.
