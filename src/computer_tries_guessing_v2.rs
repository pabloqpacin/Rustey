use std::io;
use rand::Rng;

fn guessing(low: &mut u32, high: &mut u32){
    let mut feedback = String::new();
    let mut guess = rand::thread_rng().gen_range(*low..=*high);
    while feedback.trim() != "f" {
        // println!("--Current guess is between {} and {}--", low, high);   // COMMENT THIS OUT
        println!("Beep-boop. I guess... {}", guess);
        loop {
            feedback.clear();
            io::stdin().read_line(&mut feedback).expect("Failed to read line");
            if feedback.trim() == "l" {
                *low = guess + 1;
                break;
            } else if feedback.trim() == "h" {
                *high = guess - 1;
                break;
            } else if feedback.trim() == "f" {
                println!("Well played!");
                break;
            } else {
                println!("Please enter 'l' if guess was too low,");
                println!("'h' if too high or 'f' if it was correct");
                continue;
            }
        }
        guess = rand::thread_rng().gen_range(*low..=*high);
        if high == low {
            println!("Your number is {}!", low);
            break;
        }
    }
}


fn game(){
    let mut high_s = String::new();
    loop {
        high_s.clear();
        println!("What should be the highest number in the guessing range?");
        io::stdin().read_line(&mut high_s).expect("Failed to read line");
        let _high = match high_s.trim().parse() {
            Ok(num) => Box::<u32>::new(num),
            Err(_) => {
                println!("Please enter a positive integer");
                continue;
            }
        };
        break;
    }
    let mut low: Box<u32> = Box::new(1);  
    let mut high = high_s.trim().parse().expect("Invalid input");
    println!("Starting game with guessing range: 1 - {}", high);
    println!("  THINK YOUR SECRET NUMBER");
    println!("Please provide feedback to the computer guesses:");
    println!("'l' for low guess");
    println!("'h' for high guess");
    println!("'f' for correct guess\n");
    guessing(&mut low, &mut high);
}
/*
4.2 -- let mut vec: Vec<i32> = vec![1, 2, 3]; let num: &mut i32 = &mut vec[2];
*/

pub fn play(){
    println!("\nWelcome to 'computer tries guessing'");
    println!("  You'll think a number in between 1 and whatever you decide and");
    println!("  the computer will try to guess it with a bit of your help...");
    let mut play_quit = String::new();
    let mut should_continue = true;
    loop {
        println!("Enter 'p' to play or 'q' to quit");
        io::stdin().read_line(&mut play_quit).expect("Failed to read line");       
        if play_quit.trim() == "p" {
            break;
        } else if play_quit.trim() == "q" {
            should_continue = false;
            break;
        } else {
            println!("Please enter a valid option");
            play_quit.clear();
            continue;
        }
    }
    loop {
        if !should_continue {
            println!("Closing game");
            break;
        }
        game();       
        loop {
            println!("\nEnter 'p' to play again or 'q' to quit");
            play_quit.clear();
            io::stdin().read_line(&mut play_quit).expect("Failed to read line");
            if play_quit.trim() == "p" {
                break;
            } else if play_quit.trim() == "q" {
                should_continue = false;
                break;
            } else {
                println!("Please enter a valid option");
                play_quit.clear();
                continue;
            }
        }
    }
}
