#![deny(clippy::pedantic)]

use std::marker::Sized;

pub mod arena;

pub trait Intern {
	type InternedType: ?Sized;

	fn intern(&'static self) -> Interned<Self::InternedType>;

	fn intern_owned(self) -> Interned<Self::InternedType>
	where
		Self: Sized + 'static,
	{
		Box::leak(Box::new(self)).intern()
	}
}

impl<T: ?Sized + Intern> Intern for &'static T {
	type InternedType = T::InternedType;

	fn intern(&'static self) -> Interned<Self::InternedType> {
		T::intern(self)
	}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Interned<T: ?Sized + 'static>(&'static T);

macro_rules! basic_impl {
	($ty:ty $(, $import:path)?) => {
		paste::paste! {
			mod [<$ty:snake _intern_impl>] {
				use std::sync::LazyLock;
				use crate::arena::Arena;
				use crate::Intern;
				use crate::Interned;
				$(use $import :: $ty;)?

				static ARENA: LazyLock<Arena<$ty>> = LazyLock::new(Arena::new);

				impl Intern for $ty {
                                        type InternedType = Self;

					fn intern(&'static self) -> Interned<Self> {
						Interned(ARENA.insert(self))
					}
				}
			}
		}
	};
}

basic_impl!(String);
basic_impl!(str);
basic_impl!(OsString, std::ffi);
basic_impl!(CString, std::ffi);
basic_impl!(u8);
basic_impl!(u16);
basic_impl!(u32);
basic_impl!(u64);
basic_impl!(u128);
basic_impl!(i8);
basic_impl!(i16);
basic_impl!(i32);
basic_impl!(i64);
basic_impl!(i128);

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
