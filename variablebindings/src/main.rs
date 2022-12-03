// Rust provides type safety via static typing.
// Variable bindings can be atype annotated when declared.
// However in most cases the compiler will be able to infer the type of the variable from context
// heavily reducing the annotation burden.
// Values (like literals) can be bound to variables, using the `let` binding.

// Variable bindings are immutable by default, but this can be overridden using the `mut` modifier

// Variable bindings have a scope and are constrained to live ina `block`.
// A block is a collection of statements enclosed by braces `{}`.

fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit =();

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("A copied integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    let _noisy_unused_variable = 2u32; // prefix the variable name with an underscore to suppress the compiler warning

    // Mutability
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error!
    // _immutable_binding += 1;
    // FIXME ^ Comment out this line

    // Scope and shadowing
    // This binding lives in the main function
    let long_lived_binding = 1;

    // This is a block and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;
        println!("inner short lived binding: {}", short_lived_binding);
    } // end of block

    // Error! `short_lived_binding` doesn't exist in this scope
    // println!("outer short lived binding: {}", short_lived_binding);
    // fix the previous line commeting it out

    println!("outer long lived binding: {}", long_lived_binding);

    // Variable shadowing
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        // This binding shadows the outer one
        let shadowed_binding = "abc";

        println!("after being shadowed (shadowed in inner block): {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    // This binding shadows the previous binding
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);

    // It's possible to declare variable bindings first, and initialize them later. However, this form is seldom used, as it may lead to the use of uninitialized variables.
    // The compiler forbids use of uninitialized variables, as this would lead to undefined behavior.
    // declare a variable binding
    let a_binding;

    {
        let x = 2;
        // Initialize the binding
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // Error! Use of uninitialized binding
    // println!("another binding: {}", another_binding);
    // comment oout the previous line to fix

    another_binding = 1;
    println!("another binding: {}", another_binding);

    // Freezing: when data is bound by the same name immutably, it also freezes. Frozen data can't be modified until the immutable binding goes out of scope

    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable `_mutable_integer
        let mut _mutable_integer = _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        _mutable_integer = 50;
        // comment out the previous line to fix
        // println!("mutable integer is: {}", _mutable_integer);
        // here `_mutable_integer` goes out of scope
    }

    println!("mutable integer is: {}", _mutable_integer);
    // `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
}
