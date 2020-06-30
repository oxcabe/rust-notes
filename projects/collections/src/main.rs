use std::collections::HashMap;
use std::io::{self, Write};

const OPTION_LIST: bool = true;
const NO_OPTION_LIST: bool = false;

const UPDATE_EMPLOYEE_NAME: bool = true;
const UPDATE_EMPLOYEE_DEPARTMENT: bool = false;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

#[allow(unused_variables)]
fn main() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    let mut v3: Vec<i32> = Vec::new();

    v3.push(4);
    v3.push(5);
    v3.push(6);
    v3.push(7);

    read_vec_elements(v3);

    let v4 = vec![100, 32, 57];

    for i in &v4 {
        println!("{}", i);
    }

    let mut v5 = vec![100, 32, 57];

    // Iterating
    for i in &mut v5 {
        *i += 50;
    }

    // Using Enum to store multiple types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Strings

    let mut s = String::new();
    let data = "initial contents";

    let s = data.to_string();

    // also works on literals (str in general)
    let s = "initial contents".to_string();

    // other way
    let s = String::from("initial contents");

    // Strings are UTF-8 encoded
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str(" bar");

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and cannot longer be used

    let s1 = String::from("tic");
    let s1 = String::from("tac");
    let s1 = String::from("toe");

    let s = format!("{}-{}-{}!", s1, s2, s3);

    let hello = "Здравствуйте";
    let s = &hello[0..4]; // Use carefully - might panic depending on the String

    // Iterating over Strings
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for c in "नमस्ते".bytes() {
        println!("{}", c);
    }
    // Not available in the std
    // for c in "नमस्ते".graphemes() {
    //     println!("{}", c);
    // }

    // Hash Maps (key-value collections)
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter()
        .zip(initial_scores.into_iter())
        .collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // Because of ownership, fiel_name and field_value are invalid at this point
    // No `Copy` trait

    // Accessing HashMap values
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // Iterating
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Updating a HashMap
    // * Overwriting a value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores); // Old value is overwritten

    // * Only inserting a value if the key has no value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    // Only inserts value if the key doesn't have any already
    println!("{:?}", scores); 

    // * Updating a value based on the old one
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map); // Value increments on key ocurrences

    // Exercises
    let list = [1, 3, -2, 5, 30, 5, 4, -50, 0];

    println!("The mean for the list is {}", mean(&list));
    println!("The median for the list is {}", median(&list));
    println!("The mode for the list is {}", mode(&list));
    println!("Pig latin for {}: {}", "Hello", pig_latin("Hello"));
    println!("Pig latin for {}: {}", "first", pig_latin("first"));
    println!("Pig latin for {}: {}", "apple", pig_latin("apple"));

    println!("\n=====\n");
    text_interface();
}

// Reading elements of vectors
fn read_vec_elements(v: Vec<i32>) {
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // Vec::get returns Option<&T>
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

fn mean(list: &[i32]) -> i32 {
    list.iter().fold(0, |acc, x| acc + x)/ list.len() as i32
}

fn median(list: &[i32]) -> i32 {
    let mut sorted: Vec<i32> = list.to_vec();
    sorted.sort();
    if sorted.len() % 2 == 0  {
        mean(&[
            *sorted.get(sorted.len() / 2 - 1).unwrap_or(&0), 
            *sorted.get(sorted.len() / 2).unwrap()
        ])
    } else {
        *sorted.get(sorted.len() / 2).unwrap()
    }
}

fn mode(list: &[i32]) -> i32 {
    let mut map = HashMap::new();
    for num in list {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    let mut max = (0, std::i32::MIN);
    for (number, ocurrences) in &map {
        if *ocurrences > max.1 {
            max = (**number, *ocurrences);
        }
    }
    max.0
}

fn pig_latin(s: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut ret = String::from(s);
    let first_letter = ret.to_lowercase()
        .chars().nth(0).unwrap();

    let suffix;
    if vowels.contains(&first_letter) {
        suffix = String::from("h");
    } else {
        suffix = ret.remove(0).to_string().to_lowercase();
    }

    ret = format!("{}-{}ay", ret, suffix);
    ret
}

fn text_interface() {
    let mut map: HashMap<String, String> = HashMap::new();
    println!("Test Company Employee Interface v1.0");

    loop {
        print_menu();

        let option = input_text("Please, select an option: ");
        println!("");

        let option: u8 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match option {
            // List all employees
            1 => list_employees(&map, NO_OPTION_LIST),
            // Add new employee
            2 => {
                let employee_name = input_text("Enter employee name: ");
                let employee_department = input_text("Enter employee department: ");
                create_employee(&mut map, employee_name, employee_department);
            },
            // Modify employee
            3 => {
                list_employees(&map, OPTION_LIST);
                if !map.is_empty() {
                    // Select employee to update
                    let employee = input_text("Enter employee name: ")
                        .trim().to_string();

                    // Select which field to update
                    let update_choice_string = input_text("Select field to update (name,department): ");
                    let update_choice = match update_choice_string.to_lowercase().as_str() {
                        "name" => UPDATE_EMPLOYEE_NAME,
                        "department" => UPDATE_EMPLOYEE_DEPARTMENT,
                        _ => continue,
                    };

                    // Input new value
                    let new_value = input_text(format!("Select value for the field \"{}\": ",
                        update_choice_string.to_lowercase()).as_str());

                    // Update
                    update_employee(&mut map, update_choice, employee, new_value);
                }
            },
            // Remove employee
            4 => {
                list_employees(&map, OPTION_LIST);
                if !map.is_empty() {
                    let employee = input_text("Enter employee name: ")
                        .trim().to_string();
                    remove_employee(&mut map, employee);
                }
            },
            // Exit
            5 => break,
            _ => (),
        }
    };

    println!("Goodbye!");
}

fn print_menu() {
    println!("
[1] List all employees.
[2] Add new employee.
[3] Modify employee.
[4] Remove employee.
[5] Exit."
    );
}

fn input_text(text: &str) -> String {
    print!("{}", text);
    let _ = io::stdout().flush();
    let mut option = String::new();

    io::stdin().read_line(&mut option)
        .expect("Failed to read user input");

    option.trim().to_string()
}

fn list_employees(map: &HashMap<String, String>, as_option_list: bool) {
    if map.is_empty() {
        println!("There aren't any employees at the moment.");
    } else {
        println!("List of employees:");
        for (employee, department) in map {
            if as_option_list == true {
                println!("\tEmployee: {}\t| Department: {}", employee,
                    department);
            } else {
                println!("{}, from {}.", employee, department.to_lowercase());
            }
        }
    }
}

fn create_employee(map: &mut HashMap<String, String>, name: String,
    department: String) {
    map.entry(name).or_insert(department);
}

fn update_employee(map: &mut HashMap<String, String>, update_choice: bool,
    employee: String, update_value: String) {
    if map.contains_key(&employee) {
        let old_entry = map.remove_entry(&employee).unwrap();
        match update_choice {
            UPDATE_EMPLOYEE_NAME => {
                map.insert(update_value, old_entry.1);
            },
            UPDATE_EMPLOYEE_DEPARTMENT => {
                map.insert(old_entry.0, update_value);
            },
        }
    }
}

fn remove_employee(map: &mut HashMap<String, String>, employee: String) {
    if map.contains_key(&employee) {
        map.remove(&employee);
    }
}
