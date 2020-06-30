use std::cmp::Ordering;

#[allow(unused_variables)]
fn main() {

    // Variables, mutability
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 200_000;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
    let order;

    match x.cmp(&MAX_POINTS) {
        Ordering::Less => {
            order = "smaller";
        }
        Ordering::Equal => {
            order = "equal";
        }
        Ordering::Greater => {
            order = "bigger";
        }
    }
    println!("x ({}) is {} than MAX_POINTS ({})", x, order, MAX_POINTS);

    // Shadowing - Useful case
    let spaces = "     ";
    let spaces = spaces.len();

    println!("There is a total of {} space(s)!", spaces);

    // Data types
    let guess: u32 = "42".parse().expect("Not a number!");

    let float: f32 = 3.0;
    let remainder: f32 = 50.0 % float;

    let t = true;
    let f: bool = false;

    let c = 'c';
    let z: char = 'z';
    let utf_eight = "ñ";

    // Compound data types
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("a: {}, b: {}, c: {}", a, b, c);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // Array
    let num_arr = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    let sized_num_arr: [i32; 5] = [1, 2, 3, 4, 5];
    let three_arr = [3; 5]; // [3, 3, 3, 3, 3] Remember the '='

    let first_month = months[0];
    println!("First month: {}", first_month);

    for month in months.iter() {
        println!("Month: {}", month);
    }

    println!();

    another_function(5, 6);

    let x = 5;

    let z = {
        let x = 3;  // Statement
        x + 1       // Expression
    };

    println!("Value of z: {}", z);
    println!("Value of five(): {}", five());
    println!("Value of x = {} + 1: {}", x, plus_one(x));

    println!();

    // Conditionals
    if z % 2 == 0 {
        println!("variable z is even");
    } else {
        println!("variable z is odd");
    }

    if z != 0 {
        println!("variable z is something other than zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    // Using "if" in a "let" statement
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    // Note: this can't be done with mismatched types.
    // Example: 5 and "six" <-- wrong!
    println!("The value of number is: {}", number);

    // Loops
    let mut a: u8 = 1;

    let result = loop {
        println!("again!");

        if a == 10 {
            break a * 2;
        }

        a += 1;
    };

    println!("Value of result is {}.", result);

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("Go!");
}

// Functions
fn another_function(x: i32, y: i32) {
    println!("Value of x: {}", x);
    println!("Value of y: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
