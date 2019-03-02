pub fn input(prompt: &str) -> String {
    use std::io::{self, Write};

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim_end().to_string()
}

#[allow(dead_code)]
pub fn input_yes_no(prompt: &str) -> Result<bool, String> {
    match &input(prompt)[0..1] {
        "y" | "Y" => Ok(true),
        "n" | "N" => Ok(false),
        _ => Err("Invalid input. Allowed values: y, n".to_string())
    }
}
