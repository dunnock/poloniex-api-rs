use time::get_time;
use super::Processor;
use crate::error::PoloError;

#[derive(Clone)]
#[derive(Default)]
pub struct Logger;

impl Processor for Logger {
  fn process_message(&mut self, msg: String) -> Result<(),PoloError> {
    let ts = get_time();
    println!("{}.{} {}", ts.sec, ts.nsec/1_000_000, msg);
    Ok(())
  }
}
