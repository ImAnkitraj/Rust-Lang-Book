use enum_mod::Appetizer;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_wiatlist() {
            println!("Adding to waitlist");
        }
        fn seat_at_table() {
            println!("Seating at table");
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_wiatlist();

    // Relative path
    front_of_house::hosting::add_to_wiatlist();
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

mod struct_back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    fn access_breakfast_struct_in_same_module() {
        let mut meal = Breakfast {
            toast: String::from("Rye"),
            seasonal_fruit: String::from("blueberries"),
        };
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);
    }

    impl Breakfast {
        pub fn summer(toast: String) -> Breakfast {
            Breakfast {
                toast,
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

fn eat_at_restaurant_struct() {
    let mut meal = struct_back_of_house::Breakfast::summer(String::from("Rye"));
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // struct_back_of_house::Breakfast {
    // toast: String::from("Wheat")
    // seasonal_fruit: String::from("blueberries"), // Error! seasonal_fruit is not public
    // };
}

// Enums
mod enum_mod {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn eat_at_restaurant_enum() {
    let soup = enum_mod::Appetizer::Soup;
}

mod use_mod {
    pub mod self_mod {
        pub fn eat() {
            println!("Eating");
        }
    }
}

use self::use_mod::self_mod; // use the eat function from self_mod
fn eat_at_restaurant_use() {
    // use_mod::self_mod::eat();
    // or

    self_mod::eat();
}

// as keyword
use std::fmt::Result as FmtResult;
use std::io::Result;

// mod content ion diff file

mod inline_module;
