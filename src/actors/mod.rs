pub mod book;
pub mod logger;

use crate::error::PoloError;

pub type Accountant = book::Accountant;
pub type Logger = logger::Logger;

pub trait Processor {
  fn process_message(&mut self, msg: String) -> Result<(),PoloError>;
}