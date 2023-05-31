use std::io;
use std::thread;
use std::time::Duration;

pub fn play(){
    println!("\nWelcome to supdawg");

    loop {
        let mut option = String::new();
        println!("Enter 'supdawg' to play or 'naw' to quit");

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");     // ??
        
        if option.trim() == "supdawg" {
            println!("not much wbu");
            break;
        } else if option.trim() == "naw" {
            break;
        } else {
            println!("Please enter a valid option");
        }
    }
    println!("Thanks for playing dawg!");
    thread::sleep(Duration::from_secs(2));

}