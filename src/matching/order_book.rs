use std::collections::LinkedList;
use std::collections::BinaryHeap;

pub struct OrderBook<T> {
	limit_orders: BinaryHeap<T>,
	market_orders: LinkedList<T>
}

impl<T> OrderBook<T> {
	fn place_market_order(&mut self, order: T) {
		self.market_orders.push_back(order);
	}
}
