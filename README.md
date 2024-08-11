# Treentern

This is a interning library.

## Basic usage

```rust
let a = "Hello, World".intern();
let b = "Hello, World".intern();

assert_eq!(a.as_ptr(), b.as_ptr());
```
