use super::Processor;
use crate::error::PoloError;
use time::get_time;

#[derive(Clone, Default)]
pub struct Logger;

impl Processor for Logger {
    fn process_message(&mut self, msg: String) -> Result<(), PoloError> {
        let ts = get_time();
        println!("{}.{} {}", ts.sec, ts.nsec / 1_000_000, msg);
        Ok(())
    }
}
