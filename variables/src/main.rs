// const must have a type annotated
// const can't be assigned to the return value of a function
// const cannot be mutated : const mut MUTNUM: u32 = 2000; This is not possible

// Shadowing
// We can reclare a variable with different data-type
fn main() {
    let mut x = 32;
    println!("{}", x);
    x = 6;
    println!("{}", x);
    const NUM: u32 = 1_000;
    println!("{}", NUM);

    // Shadowing example
    let y = 10;
    println!("{}", y);

    let y = "ten";
    println!("{}", y);

    // Scaler Data Types
    // Integers
    let num1: i32 = 10;
    let num2: i32 = 0xff;
    let num3: i32 = 0o10;
    let num4: i32 = 0b10;
    let num5: i32 = 1_000;
    println!(
        "Decimal: {}, Hexadecimal: {}, Octal: {}, Binary: {}, Decimal with underscore: {}",
        num1, num2, num3, num4, num5
    );
    // FLoating point numbers
    let num6: f32 = 10.0;
    let num7: f64 = 10.0;
    println!("f32: {}, f64: {}", num6, num7);
    // Floating point numbers with underscore
    let num8: f32 = 1_000.0;
    let num9: f64 = 1_000.0;
    println!(
        "f32 with underscore: {}, f64 with underscore: {}",
        num8, num9
    );
    // Floating point numbers with scientific notation
    let num10: f32 = 1e3;
    let num11: f64 = 1e3;
    println!(
        "f32 with scientific notation: {}, f64 with scientific notation: {}",
        num10, num11
    );
    // Floating point numbers with scientific notation and underscore
    let num12: f32 = 1_000e3;
    let num13: f64 = 1_000e3;
    println!(
        "f32 with scientific notation and underscore: {}, f64 with scientific notation and underscore: {}",
        num12, num13
    );
    // Boolean
    let is_true: bool = true;
    let is_false: bool = false;
    println!("Boolean: {}, {}", is_true, is_false);

    // Characters
    let char1: char = 'a';
    let char8: char = '\u{1F600}'; // Unicode character

    // Compound Data Types
    // Tuples
    let tuple1: (i32, f32, char) = (10, 20.0, 'a');
    let (i, d, c) = tuple1; // Destructuring tuple
    println!("Tuple: {:?}", tuple1);
    println!("Tuple: {:?}", (i, d, c));
    println!("Tuple: {:?}", tuple1.0);
    println!("Tuple: {:?}", tuple1.1);
    println!("Tuple: {:?}", tuple1.2);
    // Arrays
    let array1: [i32; 5] = [1, 2, 3, 4, 5]; // Array with 5 elements
    let array2: [i32; 5] = [1; 5]; // Array with same value
    println!("Array: {:?}", array1);
    println!("Array: {:?}", array2);

    // Slices
    let slice1: &[i32] = &array1[0..3]; // Slice of array
    let slice2: &[i32] = &array1[1..4]; // Slice of array
    println!("Slice: {:?}", slice1);
    println!("Slice: {:?}", slice2);

    // Functions
    let sum = add_num(10, 20);
    println!("Sum: {}", sum);

    // Control Flow
    // If else
    let num = 10;
    if num > 0 {
        println!("Positive");
    } else if num < 0 {
        println!("Negative");
    } else {
        println!("Zero");
    }

    // If else with expression
    let num = if num > 5 { 5 } else { 10 };

    // Match
    match num {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other"),
    }

    // loops
    let mut i = 0;
    loop {
        if i == 5 {
            break;
        }
        println!("{}", i);
        i += 1;
    }

    let mut c = 0;
    let res = loop {
        c += 1;
        if c == 5 {
            break c;
        }
    };

    // While loop
    let mut j = 0;
    while j < 5 {
        println!("{}", j);
        j += 1;
    }

    // for loop
    for k in 0..5 {
        println!("{}", k);
    }

    // for loop array
    let array = [1, 2, 3, 4, 5];
    for i in array.iter() {
        println!("{}", i);
    }
}

fn add_num(x: i32, y: i32) -> i32 {
    x + y
}
