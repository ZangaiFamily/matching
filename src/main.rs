extern crate rust_decimal;
use rust_decimal::Decimal;

mod matching;
use matching::*;

fn main() {
	let bid_order = BidOrder::new("11", "2");
	println!("{:?}", bid_order);
}
