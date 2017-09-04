use time::get_time;
use super::Actor;

pub struct Logger;

impl Logger {
  pub fn new() -> Logger {
    Logger { }
  }
}

impl Actor for Logger {
  fn process_message(&mut self, msg: String) -> Result<(),_> {
    let ts = get_time();
    println!("{}.{} {}", ts.sec, ts.nsec/1000000, msg);
  }
}
