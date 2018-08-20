
use super::*;
use std::cmp;

pub fn consume_limit_orders (order: &mut Order, order_book: &mut OrderBook) {
	let price_requirement = order_book.price_requirement;
	while !order.is_fulfilled() {
		match order_book.limit_orders.peek_mut() {
			Some(mut top_order) => {
				let ordering = order.price.cmp(&top_order.price);
				if ordering == price_requirement || ordering == cmp::Ordering::Equal {
					subtract_volume(order, &mut top_order);
				} else {
					break
				}
			},
			None => break
		}

		//order_book.limit_orders.peek().map(|top|
		//	match top.is_fulfilled() {
		//		true => Some(true),
		//		false => None
		//	}
		//).and_then(|_| order_book.limit_orders.pop());
		if order_book.limit_orders.peek().map_or(false, |top| top.is_fulfilled()) {
			order_book.limit_orders.pop();
		}
	}
}

pub fn consume_market_orders (order: &mut Order, order_book: &mut OrderBook) {
	while !order.is_fulfilled() {
		match order_book.market_orders.front_mut() {
			Some(mut top_order) => {
				subtract_volume(order, &mut top_order);
			}
			None => break
		}
		if order_book.market_orders.front().map_or(false, |top| top.is_fulfilled()) {
			order_book.market_orders.pop_front();
		}
	}
}

pub fn start_match (mut order: Order, market: &mut Market) {
	let (ref mut book, ref mut counter_book) = match order.side {
		OrderSide::Ask => (&mut market.ask_book, &mut market.bid_book),
		OrderSide::Bid => (&mut market.bid_book, &mut market.ask_book)
	};
	match order.kind {
		OrderKind::Limit => {
			consume_limit_orders(&mut order, counter_book);
			consume_market_orders(&mut order, counter_book)
		},
		OrderKind::Market => {
			consume_limit_orders(&mut order, counter_book)
		}
	}

	if !order.is_fulfilled() {
		println!("fulfilled");
		book.add_order(order);
	}
}

pub fn subtract_volume(order_a: &mut Order, order_b: &mut Order) -> (bool, bool) {
	let min_volume = cmp::min(order_a.volume_remained(), order_b.volume_remained());
	order_a.filled += min_volume;
	order_b.filled += min_volume;
	(order_a.is_fulfilled(), order_b.is_fulfilled())
	// broadcast trade
}

#[test]
fn test_subtract_volume() {
	let mut order_a = Order::new(1, "1", "2", OrderKind::Limit, OrderSide::Bid);
	let mut order_b = Order::new(2, "1", "3", OrderKind::Limit, OrderSide::Ask);

	let (a_fulfilled, b_fulfilled) = subtract_volume(&mut order_a, &mut order_b);
	assert_eq!(order_a.filled, Decimal::new(2, 0));
	assert_eq!(order_b.filled, Decimal::new(2, 0));
	assert!(a_fulfilled);
}

#[test]
fn test_add_orders() {
	let mut market = Market::new(1);

	let order_a = Order::new(1, "1", "1", OrderKind::Limit, OrderSide::Bid);
	let order_b = Order::new(2, "2", "1", OrderKind::Limit, OrderSide::Bid);

	start_match(order_a, &mut market);
	start_match(order_b, &mut market);

	let order_c = Order::new(3, "3", "1", OrderKind::Market, OrderSide::Bid);
	let order_d = Order::new(4, "4", "1", OrderKind::Market, OrderSide::Bid);

	start_match(order_c, &mut market);
	start_match(order_d, &mut market);

	assert_eq!(market.bid_book.limit_orders.len(), 2);
	assert_eq!(market.bid_book.market_orders.len(), 2);
}

