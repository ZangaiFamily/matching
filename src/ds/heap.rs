use std::cmp::Ord;
use std::collections::HashMap;
use std::cmp::Ordering;
use ds::WithId;

#[derive(Debug)]
pub struct Heap<T: Ord + WithId> {
	vec: Vec<T>,
	map: HashMap<u64, usize>
}

impl<T: Ord + WithId> Heap<T> {
	pub fn new() -> Heap<T> {
		Heap {
			vec: Vec::new(),
			map: HashMap::new()
		}
	}

	pub fn parent(&self, idx: usize) -> usize {
		(idx - 1) / 2
	}

	pub fn left(&self, idx: usize) -> usize {
		2 * idx + 1
	}

	pub fn right(&self, idx: usize) -> usize {
		2 * idx + 2
	}

	pub fn sift_up(&mut self, from: usize) -> usize {
		let start = 0;
		let mut pos = from;

		while pos > 0 {
			let parent = self.parent(pos);
			if self.vec[pos] <= self.vec[parent] {
				break;
			}
			self.swap(pos, parent);
			pos = parent;
		}
		pos
	}

	pub fn sift_down(&mut self, from: usize) {
		let end = self.vec.len();
		let mut pos = from;
		let mut child = 2 * from + 1;
		while child < end {
			let right = child + 1;

			if right < end && !(self.vec[child] > self.vec[right]) {
				child = right;
			}

			if self.vec[pos] >= self.vec[child] {
				break;
			}
			self.swap(pos, child);
			pos = child;
			child = 2 * pos + 1;
		}
	}

	pub fn push(&mut self, elem: T) {
		let len = self.vec.len();
		self.map.insert(elem.id(), len);
		self.vec.push(elem);
		self.sift_up(len);
	}

	pub fn swap(&mut self, idx_a: usize, idx_b: usize) {
		self.map.insert(self.vec[idx_a].id(), idx_b);
		self.map.insert(self.vec[idx_b].id(), idx_a);
		self.vec.swap(idx_a, idx_b);
	}

	pub fn remove(&mut self, id: u64) {
		let len = self.vec.len();
		let idx = self.map[&id];
		self.swap(idx, len - 1);
		self.map.remove(&((len - 1) as u64));
		if idx > 0 {
			if self.vec[self.parent(idx)] < self.vec[idx] {
				self.sift_up(idx);
			} else {
				self.sift_down(idx);
			}
		}
	}

	pub fn peek(&self) -> Option<&T> {
		self.vec.get(0 as usize)
	}

	pub fn peek_mut(&mut self) -> Option<&mut T> {
		self.vec.get_mut(0 as usize)
	}

	pub fn pop(&mut self) {
		let len = self.vec.len();
		if len == 1 {
			self.vec.pop();
		} else {
			self.swap(0, len - 1);
			self.vec.pop();
			self.map.remove(&((len - 1) as u64));
			self.sift_down(0);
		}
	}

	pub fn len(&self) -> usize {
		self.vec.len()
	}
}

struct Order {
	id: u64,
	price: u64
}

impl WithId for Order {
	fn id(&self) -> u64 {
		self.id
	}
}

impl Ord for Order {
	fn cmp(&self, other: &Order) -> Ordering {
		self.price.cmp(&other.price)
	}
}

impl PartialOrd for Order {
	fn partial_cmp(&self, other: &Order) -> Option<Ordering> {
		self.price.partial_cmp(&other.price)
	}
}

impl PartialEq for Order {
	fn eq(&self, other: &Order) -> bool {
		self.price == other.price
	}
}

impl Eq for Order {}

#[test]
fn test_ord() {
	assert!(Order{id: 1, price: 10} < Order{id: 1, price: 20});
}

#[test]
fn test_swap() {
	let mut heap = Heap::new();
	heap.push(Order{id: 1, price: 10});
	heap.push(Order{id: 2, price: 20});
	assert_eq!(heap.vec[0].id(), 2);
	assert_eq!(heap.vec[1].id(), 1);
	assert_eq!(heap.map[&1], 1);
	assert_eq!(heap.map[&2], 0);
	heap.swap(0, 1);
	assert_eq!(heap.vec[0].id(), 1);
	assert_eq!(heap.vec[1].id(), 2);
	assert_eq!(heap.map[&1], 0);
	assert_eq!(heap.map[&2], 1);
}

#[test]
fn test_remove() {

}
