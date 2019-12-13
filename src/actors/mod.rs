pub mod book;

pub use book::Accountant;

#[cfg(not(target_arch="wasm32"))]
pub mod logger;
#[cfg(not(target_arch="wasm32"))]
pub use logger::Logger;

use crate::error::PoloError;

pub trait Processor {
    fn process_message(&mut self, msg: String) -> Result<(), PoloError>;
}
