# Treentern

This is an interning library.

## Basic usage

```rust
let a = "Hello, World".intern();
let b = "Hello, World".intern();

assert_eq!(a.as_ptr(), b.as_ptr());
```

As compared to other implementations, this library aims to need no unsafe code.
It does this by having a separate arena for every type that implements the `Intern` trait.

Still, it is probably a better choice to pick another interning library (for now),
as this is mostly written as a way to learn how interning works.
