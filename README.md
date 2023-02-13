# r8brain-rs

rust interface to the r8brain free resampler: https://github.com/avaneev/r8brain-free-src

Sample rate converter designed by Aleksey Vaneev of Voxengo

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

On Macbook Pro M1, single threaded:

```
test tests::resample_192k_to_44dot1k_24bit ... bench:       2,445 ns/iter (+/- 12) = 68 MB/s
test tests::resample_44dot1k_to_192k_24bit ... bench:      11,701 ns/iter (+/- 70) = 14 MB/s
test tests::resample_48k_to_192k_24bit     ... bench:       3,221 ns/iter (+/- 19) = 52 MB/s
test tests::resample_48k_to_96k_16bit      ... bench:       1,530 ns/iter (+/- 6) = 109 MB/s
test tests::resample_48k_to_96k_24bit      ... bench:       1,771 ns/iter (+/- 7) = 94 MB/s
test tests::resample_96k_to_48k_16bit      ... bench:         711 ns/iter (+/- 6) = 236 MB/s
test tests::resample_96k_to_48k_24bit      ... bench:         819 ns/iter (+/- 10) = 205 MB/s
```
