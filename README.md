# Rust Study

A hands-on Rust learning project exploring core language concepts through annotated code examples.

## Topics Covered

### Ownership
Located in `src/ownership/study.rs`.

- **Stack vs Heap** — fixed-size data lives on the stack; dynamically sized data lives on the heap and is accessed via pointers.
- **Move vs Copy** — assigning a heap-allocated value (e.g. `String`) moves ownership; primitive types (e.g. `i32`) are copied.
- **Borrowing** — use `&variable` to pass a reference without transferring ownership (immutable by default).
- **Mutable references** — use `&mut variable` to allow mutation; only one mutable reference to a value is allowed at a time (prevents data races).
- **Reference rules** — you can have multiple immutable references OR one mutable reference, but not both simultaneously.
- **Dangling references** — Rust's compiler prevents returning references to values that have been dropped.
- **Slice type** — a reference to a contiguous sequence of elements in a collection; holds no ownership.

## Project Structure

```
src/
  main.rs              # Entry point
  ownership/
    study.rs           # Ownership, borrowing, and slice concepts
```

## Local Setup

### Prerequisites

- [Rust & Cargo](https://rustup.rs/)

### Run

```bash
cargo run
```

### Watch mode (auto-rerun on file change)

Install `cargo-watch`:

```bash
cargo install cargo-watch
```

Start the watcher:

```bash
cargo watch -w src -x run
```

Start Pattern Matching Test

```bash
cargo watch -w src -x "test pattern_matching"
```