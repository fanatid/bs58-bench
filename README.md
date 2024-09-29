### bs58 encode / decode benchmark (more than 10k items)

```
$ grep 'model name' /proc/cpuinfo | head -n 1
model name	: 13th Gen Intel(R) Core(TM) i7-13700H
$ cargo bench bs58
   Compiling bs58-bench v0.1.0 (/home/kirill/projects/bs58-bench)
    Finished `bench` profile [optimized] target(s) in 1.02s
     Running unittests src/lib.rs (target/release/deps/bs58_bench-c6c68d7f3ad920c2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/bs58.rs (target/release/deps/bs58-2e0372bc3476034b)
Gnuplot not found, using plotters backend
bs58 encode/10k         time:   [9.1995 ms 9.3758 ms 9.5914 ms]
Found 13 outliers among 100 measurements (13.00%)
  2 (2.00%) high mild
  11 (11.00%) high severe

Benchmarking bs58 encode/100k: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 9.4s, or reduce sample count to 50.
bs58 encode/100k        time:   [92.465 ms 94.006 ms 95.786 ms]
Found 14 outliers among 100 measurements (14.00%)
  1 (1.00%) high mild
  13 (13.00%) high severe

bs58 decode/10k         time:   [3.3918 ms 3.4616 ms 3.5470 ms]
Found 8 outliers among 100 measurements (8.00%)
  8 (8.00%) high severe

bs58 decode/100k        time:   [34.214 ms 34.875 ms 35.663 ms]
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) high mild
  9 (9.00%) high severe
```
