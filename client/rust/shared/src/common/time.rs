use std::time::SystemTime;

#[allow(dead_code)]
pub(crate) struct UnixTimestamp {
}

impl UnixTimestamp {
    #[allow(dead_code)]
    pub(crate) fn now() -> u64 {
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
    }
}
