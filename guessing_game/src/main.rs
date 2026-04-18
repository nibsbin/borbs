use std::io;
fn main() {
    println!("Guess ze numbah");
    println!("Input ur guess");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("wtffff");
    println!("You guessed {guess}")
}
