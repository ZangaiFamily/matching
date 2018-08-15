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

impl<T> OrderBook<T> where T: Ord + SidedOrder {
	pub fn new() -> OrderBook<T> {
		OrderBook {
			limit_orders: BinaryHeap::new(),
			market_orders: LinkedList::new()
		}
	}

	pub fn place_order(&mut self, order: Order, _type: OrderType) {
		let sided_order = T::new(order);
		match _type {
			OrderType::Limit => self.limit_orders.push(sided_order),
			OrderType::Market => self.market_orders.push_back(sided_order)
		}
	}

	fn best_limit_order(&self) -> Option<&T> {
		self.limit_orders.peek()
	}
}

#[test]
fn test_best_limit_order() {
	let order_a = Order::new("3", "1");
	let order_b = Order::new("1", "1");
	let order_c = Order::new("2", "1");
	let order_d = Order::new("3", "1");

	let mut book: OrderBook<BidOrder> = OrderBook::new();
	book.place_order(order_a, OrderType::Limit);
	book.place_order(order_b, OrderType::Limit);
	book.place_order(order_c, OrderType::Limit);
	book.place_order(order_d, OrderType::Limit);

	assert!(book.limit_orders.len() == 4);
	assert!(book.best_limit_order().unwrap().order.price == Decimal::new(3, 0));

	let order_a = Order::new("3", "1");
	let order_b = Order::new("1", "1");
	let order_c = Order::new("2", "1");
	let order_d = Order::new("3", "1");

	let mut book: OrderBook<AskOrder> = OrderBook::new();
	book.place_order(order_a, OrderType::Limit);
	book.place_order(order_b, OrderType::Limit);
	book.place_order(order_c, OrderType::Limit);
	book.place_order(order_d, OrderType::Limit);

	assert!(book.limit_orders.len() == 4);
	assert!(book.best_limit_order().unwrap().order.price == Decimal::new(1, 0));
}

#[test]
fn test_place_market_order() {
	let order = Order::new("3", "1");
	let mut book: OrderBook<AskOrder> = OrderBook::new();

	book.place_order(order, OrderType::Market);
	assert!(book.market_orders.len() == 1);
}
