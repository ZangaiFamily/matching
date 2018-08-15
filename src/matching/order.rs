use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Debug)]
pub struct Order {
	pub price: Decimal,
	pub volume: Decimal,
	filled: Decimal,
}

impl Order {
	pub fn new(price: &str, volume: &str) -> Order {
		Order {
			price: Decimal::from_str(price).unwrap(),
			volume: Decimal::from_str(volume).unwrap(),
			filled: Decimal::new(0, 0)
		}
	}
}
