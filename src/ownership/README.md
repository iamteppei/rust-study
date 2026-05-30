# Rust Ownership

- Ownership is mainly about move and copy behavior.
- Move means ownership of a value is transferred to another binding.
- After a move, the old binding is no longer valid. The value is dropped when its current owner goes out of scope.
- Copy behavior applies to types that implement the `Copy` trait (for example `i32`, `bool`, `char`). Assignment copies the value.
- In Rust, variables are immutable by default.
- Use `mut` to allow mutation.
- Borrowing rules:
  - You can have one mutable reference (`&mut T`), or many immutable references (`&T`).
  - You cannot use mutable and immutable borrows of the same value at the same time.
- Rust prevents dangling references at compile time. A dangling reference points to memory that has already been freed.
- A slice (for example `&str` or `&[T]`) is a borrowed view into contiguous data and does not own the underlying data.
- Refer to a variable using & - call reference or pointer in C programming language. In Rust, every reference has a lifetime.
