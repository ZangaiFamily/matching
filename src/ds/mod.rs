
mod heap;
pub use self::heap::Heap;

mod list;
pub use self::list::List;

pub trait WithId {
	fn id(&self) -> u64;
}
