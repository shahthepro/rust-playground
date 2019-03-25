use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Guess a number:");

        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
            .expect("Enter a number");

        let guess: u32 = guess.trim().parse()
            .expect("Not a valid number");


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!!!");
                break;
            },
        }
    }
    
}
