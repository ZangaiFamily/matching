
use super::*;

pub struct Market {
	ask_order_book: OrderBook<AskOrder>,
	bid_order_book: OrderBook<BidOrder>
}

enum OrderSide {
	Ask,
	Bid
}

enum OrderType {
	Limit,
	Market
}

impl Market {
	fn new() -> Market {
		Market {
			ask_order_book: OrderBook::new(),
			bid_order_book: OrderBook::new()
		}
	}
}

#[test]
fn test_ok() {
	let market = Market::new();
	assert!(true);
}
