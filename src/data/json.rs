use json::JsonValue;
use std::collections::HashMap;
use crate::error::PoloError;

/**
 * Trait for json::JsonValue evaluation and conversion
**/

pub trait Expect<T> {
  type Error;
  fn expect(&self, msg: &str) -> Result<T, Self::Error>;
}

impl Expect<f64> for JsonValue {
  type Error = PoloError;
  fn expect(&self, msg: &str) -> Result<f64, Self::Error> {
    let err = || PoloError::wrong_data(format!("{}: expected float got {}", msg, self));
    if self.is_string() {
      self.as_str().ok_or_else(err)?.parse::<f64>().map_err(PoloError::from)
    } else {
      self.as_f64().ok_or_else(err)
    }
  }
}

impl Expect<u16> for JsonValue {
  type Error = PoloError;
  fn expect(&self, msg: &str) -> Result<u16, Self::Error> {
    let err = || PoloError::wrong_data(format!("{}: expected u16 got {}", msg, self));
    if self.is_string() {
      self.as_str().ok_or_else(err)?.parse::<u16>().map_err(PoloError::from)
    } else {
      self.as_u16().ok_or_else(err)
    }
  }
}

impl Expect<u64> for JsonValue {
  type Error = PoloError;
  fn expect(&self, msg: &str) -> Result<u64, Self::Error> {
    let err = || PoloError::wrong_data(format!("{}: expected u64 got {}", msg, self));
    if self.is_string() {
      self.as_str().ok_or_else(err)?.parse::<u64>().map_err(PoloError::from)
    } else {
      self.as_u64().ok_or_else(err)
    }
  }
}

impl Expect<String> for JsonValue {
  type Error = PoloError;
  fn expect(&self, msg: &str) -> Result<String, Self::Error> {
    let err = || PoloError::wrong_data(format!("{}: expected string got {}", msg, self));
    let s = self.as_str().ok_or_else(err)?;
    Ok(String::from(s))
  }
}

impl Expect<HashMap<String, f64>> for JsonValue {
  type Error = PoloError;
  fn expect(&self, msg: &str) -> Result<HashMap<String, f64>, Self::Error> {
    if !self.is_object() {
      return Err(PoloError::wrong_data(format!("{}: expected object", msg)));
    }
    let mut hash = HashMap::new();
    for (rate, amount) in self.entries() {
      let amount: f64 = amount.expect("expected float amount")?;
      hash.insert(rate.to_owned(), amount);
    }; 
    Ok(hash)
  }
}
