use std::thread::{self, JoinHandle};
use time::get_time;
use bus::BusReader;

// TODO: convert to struct with trait
pub fn start(mut channel: BusReader<String>) -> JoinHandle<Result<(),String>> {
  thread::spawn(move || 
    loop {
      // TODO: maybe send reference, not String?
      let msg: String = channel.recv().map_err(|err| err.to_string())?;
      let ts = get_time();
      println!("{}.{} {}", ts.sec, ts.nsec/1000000, msg);
    }
  )
}
