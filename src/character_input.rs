struct Person {
    name: String,
    age: u32
}

impl Person {
    fn centennial_year(&self) -> u32 {
        use chrono::{Utc, Datelike};
        Utc::now().year() as u32 - self.age + 100
    }
}

fn input(prompt: &str) -> String {
    use std::io::{self, Write};

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim_end().to_string()
}

fn print_results(person: &Person) {
    let message =
        if person.age < 100 {
            "You'll be 100 years old in"
        } else {
            "You turned 100 in"
        };

    println!("Hi, {}! {} {}. ✌️", person.name, message, person.centennial_year());
}

fn input_number(prompt: &str) -> u32 {
    use std::str::FromStr;
    u32::from_str(&input(prompt)).expect("Invalid number")
}

fn main() {
    let name = input("Name: ");
    let age = input_number("Age: ");
    let copies = input_number("Copies: ");
    let person = Person { name, age };

    (0..copies).for_each(|_| { print_results(&person); })
}
