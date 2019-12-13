pub mod actors;
pub mod data;
pub mod error;
#[cfg(feature = "ws")]
pub mod push;

#[cfg(not(target_arch="wasm32"))]
pub(crate) use time::get_time;
#[cfg(target_os="wasi")]
pub fn get_time() -> time::Timespec {
    let ts =
        wasi::wasi_unstable::clock_time_get(
            wasi::wasi_unstable::CLOCK_MONOTONIC, 1, // precision... seems ignored though?
        )
        .unwrap();
    time::Timespec::new((ts / 1_000_000_000) as i64, (ts % 1_000_000_000) as i32)
}

#[cfg(test)]
mod tests;
