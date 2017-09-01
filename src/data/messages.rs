use json::{self, JsonValue};
use std::convert::TryFrom;
use std::str::FromStr;
use super::book::Book;

// ["t","714109",1,"0.12900000","1.03377186",1504163835]
#[derive(Debug, Clone)]
pub struct TradeRecord {
  pub id: u64,
  pub tid: String,
  pub rate: f64, 
  pub amount: f64,
}

// ["o",1,"0.12774723","0.00000000"]
#[derive(Debug, Clone)]
pub struct BookRecord {
  pub rate: f64, 
  pub amount: f64,
}

#[derive(Debug, Clone)]
pub enum RecordUpdate {
  SellTotal(BookRecord),
  BuyTotal(BookRecord),
  Sell(TradeRecord),
  Buy(TradeRecord),
  Initial(Book)
}

// book update message 
// ex: [189,4811375,[["o",1,"0.12774723","0.00000000"],["t","714109",1,"0.12900000","1.03377186",1504163835]]]
#[derive(Debug, Clone)]
pub struct BookUpdate {
  pub book_id: u64, 
  pub record_id: u64,
  pub records: Vec<RecordUpdate>
}


/**
 * internal trait for serde_json::Value evaluation and conversion
**/
trait Expect<T> {
  type Error;
  fn expect(&self, msg: &str) -> Result<T, Self::Error>;
}

impl Expect<f64> for JsonValue {
  type Error = String;
  fn expect(&self, msg: &str) -> Result<f64, Self::Error> {
    let err = || format!("{}: expected float got {}", msg, self);
    if self.is_string() {
      self.as_str().ok_or_else(err)?.parse::<f64>().map_err(|err| err.to_string())
    } else {
      self.as_f64().ok_or_else(err)
    }
  }
}

impl Expect<u64> for JsonValue {
  type Error = String;
  fn expect(&self, msg: &str) -> Result<u64, Self::Error> {
    let err = || format!("{}: expected float got {}", msg, self);
    if self.is_string() {
      self.as_str().ok_or_else(err)?.parse::<u64>().map_err(|err| err.to_string())
    } else {
      self.as_u64().ok_or_else(err)
    }
  }
}

impl Expect<String> for JsonValue {
  type Error = String;
  fn expect(&self, msg: &str) -> Result<String, Self::Error> {
    let err = || format!("{}: expected string got {}", msg, self);
    Ok(String::from(self.as_str().ok_or_else(err)?))
  }
}

/**
 * TradeRecord conversion traits
 * use: 
 *  let val = match serde_json::from_str(r#"["t","714109",1,"0.12900000","1.03377186",1504163835]"#) {
 *     serde_json::Array(vec) => vec,
 *     _ => Err(())
 *  };
 *  let records:: TradeRecord = TradeRecord::try_from(&val)
 **/

impl<'a> TryFrom<&'a JsonValue> for TradeRecord {
  type Error = String;
  fn try_from(v: &'a JsonValue) -> Result<Self, Self::Error> {
    if v.len() != 6 {
      return Err(format!("trade record does not have 6 items {:?}", v));
    }

    let id: u64 = v[5].expect("trade record id")?;
    let tid: String = v[1].expect("trade record tid")?;
    let rate: f64 = v[3].expect("trade record rate")?;
    let amount: f64 = v[4].expect("trade record amount")?;

    Ok(Self {id, tid, rate, amount})
  }
}


/**
 * BookRecord conversion traits
 * use: 
 *  let val = match serde_json::from_str(r#"["o",0,"0.12900000","1.03377186"]"#) {
 *     serde_json::Array(vec) => vec,
 *     _ => Err(())
 *  };
 *  let records:: BookRecord = BookRecord::try_from(&val)
 **/

impl<'a> TryFrom<&'a JsonValue> for BookRecord {
  type Error = String;
  fn try_from(v: &'a JsonValue) -> Result<Self, Self::Error> {
    if v.len() != 4 {
      return Err(format!("book record does not have 4 items {:?}", v));
    }

    let rate: f64 = v[2].expect("book record rate")?;
    let amount: f64 = v[3].expect("book record amount")?;

    Ok(Self {rate, amount})
  }
}



/**
 * RecordUpdate enum conversion traits
 * use: 
 *  let val = match serde_json::from_str(r#"["o",0,"0.12900000","1.03377186"]"#);
 *  let records:: RecordUpdate = RecordUpdate::try_from(&val)
 **/

impl<'a> TryFrom<&'a JsonValue> for RecordUpdate {
  type Error = String;
  
