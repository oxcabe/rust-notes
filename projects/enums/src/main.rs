enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddrV2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor (u8, u8, u8),
}

impl Message {
    fn call(&self) {
        // method body
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(coin: &Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }
}

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(u8, u8, u8);

// Matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[allow(unused_variables)]
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddrV2::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option
    let some_number = Some(5);
    let some_string = Some("a string");
    
    let absent_number: Option<i32> = None;

    // match control flow operator
    let quarter: Coin = Coin::Quarter(UsState::Alaska);
    println!("Value: {}", Coin::value_in_cents(&quarter));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(1) => println!("one"),
        Some(3) => println!("three"),
        _ => (), // do nothing for default cases
    }
    // using if let
    if let Some(3) = some_u8_value {
        println!("three");
    }

    // {if,else} let
    let mut count = 0;
    if let Coin::Quarter(state) = quarter {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
