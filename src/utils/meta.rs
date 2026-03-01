/// # Usage
///
/// ```
/// use utils::embed_metadata;
///
/// embed_metadata!(VERSION, ".version", &[50]);
/// ```
#[macro_export]
macro_rules! embed_metadata {
    ($name:ident, $section:literal, $value:expr) => {
        #[used]
        #[unsafe(link_section = $section)]
        #[unsafe(no_mangle)]
        static $name: [u8; $value.len()] = *$value;
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn embed_metadata() {
        embed_metadata!(TEST_META, ".test_section", &[1, 2, 3, 4]);

        assert_eq!(TEST_META, [1, 2, 3, 4]);

        let ptr = TEST_META.as_ptr();
        unsafe {
            assert_eq!(*ptr, 1);
        }
    }
}
