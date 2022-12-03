// Rust has two different types of constants which can be declared in any scope including global.
// Both require explicit type annotation:
// `const`: an unchangeable value (the common case)
// `static`: a possible `mut`able variable with `static` lifetime. The static lifetime is inferred
// and does not have to be specified. Accessing or modifying a mutable static variable is `unsafe`.

// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access `constant` in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Access `constant` in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is: {}", THRESHOLD);
    println!("{} is {} than threshold", n, if is_big(n) { "bigger" } else { "smaller" });

    // Error! Cannot modify a constant
    // THRESHOLD = 5;
    // comment out the previous line to fix the error.
}
