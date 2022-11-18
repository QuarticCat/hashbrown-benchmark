# Hashbrown Benchmark

Check the impact of hashmap sizes on the performance of hashmap lookups.

## Run

```console
$ cargo bench -- --quiet
```

## Results

CPU: Ryzen 3700X

![results-graph](results.svg)

<details>
  <summary>Click me</summary>

```text
num_entries: 2^13       time:   [4.2082 ms 4.2197 ms 4.2339 ms]

num_entries: 2^14       time:   [4.3163 ms 4.3382 ms 4.3634 ms]

num_entries: 2^15       time:   [4.3753 ms 4.3949 ms 4.4168 ms]

num_entries: 2^16       time:   [4.4192 ms 4.4505 ms 4.4866 ms]

num_entries: 2^17       time:   [4.6452 ms 4.6968 ms 4.7545 ms]

num_entries: 2^18       time:   [5.7159 ms 5.7920 ms 5.8720 ms]

num_entries: 2^19       time:   [6.8523 ms 6.9242 ms 7.0009 ms]

num_entries: 2^20       time:   [8.0416 ms 8.1426 ms 8.2457 ms]

num_entries: 2^21       time:   [9.6515 ms 9.8732 ms 10.115 ms]

num_entries: 2^22       time:   [11.315 ms 11.506 ms 11.709 ms]

num_entries: 2^23       time:   [20.175 ms 20.384 ms 20.607 ms]

num_entries: 2^24       time:   [24.855 ms 24.940 ms 25.044 ms]

num_entries: 2^25       time:   [26.532 ms 26.859 ms 27.237 ms]

num_entries: 2^26       time:   [27.130 ms 27.250 ms 27.390 ms]

num_entries: 2^27       time:   [27.654 ms 27.897 ms 28.200 ms]
```

</details>

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](/LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE-MIT](/LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.
