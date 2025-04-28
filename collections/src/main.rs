use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;
fn main() {
    let a = [1, 3];
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);

    // vec macro
    let v2 = vec![1, 2, 3];
    print!("{}", v2[1]);
    print!("{}", &v2[1]);

    match vec.get(10) {
        Some(x) => println!("{}", x),
        None => println!("None"),
    };

    for i in &v2 {
        println!("{}", i);
    }

    let mut v3 = vec![1, 2, 3];

    for i in &mut v3 {
        *i += 10;
    }

    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(10),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("Hello")),
    ];

    match &row[0] {
        SpreadSheetCell::Int(i) => println!("Int: {}", i),
        SpreadSheetCell::Float(f) => println!("Float: {}", f),
        SpreadSheetCell::Text(s) => println!("Text: {}", s),
    }

    // Strings

    let mut s1 = String::new();
    let s2 = "Hello".to_string();
    let s3 = String::from("Hello");

    s1.push_str("Hello");
    s1.push(char::from(10));

    let s4 = String::from("Hello");
    let s5 = String::from("World");
    let s6 = s4 + &s5; // s4 is moved here, so it can't be used 
    let s7 = format!("{} {}", s5, s6); // It does not take any ownerships

    let s8 = String::from("नमस्ते");

    for c in s8.bytes() {
        print!("{} ", c);
    }

    print!("\n");
    for c in s8.chars() {
        print!("{} ", c);
    }
    print!("\n");
    for c in s8.graphemes(true) {
        print!("{} ", c);
    }

    // Hash Map
    let blue = String::from("Blue");
    let mut h: HashMap<&String, i32> = HashMap::new();

    h.insert(&blue, 1);

    let val = h.get(&blue);
    println!("hash map {} ", val.unwrap());

    let yellow = String::from("Yellow");
    h.entry(&yellow).or_insert(10);
    h.entry(&yellow).or_insert(30);

    let word = String::from("Hello world , this is my world");

    let mut h: HashMap<&str, i32> = HashMap::new();
    for word in word.split_whitespace() {
        let count = h.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?} ", h);
    return;
}
