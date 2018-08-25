#![feature(test)]
extern crate rust_decimal;
use rust_decimal::Decimal;
extern crate serde_json;
use serde_json::{Value, Error};
extern crate test;

use std::fs::read_to_string;
use std::io::prelude::*;
use std::io::BufReader;

mod structs;
use structs::*;

mod matching;

mod ds;
//use ds::*;

fn main() {

}

use test::Bencher;
#[bench]
fn bench_matching(b: &mut Bencher) {
	b.iter(||{
		let mut market = Market::new(20);

		let content: String = std::fs::read_to_string("/Users/cichol/etc_btc.log").unwrap();
		let mut lines = content.lines();

		while let Some(line) = lines.next() {
			let v: Value = serde_json::from_str(line).unwrap();
			//println!("{:?}", line);
			match v["action"].as_str().unwrap() {
				"submit" => {
					let order = Order::new(
						v["order"]["id"].as_u64().unwrap(),
						v["order"]["price"].as_str().unwrap(),
						v["order"]["volume"].as_str().unwrap(),
						OrderKind::Limit,
						if v["order"]["type"] == "ask" {
							OrderSide::Ask
						} else {
							OrderSide::Bid
						}
					);
					matching::start_match(order, &mut market);
				},
				"cancel" => {
					let order = Order::new(
						v["order"]["id"].as_u64().unwrap(),
						v["order"]["price"].as_str().unwrap(),
						v["order"]["volume"].as_str().unwrap(),
						OrderKind::Limit,
						if v["order"]["type"] == "ask" {
							OrderSide::Ask
						} else {
							OrderSide::Bid
						}
					);
					market.cancel_order(order);
				},
				_ => ()
			}
		}
		println!("{:?}", market.ask_book.limit_orders.len());
		println!("{:?}", market.bid_book.limit_orders.len());
	});
}
