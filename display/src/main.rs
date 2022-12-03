// To customize the putput appearance `fmt::Display` is used and it must be implemented
// for our type. Implementing it looks like this>
use std::fmt;

// Define a structure with which `fmt::Display` will be implemented.
// This is a tuple struct named `Structure` that contains an `i32`.
struct Structure(i32);

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Structure {
    // this trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

// A structure holding two numbers. `Debug` will be derived so the results can
// be compared with `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // use `self.number` to refer to each positional data point.
        write!(f, "({} {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly implement `Display` for `Point2D`.
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y; {}", self.x, self.y)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

// Similarly implement `Display` for `Complex`.
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{} + {}i", self.real, self.imag)
    }
}
fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small reange is {small}",
            big = big_range,
             small = small_range);

    println!("The big range is {big:?} and the small reange is {small:?}",
            big = big_range,
            small = small_range);

    let point = Point2D {x: 3.3, y: 7.2};

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let complex = Complex {
        real: 3.3, imag: 7.2};

    println!("Compare complex:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    // println!("What does Point2D look like in binary: {:b}?", point);
    // fmt::Display has been implemented but fmt::Binary has not
    // and therefore cannot be used.
}