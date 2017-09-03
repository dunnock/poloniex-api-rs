use ::data::messages::{BookUpdate};
use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};
use std::str::FromStr;
use bus::BusReader;
use std::sync::mpsc::RecvError;

// TODO: maybe pass reference, not String?
// TODO: convert to struct with trait
pub fn start(mut channel: BusReader<String>) -> JoinHandle<Result<(),RecvError>> {
  thread::spawn(move || 
    loop {
      let msg: String = channel.recv()?;
      println!("{:?}", BookUpdate::from_str(&msg));
    }
  )
}
