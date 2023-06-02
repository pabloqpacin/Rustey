// https://www.reddit.com/r/linux/comments/9nr0r/what_are_some_good_terminal_games_like_nethack/
// structs

use std::io;
use rand::Rng;
use std::thread;
use std::time::Duration;


pub fn play(){
    println!("\nWelcome to 'RPG-CLI'");
    println!("  yadda yadda");
    println!("  yadda yadda");
    let mut play_quit = String::new();
    let mut should_continue = true;
    loop {
        // println!("Enter 'p' to play or 'q' to quit");
        println!("Are you ready for an adventure? [y/n]");
        io::stdin().read_line(&mut play_quit).expect("Failed to read line");       
        if play_quit.trim() == "y" {
            break;
        } else if play_quit.trim() == "n" {
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
            thread::sleep(Duration::from_secs(1));
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


fn game(){
    println!("deez");
}