
mod order;
pub use self::order::Order;
pub use self::order::SidedOrder;

mod bid_order;
pub use self::bid_order::BidOrder;

mod ask_order;
pub use self::ask_order::AskOrder;

mod order_book;
pub use self::order_book::OrderBook;

mod market;
pub use self::market::Market;

pub enum OrderType {
	Limit,
	Market
}

pub enum OrderSide {
	Ask,
	Bid
}
