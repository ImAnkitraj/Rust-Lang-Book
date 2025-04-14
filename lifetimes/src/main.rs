// Traits and Lifetimes
fn longest_with_an_announcement<'a, T>(s1: &'a str, s2: &'a str, ann: T) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement! {}", ann);
    if s1.len() > s2.len() { s1 } else { s2 }
}

// Lifetime rules
// 1. Each parameter that is a reference gets its own lifetime parameter
// 2. if there is only one input lifetime, the output lifetime is the same as the input lifetime
// 3. if there are multiple input lifetimes,
//     but one of them is &self or &mut self the lifetime of the self is assigned to all the output lifetime parameter
// 4. if there are multiple input lifetimes, the output lifetime is the shortest of the input lifetimes

fn main() {
    // static lifetimes
    let s: &'static str = "hello world"; // 'static lifetime
}

fn lifetime1() {
    let s1 = String::from("hello");
    let s2 = String::from("world");

    let result = get_longest(&s1, &s2);
    println!("The longest string is {}", result);
}

fn lifetime2() {
    let s1 = String::from("hello");
    {
        let s2 = String::from("world");

        let result = get_longest(&s1, &s2);
        println!("The longest string is {}", result);
    }
}

// fn lifetime2() {
//     let s1 = String::from("hello");
//     let result;
//     {
//         let s2 = String::from("world");

//         result = get_longest(&s1, &s2); // error: `s2` does not live long enough
//     }
//     println!("The longest string is {}", result);
// }

// generics lifetimes
// lifetime of the return value would be same as the smalled lifetime of the arguments
fn get_longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

fn dingling() {
    let p;
    {
        let x = 5;
        p = &x;
    }
    print!("p: {}", p);
}

fn not_dangling() {
    let x = 5;
    let p = &x;
    print!("p: {}", p);
}

// struct with lifetime
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn important_excerpt() {
    let s: String = String::from("hello.world");
    let first_word = s.split('.').next().unwrap();
    let i = ImportantExcerpt { part: first_word };
    println!("i: {}", i.part);
}
