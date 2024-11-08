use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess blahaj's number!");
    let blahajs_num = rand::thread_rng().gen_range(1..=100);
    
    //println!("Blahaj's number is: {blahajs_num}");
    
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Blahaj didn't understand :(");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&blahajs_num) {
            Ordering::Less => println!("Awww, too small!"),
            Ordering::Greater => println!("Awww, to big!"),
            Ordering::Equal => {
                println!("Wow, blahaj is proud!");
                break;
            }
        }
    }  
}
