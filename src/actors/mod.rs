pub mod book;
pub mod logger;

use bus::BusReader;
use std::thread::{self, JoinHandle};
use ::PoloError;

pub type ActorResult = Result<u32,PoloError>;

pub trait Actor: Send + Clone + 'static {
  fn process_message(&mut self, msg: String) -> Result<(),PoloError>;

  fn listen(&mut self, mut channel: BusReader<Option<String>>) -> ActorResult {
    let mut counter = 0;
    loop {
      // TODO: maybe send reference, not String?
      let msg = channel.recv().unwrap();
      counter = counter + 1;
      if let Some(text) = msg {
        self.process_message(text).unwrap();
      } else {
        break;
      }
    };
    Ok(counter)
  }

  fn subscribe(&mut self, channel: BusReader<Option<String>>) -> JoinHandle<ActorResult> {
    let mut actor = self.clone();
    thread::spawn(move || actor.listen(channel))
  }
}