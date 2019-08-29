use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){

    println!("Guess the number dear");

    let secret_number = rand::thread_rng().gen_range(1,101);

    loop {
        println!("Please input your Guess");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to readline");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}