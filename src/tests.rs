  use super::model;
  use super::push;
  use tokio_core::reactor::Core;
  use websocket::WebSocketError::{ResponseError};

  #[test]
  fn model_works() {
      let records = vec![model::Record { 
          kind: model::OrderType::Ask,
          rate: 0.001,
          amount: 10.1
      }];
      let b: model::Book = model::Book {
          pairs: model::TradePairs::BtcBch,
          records
      };
  }

  #[test]
  fn test_push_connect() {
      let mut core = Core::new().unwrap();
      let client = push::connect(&core.handle());
      match core.run(client) {
          Ok(client) => (),
          Err(error) => match error { 
              ResponseError(err) => println!("Warning! bad response {:?}", err), 
              _ => panic!("cannot construct socket {:?}", error) 
          }
      };
  }
