// A Rust program is (mostly) made up of a series of statements:
// fn main() {
    // statement 1
    // statement 2
    // statement 3
// }

// There are a few kinds of statements in Rust.
// The most common two are declaring a variable binding
// and using `;` with an expression:

fn main() {
    // variable binding
    let x = 5;

    // expression
    x;
    x + 1;
    15;

    // Blocks are expressions too, so they can be used as values in assignments.
    // The last expression in the block will be assigned to the place expression
    // such as a local variable. However, if the last expression of the block
    // ends with a semicolon, the return value will be ().
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        println!("x_squared is {:?}", x_squared);
        let x_cube = x_squared * x;
        println!("x_cube is {:?}", x_cube);

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
