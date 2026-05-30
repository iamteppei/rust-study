mod ownership;
mod pattern_matching;
mod functional_programming;

// main.rs is crate root (binary crate). It's different with lib.rs - also a crate root - which is library crate
fn main() {
    // Rust Ownership
    ownership::start();

    // pattern matching.
    pattern_matching::start();

    functional_programming::start();
}
