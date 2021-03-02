use std::io;  // Use the io library from standard. Allows you to accept user input
use std::cmp::Ordering; // for Less, Greater, Equal
use rand::Rng; // We bring the Rng trait from the rand crate into this scope

fn main() { // Entry point to the program
    println!("Guess the numbers!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new(); // Creating a place to store the user input, as 'guess'. let is also used to create a variable
                                         // String::new(), returns a new instance of String. 'new' creates an empty string
    io::stdin()
        .read_line(&mut guess) // read_line takes user input and places it into a string. The & is a reference...(ch4)
        .expect("Failed to read line"); // part of Result return value, will get a warning without it
    
    // 'Shadowing' or reusing the guess var with a new one. Often used when you want to convert value types    
    let guess: u32 = guess.trim().parse().expect("Please try a number!");
        
    println!("You guessed: {}", guess); // One value per set of {}
// match is used to decide what to do based on which variant of Ordering is returned. It is made up of 'arms'
    match guess.cmp(&secret_number) {    // cmp compares two values, in this case 'guess' and 'secret number'.
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
