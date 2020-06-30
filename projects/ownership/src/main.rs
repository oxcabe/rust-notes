#[allow(unused_variables)]
fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world!");

    let mut t = s.clone();
    t.push_str(" - oxcabe");

    println!("{}", s);
    println!("{}", t);

    test_ownership_one();
    test_ownership_two();

    let (s2, len) = calculate_length(s);
    println!("The length of {} is {}.", s2, len);

    // Using references
    println!("The length of {} is {}.", s2, calculate_length_ref(&s2));

    let s1 = String::from("hello");
    let len = calculate_length_ref(&s1);

    let mut s1 = String::from("Hello, ");
    append_end(&mut s1);

    // Scopes example
    {
        let r1 = &mut s1;
    }
    let mut r2 = &mut s1;

    // This is legal somehow
    let r3 = &r2;
    // let r4 = &mut r2;

    // It's not legal, it blows up here
    // println!("{} {} {}", r2, r3, r4);
    println!("{}", r2);

    let r4 = &mut r2;
    println!("{}", r4);

    // Slice type
    let s = String::from("Hello rusty world");
    println!("First word of \"{}\" ends at \
             position {}.",
        s, first_word(&s));

    let hello = &s[0..5];       // or &s[..5];
    let world = &s[6..11];      // or &s[6..];
    let slice = &s[0..s.len()]; // or &s[..];

    println!("First word of \"{}\" is \
             {}. Second word is {}.",
        s, first_word_slices(&s),
        second_word_slices(&s));
}

fn first_word(s: &String) -> usize {
    for (i, &c) in s
        .as_bytes().iter().enumerate() {
        if c == b' ' {
            return i;
        }
    }
    s.len()
}

// Can take String, str and slices (str as well) as arguments
fn first_word_slices(s: &str) -> &str {
    for (i, &c) in s
        .as_bytes().iter().enumerate() {
            if c == b' ' {
                return &s[..i];
            }
        }

    &s[..]
}

fn second_word_slices(s: &str) -> &str {
    let mut index_first = 0;
    for (i, &c) in s
        .as_bytes().iter().enumerate() {
            if c == b' ' {
                if index_first == 0 {
                    index_first = i + 1;
                } else {
                    return &s[index_first..i];
                }
            }
        }
    &s[..]
}

fn test_ownership_one() {
    let s = String::from("Hello");
    let x = 5;

    takes_ownership(s);
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

#[allow(unused_variables)]
fn test_ownership_two() {
    let s1 = gives_ownership();
    let s2 = String::from("Hello"); // moves into s3 after takes_and_gives_back
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("gives_ownership");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// We have to move the string into the function. Solution => use references
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn append_end(string: &mut String) {
    string.push_str("world.");
}
