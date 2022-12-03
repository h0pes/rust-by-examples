// Rust provides several mechanisms to change or define the type of primitive and user defined types:
// 1) Casting between prmitive types
// 2) Specifying the desired type of literals
// 3) Using type inference
// 4) Aliasing types

// Suppress all warnings from casts which overflow.
#![allow(overflowing_literals)]

fn main() {
    // 1) Casting
    // Rust provides no implicit type conversion (coercion) between primitive types.
    // But explicit type conversion (casting) can be performed using the `as` keyword.
    // Rules for converting between integral types follow C conventions generally,
    // except in cases wher eC has undefined behaviour. The behaviour of all casts between
    // integral types is well defined in Rust.
    let decimal = 65.4321_f32;

    // Error! No implicit conversion
    // let integer: u8 = decimal;
    // Comment out the previous line to fix the error.

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    // Error! There are limitations in convesion rules.
    // A float cannot be directly converted to a char.
    // let character = decimal as char;
    // Comment out the previous line to fix the error.

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // When casting any value to an unsigned type, `T`,
    //  T::MAX + 1 is added or subtracted until the value
    // fits into the new type.

    // 1000 already fits in a `u16`
    println!("1000 as u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // Under the hood the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bit (MSB) get truncated.
    println!("1000 as u8 is: {}", 1000 as u8);
    // -1 + 256 = 255
    println!("-1 as a u8 is: {}", (-1i8) as u8);

    // For positive numbers this is the same as the modulo operator
    println!("1000 mod 256 is: {}", 1000 % 256);

    // When casting to a signed type the bitwise result is the same as
    //  first casting to the corresponding unsigned tpye. If the most
    // significant bit of that value is 1, then the value is negative.

    // Unless it already fits, of course.
    println!("128 as i16 is: {}", 128 as i16);

    // 128 as i8 -> -128, whose two's complement in eight bits is:
    println!("128 as i8 is: {}", 128 as i8);

    // Repeating the example above
    // 1000 as u8 -> 232
    println!("1000 as u8 is: {}", 1000 as u8);
    // and the two's complement of 232 is -24
    println!("232 as i8 is: {}", 232 as i8);

    // Since Rust 1.45, the `as` keyword perform a `saturating cast`
    // when casting from float to int. If the floating point value
    // exceeds the upper bound or is less than the lower bound, the returned value
    // will be equal to the bound crossed.

    // 300.0 is 255
    println!("300.0 is {}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is: {}", -100.0_f32 as u8);
    // nan as u8 is 0
    println!("nan as u8 is {}", f32::NAN as u8);

    // This behaviour incurs a small runtime cost and can be avoided
    // with unsafe methods, however the results might overflow and
    // return unsound values. Use these methods wisely:
    unsafe {
        // 300.0 is 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }

    // 2) Literals
    // Numeric literals can be type annotated by adding the type as a suffix.
    // As an example, to specify that the literal `42` should have the type `i32`
    // write `42i32`.

    // The type of unsuffixed numeric literals will depend on how they are used.
    // If no constraint exists, the compiler will use `i32` for integers and `f64`
    // for floating -point numbers.

    // Suffixed literals, their type are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals, their tpyes depend on how they are used
    let i = 1;
    let f = 1.0;
    let c = 'c';

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
    println!("size of `c` in bytes: {}", std::mem::size_of_val(&c));

    // 3) Inference
    // The type inference engine is pretty smart. It does more than looking at the type of the value
    // xpression during an initialization. It also looks at how the variable is used afterwards to infer its type. Here's an advanced example of type inference:

    // Because of the annotation, the compiler knows that `elem` has type `u8`.
    let elem = 5u8;

    // Create an empty vector (a growable array)
    let mut vec = Vec::new();
    // At this point the compiler doesn-t know the exact type of `vec`
    // it just knows that it's a vector of something (`Vec<_>`).

    // Insert `elem` in the vector
    vec.push(elem);
    // Now the compiler knows that `vec` is a vector of `u8` (`Vec<u8>`)

    // TODO: try commenting out the `vec.push` instruction
    println!("{:?}", vec);

    // 4) Aliasing
    // The `type` statement can be used to give a new name to an existing type. Types must have
    // `UpperCamelCase` names, or the compiler will raise a warning. The exception to this rule are the primitive types: `usize`, `f32`, etc.

    // `NanoSecond`, `Inch` and `U64` are NEW NAMES for `u64`.
    type NanoSecond = u64;
    type Inch = u64;
    type U64 = u64;

    // `NanoSecond` = `Inch` = `U64` = `u64`
    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;

    // Note that type aliases don't provide any extra type safety, because aliases are not new types
    println!("{} nanoseconds + {} inches = {} unit?",
                nanoseconds,
                inches,
                nanoseconds + inches);
}
