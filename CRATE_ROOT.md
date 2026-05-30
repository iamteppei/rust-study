# Wiring the Crate Root in Rust

## What is the crate root?

Every Rust binary starts at `src/main.rs` (or `src/lib.rs` for a library).
This file is called the **crate root**.
Rust does not automatically discover other `.rs` files in `src/`.
You must explicitly declare each module from the crate root (or from a parent module) using the `mod` keyword.

---

## The module system at a glance

```
src/
  main.rs             ← crate root
  ownership.rs        ← module bridge file (declares submodule)
  ownership/
    main.rs           ← module implementation
  pattern_matching.rs ← module bridge file (declares submodule)
  pattern_matching/
    main.rs           ← module implementation
```

---

## Step 1 — Declare the module in the crate root

`src/main.rs`:
```rust
mod ownership;          // tells Rust to look for src/ownership.rs or src/ownership/mod.rs
mod pattern_matching;   // tells Rust to look for src/pattern_matching.rs or src/pattern_matching/mod.rs

fn main() {
    ownership::start();
    pattern_matching::start();
}
```

Without `mod pattern_matching;` here, Rust treats the `src/pattern_matching/` folder as if it does not exist.
No code inside it will compile or be testable via `cargo test`.

---

## Step 2 — Provide a bridge file for folder modules

When a module lives inside a folder (e.g. `src/pattern_matching/`), Rust needs one of:
- `src/pattern_matching.rs` — a file at the same level as the folder, **or**
- `src/pattern_matching/mod.rs` — a file inside the folder (older convention).

This project uses the first style.

`src/pattern_matching.rs`:
```rust
mod main;               // declares src/pattern_matching/main.rs as a child module
pub use main::start;    // re-exports start() so callers can use pattern_matching::start()
```

---

## Step 3 — Implement the module

`src/pattern_matching/main.rs`:
```rust
pub fn start() {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_init() {
        assert_eq!(1, 1);
    }
}
```

`pub fn start()` must be public so it is accessible from `main.rs` via `pattern_matching::start()`.

---

## Why unit tests only run after wiring

Unit tests inside `src/` are compiled as part of the crate binary.
If a file is not reachable from the crate root via `mod` declarations, Rust never compiles it,
so `cargo test` cannot find or run any tests it contains.

| Scenario | `#[test]` discovered? |
|---|---|
| Module declared with `mod` in crate root | Yes |
| Module not declared anywhere | No |
| File in `tests/` folder | Always yes (integration tests are separate targets) |

---

## Integration tests vs unit tests

| | Unit tests (`src/`) | Integration tests (`tests/`) |
|---|---|---|
| Location | Inside `src/**/*.rs` with `#[cfg(test)]` | `tests/*.rs` files |
| Must be wired? | Yes, via `mod` declarations | No, Cargo finds them automatically |
| Can access private items? | Yes | No (only `pub` API) |
| Run with | `cargo test` | `cargo test --test <filename>` |

---

## Module path produced

Once wired, the unit test in `src/pattern_matching/main.rs` is reachable at:

```
pattern_matching::main::tests::test_init
```

This is the full module path: `crate → pattern_matching → main → tests → test_init`.

---

## Summary

1. Rust does not auto-discover source files — you must declare them with `mod`.
2. `mod <name>` in the crate root tells Rust to look for `src/<name>.rs` or `src/<name>/mod.rs`.
3. For folder modules, a bridge file re-exports public items.
4. Unit tests in undeclared files are invisible to `cargo test`.
5. Files in `tests/` are always independent integration test targets, no wiring needed.
