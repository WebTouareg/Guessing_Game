use std::io;

/*************  ✨ Command ⭐  *************/
/// This is the main function of the program. It will print a message asking the user
/// for a guess, read that guess from the standard input, and then print out the
/// guess it received.
/*******  d63a1b81-33f0-4269-b47f-6dbb34ed694c  *******/

fn main () {
    println!("Guess the number");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

// use std::io;

// Original code from the Rust Book
// fn main() {
//     println!("Guess the number!");
//
//     println!("Please input your guess.");
//
//     let mut guess = String::new();
//
//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");
//
//     println!("You guessed: {guess}");
// }