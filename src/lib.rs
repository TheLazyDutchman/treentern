use std::{
	cell::RefCell,
	collections::HashMap,
	hash::{BuildHasher, Hash},
	marker::Sized,
};

pub trait Intern {
	fn intern(self) -> Interned<Self>
	where
		Self: Sized;
}

pub struct Interned<T: 'static>(&'static T);

pub struct Arena<T: 'static, H = std::hash::RandomState> {
	values: RefCell<Vec<&'static T>>,
	indices: RefCell<HashMap<&'static T, Index, H>>,
}

#[derive(Clone, Copy)]
struct Index(usize);

impl<T, H: Default> Default for Arena<T, H> {
	fn default() -> Self {
		Self {
			values: RefCell::default(),
			indices: RefCell::default(),
		}
	}
}

impl<T> Arena<T> {
	pub fn new() -> Self {
		Self::default()
	}
}

impl<T, H> Arena<T, H> {
	fn get(&'static self, index: Index) -> &'static T {
		self.values.borrow()[index.0]
	}

	pub fn insert(&'static self, value: T) -> &'static T
	where
		T: Hash + Eq,
		H: BuildHasher,
	{
		if let Some(index) = self
			.indices
			.borrow()
			.get(&value)
		{
			return self.get(*index);
		}

		let index = Index(self.values.borrow().len());
		let value = Box::leak(Box::new(value));
		self.values
			.borrow_mut()
			.push(value);
		self.indices
			.borrow_mut()
			.insert(value, index);
		value
	}
}

#[cfg(test)]
mod test {
	use crate::Arena;

	#[test]
	fn insert_arena() {
		let arena = Box::leak(Box::new(Arena::new()));

		let a = arena.insert(String::from("Hello, World"));
		let b = arena.insert(String::from("Bonjour"));
		let c = arena.insert(String::from("Hello, World"));

		assert_ne!(a, b);
		assert_eq!(a, c);

		assert_eq!(a.as_ptr(), c.as_ptr());
	}
}
