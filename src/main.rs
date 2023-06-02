// mod rock_paper_scissors;
// mod computer_tries_guessing_v2;
mod computer_tries_guessing;
// mod hangman;
mod rpg_cli;
mod supdawg;
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    loop {
        let mut option = String::new();
        println!("\nPlease choose an option: ");
        println!("1 - Rock papers scisors");
        println!("2 - Supdawg");
        println!("3 - Computer tries guessing");
        println!("4 - Hangman");
        println!("5 - RPG-CLI");
        println!("q - Quit");

        io::stdin()
            .read_line(&mut option)
            .expect("Failed ro read line");     // ??

        let option: u32 = match option.trim().parse() {
            Ok(num) => {
                if num >= 1 && num <= 5 {   // if (1..=5).contains(&num) {
                    num
                } else {
                    println!("Please choose a valid option");
                    continue;    
                }
            }
            Err(_) => {
                if option.trim() == "q" {
                    println!("You chose to quit");
                    break;
                }
                println!("Please choose a valid option");
                continue;
            }
        };
        println!("Loading game {}...", option);
        thread::sleep(Duration::from_secs(1));
        match option {
            // 1 => rock_paper_scissors::play();
            2 => {supdawg::play();}
            3 => {computer_tries_guessing::play();}
            // 4 => {hangman::play();}
            5 => {rpg_cli::play();}
            _ => println!("WIP - Not yet implemented"),
        }
    }
}
