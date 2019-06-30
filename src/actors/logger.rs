use time::get_time;
use super::Processor;
use crate::error::PoloError;

#[derive(Clone)]
pub struct Logger;

impl Logger {
  pub fn new() -> Logger {
    Logger { }
  }
}

impl Processor for Logger {
  fn process_message(&mut self, msg: String) -> Result<(),PoloError> {
    let ts = get_time();
    println!("{}.{} {}", ts.sec, ts.nsec/1000000, msg);
    Ok(())
  }
}
