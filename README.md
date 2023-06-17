# regex-bench-rs

Created by @code-ape to try and help @BurntSushi with issue: https://github.com/rust-lang/regex/issues/1013

Tested with rust version: `1.70.0`

This allows for testing of thread throughput for `memchr` with thread pinning to rule out OS scheduler issues.

Example useage:

```bash
TARGET_CPU_CORES="0,1,2"
NUM_SECONDS=30
cargo run --release $TARGET_CPU_CORES $NUM_SECONDS
#     Finished release [optimized] target(s) in 0.03s
#      Running `target/release/regex-bench-rs 0,1,2 30`
# cpu cores available = [0, 1, 2, 3, 4, 5, 6, 7]
# core #2, cycles = 289000, secs = 30.114256, cycles/sec = 9596.784
# core #1, cycles = 287000, secs = 30.176537, cycles/sec = 9510.7
# core #0, cycles = 278000, secs = 30.216137, cycles/sec = 9200.382
# total 30.217348
```
