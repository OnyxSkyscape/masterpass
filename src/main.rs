use masterpass::generate_passphrase;
use rpassword;
use std::io::{self, Write};

#[allow(dead_code)]
fn ask_for_passphrase(prompt: &str) -> String {
    let input = rpassword::prompt_password(prompt).unwrap();
    input.trim().to_owned()
}

fn ask_for_input(prompt: &str) -> String {
    let mut buf = String::new();
    print!("{}", prompt);
    io::stdout().flush().expect("Couldn't flush stdout");
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_owned()
}

fn main() {
    let master_passphrase = ask_for_passphrase("Master passphrase: ");
    let service = ask_for_input("Service name: ");
    let secret = ask_for_input("Unique secret (optional): ");
    let length = ask_for_input("Length of the passhrase (in words): ");

    let length = length.parse::<u16>().unwrap();
    let service_passphrase = generate_passphrase(master_passphrase, service, secret, length);
    println!("{}", service_passphrase);
}
