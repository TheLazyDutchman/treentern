use std::{marker::Sized, sync::LazyLock};

use arena::Arena;

pub mod arena;

pub trait Intern {
	fn intern(&'static self) -> Interned<Self>;

	fn intern_owned(self) -> Interned<Self>
	where
		Self: Sized,
	{
		Box::leak(Box::new(self)).intern()
	}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Interned<T: ?Sized + 'static>(&'static T);

static STRING_ARENA: LazyLock<Arena<String>> = LazyLock::new(Arena::new);

impl Intern for String {
	fn intern(&'static self) -> Interned<Self> {
		Interned(STRING_ARENA.insert(self))
	}
}

static STR_ARENA: LazyLock<Arena<str>> = LazyLock::new(Arena::new);

impl Intern for str {
	fn intern(&'static self) -> Interned<Self> {
		Interned(STR_ARENA.insert(self))
	}
}

#[cfg(test)]
mod test {
	use crate::Intern;

	#[test]
	fn intern_string() {
		let a = String::from("Hello, World").intern_owned();
		let b = String::from("Bonjour").intern_owned();
		let c = String::from("Hello, World").intern_owned();

		assert_ne!(a, b);
		assert_eq!(a, c);

		assert_eq!(a.0.as_ptr(), c.0.as_ptr());
	}

	#[test]
	fn intern_str() {
		let a = "Hello, World".intern();
		let b = "Bonjour".intern();
		let c = "Hello, World".intern();

		assert_ne!(a, b);
		assert_eq!(a, c);

		assert_eq!(a.0.as_ptr(), c.0.as_ptr());
	}
}
