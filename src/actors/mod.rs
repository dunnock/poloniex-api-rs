pub mod book;

pub use book::Accountant;

#[cfg(not(target_arch="wasm32"))]
pub mod logger;
#[cfg(not(target_arch="wasm32"))]
pub use logger::Logger;

#[cfg(not(target_arch="wasm32"))]
pub(crate) use time::get_time;
#[cfg(target_os="wasi")]
pub fn get_time() -> time::Timespec {
    let ts = unsafe {
        wasi::clock_time_get(
            wasi::CLOCKID_MONOTONIC, 1, // precision... seems ignored though?
        )
        .unwrap()
    };
    time::Timespec::new((ts / 1_000_000_000) as i64, (ts % 1_000_000_000) as i32)
}

use crate::error::PoloError;

pub trait Processor {
    fn process_message(&mut self, msg: String) -> Result<(), PoloError>;
}
