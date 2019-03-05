use std::str;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let characters: &[u8] =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()-]=+.<,>:~".as_bytes();
    let mut rng = thread_rng();
    let password_length = 8;
    let chars: Vec<u8> =
        characters.choose_multiple(&mut rng, password_length).
        cloned().collect();

    println!("{}", str::from_utf8(&chars).unwrap());
}
