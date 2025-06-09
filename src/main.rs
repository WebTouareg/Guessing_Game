use std::io;

/*************  ✨ Command ⭐  *************/
/// This is the main function of the program. It will print a message asking the
/// userfor a guess, read that guess from the standard input, and then print out
///  the guess it received.

// Old method for arguments before using the macro format_args! in 2022 release
// println!("You guessed: {guess}"); vs println!("You guessed: {}", guess);

//fn main () {
//    println!("Guess the number");
//
//    println!("Please input your guess.");
//
//    let mut guess = String::new();
//
//    io::stdin()
//        .read_line(&mut guess)
//        .expect("Failed to read line");
//
//    println!("You guessed: {}", guess);
//}

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
