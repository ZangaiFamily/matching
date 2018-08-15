use std::cmp::Ordering;
use super::Order;

#[derive(Debug)]
pub struct AskOrder {
	pub order: Order
}

impl AskOrder {
	pub fn new(price: &str, volume: &str) -> AskOrder {
		AskOrder {
			order: Order::new(price, volume)
		}
	}
}

impl Ord for AskOrder {
	fn cmp(&self, other: &AskOrder) -> Ordering {
		other.order.price.cmp(&self.order.price)
	}
}

impl PartialOrd for AskOrder {
	fn partial_cmp(&self, other: &AskOrder) -> Option<Ordering> {
		other.order.price.partial_cmp(&self.order.price)
	}
}

impl PartialEq for AskOrder {
	fn eq(&self, other: &AskOrder) -> bool {
		self.order.price == other.order.price
	}
}

impl Eq for AskOrder {}

#[test]
fn lower_price_should_be_greater() {
	let order_a = AskOrder::new("1", "1");
	let order_b = AskOrder::new("2", "1");

	assert!(order_a > order_b);
}
