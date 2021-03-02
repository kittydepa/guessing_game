use std::io;  // Use the io library from standard. Allows you to accept user input

fn main() { // Entry point to the program
    println!("Guess the numbers!");

    println!("Please input your guess.");

    let mut guess = String::new(); // Creating a place to store the user input, as 'guess'. let is also used to create a variable
                                         // String::new(), returns a new instance of String. 'new' creates an empty string
    io::stdin()
        .read_line(&mut guess) // read_line takes user input and places it into a string. The & is a reference...(ch4)
        .expect("Failed to read line"); // part of Result return value, will get a warning without it
    
    println!("You guessed: {}", guess); // One value per set of {}
}
