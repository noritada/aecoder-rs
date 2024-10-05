pub mod binding {
    #![allow(non_camel_case_types)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn aec_binding_works() {
            assert_eq!(
                (AEC_VERSION_MAJOR, AEC_VERSION_MINOR, AEC_VERSION_PATCH),
                (1, 1, 3)
            );
        }
    }
}
