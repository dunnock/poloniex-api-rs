use std::collections::VecDeque;
use std::fmt::Debug;
use time::Timespec;

#[derive(Clone, Debug, PartialEq)]
pub struct Timeseries<D: Debug> {
    pub data: VecDeque<D>,
    timestamps: VecDeque<Timespec>,
}

pub trait WithTime {
    fn get_time(&self) -> Timespec;
}

impl<D: WithTime + Debug> Default for Timeseries<D> {
    fn default() -> Self {
        Timeseries {
            data: VecDeque::with_capacity(1000),
            timestamps: VecDeque::with_capacity(1000),
        }
    }
}

impl<D: WithTime + Debug> Timeseries<D> {
    pub fn add(&mut self, rec: D) {
        let timestamp = rec.get_time();
        self.data.push_front(rec);
        self.timestamps.push_front(timestamp);
    }

    pub fn drain_until(&mut self, until: Timespec) {
        //    let mut drain: Vec<D> = Vec::with_capacity(100);
        while let Some(timestamp) = self.timestamps.pop_back() {
            if timestamp < until {
                if self.data.pop_back().is_none() {
                    panic!(
                        "Timeseries::drain_until data and timestamp collections mismatch {:?} {:?}",
                        self.data, self.timestamps
                    );
                }
            } else {
                self.timestamps.push_back(timestamp);
                break;
            }
        }
        //    drain
    }

    pub fn vec_after(&self, after: Timespec) -> Vec<&D> {
        let mut items = Vec::new();
        let mut dataiter = self.data.iter();
        for timestamp in self.timestamps.iter() {
            if *timestamp > after {
                if let Some(item) = dataiter.next() {
                    items.push(item);
                } else {
                    panic!(
                        "Timeseries::vec_after data and timestamp collections mismatch {:?} {:?}",
                        self.data, self.timestamps
                    );
                }
            } else {
                break;
            }
        }
        items
    }
}
