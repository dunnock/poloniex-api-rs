use super::book::{Book, BookAccounting};


#[derive(Clone, Debug, PartialEq)]
pub struct BookStats {
  pub max_sell: f32,
  pub min_buy: f32,
  pub sum_sell: f32,
  pub sum_buy: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BookWithStats {
  book: Book,
  stats: BookStats
}

// BookStats operations
impl BookStats {
  pub fn new() -> BookStats {
    BookStats {
      max_sell: 0.0,
      min_buy: 0.0,
      sum_sell: 0.0,
      sum_buy: 0.0
    }
  }
}

// BookWithStats operations
impl BookWithStats {
  pub fn new(book: Book) -> BookWithStats {
    BookWithStats {
      book,
      stats: BookStats::new()
    }
  }
}


impl BookAccounting for BookWithStats {
  fn update_sell(&mut self, rate: String, amount: f32) -> Option<f32> {
    self.book.sell.insert(rate, amount)
  }
  fn update_buy(&mut self, rate: String, amount: f32) -> Option<f32> {
    self.book.buy.insert(rate, amount)
  }
  fn book_ref(&self) -> &Book {
    &self.book
  }
}
