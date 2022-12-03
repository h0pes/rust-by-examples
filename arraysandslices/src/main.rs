// An array is a collection of objects of the same type `T`,
// stored in contiguous memory. Arrays are created using brackets `[]`,
// and their length, which is known at compile time, is part of their
// type signature `[T; lenght]`.
//
// Slices are similar to arrays, but their length is not known at compile time.
// A slice is a two-word object, the first word is a pointer to the data
// and the second word is the length of the slice.
// The word size is the same as `usize`, determined by the processor architecture (i.e. 64 bits on x86-64).
// Slices can be used to borrow a section of an array and have the type signature `&[T]`.

use std::mem;

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
    println!("slice elements: {:?}", slice);
}

fn main() {
    // Fixed size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at zero
    println!("first element of the array is: {}", xs[0]);
    println!("second element of the array is: {}", xs[1]);

    // `len` returns the count of elements in the array
    println!("number of elements in the array: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    //Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
    analyze_slice(&xs[1..4]);

    // Example of empty slice `&[]`
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); //same as above but more verbose

    // Arrays can be safelt accessed using `.get`
    // which returns an `Option`.
    // This can be matched as shown below or used with `.expect()`
    // if we would like the program to exit with a nice message instead of
    // happily continue.
    for i in 0..xs.len() + 1 { //ERORR: one element too far
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("{}: out of bounds", i),
        }
    }

    // Out of bound indexing causes compiler error
    // println!("{}", xs[5]);
}