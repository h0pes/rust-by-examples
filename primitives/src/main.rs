// Rust provides access to a wide variety of primitives.
// `signed integers`: i8, i16, i32, i64, i128 and isize (pointer size)
// `unsigned integers`: u8, u16, u32, u64, u128 and usize (pointer size)
// `floating point`: f32, f64
// `char` Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
// `bool` either true or false
// and the `unit type ()`, whose only possible value is an empty tuple: ()
// Despite the value of a unit type being a tuple, it is not considered a compound type because it does not contain multiple values.

// Compund type:
// `arrays` like [1, 2, 3]
// tuples like (1, true)

// Variables can always be `type annotated`. Numbers may additionally be annotated via a `suffix` or by `default`. Integers default to `i32` and `floats` to `f64`. Rust can also infer types from context.

fn main() {
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0; // regular annotation
    let an_integer = 5i32; // suffix annotation (type annotation inserted by editor plugin)

    // Or a default will be used.
    let default_float = 3.0; // default float is `f64`
    let default_integer = 7; // default integer is `i32`

    // A type can also be inferred from context.
    let mut inferred_type = 12; // type `i64` is inferred from another line
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed.
    let mut mutable = 12; // mutable `i32`
    mutable = 21;

    // Error! The type of a variable cannot be changed.
    // mutable = true;

    // Variables can be overwritten with shadowing.
    let mutable = true;
}
