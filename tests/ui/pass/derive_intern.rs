use treentern::derive::Intern;
use treentern::Intern;

#[derive(Debug, Intern, PartialEq, Eq, Hash)]
struct User {
	first_name: String,
	last_name: String,
}

fn main() {
	let a = User {
		first_name: "charlie".to_string(),
		last_name: "cayne".to_string(),
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
		first_name: "charlie".to_string(),
		last_name: "cayne".to_string(),
	}
	.intern_owned();

	assert_eq!(a, d);
	assert_ne!(a, b);
	assert_ne!(b, c);
	assert_ne!(a, c);
}
