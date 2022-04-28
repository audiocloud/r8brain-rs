use std::ffi::c_void;
use std::os::raw::{c_double, c_int};
use std::ptr::null_mut;
use std::slice;

const R8B_RESAMPLER_16_BIT: c_int = 0;
const R8B_RESAMPLER_16_BIT_IR: c_int = 1;
const R8B_RESAMPLER_24_BIT: c_int = 2;

#[link(name = "r8brain")]
extern "C" {
    fn r8b_create(
        src_rate: c_double,
        dst_rate: c_double,
        max_input_len: c_int,
        req_trans_band: c_double,
        resampler: c_int,
    ) -> *const c_void;

    fn r8b_delete(resampler: *const c_void);

    fn r8b_clear(resampler: *const c_void);

    fn r8b_process(resampler: *const c_void, input: *const f64, len: c_int, output: *mut *const f64) -> c_int;
}

/// The r8brain resampler. keep it around to submit samples for processing, drop when not needed
/// anymore
pub struct Resampler {
    /// maximum input slice len for [`Resampler::process()`]
    max_input_len: usize,

    /// r8brain resampler handle
    ptr: *const c_void,
}

unsafe impl Send for Resampler {}

/// Different resampling precision profiles. Note: i/o is always `f64`, but it doesn't make sense
/// to filter beyond the noise floor of realistic input.
///
/// One of:
/// - [`PrecisionProfile::Bits16`]
/// - [`PrecisionProfile::Bits16ForImpulseResponses`]
/// - [`PrecisionProfile::Bits24`]
/// - [`PrecisionProfile::Bits32`]
pub enum PrecisionProfile {
    /// 16-bit
    Bits16,

    /// 16-bit, optimized for impulse responses
    Bits16ForImpulseResponses,

    /// 24-bit
    Bits24,

    /// 32-bit inputs
    Bits32,
}

impl Resampler {
    /// Create a new resampler
    ///
    /// # Arguments
    ///
    /// * `src_rate`: source sample rate (48000.0) or ratio (1.0)
    /// * `dst_rate`: source sample rate (96000.0) or ratio (2.0)
    /// * `max_input_len`: maximum number of input samples to be submitted in one go (important!)
    /// * `req_trans_band`: transition band in percent, usually 2.0
    /// * `profile`: precision profile, see [PrecisionProfile] for options
    ///
    /// returns: Resampler
    ///
    /// # Examples
    ///
    /// ```
    /// use r8brain_rs::{PrecisionProfile, Resampler};
    ///
    /// let mut resampler = Resampler::new(48000.0, 96000.0, 4096, 2.0, PrecisionProfile::Bits24);
    ///
    /// let input = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
    /// let mut output = [0.0; 128];
    /// let output_len = resampler.process(&input, &mut output);
    /// let resampled = &output[..output_len];
    /// ```
    pub fn new(src_rate: f64, dst_rate: f64, max_input_len: usize, req_trans_band: f64, profile: PrecisionProfile) -> Self {
        use PrecisionProfile::*;

        Self {
            max_input_len,
            ptr: unsafe {
                r8b_create(
                    src_rate,
                    dst_rate,
                    max_input_len as c_int,
                    req_trans_band,
                    match profile {
                        Bits16 => R8B_RESAMPLER_16_BIT,
                        Bits16ForImpulseResponses => R8B_RESAMPLER_16_BIT_IR,
                        Bits24 | Bits32 => R8B_RESAMPLER_24_BIT,
                    },
                )
            },
        }
    }

    /// Create a new resampler with sensible defaults. Never feed it more than 8192 samples as input
    ///
    /// # Arguments
    ///
    /// * `src_rate`: source sample rate (48000.0) or ratio (1.0)
    /// * `dst_rate`: source sample rate (96000.0) or ratio (2.0)
    ///
    /// returns: Resampler resampling instance
    ///
    /// # Examples
    ///
    /// See [new]
    pub fn default(src_rate: f64, dst_rate: f64) -> Self {
        Self::new(src_rate, dst_rate, 8192, 2.0, PrecisionProfile::Bits32)
    }

    /// Clear the internal state of the resampler aside from the parameters supplied with
    /// [`Self::new()`]
    pub fn clear(&mut self) {
        unsafe {
            r8b_clear(self.ptr);
        }
    }

    /// Get the maximum configured input slice length
    pub fn max_input_len(&self) -> usize {
        self.max_input_len
    }

    ///
    ///
    /// # Arguments
    ///
    /// * `input`: slice of input samples to submit for resampling
    /// * `output`: a destination slice with enough room (!) to receive samples
    ///
    /// returns: usize number of samples generated
    ///
    /// # Examples
    ///
    /// See [`Self::new()`]
    pub fn process(&mut self, input: &[f64], output: &mut [f64]) -> usize {
        // make sure the user obeys [`Self::max_input_len`]
        assert!(input.len() <= self.max_input_len);

        unsafe {
            // prepare a pointer that r8b will overwrite
            let mut ptr: *const f64 = null_mut();

            // process the samples and wait for out_len to be populated
            let out_len = r8b_process(self.ptr, input.as_ptr(), input.len() as c_int, &mut ptr) as usize;

            // make sure we have room
            assert!(out_len <= output.len());

            // copy samples
            let slice = slice::from_raw_parts(ptr, out_len);
            output[0..out_len].copy_from_slice(slice);

            out_len
        }
    }

    /// Feed the resampler zeroes until it produces all-zeroes as output
    ///
    /// # Arguments
    ///
    /// * `output`: Storage for the ringing samples; Experimentally, should be at least 8k samples
    ///
    /// returns: usize number of samples generated
    pub fn flush(&mut self, output: &mut [f64]) -> usize {
        let zeroes = [0.0; 64];
        let mut pos = 0;
        while pos < (output.len() - 128) {
            let done = self.process(&zeroes, &mut output[pos..]);
            if done > 0 && output[pos..pos + done].iter().all(|f| *f == 0.0) {
                return pos + done;
            }
            pos += done;
        }
        pos
    }
}

impl Drop for Resampler {
    fn drop(&mut self) {
        unsafe {
            r8b_delete(self.ptr);
        }
    }
}

// #[cfg(test)]
// mod tests;
