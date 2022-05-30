use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let rnd = rand::thread_rng().gen_range(1..101);

    println!("real {} ", rnd);

    loop {
        let mut guess = String::new();
        println!("guess a number : ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please type a number");
                continue;
            }
        };

        match guess.cmp(&rnd) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("winnn <3");
                break;
            }
        }
        println!("you guessed: {}", guess);
    }
}
