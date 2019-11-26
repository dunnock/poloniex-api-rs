extern crate test;

/** 
 ** TESTS TESTS TESTS
 **/

#[cfg(test)]
mod tests {
  use crate::data::messages::BookUpdate;
  use std::str::FromStr;
  use test::Bencher;
  use json;

  #[bench]
  fn json_read_order_updates(b: &mut Bencher) {
    let order = r#"[189,4811424,[["o",1,"0.12906425","0.02691207"],["t","714116",0,"0.12906425","0.05946471",1504163848]]]"#;
    b.iter(|| BookUpdate::from_str(&order));
  }

  #[bench]
  fn json_read_init_update(b: &mut Bencher) {
    let order = r#"[189, 5130995, [["i", {"currencyPair": "BTC_BCH", "orderBook": [{"0.13161901": 0.23709568, "0.13164313": "0.17328089"}, {"0.13169621": 0.2331}]}]]]"#;
    b.iter(|| BookUpdate::from_str(&order));
  }

  #[bench]
  fn json_read_order_updates_json(b: &mut Bencher) {
    let order = r#"[189,4811424,[["o",1,"0.12906425","0.02691207"],["t","714116",0,"0.12906425","0.05946471",1504163848]]]"#;
    b.iter(|| json::parse(order));
  }
}