use json::{self, JsonValue};
use std::convert::TryFrom;
use std::str::FromStr;
use super::book::{Book, TradePairs, Record};
use ::PoloError;

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

impl Expect<u64> for JsonValue {
  type Error = PoloError;
  fn expect(&self, msg: &str) -> Result<u64, Self::Error> {
    let err = || PoloError::wrong_data(format!("{}: expected float got {}", msg, self));
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
  type Error = PoloError;
  fn try_from(v: &'a JsonValue) -> Result<Self, Self::Error> {
    if v.len() != 6 {
      return Err(PoloError::wrong_data(format!("trade record does not have 6 items {:?}", v)));
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
 *  let val = match json::parse(r#"["o",0,"0.12900000","1.03377186"]"#) {
 *     serde_json::Array(vec) => vec,
 *     _ => Err(())
 *  };
 *  let records:: BookRecord = BookRecord::try_from(&val)
 **/

impl<'a> TryFrom<&'a JsonValue> for BookRecord {
  type Error = PoloError;
  fn try_from(v: &'a JsonValue) -> Result<Self, Self::Error> {
    if v.len() != 4 {
      return Err(PoloError::wrong_data(format!("book record does not have 4 items {:?}", v)));
    }

    let rate: f64 = v[2].expect("book record rate")?;
    let amount: f64 = v[3].expect("book record amount")?;

    Ok(Self {rate, amount})
  }
}


/**
 * RecordUpdate enum conversion traits
 * use: 
 *  let val = match json::parse(r#"["o",0,"0.12900000","1.03377186"]"#);
 *  let records:: RecordUpdate = RecordUpdate::try_from(&val)
 **/

impl<'a> TryFrom<&'a JsonValue> for RecordUpdate {
  type Error = PoloError;
  
  fn try_from(v: &'a JsonValue) -> Result<Self, Self::Error> {
    let err = |msg| Err(PoloError::wrong_data(format!("book update record {} {:?}", msg, v)));
    if v.len() < 2 {
      return err("has less than 2 items");
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
      Some("i") => {
        let book = Book::try_from(&v[1])?;
        Ok(RecordUpdate::Initial(book))
      },
      _ => err("has unknown type")
    }
  }
}



/**
 * Book conversion traits
 * use: 
 *  let val = match json::parse(r#"["i", {"currencyPair": "BTC_BCH", "orderBook": [{0.13161901: "0.23709568", 0.13164313: "0.17328089"}, {0.13169621: "0.2331"}]]"#) {
 *     json::Array(vec) => vec,
 *     _ => Err(())
 *  };
 *  let records:: Book = Book::try_from(&val)
 **/

impl<'a> TryFrom<&'a JsonValue> for Book {
  type Error = PoloError;
  fn try_from(v: &'a JsonValue) -> Result<Self, Self::Error> {
    let err = |msg| Err(PoloError::wrong_data(format!("{} {:?}", msg, v)));

    if !v.is_object() {
      return err("initial book is not object");
    }
    if v["orderBook"].len()!=2 || 
       !v["orderBook"][0].is_object() || 
       !v["orderBook"][1].is_object() 
    {
      return err("initial book orderBook bad format");
    }

    let pairs = TradePairs::try_from(&v["currencyPair"])?;

    let to_records = |obj: &JsonValue| obj.entries().map(Record::try_from).collect::<Result<_,_>>();
    let sell: Vec<Record> = to_records(&v["orderBook"][0])?;
    let buy: Vec<Record> = to_records(&v["orderBook"][1])?;

    Ok(Self {pairs, sell, buy})
  }
}

impl<'a> TryFrom<&'a JsonValue> for TradePairs {
  type Error = PoloError;
  fn try_from(v: &'a JsonValue) -> Result<Self, Self::Error> {
    let err = |msg| Err(PoloError::wrong_data(format!("{} {:?}", msg, v)));

    if !v.is_string() {
      return err("book's trade pairs is not string");
    };
    match v.as_str() {
      Some("BTC_BCH") => Ok(TradePairs::BtcBch),
      Some("BTC_ETH") => Ok(TradePairs::BtcEth),
      _ => err("unknown trade pair")
    }
  }
}

impl<'a> TryFrom<(&'a str, &'a JsonValue)> for Record {
  type Error = PoloError;
  fn try_from((srate, vamount): (&'a str, &'a JsonValue)) -> Result<Self, Self::Error> {
    let rate: f64 = srate.parse::<f64>()?;
    let amount: f64 = vamount.expect("record amount")?;
    Ok(Self { rate, amount })
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
  type Err = PoloError;
  fn from_str(order: &str) -> Result<Self, Self::Err> {
    let v = json::parse(order)?;
    BookUpdate::try_from(v)
  }
}

impl TryFrom<JsonValue> for BookUpdate {
  type Error = PoloError;
  fn try_from(v: JsonValue) -> Result<Self, Self::Error> {
    let err = |msg| Err(PoloError::wrong_data(format!("{} {:?}", msg, v)));

    if v.len() != 3 {
      return err("book update is not triple");
    }
    if !v[2].is_array() {
      return err("book update records: expected array got");
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

  #[test]
  fn json_deserialize_order_update_initial() {
    let order = r#"[189, 5130995, [["i", {"currencyPair": "BTC_BCH", "orderBook": [{"0.13161901": 0.23709568, "0.13164313": "0.17328089"}, {"0.13169621": 0.2331}]}]]]"#;
    match BookUpdate::from_str(order) {
      Err(error) => panic!("failed to process json {:?}", error),
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