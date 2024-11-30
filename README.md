### bs58 encode / decode benchmark (more than 10k items)

```
$ grep 'model name' /proc/cpuinfo | head -n 1
model name	: 13th Gen Intel(R) Core(TM) i7-13700H
```
```
$ cargo bench bs58
bs58 encode/10k         time:   [9.2567 ms 9.4452 ms 9.6757 ms]
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) low mild
  11 (11.00%) high severe

Benchmarking bs58/fd encode/10k: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.0s, enable flat sampling, or reduce sample count to 70.
bs58/fd encode/10k      time:   [967.62 µs 990.81 µs 1.0205 ms]
Found 16 outliers among 100 measurements (16.00%)
  2 (2.00%) low severe
  1 (1.00%) low mild
  2 (2.00%) high mild
  11 (11.00%) high severe

Benchmarking bs58 encode/100k: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 9.4s, or reduce sample count to 50.
bs58 encode/100k        time:   [92.858 ms 94.427 ms 96.228 ms]
Found 16 outliers among 100 measurements (16.00%)
  1 (1.00%) high mild
  15 (15.00%) high severe

bs58/fd encode/100k     time:   [9.5943 ms 9.8016 ms 10.060 ms]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  6 (6.00%) high severe

bs58 decode/10k         time:   [3.4021 ms 3.4739 ms 3.5602 ms]
Found 14 outliers among 100 measurements (14.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild
  10 (10.00%) high severe

Benchmarking bs58/fd decode/10k: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.3s, enable flat sampling, or reduce sample count to 60.
bs58/fd decode/10k      time:   [1.0291 ms 1.0501 ms 1.0749 ms]
Found 14 outliers among 100 measurements (14.00%)
  14 (14.00%) high severe

bs58 decode/100k        time:   [34.136 ms 34.794 ms 35.557 ms]
Found 10 outliers among 100 measurements (10.00%)
  10 (10.00%) high severe

bs58/fd decode/100k     time:   [10.219 ms 10.442 ms 10.714 ms]
Found 11 outliers among 100 measurements (11.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  9 (9.00%) high severe
```
