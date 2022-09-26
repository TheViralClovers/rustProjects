use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //generate random number
    loop{ //entering the loop
        println!("Input your guess: ");

        let mut guess = String::new(); //assign an empty string object to guess variable
        
        io::stdin()
            .read_line(&mut guess) //read user number input
            .expect("Failed to read line"); //error whilst reading string
        
        let guess : u32 = match guess.trim().parse() { //Check whether the string we are using is a number
            Ok(num) => num, //If its a number, we accept it
            Err(_)=>continue, //If its not a number , we reject it
        };
        
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"), //equivalent to guess < secret_number
            Ordering::Greater => println!("Too big"), //equivalent to guess > secret_number
            Ordering::Equal => { //equivalent to guess == secret_number
                println!("You win!");
                break;
            }
        }
    }
}
