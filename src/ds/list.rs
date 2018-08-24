use std::cmp::Ord;
use std::collections::HashMap;
use std::cmp::Ordering;
use ds::WithId;

#[derive(Debug)]
pub struct List<T: WithId> {
	map: HashMap<u64, Node<T>>,
	head: u64,
	tail: u64,
	len: usize
}

#[derive(Debug)]
pub struct Node<T> {
	prev: Option<u64>,
	next: Option<u64>,
	data: T
}

impl<T: WithId> List<T> {
	pub fn new() -> List<T> {
		List {
			map: HashMap::new(),
			head: 0,
			tail: 0,
			len: 0
		}
	}

	pub fn len(&self) -> usize {
		self.len
	}

	pub fn front(&self) -> Option<&T> {
		self.map.get(&self.head).map(|x| &x.data)
	}

	pub fn front_mut(&mut self) -> Option<&mut T> {
		self.map.get_mut(&self.head).map(|x| &mut x.data)
	}

	pub fn push_back(&mut self, elem: T) {
		let id = elem.id();
		if id == 0 {
			panic!("id cannot be 0");
		}
		if self.len == 0 {
			self.map.insert(id, Node{
				prev: None,
				next: None,
				data: elem
			});
			self.head = id;
			self.tail = id;
		} else {
			self.map.get_mut(&self.tail).map(|x| x.next = Some(id));
			self.map.insert(id, Node{
				prev: Some(self.tail),
				next: None,
				data: elem
			});
			self.tail = id;
		}
		self.len += 1;
	}

	pub fn remove(&mut self, id: u64) {
		let (prev, next) = {
			let node = &self.map[&id];
			(node.prev, node.next)
		};
		match (prev, next) {
			(None, None) => {
				self.head = 0;
				self.tail = 0;
				self.len = 0;
			},
			(None, Some(next_id)) => {
				self.head = next_id;
				self.map.get_mut(&next_id).map(|x| x.prev = None);
				self.len -= 1;
			},
			(Some(prev_id), None) => {
				self.tail = prev_id;
				self.map.get_mut(&prev_id).map(|x| x.next = None);
				self.len -= 1;
			},
			(Some(prev_id), Some(next_id)) => {
				self.map.get_mut(&prev_id).map(|x| x.next = Some(next_id));
				self.map.get_mut(&next_id).map(|x| x.prev = Some(prev_id));
				self.len -= 1;
			}
		}
	}

	pub fn pop_front(&mut self) {
		let head = self.head;
		self.remove(head)
	}
}

#[derive(Debug)]
struct Order {
	id: u64
}

impl WithId for Order {
	fn id(&self) -> u64 {
		self.id
	}
}

#[test]
fn test_push_back() {
	let mut list = List::new();
	list.push_back(Order{id: 1});
	assert_eq!(list.len(), 1);
	assert_eq!(list.head, 1);
	assert_eq!(list.tail, 1);
	list.map.get(&list.head).map(|x| assert!(x.prev.is_none()));
	list.map.get(&list.head).map(|x| assert!(x.next.is_none()));
	list.push_back(Order{id: 2});
	assert_eq!(list.len(), 2);
	assert_eq!(list.head, 1);
	assert_eq!(list.tail, 2);
	list.map.get(&list.head).map(|x| assert!(x.prev.is_none()));
	list.map.get(&list.head).map(|x| assert!(x.next.is_some()));
	list.map.get(&list.tail).map(|x| assert!(x.prev.is_some()));
	list.map.get(&list.tail).map(|x| assert!(x.next.is_none()));
}

#[test]
fn test_pop_front() {
	let mut list = List::new();
	list.push_back(Order{id: 1});
	list.push_back(Order{id: 2});
	list.push_back(Order{id: 3});
	list.pop_front();

	assert_eq!(list.len(), 2);
	assert_eq!(list.head, 2);
}

#[test]
fn test_remove() {
	//(None, None)
	let mut list = List::new();
	list.push_back(Order{id: 1});

	list.remove(1);
	assert_eq!(list.len(), 0);
	assert_eq!(list.head, 0);
	assert_eq!(list.tail, 0);

	//(Some, Some)
	let mut list = List::new();
	list.push_back(Order{id: 1});
	list.push_back(Order{id: 2});
	list.push_back(Order{id: 3});

	list.remove(2);
	assert_eq!(list.len(), 2);
	assert_eq!(list.head, 1);
	assert_eq!(list.tail, 3);
	list.map.get(&list.head).map(|x| assert!(x.prev.is_none()));
	list.map.get(&list.head).map(|x| assert!(x.next.is_some()));
	list.map.get(&list.head).and_then(|x| x.next).map(|x| assert_eq!(x, 3));
	list.map.get(&list.tail).map(|x| assert!(x.next.is_none()));
	list.map.get(&list.tail).map(|x| assert!(x.prev.is_some()));
	list.map.get(&list.tail).and_then(|x| x.prev).map(|x| assert_eq!(x, 1));

	//(Some, None)
	let mut list = List::new();
	list.push_back(Order{id: 1});
	list.push_back(Order{id: 2});
	list.push_back(Order{id: 3});

	list.remove(3);
	assert_eq!(list.len(), 2);
	assert_eq!(list.head, 1);
	assert_eq!(list.tail, 2);
	list.map.get(&list.head).map(|x| assert!(x.prev.is_none()));
	list.map.get(&list.head).map(|x| assert!(x.next.is_some()));
	list.map.get(&list.head).and_then(|x| x.next).map(|x| assert_eq!(x, 2));
	list.map.get(&list.tail).map(|x| assert!(x.next.is_none()));
	list.map.get(&list.tail).map(|x| assert!(x.prev.is_some()));
	list.map.get(&list.tail).and_then(|x| x.prev).map(|x| assert_eq!(x, 1));
}
