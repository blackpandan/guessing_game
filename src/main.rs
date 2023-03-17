use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number: u8 = rand::thread_rng().gen_range(0..=5);

    println!("\nHello guess a Number from 0 to 5");

    loop {
        let mut guessed_word: String = String::new();
        println!(
            "\n
---------------------------
---------------------------
Please input your guess: "
        );

        io::stdin()
            .read_line(&mut guessed_word)
            .expect("Failed to readline");

        let guessed_word: u8 = match guessed_word.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Guess a Number from 0 to 5. !!!!");
                continue;
            }
        };

        println!(
            "
you guessed: {guessed_word}
"
        );

        match guessed_word.cmp(&secret_number) {
            Ordering::Less => println!(
                "\nYou guessed too small !!!!
---------------------------
"
            ),
            Ordering::Greater => println!(
                "\nYou guessed too big !!!
---------------------------"
            ),
            Ordering::Equal => {
                println!(
                    "\nYou guessed the right number!!
---------------------------"
                );
                break;
            }
        }
    }
}
