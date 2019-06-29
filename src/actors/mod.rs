pub mod book;
pub mod logger;

use bus::BusReader;
use crate::error::PoloError;

pub type Accountant = book::Accountant;
pub type Logger = logger::Logger;

pub type ActorResult = Result<u32,PoloError>;

pub trait Actor: Send + Clone {
  fn process_message(&mut self, msg: String) -> Result<(),PoloError>;

  fn listen(&mut self, mut channel: BusReader<Option<String>>) -> ActorResult {
    let mut counter = 0;
    loop {
      // TODO: maybe send reference, not String?
      let msg = channel.recv().unwrap();
      counter = counter + 1;
      if let Some(text) = msg {
        self.process_message(text)
          .unwrap_or_else(|err| println!("Error processing message {:?}", err));
      } else {
        break;
      }
    };
    Ok(counter)
  }
}