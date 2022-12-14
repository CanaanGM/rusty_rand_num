use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
   // println!("rand num {}", secret_number);

    loop{
        let mut guess = String::new();
        println!("Plaeas input ur guess ! ");
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 =match guess.trim().parse()
        {
            Ok(num) => num, 
            Err(_) => continue,
        };
        println!("u've guessed {}", guess);        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("too great"),
            Ordering::Equal =>  {
                println!("You've Won");
                break;
            }
        }
    }
}
