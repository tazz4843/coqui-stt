macro_rules! handle_error {
    ($fn: expr) => {
        match crate::Error::from_c_int(unsafe { $fn }) {
            Some(e) => Err(e),
            None => Ok(()),
        }
    };
}
