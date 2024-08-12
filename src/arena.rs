use std::{
	collections::HashMap,
	hash::{BuildHasher, Hash},
	sync::Mutex,
};

pub struct Arena<T: ?Sized + 'static, H = std::hash::RandomState> {
	values: Mutex<Vec<&'static T>>,
	indices: Mutex<HashMap<&'static T, Index, H>>,
}

#[derive(Clone, Copy)]
struct Index(usize);

impl<T: ?Sized, H: Default> Default for Arena<T, H> {
	fn default() -> Self {
		Self {
			values: Mutex::default(),
			indices: Mutex::default(),
		}
	}
}

impl<T: ?Sized> Arena<T> {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}
}

impl<T: ?Sized, H> Arena<T, H> {
	fn get(&'static self, index: Index) -> &'static T {
		self.values.lock().unwrap()[index.0]
	}

	/// Insert a value into the arena
	///
	/// # Panics
	/// The arena uses a [`Mutex`] internally, which can become poisoned if a thread panics
	/// while holding the mutex
	pub fn insert(&'static self, value: &'static T) -> &'static T
	where
		T: Hash + Eq,
		H: BuildHasher,
	{
		if let Some(index) = self
			.indices
			.lock()
			.unwrap()
			.get(&value)
		{
			return self.get(*index);
		}

		let index = Index(
			self.values
				.lock()
				.unwrap()
				.len(),
		);
		self.values
			.lock()
			.unwrap()
			.push(value);
		self.indices
			.lock()
			.unwrap()
			.insert(value, index);
		value
	}

	pub fn insert_owned(&'static self, value: T) -> &'static T
	where
		T: Hash + Eq + Sized,
		H: BuildHasher,
	{
		self.insert(Box::leak(Box::new(value)))
	}
}

#[cfg(test)]
mod test {
	use super::Arena;

	#[test]
	fn insert_arena() {
		let arena = Box::leak(Box::new(Arena::new()));

		let a = arena.insert_owned(String::from("Hello, World"));
		let b = arena.insert_owned(String::from("Bonjour"));
		let c = arena.insert_owned(String::from("Hello, World"));

		assert_ne!(a, b);
		assert_eq!(a, c);

		assert_eq!(a.as_ptr(), c.as_ptr());
	}

	#[test]
	fn unsized_type() {
		let arena: &Arena<str> = Box::leak(Box::new(Arena::new()));

		let a = arena.insert("Hello, World");
		let b = arena.insert("Bonjour");
		let c = arena.insert("Hello, World");

		assert_ne!(a, b);
		assert_eq!(a, c);

		assert_eq!(a.as_ptr(), c.as_ptr());
	}
}
