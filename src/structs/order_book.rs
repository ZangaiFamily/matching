use std::collections::LinkedList;
use std::collections::BinaryHeap;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp;
use rust_decimal::Decimal;
use super::*;

#[derive(Debug)]
pub struct OrderBook {
	pub side: OrderSide,
	pub price_requirement: Ordering,
	pub limit_orders: BinaryHeap<Order>,
	pub market_orders: LinkedList<Order>
}

impl OrderBook where {
	pub fn new(side: OrderSide) -> OrderBook {
		let price_requirement = match side {
			OrderSide::Ask => Ordering::Greater,
			OrderSide::Bid => Ordering::Less
		};
		OrderBook {
			side: side,
			price_requirement: price_requirement,
			limit_orders: BinaryHeap::new(),
			market_orders: LinkedList::new()
		}
	}

	pub fn add_order(&mut self, order: Order) {
		assert!(order.side == self.side);
		match order.kind {
			OrderKind::Limit => self.limit_orders.push(order),
			OrderKind::Market => self.market_orders.push_back(order)
		}
	}

	pub fn consume(&mut self, order: &mut Order) {
		while !order.is_fulfilled() {
			match self.limit_orders.peek_mut() {
				Some(top_order) => {
					//let ordering = order.price.cmp(&top_order.price);
					//if ordering == self.price_requirement || ordering == Ordering::Equal {
					//	self.process_volume(order, &mut top_order);
					//	if top_order.is_fulfilled() { self.limit_orders.pop(); }
					//} else {
					//	break
					//}
				},
				None => {
					break;
				}
			}
		}

		while !order.is_fulfilled() {
			match self.market_orders.front_mut() {
				Some(top_order) => {

				},
				None => {
					break
				}
			}
		}
	}

	pub fn process_volume(&self, order_a: &mut Order, order_b: &mut Order) {
		let min_volume = cmp::min(order_a.volume_remained(), order_b.volume_remained());
		order_a.volume -= min_volume;
		order_b.volume -= min_volume;
		// broadcast trade
	}

}

#[test]
fn test_best_limit_order() {
	let order_a = Order::new(1, "3", "1", OrderKind::Limit, OrderSide::Bid);
	let order_b = Order::new(1, "1", "1", OrderKind::Limit, OrderSide::Bid);
	let order_c = Order::new(1, "2", "1", OrderKind::Limit, OrderSide::Bid);
	let order_d = Order::new(1, "3", "1", OrderKind::Limit, OrderSide::Bid);

	let mut book = OrderBook::new(OrderSide::Bid);
	book.add_order(order_a);
	book.add_order(order_b);
	book.add_order(order_c);
	book.add_order(order_d);

	assert!(book.limit_orders.len() == 4);
	assert!(book.limit_orders.peek().unwrap().price == Decimal::new(3, 0));

	let order_a = Order::new(1, "3", "1", OrderKind::Limit, OrderSide::Ask);
	let order_b = Order::new(1, "1", "1", OrderKind::Limit, OrderSide::Ask);
	let order_c = Order::new(1, "2", "1", OrderKind::Limit, OrderSide::Ask);
	let order_d = Order::new(1, "3", "1", OrderKind::Limit, OrderSide::Ask);

	let mut book = OrderBook::new(OrderSide::Ask);
	book.add_order(order_a);
	book.add_order(order_b);
	book.add_order(order_c);
	book.add_order(order_d);

	assert!(book.limit_orders.len() == 4);
	assert!(book.limit_orders.peek().unwrap().price == Decimal::new(1, 0));
}

#[test]
fn test_place_market_order() {
	let order = Order::new(1, "3", "1", OrderKind::Market, OrderSide::Ask);
	let mut book = OrderBook::new(OrderSide::Ask);

	book.add_order(order);
	assert!(book.market_orders.len() == 1);
}
