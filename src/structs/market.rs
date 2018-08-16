
use super::*;

pub struct Market {
	id: u64,
	ask_book: OrderBook,
	bid_book: OrderBook
}

impl Market {
	fn new(id: u64) -> Market {
		Market {
			id: id,
			ask_book: OrderBook::new(OrderSide::Ask),
			bid_book: OrderBook::new(OrderSide::Bid)
		}
	}

	fn add_order(&mut self, order: Order) {
		match order.side {
			OrderSide::Ask => self.ask_book.add_order(order),
			OrderSide::Bid => self.bid_book.add_order(order)
		}
	}

	//fn match_order(&mut self, order: Order) {
	//	let (book, counter_book) = match order.side {
	//		OrderSide::Ask => (self.ask_book, self.bid_book),
	//		OrderSide::Bid => (self.bid_book, self.ask_book)
	//	};
	//	while counter_book.consume(order) {}
	//	if !order.is_fulfilled() {
	//		book.add_order(order);
	//	}
	//}

}

#[test]
fn test_ok() {
	let market = Market::new(1);
	assert!(true);
}
