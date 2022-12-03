// This structure cannot be printed either with `fmt::Display` or
// `fmt::Debug`.
struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation
// required for this `struct` to be printable with `fmt::Debug`.
#[derive(Debug)]
struct DebugPrintable(i32);

// All `std` library types are automatically printable with `{:?} too
// Derive the `fmt::Debug` implementation for `Structure` which is a structure
// that contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable also.
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // Printing with `{:?} is similar to printing with `{}`
    println!("{:?} months in a year", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
            "Slater",
            "Christian",
             actor="actor's");
    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if we want this to just show a `7`?
    println!("Now {:?} will print", Deep(Structure(7)));

    // Rust also provides "pretty printing" with `{:#?}.
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "Peter";
    let age = 27;
    let peter = Person {name, age};

    // Pretty print for `Person` struct
    println!("{:#?}", peter);
}