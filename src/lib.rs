mod tests {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    #[test]
    fn default() {
        assert_eq!(0, unsafe { IntegerWrapper::default().Get() });
    }

    #[test]
    fn set_and_get() {
        let mut int = IntegerWrapper::default();

        unsafe { int.Set(10) };

        assert_eq!(10, unsafe { int.Get() });
    }
}
