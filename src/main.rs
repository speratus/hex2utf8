use std::borrow::Borrow;
use std::io;
use std::str;

fn main() -> io::Result<()>{
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    println!("Please enter a two digit hex number");
    stdin.read_line(&mut buffer)?;
    println!("Your hex number: {}", &buffer[0..2]);
    let parsed = u8::from_str_radix(&buffer[0..2], 16);
    let p = parsed.expect("Failed to parse hex number");
    let char = String::from_utf8(vec![p]).expect("Invalid UTF-8");
    println!("Your Char: {}", char);
    Ok(())
}
