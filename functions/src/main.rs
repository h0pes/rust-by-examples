// Functions are declared using th e`fn` keyword.
// Its arguments are type annotated, like variables, and
// if the function returns a value, the return type must be specified after an arrow `->`
// The final expression in the the function will be used as the return value.
// Alternatively, the `return` statement can be used to return a valure earlier
// from within the function, even from inside loops or if statements.

// Let's rewrite FizzBuzz using functions.
// Unlike C or C++ there is no restrictions on the order of function definitions.

// Associated functions and methods
// Some functions are connected to a particular type.
// These come in two forms: associated functions and methods.
// Associated functions are  functions that are defined on a type generally,
// while methods are associated functions that are called on a particular instance of a type.

struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions and methods go in here.
impl Point {
    // This is an associated function because this function is associated with a particular type
    // that is Point.
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function taking two arguments:
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y}
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the caller object.
    // In this case `Self` is a `Rectangle`.
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the caller.
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;

    }
}
// `Pair` owns resources: two heap allocated integers.
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method consumes the resources of the caller object
    // `self` desugars to `self: Self`.
    fn destroy(self) {
        // Destructure `self`
        let Pair(first, second) = self;

        println!("Destroying Pair ({}, {})", first, second);

        // `first` and `second` go out of scope and get freed.
    }
}
fn main() {
    // We can use this function here and define it somewhere later.
    fizzbuzz_to(100);
}

// Function that returns a boolean value
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // Corner case, ealy return
    if rhs == 0 {
        return false;
    }

    // This is an expression, the `return` keyword is not necessary here.
    lhs % rhs == 0
}

// Functions that don't return a value, actually return the unit type `()`.
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// When a function returns the unit type `()`, the return type can be omitted from the signature
fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }

    // Assocaited functions and methods
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator.
    // Note that the first argument `&self` is implicitly passed so that
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`.
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable object
    // rectangle.translate(1.0, 0.0);
    // TODO ^ try uncommenting this line

    // Okay! Mutable objects can call mutable methods
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`.
    // pair.destroy();
    // TODO ^ try cuncommenting this line.
}
