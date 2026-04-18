use rand::RngExt;
use std::cmp::Ordering;
use std::io;
fn main() {
    let mut rng = rand::rng();
    let secret_number = rng.random_range(1..=10);
    println!("Secret number: 69420. Jk, no cheating");

    loop {
        println!("Input ur guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("wtffff");
        println!("You guessed {guess}");

        //convert to num
        let guess: u32 = match guess.trim().parse() {
            Ok(asdf) => asdf,
            Err(_) => {println!("must be positive integer"); continue;}
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("tew smol"),
            Ordering::Greater => println!("tew big"),
            Ordering::Equal => {
                println!("just right");
                break;
            }
        }
    }
}
