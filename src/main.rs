use std::io;  // Use the io library from standard. Allows you to accept user input

fn main() { // Entry point to the program
    println!("Guess the numbers!");

    println!("Please input your guess.");

    let mut guess = String::new(); // Creating a place to store the user input, as 'guess'. let is also used to create a variable

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {}", guess);
}
