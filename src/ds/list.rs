use std::cmp::Ord;
use std::collections::HashMap;
use std::cmp::Ordering;
use ds::WithId;

#[derive(Debug)]
pub struct List<T: WithId> {
	map: HashMap<u64, Node<T>>,
	head: Option<u64>,
	tail: Option<u64>,
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
			head: None,
			tail: None,
			len: 0
		}
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.len
	}

	#[inline]
	pub fn front(&self) -> Option<&T> {
		self.head.and_then(|id| self.map.get(&id).map(|x| &x.data))
	}

	#[inline]
	pub fn front_mut(&mut self) -> Option<&mut T> {
		self.head.and_then(move |id| self.map.get_mut(&id).map(|x| &mut x.data))
	}

	#[inline]
	pub fn push_back(&mut self, elem: T) {
		let id = elem.id();
		if self.len == 0 {
			self.map.insert(id, Node{
				prev: None,
				next: None,
				data: elem
			});
			self.head = Some(id);
			self.tail = Some(id);
		} else {
			let tail_id = self.tail.unwrap();
			self.map.get_mut(&tail_id).map(|x| x.next = Some(id));
			self.map.insert(id, Node{
				prev: Some(tail_id),
				next: None,
				data: elem
			});
			self.tail = Some(id);
		}
		self.len += 1;
	}

	#[inline]
	pub fn remove(&mut self, id: u64) {
		let (prev, next) = {
			let node = &self.map[&id];
			(node.prev, node.next)
		};
		match (prev, next) {
			(None, None) => {
				self.head = None;
				self.tail = None;
			},
			(None, Some(next_id)) => {
				self.head = Some(next_id);
				self.map.get_mut(&next_id).map(|x| x.prev = None);
			},
			(Some(prev_id), None) => {
				self.tail = Some(prev_id);
				self.map.get_mut(&prev_id).map(|x| x.next = None);
			},
			(Some(prev_id), Some(next_id)) => {
				self.map.get_mut(&prev_id).map(|x| x.next = Some(next_id));
				self.map.get_mut(&next_id).map(|x| x.prev = Some(prev_id));
			}
		}
		self.len -= 1;
		self.map.remove(&id);
	}

	#[inline]
	pub fn pop_front(&mut self) {
		let head = self.head.unwrap();
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
	assert_eq!(list.head, Some(1));
	assert_eq!(list.tail, Some(1));
	list.map.get(&list.head.unwrap()).map(|x| assert!(x.prev.is_none()));
	list.map.get(&list.head.unwrap()).map(|x| assert!(x.next.is_none()));
	list.push_back(Order{id: 2});
	assert_eq!(list.len(), 2);
	assert_eq!(list.head, Some(1));
	assert_eq!(list.tail, Some(2));
	list.map.get(&list.head.unwrap()).map(|x| assert!(x.prev.is_none()));
	list.map.get(&list.head.unwrap()).map(|x| assert!(x.next.is_some()));
	list.map.get(&list.tail.unwrap()).map(|x| assert!(x.prev.is_some()));
	list.map.get(&list.tail.unwrap()).map(|x| assert!(x.next.is_none()));
}

#[test]
fn test_pop_front() {
	let mut list = List::new();
	list.push_back(Order{id: 1});
	list.push_back(Order{id: 2});
	list.push_back(Order{id: 3});
	list.pop_front();

	assert_eq!(list.len(), 2);
	assert_eq!(list.head, Some(2));
}

#[test]
fn test_remove() {
	//(None, None)
	let mut list = List::new();
	list.push_back(Order{id: 1});

	list.remove(1);
	assert_eq!(list.len(), 0);
	assert_eq!(list.map.len(), 0);
	assert_eq!(list.head, None);
	assert_eq!(list.tail, None);

	//(Some, Some)
	let mut list = List::new();
	list.push_back(Order{id: 1});
	list.push_back(Order{id: 2});
	list.push_back(Order{id: 3});

	list.remove(2);
	assert_eq!(list.len(), 2);
	assert_eq!(list.map.len(), 2);
	assert_eq!(list.head, Some(1));
	assert_eq!(list.tail, Some(3));
	list.map.get(&list.head.unwrap()).map(|x| assert!(x.prev.is_none()));
	list.map.get(&list.head.unwrap()).map(|x| assert!(x.next.is_some()));
	list.map.get(&list.head.unwrap()).and_then(|x| x.next).map(|x| assert_eq!(x, 3));
	list.map.get(&list.tail.unwrap()).map(|x| assert!(x.next.is_none()));
	list.map.get(&list.tail.unwrap()).map(|x| assert!(x.prev.is_some()));
	list.map.get(&list.tail.unwrap()).and_then(|x| x.prev).map(|x| assert_eq!(x, 1));

	//(Some, None)
	let mut list = List::new();
	list.push_back(Order{id: 1});
	list.push_back(Order{id: 2});
	list.push_back(Order{id: 3});

	list.remove(3);
	assert_eq!(list.len(), 2);
	assert_eq!(list.map.len(), 2);
	assert_eq!(list.head, Some(1));
	assert_eq!(list.tail, Some(2));
	list.map.get(&list.head.unwrap()).map(|x| assert!(x.prev.is_none()));
	list.map.get(&list.head.unwrap()).map(|x| assert!(x.next.is_some()));
	list.map.get(&list.head.unwrap()).and_then(|x| x.next).map(|x| assert_eq!(x, 2));
	list.map.get(&list.tail.unwrap()).map(|x| assert!(x.next.is_none()));
	list.map.get(&list.tail.unwrap()).map(|x| assert!(x.prev.is_some()));
	list.map.get(&list.tail.unwrap()).and_then(|x| x.prev).map(|x| assert_eq!(x, 1));
}
