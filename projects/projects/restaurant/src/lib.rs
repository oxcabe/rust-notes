mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // Calls serve_order on line 36, even if it's private
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad
    }
}

pub fn breakfast_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("wheat");
    // meal.seasonal_fruit = String::from("blueberries"); // Wouldn't work - private
    println!("I'd like {} toast, please!", meal.toast);
}

pub fn meal_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order1 = back_of_house::Appetizer::Salad;
}

// use keyword : absolute path
use crate::front_of_house::hosting as hosting2;
// use keyword : relative path
// use front_of_house::hosting;

// Idiomatic way
// non idiomatic way would be using front_of_house::hosting::add_to_waitlist;
pub fn use_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// More idiomatics | Commented so they don't error
use std::fmt::Result;
use std::io::Result as IoResult;

// fn function1() -> Result {}
// fn function2() -> IoResult<()> {}

// Re-exporting names | scope gets private so pub is needed
// This is important for encapsulation and lib development
pub use crate::front_of_house::hosting as Hosting;

pub fn re_export_at_restaurant() {
    Hosting::add_to_waitlist();
    Hosting::add_to_waitlist();
    Hosting::add_to_waitlist();
}

// Nested paths
// use std::io;
// use std::cmp::Ordering;
// ===> use std::{cmp::Ordering, io};

// Another example:
// use std::io;
// use std::io::Write;
// ===> use std::io{self, Write};

// Glob operator | Good for tests | Prelude pattern
use std::collections::*;

// crate
// |
// --- front_of_house
//     |
//     --- hosting
//     |   |
//     |   --- add_to_waitlist
//     |   |
//     |   --- seat_at_table
//     --- serving
//         |
//         --- take_order
//         |
//         --- serve_order
//         |
//         --- take_payment

