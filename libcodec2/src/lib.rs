extern crate codec2_sys as ffi;

pub enum Modes {
    Mode1400 = ffi::CODEC2_MODE_1400 as isize,
}

pub struct Codec2(*mut ffi::CODEC2);

impl Codec2 {
    pub fn new() -> Self {
        let mode = ffi::CODEC2_MODE_1400.try_into().unwrap();
        let c = unsafe { ffi::codec2_create(mode) };
        if c.is_null() {
            panic!("error")
        }
        Codec2(c)
    }

    pub fn destroy(&self) {
        unsafe { ffi::codec2_destroy(self.0) }
    }

    /// Encode an array of speech samples into an array of bits
    pub fn encode(self, speech: &[i16], bits: &mut [u8]) {
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
    use super::Codec2;
    #[test]
    fn it_works() {
        let _c = Codec2::new();
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
