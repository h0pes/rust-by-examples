// There are three types of structures `struct` that can be created
// using the `struct` keyword:
// 1) Tuple structs, which are baseically named tuples
// 2) The classic C structs
// 3) Unit structs, which are field-less and are useful for generics.

// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

#[derive(Debug)]
// A tuple struct with two fields
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    p1: Point,
    p2: Point,
}

// A function which calculates the area of a `Rectangle`
fn rect_area(point_one: Point, point_two: Point) -> f32 {
    let base = (point_one.x - point_two.x).abs();
    println!("base is: {}", base);
    let height = (point_one.y - point_two.y).abs();
    println!("height is: {}", height);
    let area = base * height;
    println!("area is: {}", area);
    return area;
}

// A better function to calculate the area of a rectangle
fn rectangle_area(rect: Rectangle) -> f32 {
    let Rectangle {
        p1: Point {x: x1, y: y1 },
        p2: Point {x: x2, y: y2},
    } = rect;
    ((x1 - x2) * (y1 - y2)).abs()
}

fn square(point: Point, f: f32) -> Rectangle {
    Rectangle {
        p1: point,
        p2: Point {x: f, y:f},
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: {} {}", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let new_point = Point { x: 5.2, ..point};

    // `new_point.y` will be the same as `point.y` because we used that field from `point`
    println!("second point: ({}, {})", new_point.x, new_point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_x, y: my_y },
        p2: point,
    };

    // calculate the rectangle area
    let area = rectangle_area(_rectangle);
    println!("rectangle area: {}", area);

    // execute the square function
    let _square = square(Point { x: 0.6, y: 0.6 }, 1.2);
    println!("square is: {:?}", _square);

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
