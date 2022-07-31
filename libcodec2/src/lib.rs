extern crate codec2_sys as ffi;

pub enum Modes {
    Mode3200 = ffi::CODEC2_MODE_3200 as isize,
    Mode2400 = ffi::CODEC2_MODE_2400 as isize,
    Mode1600 = ffi::CODEC2_MODE_1600 as isize,
    Mode1400 = ffi::CODEC2_MODE_1400 as isize,
    Mode1300 = ffi::CODEC2_MODE_1300 as isize,
    Mode1200 = ffi::CODEC2_MODE_1200 as isize,
    Mode700C = ffi::CODEC2_MODE_700C as isize,
    Mode450 = ffi::CODEC2_MODE_450 as isize,
}

pub struct Codec2(*mut ffi::CODEC2);

impl Codec2 {
    pub fn new(mode: Modes) -> Self {
        let c = unsafe { ffi::codec2_create(mode as i32) };
        if c.is_null() {
            panic!("error")
        }
        Codec2(c)
    }

    pub fn destroy(&self) {
        unsafe { ffi::codec2_destroy(self.0) }
    }

    pub fn samples_per_frame(&self) -> usize {
        unsafe { ffi::codec2_samples_per_frame(self.0).try_into().unwrap() }
    }

    pub fn bytes_per_frame(&self) -> usize {
        unsafe { ffi::codec2_bytes_per_frame(self.0).try_into().unwrap() }
    }

    /// Encode an array of speech samples into an array of bits
    pub fn encode(&self, speech: &[i16], bits: &mut [u8]) {
        let nsamples = self.samples_per_frame();
        let nbits = self.bytes_per_frame();
        if speech.len() != nsamples {
            panic!(
                "Samples buffer must be of length {}, got {}",
                nsamples,
                speech.len()
            );
        }
        if bits.len() != nbits {
            panic!(
                "Bits buffer must be of length {}, got {}",
                nbits,
                bits.len()
            );
        }
        unsafe {
            ffi::codec2_encode(self.0, bits.as_ptr() as *mut _, speech.as_ptr() as *mut _);
        }
    }
}

impl Drop for Codec2 {
    fn drop(&mut self) {
        self.destroy();
    }
}

#[cfg(test)]
mod tests {
    use super::{Codec2, Modes};
    #[test]
    fn test_codec2() {
        let c = Codec2::new(Modes::Mode1400);
        assert_eq!(c.bytes_per_frame(), 7);
        assert_eq!(c.samples_per_frame(), 320);
    }
}
