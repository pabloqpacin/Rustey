/*
TODO:
- Vector rather than Array? any other structure?
- accept rock paper scissor as well as r p s
- use emojis
*/

use std::io;
use rand::Rng;

pub fn play(){
    println!("\nWelcome to: rock paper scissors");

    const OPTIONS: [char; 3] = ['r', 'p', 's'];
    let mut count: i32 = 0;

    while count < 3 {

        let mut computer_index = rand::thread_rng().gen_range(0..3);
        let mut computer_choice = OPTIONS[computer_index];
        println!("Computer choice is: {}", computer_choice);
        
        let mut user: char = 'z';   // placeholder
        println!("Please enter r for rock, p for paper or s for scissors");
        // io::stdin()
        //     .read_char(&mut user)
        //     .expect("Failed to read line");     // ??   
        
        println!("User choice is: {}", user);

        count+=1;
    }



}