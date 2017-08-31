use serde_json;
use std::convert::TryFrom;
use std::str::FromStr;
use serde_json::value::Value;


// ["t","714109",1,"0.12900000","1.03377186",1504163835]
pub struct TradeRecord {
  pub id: u64,
  pub tid: String,
  pub rate: f64, 
  pub amount: f64,
}

// ["o",1,"0.12774723","0.00000000"]
pub struct BookRecord {
  pub rate: f64, 
  pub amount: f64,
}

pub enum RecordUpdate {
  SellTotal(BookRecord),
  BuyTotal(BookRecord),
  Sell(TradeRecord),
  Buy(TradeRecord)
}

// book update message 
// ex: [189,4811375,[["o",1,"0.12774723","0.00000000"],["t","714109",1,"0.12900000","1.03377186",1504163835]]]
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

trait ExpectRef<T> {
  type Error;
  fn expect_ref(&self, msg: &str) -> Result<&T, Self::Error>;
}

impl Expect<f64> for Value {
  type Error = String;
  fn expect(&self, msg: &str) -> Result<f64, Self::Error> {
    let err = || format!("{}: expected float got {}", msg, self);
    match *self {
      Value::Number(ref fl) if fl.is_f64() => fl.as_f64().ok_or_else(err),
      Value::String(ref s) => s.parse::<f64>().map_err(|err| err.to_string()),
      _ => Err(err())
    }
  }
}

impl Expect<u64> for Value {
  type Error = String;
  fn expect(&self, msg: &str) -> Result<u64, Self::Error> {
    let err = || format!("{}: expected float got {}", msg, self);
    match *self {
      Value::Number(ref u) if u.is_u64() => u.as_u64().ok_or_else(err),
      Value::String(ref s) => s.parse::<u64>().map_err(|err| err.to_string()),
      _ => Err(err())
    }
  }
}

impl Expect<String> for Value {
  type Error = String;
  fn expect(&self, msg: &str) -> Result<String, Self::Error> {
    match *self {
      Value::String(ref s) => Ok(s.clone()),
      _ => Err(format!("{}: expected string got {}", msg, self))
    }
  }
}

impl ExpectRef<String> for Value {
  type Error = String;
  fn expect_ref(&self, msg: &str) -> Result<&String, Self::Error> {
    match *self {
      Value::String(ref s) => Ok(s),
      _ => Err(format!("{}: expected string got {}", msg, self))
    }
  }
}

impl ExpectRef<Vec<Value>> for Value {
  type Error = String;
  fn expect_ref(&self, msg: &str) -> Result<&Vec<Value>, Self::Error> {
    match *self {
      Value::Array(ref v) => Ok(v),
      _ => Err(format!("{}: expected array got {}", msg, self))
    }
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

impl<'a> TryFrom<&'a Vec<Value>> for TradeRecord {
  type Error = String;
  fn try_from(v: &'a Vec<Value>) -> Result<Self, Self::Error> {
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

impl<'a> TryFrom<&'a Vec<Value>> for BookRecord {
  type Error = String;
  fn try_from(v: &'a Vec<Value>) -> Result<Self, Self::Error> {
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

impl<'a> TryFrom<&'a Value> for RecordUpdate {
  type Error = String;
  
  fn try_from(v: &'a Value) -> Result<Self, Self::Error> {
    let err = |msg| Err(format!("book update record {} {:?}", msg, v));
    let arr: &Vec<Value> = v.expect_ref("book record update")?;
    if arr.len() < 3 {
      return err("has less than 3 items");
    }

    let obj: &String = arr[0].expect_ref("book record object")?;
    match obj.as_ref() {
      "o" => {
        let direction: u64 = arr[1].expect("book record direction")?;
        let record = BookRecord::try_from(arr)?;
        match direction {
          0 => Ok(RecordUpdate::SellTotal(record)),
          1 => Ok(RecordUpdate::BuyTotal(record)),
          _ => err("has unknown dir")
        }
      },
      "t" => {
        let direction: u64 = arr[2].expect("book record direction")?;
        let record = TradeRecord::try_from(arr)?;
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
    let v: Value = serde_json::from_str(order).map_err(|err| err.to_string())?;
    Ok(BookUpdate::try_from(v)?)
  }
}

impl TryFrom<Value> for BookUpdate {
  type Error = String;
  fn try_from(v: Value) -> Result<Self, Self::Error> {
    let arr: &Vec<Value> = v.expect_ref("book update")?;
    if arr.len() != 3 {
      return Err(format!("book update is not triple {:?}", arr));
    }

    let book_id: u64 = arr[0].expect("book update bookId")?;
    let record_id: u64 = arr[1].expect("book update recordId")?;
    let records_val: &Vec<Value> = arr[2].expect_ref("book update records")?;
    let mut records: Vec<RecordUpdate> = vec![];
    for record in records_val {
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

  #[test]
  fn deserialize_order_update() {
    let order = r#"[189,4811424,[["o",1,"0.12906425","0.02691207"],["t","714116",0,"0.12906425","0.05946471",1504163848]]]"#;
    match BookUpdate::from_str(order) {
      Err(error) => panic!("failed to process json {}", error),
      _ => ()
    }
  }

  #[bench]
  fn read_order_updates(b: &mut Bencher) {
    let order = r#"[189,4811424,[["o",1,"0.12906425","0.02691207"],["t","714116",0,"0.12906425","0.05946471",1504163848]]]"#;
    b.iter(|| BookUpdate::from_str(&order));
  }
}