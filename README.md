# r8brain-rs

rust interface to the r8brain free resampler: https://github.com/avaneev/r8brain-free-src

## Example usage

```rust
use r8brain_rs::{PrecisionProfile, Resampler};

fn test() {
    let mut resampler = Resampler::new(48000.0, 96000.0, 4096, 2.0, PrecisionProfile::Bits24);

    let input = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
    let mut output = [0.0; 128];
    let output_len = resampler.process(&input, &mut output);
    let resampled = &output[..output_len];

    // do not be alarmed if you have to call .process() many times
    // before a non-empty slice is yielded. This is normal.
}
```

## Performance

On Macbook Pro M1, single threaded, downsampling 2:1 (like 96k to 48k).

```
   Compiling r8brain-rs v0.1.0 (r8brain-rs)
    Finished bench [optimized] target(s) in 1.13s
     Running unittests (target/release/deps/r8brain_rs-af4e593fafe2edbe)

running 3 tests
test tests::test_resampler_basic ... ignored
test tests::bench_performance_16 ... bench:         717 ns/iter (+/- 3) = 234 MB/s
test tests::bench_performance_24 ... bench:         817 ns/iter (+/- 7) = 205 MB/s

test result: ok. 0 passed; 0 failed; 1 ignored; 2 measured; 0 filtered out; finished in 0.84s
```