use treentern::{derive::Intern, Intern};

#[derive(Debug, Intern, PartialEq, Eq, Hash)]
#[intern_derive(Debug, PartialEq, Eq, Hash)]
struct User {
	#[intern]
	first_name: String,

	#[intern]
	last_name: String,
}

fn main() {
	let a = User {
		first_name: "Charlie".to_string(),
		last_name: "Cayne".to_string(),
	}
	.intern_owned();

	let b = User {
		first_name: "Charlie".to_string(),
		last_name: "Dwayne".to_string(),
	}
	.intern_owned();

	let c = User {
		first_name: "Dwayne".to_string(),
		last_name: "Johnson".to_string(),
	}
	.intern_owned();

	let d = User {
		first_name: "Charlie".to_string(),
		last_name: "Cayne".to_string(),
	}
	.intern_owned();

	assert_eq!(a, d);
	assert_ne!(a, b);
	assert_ne!(b, c);
	assert_ne!(a, c);

	assert_eq!(a.first_name.as_ptr(), b.first_name.as_ptr());
	assert_eq!(b.last_name.as_ptr(), c.first_name.as_ptr());
}
