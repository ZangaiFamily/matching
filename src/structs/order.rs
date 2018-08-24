use std::cmp::Ordering;
use rust_decimal::Decimal;
use std::str::FromStr;
use ds::{WithId};

#[derive(Debug, PartialEq)]
pub enum OrderKind {
	Limit,
	Market
}

#[derive(Debug, PartialEq)]
pub enum OrderSide {
	Ask,
	Bid
}

#[derive(Debug)]
pub struct Order {
	pub id: u64,
	pub price: Decimal,
	pub volume: Decimal,
	pub filled: Decimal,
	pub kind: OrderKind,
	pub side: OrderSide
}

impl Order {
	pub fn new(id: u64, price: &str, volume: &str, kind: OrderKind, side: OrderSide) -> Order {
		Order {
			id: id,
			price: Decimal::from_str(price).unwrap(),
			volume: Decimal::from_str(volume).unwrap(),
			filled: Decimal::new(0, 0),
			kind: kind,
			side: side
		}
	}

	pub fn is_fulfilled(&self) -> bool {
		self.volume == self.filled
	}

	pub fn volume_remained(&self) -> Decimal {
		self.volume - self.filled
	}
}

impl Ord for Order {
	fn cmp(&self, other: &Order) -> Ordering {
		match self.side {
			OrderSide::Ask => other.price.cmp(&self.price),
			OrderSide::Bid => self.price.cmp(&other.price)
		}
	}
}

impl PartialOrd for Order {
	fn partial_cmp(&self, other: &Order) -> Option<Ordering> {
		match self.side {
			OrderSide::Ask => other.price.partial_cmp(&self.price),
			OrderSide::Bid => self.price.partial_cmp(&other.price)
		}
	}
}

impl PartialEq for Order {
	fn eq(&self, other: &Order) -> bool {
		self.price == other.price
	}
}

impl Eq for Order {}

impl WithId for Order {
	fn id(&self) -> u64 {
		self.id
	}
}

#[test]
fn lower_price_should_be_greater_for_ask_order() {
	let order_a = Order::new(1, "1", "1", OrderKind::Limit, OrderSide::Ask);
	let order_b = Order::new(1, "2", "1", OrderKind::Limit, OrderSide::Ask);

	assert!(order_a > order_b);
}

#[test]
fn higher_price_should_be_greater_for_bid_order() {
	let order_a = Order::new(1, "1", "1", OrderKind::Limit, OrderSide::Bid);
	let order_b = Order::new(1, "2", "1", OrderKind::Limit, OrderSide::Bid);

	assert!(order_a < order_b);
}
