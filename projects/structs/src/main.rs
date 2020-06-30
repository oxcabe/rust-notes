struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple-like struct
struct Color(u8, u8, u8);
struct Point(i32, i32, i32);

// Unit-like struct
struct Unit();

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Methods (they use &self)
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width 
            && self.height > rect.height
    }

    // Associated functions (without &self)
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

// Field init shorthand
fn build_new_user(username: &str,
    email: &str, active: bool) -> User {
    User {
        username: String::from(username),
        email: String::from(email),
        sign_in_count: 1,
        active,
    }
}

fn print_user(user: User) {
    println!("- User -\n\
              Username:         {}\n\
              E-mail:           {}\n\
              Active:           {}\n\
              Sign-in count:    {}",
        user.username, user.email, user.active,
        user.sign_in_count);
}

fn example_program() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn example_program_tuple() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn example_program_struct() {
    let rect = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 40, height: 40 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect)
    );

    println!("rect is {:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels.",
       rect.area() 
    );

    println!(
        "Can rect hold rect2?: {}. \
        And rect3?: {}.", 
        rect.can_hold(&rect2),
        rect.can_hold(&rect3)
    );

    // Associated function syntax goes with ::, not .
    let square = Rectangle::square(10);

    println!(
        "Area of square ({}) is: {}",
        square.width, square.area()
    );
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[allow(unused_variables)]
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("someone1@example.com");

    let user2 = build_new_user("someusername456",
        "someone2@example.com", true);

    // Struct update syntax
    let user3 = User {
        email: String::from("someone3@example.com"),
        username: String::from("anotherusername789"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    let unit = Unit();

    print_user(user1);
    print_user(user2);
    print_user(user3);

    example_program();
    example_program_tuple();
    example_program_struct();
}
