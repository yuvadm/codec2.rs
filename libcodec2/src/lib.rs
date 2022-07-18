extern crate codec2_sys;

#[cfg(test)]
mod tests {
    use codec2_sys::CODEC2;
    #[test]
    fn it_works() {
        let _c: CODEC2;
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
