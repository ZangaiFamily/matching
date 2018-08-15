use std::collections::LinkedList;
use std::collections::BinaryHeap;
use std::cmp::Ord;
use rust_decimal::Decimal;
use super::*;

#[derive(Debug)]
pub struct OrderBook<T: Ord> {
	pub limit_orders: BinaryHeap<T>,
	pub market_orders: LinkedList<T>
}

impl<T> OrderBook<T> where T: Ord {
	pub fn new() -> OrderBook<T> {
		OrderBook {
			limit_orders: BinaryHeap::new(),
			market_orders: LinkedList::new()
		}
	}

	fn place_limit_order(&mut self, order: T) {
		self.limit_orders.push(order);
	}

	fn place_market_order(&mut self, order: T) {
		self.market_orders.push_back(order);
	}

	fn best_limit_order(&self) -> Option<&T> {
		self.limit_orders.peek()
	}
}

#[test]
fn test_best_limit_order() {
	let order_a = BidOrder::new("3", "1");
	let order_b = BidOrder::new("1", "1");
	let order_c = BidOrder::new("2", "1");
	let order_d = BidOrder::new("3", "1");

	let mut book = OrderBook::new();
	book.place_limit_order(order_a);
	book.place_limit_order(order_b);
	book.place_limit_order(order_c);
	book.place_limit_order(order_d);

	assert!(book.best_limit_order().unwrap().order.price == Decimal::new(3, 0));

	let order_a = AskOrder::new("3", "1");
	let order_b = AskOrder::new("1", "1");
	let order_c = AskOrder::new("2", "1");
	let order_d = AskOrder::new("3", "1");

	let mut book = OrderBook::new();
	book.place_limit_order(order_a);
	book.place_limit_order(order_b);
	book.place_limit_order(order_c);
	book.place_limit_order(order_d);

	assert!(book.best_limit_order().unwrap().order.price == Decimal::new(1, 0));
}
