use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};
use time::get_time;
use bus::BusReader;
use std::sync::mpsc::RecvError;

// maybe pass reference, not String?
// TODO: convert to struct with trait
pub fn start(mut channel: BusReader<String>) -> JoinHandle<Result<(),RecvError>> {
  thread::spawn(move || 
    loop {
      let msg: String = channel.recv()?;
      let ts = get_time();
      println!("{}.{} {}", ts.sec, ts.nsec/1000000, msg);
    }
  )
}
