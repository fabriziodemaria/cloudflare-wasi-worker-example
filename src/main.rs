use std::io::{self, Write};

fn main() {
    // Prompt the user for input
    print!("Enter some text: ");
    io::stdout().flush().unwrap(); // Make sure the prompt is displayed immediately
    
    // Read the input from stdin
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Remove any trailing newline characters
    let input = input.trim();

    // Print the input
    println!("Received message: {}", input);
}
