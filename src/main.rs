mod rock_paper_scissors;
mod supdawg;
use std::io;

fn main() {
    loop {
        let mut option = String::new();
        println!("\nPlease choose an option: ");
        println!("1 - Rock papers scisors");
        println!("2 - Supdawg");
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
        println!("You chose option {}", option);

        match option {
            1 => {
                println!("Loading game: rock paper scissors");
                rock_paper_scissors::play();
            }
            2 =>  {
                println!("Loading game: supdawg");
                supdawg::play();
            }
            _ => println!("WIP - Not yet implemented"),
        }
    }
}
