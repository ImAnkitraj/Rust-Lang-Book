fn main() {
    // Ownership rules
    // 1. Each value in Rust has a variable that is its "owner".
    // 2. A value can only have one owner at a time.
    // 3. When the owner of a value goes out of scope, Rust will automatically drop the value.

    {
        // s is not valid here, it’s not yet declared
        let s = String::from("Hello"); // s is valid from this point forward
        // do stuff with s
    } // s is no longer valid here

    let x = 5;
    let y = x; // y is a copy of x, so x is still valid

    let s1 = String::from("Hello");
    let s2 = s1; // Move (not a shallow copy)
    // s2 is now the owner of the string, s1 is no longer valid

    // Ownership with functions
    let s3 = s2.clone(); // Deep copy // More expensive
    takes_ownership(s2); // s2 is moved into the function
    // println!("{}", s2); // This would cause an error because s2 is no longer valid

    makes_copy(x); // x is copied into the function
    println!("{}", x); // x is still valid here

    let s4 = gives_ownership(); // gives ownership to s4
    println!("{}", s4); // s4 is valid here

    let s5 = String::from("Hello");
    let s6 = takes_and_gives_back(s5); // s5 is moved into the function and then returned
    // println!("{}", s5); // This would cause an error because s5 is no longer valid
    println!("{}", s6); // s6 is valid here

    // References
    // They are immutable by default
    let s7 = String::from("Hello");
    let len = calculate_length(&s7); // s7 is moved into the function
    println!("{} {}", len, s7); // This would not cause an error because s7 is passed by reference

    // Mutable references
    // Mutable references allow you to change the value of the variable
    // You can only have one mutable reference to a variable in a particular scope
    // You can have multiple immutable references to a variable in a particular scope
    // You cannot have a mutable reference and an immutable reference to a variable in the same scope
    let mut s8 = String::from("Hello");
    let len = calculate_length_mut(&mut s8); // s8 is moved into the function
    println!("{} {}", len, s8); // This would not cause an error because s8 is passed by mutable reference

    // Dangling references
    // A dangling reference is a reference that points to a location in memory that has been freed
    // This can happen if you return a reference to a variable that goes out of scope
    // The following code would cause a dangling reference error
    let ref_to_nothing = dangle();

    // The Rules of References
    // 1. At any given time, you can have either one mutable reference or any number of immutable references.
    // 2. References must always be valid.

    // Slices
    // A slice is a reference to a contiguous sequence of elements in a collection
    // Slices are used to borrow a portion of a collection without taking ownership of it
    // Slices are useful when you want to pass a portion of a collection to a function without taking ownership of it
    // Slices are created using the syntax &collection[start..end]
    // Slices are used to borrow a portion of a collection without taking ownership of it
    // Slices are useful when you want to pass a portion of a collection to a function without taking ownership of it
    // Slices are created using the syntax &collection[start..end]
    // Slices are used to borrow a portion of a collection without taking ownership of it
    let s9 = String::from("Hello, world!");
    let hello = &s9[0..5]; // Borrow a slice of the string
    let world = &s9[7..12]; // Borrow another slice of the string
    println!("{} {}", hello, world); // Print the slices

    let first = first_word(&s9);
    println!("First word length: {}", first); // This would not cause an error because s9 is passed by reference
    println!("{}", s9); // s9 is still valid here
}

fn takes_ownership(s: String) {
    println!("{}", s); // s is valid here
} // s is dropped here

fn makes_copy(x: i32) -> i32 {
    x // x is copied here
} // x is dropped here, but it’s a copy so it’s still valid

fn gives_ownership() -> String {
    let s = String::from("Hello"); // s is valid from this point forward
    s // s is returned and moved out of the function
}

fn takes_and_gives_back(s: String) -> String {
    println!("{}", s);
    s // s is returned and moved out of the function
}

fn calculate_length(s: &String) -> usize {
    // Referneces are immutable by default
    // s.push_str(" World"); // This would cause an error because s is immutable
    let length = s.len(); // len() returns the length of the string
    length // length is returned
} // s is not dropped here because it’s passed by reference

fn calculate_length_mut(s: &mut String) -> usize {
    s.push_str(" World"); // This is allowed because s is mutable
    let length = s.len(); // len() returns the length of the string
    length // length is returned
} // s is not dropped here because it’s passed by reference

fn dangle() -> &String {
    let s = String::from("Hello"); // s is valid from this point forward
    &s // s is returned and moved out of the function
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); // Convert the string to bytes
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // Return the index of the first space
        }
    }
    &s[..] // Return the entire string if no space is found
}
