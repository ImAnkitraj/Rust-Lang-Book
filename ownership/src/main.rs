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

    let s7 = String::from("Hello");
    let len = calculate_length(&s7); // s7 is moved into the function
    println!("{} {}", len, s7); // This would not cause an error because s7 is passed by reference

    let mut s8 = String::from("Hello");
    let len = calculate_length_mut(&mut s8); // s8 is moved into the function
    println!("{} {}", len, s8); // This would not cause an error because s8 is passed by mutable reference
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
