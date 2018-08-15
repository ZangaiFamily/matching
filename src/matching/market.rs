
use super::*;

pub struct Market {
	id: u32,
	ask_order_book: OrderBook<AskOrder>,
	bid_order_book: OrderBook<BidOrder>
}

impl Market {
	fn new(id: u32) -> Market {
		Market {
			ask_order_book: OrderBook::new(),
			bid_order_book: OrderBook::new()
		}
	}

	fn place_order(&mut self, order: Order, side: OrderSide, _type: OrderType) {
		match side {
			OrderSide::Ask => self.ask_order_book.place_order(order, _type),
			OrderSide::Bid => self.bid_order_book.place_order(order, _type)
		}
	}
}

#[test]
fn test_ok() {
	let market = Market::new(1);
	assert!(true);
}
