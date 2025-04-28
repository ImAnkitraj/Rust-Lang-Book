#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // Immutable
    let user1 = User {
        email: String::from("abc@xyz.com"),
        username: String::from("abc"),
        sign_in_count: 1,
        active: true,
    };

    let name = user1.username;

    // Mutable
    let mut user2 = User {
        email: String::from("xyz@abc.com"),
        username: String::from("abc"),
        sign_in_count: 1,
        active: true,
    };

    user2.username = String::from("xyz");

    // Struct update syntax
    let user3 = build_user("pqr@rst.com".to_string(), "pqr".to_string());

    let user4 = User {
        email: String::from("rst@pqr.com"),
        ..user3
    };

    // Tuple Struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let area = calculate_area(&rect);
    println!("Area: {}", area);
    let area = rect.area();
    println!("Area: {}", area);
    println!("Reactangle: {:#?}", rect);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 50,
        height: 60,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(10);
    println!("Square: {:#?}", square);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}

fn calculate_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