  fn try_from(v: &'a JsonValue) -> Result<Self, Self::Error> {
    let err = |msg| Err(format!("book update record {} {:?}", msg, v));
    if v.len() < 3 {
      return err("has less than 3 items");
    }

    match v[0].as_str() {
      Some("o") => {
        let direction: u64 = v[1].expect("book record direction")?;
        let record = BookRecord::try_from(v)?;
        match direction {
          0 => Ok(RecordUpdate::SellTotal(record)),
          1 => Ok(RecordUpdate::BuyTotal(record)),
          _ => err("has unknown dir")
        }
      },
      Some("t") => {
        let direction: u64 = v[2].expect("book record direction")?;
        let record = TradeRecord::try_from(v)?;
        match direction {
          0 => Ok(RecordUpdate::Sell(record)),
          1 => Ok(RecordUpdate::Buy(record)),
          _ => err("has unknown dir")
        }
      },
      _ => err("has unknown type")
    }
  }
}



/**
 * BookUpdate conversion traits
 * use: 
 *  let update:: BookUpdate = BookUpdate::from_str(
 *    r#"[189,4811375,[["o",1,"0.12774723","0.00000000"],["t","714109",1,"0.12900000","1.03377186",1504163835]]]"#
 *  )
 **/

impl FromStr for BookUpdate {
  type Err = String;
  fn from_str(order: &str) -> Result<Self, Self::Err> {
    let v = json::parse(order).map_err(|err| err.to_string())?;
    Ok(BookUpdate::try_from(v)?)
  }
}

impl TryFrom<JsonValue> for BookUpdate {
  type Error = String;
  fn try_from(v: JsonValue) -> Result<Self, Self::Error> {
    if v.len() != 3 {
      return Err(format!("book update is not triple {:?}", v));
    }
    if !v[2].is_array() {
      return Err(format!("book update records: expected array got {:?}", v));
    }

    let book_id: u64 = v[0].expect("book update book_id")?;
    let record_id: u64 = v[1].expect("book update record_id")?;
    let mut records: Vec<RecordUpdate> = vec![];
    for record in v[2].members() {
      records.push(RecordUpdate::try_from(record)?)
    };

    Ok(Self { book_id, record_id, records })
  }
}


#[cfg(test)]
mod tests {
  use super::BookUpdate;
  use std::str::FromStr;
  use test::Bencher;
  use json;

  #[test]
  fn json_deserialize_order_update() {
    let order = r#"[189,4811424,[["o",1,"0.12906425","0.02691207"],["t","714116",0,"0.12906425","0.05946471",1504163848]]]"#;
    match BookUpdate::from_str(order) {
      Err(error) => panic!("failed to process json {}", error),
      _ => ()
    }
  }

  #[test]
  fn json_deserialize_order_update_err1() {
    let order = r#"[189,4811424,[["o",1,"bad","0.02691207"],["t","714116",0,"0.12906425","0.05946471",1504163848]]]"#;
    match BookUpdate::from_str(order) {
      Ok(val) => panic!("processed wrong json {:?}", val),
      _ => ()
    }
  }

  #[test]
  fn json_deserialize_order_update_err2() {
    let order = r#"[189,4811424]"#;
    match BookUpdate::from_str(order) {
      Ok(val) => panic!("processed wrong json {:?}", val),
      _ => ()
    }
  }

  #[test]
  fn json_deserialize_order_update_err3() {
    let order = r#"[189,4811424,[["f",1,"0.120000","0.02691207"],["t","714116",0,"0.12906425","0.05946471",1504163848]]]"#;
    match BookUpdate::from_str(order) {
      Ok(val) => panic!("processed wrong json {:?}", val),
      _ => ()
    }
  }


  #[test]
  fn json_deserialize_order_update_err4() {
    let order = r#"[189,4811424,[["o",3,"0.120000","0.02691207"],["t","714116",0,"0.12906425","0.05946471",1504163848]]]"#;
    match BookUpdate::from_str(order) {
      Ok(val) => panic!("processed wrong json {:?}", val),
      _ => ()
    }
  }

  #[bench]
  fn json_read_order_updates(b: &mut Bencher) {
    let order = r#"[189,4811424,[["o",1,"0.12906425","0.02691207"],["t","714116",0,"0.12906425","0.05946471",1504163848]]]"#;
    b.iter(|| BookUpdate::from_str(&order));
  }

  #[bench]
  fn json_read_order_updates_json(b: &mut Bencher) {
    let order = r#"[189,4811424,[["o",1,"0.12906425","0.02691207"],["t","714116",0,"0.12906425","0.05946471",1504163848]]]"#;
    b.iter(|| json::parse(order));
  }
}