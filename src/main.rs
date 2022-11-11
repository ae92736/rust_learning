use std::io;

fn main() 
{
    println!("Hello user!");
    println!("Input something...");
    let mut str: String = String::new();
    io::stdin().read_line(&mut str).expect("Failed to read line");
    println!("your input is {}", str)

}