#[test]
fn test_matching_limit_order() {
	let mut market = Market::new(1);

	let order_a = Order::new(1, "1", "1", OrderKind::Limit, OrderSide::Bid);
	let order_b = Order::new(2, "2", "1", OrderKind::Limit, OrderSide::Bid);
	let order_c = Order::new(3, "3", "1", OrderKind::Limit, OrderSide::Bid);
	let order_d = Order::new(4, "4", "1", OrderKind::Limit, OrderSide::Bid);

	market.add_order(order_a);
	market.add_order(order_b);
	market.add_order(order_c);
	market.add_order(order_d);

	let order = Order::new(5, "3", "3", OrderKind::Limit, OrderSide::Ask);

	start_match(order, &mut market);

	assert_eq!(market.ask_book.limit_orders.peek().unwrap().id, 5);
	assert_eq!(market.bid_book.limit_orders.peek().unwrap().id, 2);
	assert_eq!(market.ask_book.limit_orders.peek().unwrap().filled, Decimal::new(2, 0));
	assert_eq!(market.bid_book.limit_orders.len(), 2);

	let mut market = Market::new(1);

	let order_a = Order::new(1, "1", "1", OrderKind::Limit, OrderSide::Bid);
	let order_b = Order::new(2, "2", "1", OrderKind::Limit, OrderSide::Bid);
	let order_c = Order::new(3, "3", "1", OrderKind::Limit, OrderSide::Bid);
	let order_d = Order::new(4, "4", "1", OrderKind::Limit, OrderSide::Bid);

	market.add_order(order_a);
	market.add_order(order_b);
	market.add_order(order_c);
	market.add_order(order_d);

	let order = Order::new(5, "3", "1", OrderKind::Limit, OrderSide::Ask);

	start_match(order, &mut market);

	assert_eq!(market.bid_book.limit_orders.peek().unwrap().id, 3);
	assert_eq!(market.ask_book.limit_orders.peek(), None);
	assert_eq!(market.bid_book.limit_orders.len(), 3);
}

#[test]
fn test_matching_market_order() {
	let mut market = Market::new(1);

	let order_a = Order::new(1, "1", "1", OrderKind::Market, OrderSide::Bid);
	let order_b = Order::new(2, "1", "1", OrderKind::Market, OrderSide::Bid);
	let order_c = Order::new(3, "1", "1", OrderKind::Market, OrderSide::Bid);
	let order_d = Order::new(4, "1", "1", OrderKind::Market, OrderSide::Bid);

	market.add_order(order_a);
	market.add_order(order_b);
	market.add_order(order_c);
	market.add_order(order_d);

	let order = Order::new(5, "3", "3", OrderKind::Limit, OrderSide::Ask);

	start_match(order, &mut market);

	assert_eq!(market.bid_book.market_orders.front().unwrap().id, 4);
	assert_eq!(market.bid_book.market_orders.len(), 1);
	assert_eq!(market.ask_book.limit_orders.peek(), None);

	let order = Order::new(5, "3", "3", OrderKind::Market, OrderSide::Ask);

	start_match(order, &mut market);

	assert_eq!(market.bid_book.market_orders.front().unwrap().id, 4);
	assert_eq!(market.bid_book.market_orders.len(), 1);
	assert_eq!(market.ask_book.market_orders.front().unwrap().id, 5);
}

#[test]
fn test_matching_limit_order_then_market_other() {
	let mut market = Market::new(1);

	let order_a = Order::new(1, "1", "1", OrderKind::Limit, OrderSide::Bid);
	let order_b = Order::new(2, "1", "1", OrderKind::Market, OrderSide::Bid);
	let order_c = Order::new(3, "3", "1", OrderKind::Limit, OrderSide::Bid);
	let order_d = Order::new(4, "4", "1", OrderKind::Limit, OrderSide::Bid);

	market.add_order(order_a);
	market.add_order(order_b);
	market.add_order(order_c);
	market.add_order(order_d);

	let order = Order::new(5, "3", "2.5", OrderKind::Limit, OrderSide::Ask);

	start_match(order, &mut market);

	assert_eq!(market.bid_book.limit_orders.len(), 1);
	assert_eq!(market.bid_book.market_orders.len(), 1);
	assert_eq!(market.bid_book.limit_orders.peek().unwrap().id, 1);
	assert_eq!(market.bid_book.market_orders.front().unwrap().filled, Decimal::new(5, 1));
}
