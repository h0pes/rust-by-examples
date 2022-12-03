#![allow(unreachable_code)]
#[allow(dead_code)]

enum Color {
    // These 3 are specified solely by their name.
    Red,
    Blue,
    Green,
    // These likewise tie `u32` tuples to different names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

enum Temperature {
    Celsius(i32),
    Farenheit(i32),
}


// A function `age` which returns a 32bit unsigned integer value
fn age() -> u32 {
    15
}

enum Boh {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    // if/else
    // Branching with `if-else` is similar to other languages.
    // Unlike many of them, the boolean condition doesn't need to be surrounded
    // by parentheses and each condition is followed by a block.
    // `if-else` conditionals are expressions and all branches must return the same type.
    let n = 20;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            // This expression must return an `i32` as well
            n / 2 //try suppressing this expression with a semicolon
        };
    println!("{:?} -> {:?}", n, big_n);

    // loop
    // Rust provides a `loop` keyword to indicate an infinite loop.
    // The `break` statement can be used to exit a loop at anytime,
    // whereas the `continue` statement can be used to skip the rest of the
    // iteration and start a new one.
    let mut count = 0u32;

    println!("Let's count until infinity...");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }
        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit the loop
            break;
        }
    }

    // Nesting and labels
    // It's possible to `break` or `continue` outer loops when dealing with nested loops.
    // In these cases the loops must be annotated with some 'label' and the label
    // must be passed to the `break/continue` statement.

    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entering the inner loop");

            // This would break only the inner loop
            // break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");

    // Returning from loops
    // One of the uses of a `loop` is to retry an operation until it succeeds.
    // If the operation returns a value though, you might need to pass it to the
    // rest of the code: put it after the `break` and it will be returned by the `loop` expression.
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    // while
    // The `while` keyword can be used to run a loop while a condition is true
    // A FizzBuzz using a `while` loop
    println!("Playing fizzbuzz from 1 to 100 with a 'while' loop");
    // Setting a counter variable
    let mut n = 1;

    // loop while `n` is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // increment counter
        n += 1;
    }

    // for and range
    // The `for` construct can be used to iterate through an `Iterator`.
    // One of easiest ways to create an iterator is to use range notation `a..b`.
    // This yelds values from `a` (inclusive) to `b` (exclusive) in steps of one.
    // Write FizzBuzz using a `for` loop.
    println!("Playing fizzbuzz from 1 to 100 with a 'for' loop");
    for n in 1..101 { //this can be written also as `for n in 1..=100` meaning both ends are inclusive
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // for and iterators
    // the `for in` construct is able to interact with an `Iterator` in several ways.
    // `iter`, `into_iter` and `iter_mut` all handle the conversion of a collection into an interator in different ways, by providing different views on the data within.

    // `iter` borrows each element of the collection through each iteration, leaving the collection untouched and available for reuse after the loop.
    println!("using 'iter'");
    let names = vec!["Bob", "John", "Frank"];

    for name in names.iter() {
        match name {
            &"Frank" => println!("There is Frank among us"),
            // TODO Try deleting the & and matching just "Frank"
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    // `into_iter` consumes the colelction so that on each iteration the exact data is provided.
    // Once the collection has been consumed it is no longer available fo reuse as it has been
    // 'moved' within the loop
    println!("using 'into_iter'");
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // println!("names: {:?}", newnames);
    // FIXME ^ Comment out this line

    // `iter_mut` borrows each element of the collection allowing for the collection to be modified in place
    println!("using 'iter_mut'");
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);

    // match
    // Rust provides pattern matching via the `match` keyword which can be used like a C switch.
    // The first matching arm is evaluated and all possible values must be covered.
    let number = 13; //try different values for number

    println!("Tell me about {}", number);
    match number {
        // match a single value
        1 => println!("One"),
        // match several values
        2 | 3| 5 | 7 | 11 | 13 => println!("This is a prime"),
        // TODO try adding 13 to the list of prime values
        13..=19 => println!("A teen"),
        // handle the rest of the cases
        _ => println!("Nothing special"),
        // TODO try commenting out this catch-all arm.
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // TODO try commenting out one of these arms.
    };

    println!("{} -> {}", boolean, binary);

    // Destructuring: a `match` block can destructure items in a variety of ways
    // 1) Destructuring Tuples
    // 2) Destructuring Arrays and Slices
    // 3) Destructuring Enums
    // 4) Destructuring Pointers
    // 5) Destructuring Structures

    // Destructuring Tuples
    let triple = (0, -2, 3);
    // TODO ^ Try different values for `triple`

    println!("Tell me about {:?}", triple);
    // Match can be used to destructure a tuple
    match triple {
        // Destructure the second and third elements
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
        (.., 2)  => println!("last is `2` and the rest doesn't matter"),
        (3, .., 4)  => println!("First is `3`, last is `4`, and the rest doesn't matter"),
        // `..` can be used to ignore the rest of the tuple
        _      => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
    }

    // Destructuring Arrays and Slices
        // Try changing the values in the array, or make it a slice!
    let array = [1, -2, 6];

    match array {
        // Binds the second and the third elements to the respective variables
        [0, second, third] =>
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),

        // Single values can be ignored with _
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),

        // You can also bind some and ignore the rest
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),
        // The code below would not compile
        // [-1, second] => ...

        // Or store them in another array/slice (the type depends on
        // that of the value that is being matched against)
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),

        // Combining these patterns, we can, for example, bind the first and
        // last values, and store the rest of them in a single array
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }

    // Destructurin Enums
    // `allow dead_code` required to silence warnings because only one variant is used.
    let color = Color::CMYK(122, 17, 40, 1);
    // TODO try differnt variants for `color`

    println!("What color is it?");
    // An `enum` can be destructured using a `match`.
    match color {
        Color::Red => println!("The color is red"),
        Color::Blue => println!("The color is blue"),
        Color::Green => println!("The color is green"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, Green: {}, Blue: {}", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, Saturation: {}, value: {}", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, Saturation: {}, Lightness: {}", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, Magenta: {}, Yellow: {}", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, Magenta: {}, Yellow: {}, key (black): {}", c, m, y, k),
        // Don't need another arm because alla variants have been examined.
    }

    // Destructuring pointers&references
    // For pointers a distinction needs to be made between destructuring and dereferencing
    // as they are different concepts
    // Dereferencing uses `*`
    // Destructuring uses `&`, `ref` and `ref mut`.

    // Assign a reference of type `i32`. The `&val` signifies there is a reference being assigned.
    let reference = &4;

    match reference {
        // If `reference` is pattern matched against `&val`, it results in a comparison like:
        // `&i32`
        // `&val`
        // We see that if the matching `&`s are dropped, then the `i32` should be assigned to `val`.
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the `&`, you dereference before matching.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // What if we don't start with a reference? `reference` was a `&`
    // because the right side was already a reference.
    // This is not a reference because the right side is not one.
    let _not_a_reference = 3;

    // Rust provides `ref` for exactly this purpose. It modifies the assignment
    // so that a reference is created for the element; this reference is assigned.
    let ref _is_a_reference = 3;

    // Accordingly by defining 2 values without references, references can be retrieved
    // via `ref` and `ref mut`.
    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // And we can use `ref mut similarly
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can add anything to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }

    // Destructuring structs
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // Try channging the values in the struct to see what happens
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b is {}, y is {}", b, y),

        // You can destructure structs and rename the variables, the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // And you can also ignore some variables
        Foo { y, .. } => println!("y = {}, we don-t care about x", y),

        // This will give an error: pattern does not mention field `x`
        // Foo { y } => println!("y = {}", y),
    }

    // Guards
    // A `match` `gaurd` can be added to filter the arm
    let temperature = Temperature::Celsius(35);
    // TODO try different values for `temperature`

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        // the `if` condition part is a `guard`
        Temperature::Celsius(t) => println!("{}C is below 30 Celsius", t),

        Temperature::Farenheit(t) if t > 86 => println!("{}F is above 86 Farenheit", t),
        Temperature::Farenheit(t) => println!("{}F is below 86 Farenheit", t),
    }

    // Binding
    // Indirectly accessing a variable makes it impossible to branch and use that variable
    // without re-binding. `match` provides the `@` sigil for binding values to names.
    println!("Tell me what type of person you are");

    match age() {
        0             => println!("I haven't celebrated my first birthday yet"),
        // Could `match` 1 ..=12 directly but then what age would the child be?
        // Instead bind to `n` for the sequence 1 ..=12. Now the age can be reported.
        n @ 1  ..= 12 => println!("I'm a child of age: {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age: {:?}", n),
        // Nothing bound. Return the result.
        n             => println!("I'm an old person of age: {:?}", n),
    }

    // if let
    // For some use cases, when matching enums, `match` is awkward. For example:
    // Make `optional` of type `Option<i32>`
    let optional = Some(7);

    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
            // ^ Needed 2 indentations just so we could destructure `i` from the option.
        },
        _ => {},
        // ^ Required because `match` is exhaustive. Doesn't it seem like wasted space?
    };

    // `if let` is cleaner for this use case and in addition allows various failure options to be specified.

    // All have type `Option<i32>`
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` constructs reads: "if `let` destructures `number` into `Some(i)`,
    // evaluate the block (`{}`)".
    if let Some(i) = number {
        println!("Matched {:?}", i);
    }

    // If you need to specifiy a failure, use an `else` block:
    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        // Destructure failed. Change to the failure case.
        println!("Didn't match a number. Let's go with a letter.");
    }

    // Provide an altered failing condition.
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}", i);
    // Destructure failed. Evaluate an `else if` condition to see if
    // the alternate failure branch should be taken:
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter.");
    } else {
        // The condition evaluated false. This branch is the default:
        println!("I don't like letters. Let's go with an emoticon.");
    }

    // In the same way `if let` can be used to match any enum value:
    // Create example variables
    let a = Boh::Bar;
    let b = Boh::Baz;
    let c = Boh::Qux(100);

    // Variable a matches Foo::Bar
    if let Boh::Bar = a {
        println!("a is foobar");
    }

    // Variable b does not match Foo::Bar
    // So this will print nothing
    if let Boh::Bar = b {
        println!("b is foobar");
    }

    // Variable c matches Foo::Qux which has a value
    // Similar to Some() in the previous example
    if let Boh::Qux(value) = c {
        println!("c is {}", value);
    }

    // Binding also works with `if let`
    if let Boh::Qux(value @ 100) = c {
        println!("c is one hundred");
    }

    // while let
    // Similar to `if let`, `while let` can make awkward `match` sequences more tolerable.
        // Make `optional` of type `Option<i32>`
    let mut optional = Some(0);

    // This reads: "while `let` destructures `optional` into
    // `Some(i)`, evaluate the block (`{}`). Else `break`.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ Less rightward drift and doesn't require
        // explicitly handling the failing case.
    }
    // ^ `if let` had additional optional `else`/`else if`
    // clauses. `while let` does not have these.
}
