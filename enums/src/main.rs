use std::sync::OnceState;

enum IpAddrType {
    V4,
    V6,
}

struct IpAdd {
    kind: IpAddrType,
    address: String,
}

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum IndianState {
    Maharashtra,
    Gujarat,
    Rajasthan,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(IndianState),
}

// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    let ipv4 = IpAddrType::V4;
    let ipv6 = IpAddrType::V6;

    let ipv = IpAddrKind::V4(1, 2, 3, 4);

    let ip = IpAdd {
        kind: IpAddrType::V4,
        address: String::from("127.0.0.1"),
    };

    // Option<T> enum usage
    let some_number = Some(5);
    let some_string = Some("Hello");
    let absent_number: Option<i32> = None;

    let x = 5;
    let y = Some(10);
    let z = x + y.unwrap_or(0); // Unwraps the value inside Some or returns 0 if None

    // Matching on enums
    let coin = Coin::Quarter(IndianState::Maharashtra);
    let value = find_value_in_cents(coin);
    println!("The value of the coin is: {}", value);

    // Using match with Option
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Five plus one: {:?} {:?}", six, none);

    // With if let
    let some_value = Some(5);
    if let Some(5) = some_value {
        println!("Matched with 5");
    } else {
        println!("Did not match with 5");
    }
}

fn find_value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            print!("Quarter from {:?}\n", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None, // This matches all other enum values other than Some(i)
    }
}
