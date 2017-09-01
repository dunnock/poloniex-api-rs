use ::data::messages::{BookUpdate};
use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};
use std::str::FromStr;


pub fn start_updater() -> (Sender<String>, JoinHandle<bool>) {
  let (tx, rx) = mpsc::channel();
  let th1 = thread::spawn(move || 
    loop {
      let msg: String = rx.recv().unwrap();
      println!("{:?}", BookUpdate::from_str(&msg));
    }
  );
  (tx, th1)
}
