use std::fs::{self, File};
// main method can only return nothing or a Result
fn main() -> Result<(), std::io::Error> {
    // Result enum
    let result: Result<i32, &str> = Ok(42);
    print!("Result: {:?} ", result.unwrap());
    match result {
        Ok(value) => println!("Result is: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    // If value is there it will unwrap else panic
    let result = result.unwrap();

    // Error Kind
    let f = File::open("non_existent_file.txt");
    let f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => match File::create("non_existent_file.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {}", e),
            },
            _ => {
                panic!("Other error: {}", e)
            }
        },
    };
    print!("File opened successfully: {:?}", f);

    // Return value for main()
    Ok(())

    // Panic macro
    // a();
}

// fn read_username_from_file() -> Result<String, std::io::Error> {
//     let f = File::open("hello.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// Optional chaining
fn read_username_from_file() -> Result<String, std::io::Error> {
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    // Using chaining
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    // Using direct read_to_string method
    fs::read_to_string("hello.txt")
}

fn a() {
    b(12);
}

fn b(num: i32) {
    if num == 22 {
        panic!("This is a panic message with number: {}", num);
    }
}
