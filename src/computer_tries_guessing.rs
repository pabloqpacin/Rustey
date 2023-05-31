use std::io;
// use rand::Rng;

fn guessing(low: &mut u32, high: u32){
    println!("\nlow is {} - high is {}", low, high);
}

fn game(){
    let mut high_s = String::new();
    loop {
        high_s.clear();
        println!("What should be the highest number in the guessing range?");
        io::stdin().read_line(&mut high_s).expect("Failed to read line");
        let high: u32 = match high_s.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a positive integer");
                continue;
            }
        };
        break;
    }
    let mut low = Box::new(1);  
    let mut high: u32 = high_s.trim().parse().expect("Invalid input");
    println!("Starting game with guessing range: 1 - {}", high);
    println!("Please provide feedback to the computer guesses:");
    println!("'l' for low guess");
    println!("'h' for high guess");
    println!("'f' for correct guess");
    guessing(&mut low, high);
}



pub fn play(){
    println!("\nWelcome to 'computer tries guessing'");
    println!("You'll think a number in between 1 and whatever you decide and");
    println!("the computer will try to guess it with a bit of your help...");
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
                continue;
            }
        }
    }
}


/*
pub fn guessing(x,y){
    let mut feedback = String::new();
    println!("Now just think a number between 1 and {} and give the right feedback!", high);
    println!("Feedback: 'h' for high - 'l' for low - 'f' if guessed correctly!");

    while feedback != "f" {
        if high == low { 
            println!("Your number is {} *wink wink*", low);
            break;
        }
        
        let mut guess: i32 = rand::thread_rng().gen_range({low}..={high});
        println!("Beep-boop. I guess... {}", guess);

    }

}
*/