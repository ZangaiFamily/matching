use std::cmp::Ordering;
use super::*;

#[derive(Debug)]
pub struct BidOrder {
	pub order: Order
}

impl SidedOrder for BidOrder {
	fn new(order: Order) -> BidOrder {
		BidOrder {
			order: order
		}
	}
}

impl Ord for BidOrder {
	fn cmp(&self, other: &BidOrder) -> Ordering {
		self.order.price.cmp(&other.order.price)
	}
}

impl PartialOrd for BidOrder {
	fn partial_cmp(&self, other: &BidOrder) -> Option<Ordering> {
		self.order.price.partial_cmp(&other.order.price)
	}
}

impl PartialEq for BidOrder {
	fn eq(&self, other: &BidOrder) -> bool {
		self.order.price == other.order.price
	}
}

impl Eq for BidOrder {}

#[test]
fn higher_price_should_be_greater() {
	let order_a = BidOrder::new(Order::new("1", "1"));
	let order_b = BidOrder::new(Order::new("2", "1"));

	assert!(order_a < order_b);
}
