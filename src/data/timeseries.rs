use std::collections::VecDeque;
use time::Timespec;
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub struct Timeseries<D: Debug> {
  pub data: VecDeque<D>,
  timestamps: VecDeque<Timespec>
}

pub trait WithTime {
  fn get_time(&self) -> Timespec;
}

impl<D: WithTime+Debug> Timeseries<D> {
  pub fn new() -> Timeseries<D> {
    Timeseries {
      data: VecDeque::with_capacity(1000),
      timestamps: VecDeque::with_capacity(1000)
    }
  }

  pub fn add(&mut self, rec: D) {
    let timestamp = rec.get_time();
    self.data.push_front(rec);
    self.timestamps.push_front(timestamp);
  }

  pub fn drain_until(&mut self, until: Timespec) {
//    let mut drain: Vec<D> = Vec::with_capacity(100);
    loop {
      if let Some(timestamp) = self.timestamps.pop_back() {
        if timestamp < until {
          if let None = self.data.pop_back() {
            panic!("Timeseries::drain_until data and timestamp collections mismatch {:?} {:?}", self.data, self.timestamps);
          }
        } else {
          self.timestamps.push_back(timestamp);
          break;
        }
      } else {
          break;
      }
    };
//    drain
  }

  pub fn vec_after(&self, after: Timespec) -> Vec<&D> {
    let mut items = Vec::new();
    let mut dataiter = self.data.iter();
    let mut timeiter = self.timestamps.iter();
    loop {
      if let Some(timestamp) = timeiter.next() {
        if *timestamp > after {
          if let Some(item) = dataiter.next() {
            items.push(item);
          } else {
            panic!("Timeseries::vec_after data and timestamp collections mismatch {:?} {:?}", self.data, self.timestamps);
          }
        } else {
          break;
        }
      } else {
        break;
      }
    };
    items
  }
}
