use std::io;
fn main() {
    let choice:&str = loop {
        println!("Select '1' or '2':\n(1) Farenheight to Celsius\n(2) Celsius to Farenheight");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("wtf");

        let guess:&str = match choice.as_str().trim() {
            "1" => {
                println!("Enter your Farenheight value: ");
                "1"
            },
            "2" => {
                println!("Enter your Celsius Value: ");
                "2"
            },
            other => {println!("{} is an invalid option", other); continue},
        };
        break guess
    };

    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("???");

    let num: f64 = match num.trim().parse() {
        Ok(x) => x,
        Err(_) => {
            println!("Must be float");
            0.0
        }
    };

    if choice == "1" {
        println!("{num} converted to celsius is {}",far2cel(num))
    } else {
        println!("{num} converted to farenheight is {}", cel2far(num))
    };
}

fn far2cel(far: f64) -> f64 {
    (far - 32.0) * 5_f64 / 9.0
}

fn cel2far(cel: f64) -> f64 {
    (cel) * 9_f64 / 5.0 + 32.0
}
