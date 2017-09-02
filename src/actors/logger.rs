use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};
use time::get_time;

pub fn start_logger() -> (Sender<String>, JoinHandle<bool>) {
  let (tx, rx) = mpsc::channel();
  let th1 = thread::spawn(move || 
    loop {
      let msg: String = rx.recv().unwrap();
      let ts = get_time();
      println!("{}.{} {}", ts.sec, ts.nsec/1000000, msg);
    }
  );
  (tx, th1)
}
