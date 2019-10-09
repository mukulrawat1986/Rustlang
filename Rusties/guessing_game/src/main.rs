use std::io;

fn main() {
    println!("Guess the number");

    println!("Please input your guess");

    let mut guess = String::new();

    // io::stdin().read_line(&mut guess)
        // .expect("Failed to read line");

    match io::stdin().read_line(&mut guess) {
        Ok(n) => {
            println!("Bytes read: {}", n);
            println!("You guessed: {}", guess)
        }
        Err(error) => println!("Error: {}", error),
    }

    // println!("You guessed: {}", guess);

}