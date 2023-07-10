// https://www.reddit.com/r/linux/comments/9nr0r/what_are_some_good_terminal_games_like_nethack/
// structs
// ncurses TUI: "new game || load game || options || quit"
// inventory??

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

/* ========================================================================== */


enum Classes {
    warrior, thief, mage, priest,
}

// Built in, long cooldown, consume lotta mana
enum Ultraskill {
    warrior_wrath, thief_stealth, super_mana,
}

struct Character_stats {
    class: Classes,
    ultraskill: Ultraskill,
    // Go up automatically
    xp: u32,        // starts at 0
    level: u32,     // starts at 1
    vigor: u32,     // scales with class
    mana: u32,      // scales with class
    endurance: u32,     // governs: 'movement' + weight // movement as in DarkSouls: can sprint and can attack
    // Player can level 'em up
    strength: u32,
    dex: u32,
    faith: u32,
    intelligence: u32,
}

enum Weapons {
    dagger, straight_sword, greatsword, colossal_sword, thrusting_sword,
    curved_sword, curved_greatsword, katana, twinblade,
    axe, greateaxe, hammer, flail, great_hammer, colossal_weapon,
    spear, great_spear, halberd, reaper, whip, fists, claws,
    light_bow, bow, great_bow, crossbow, ballista,
    glintstone_staff, sacred_seal, torch,
}

enum Weapon_skills {
    wild_strikes, lion_claw, war_cry, // ashes of war lol
}

struct Weapon_stats {
    thrusting_damage: u32,
    strike_damage: u32,
    slash_damage: u32,
    poise_damage: u32,
    bleed: u32,
    toxic: u32,
    burn: u32,
    crit: u32,
    speed: <u32>,
    weight: <u32>,
    can_cast: bool,
    scaling: // ??
    weapon_skill: Weapon_skills,
    min_strength: u32
    min_dex: u32
    min_faith: u32
    min_int: u32
}

enum Spells {
    heal, heal_overtime, fireball, fire_aoe, lightning,
    magic, magic_aoe,buff_self, debuff_enemy,
}

/* ========================================================================== */

struct Character {
    active: bool,               // 3 deaths is game_over
    name: String,
    class: &'a Classes  // ?? -- other struct? enum? ... // includes stats + ultraskill1
    weapon1: Weapons,
    weapon2: Weapons,
    spells: Vec<&'a Spells>,    // can be anything from 0 to 4

}


/* ========================================================================== */


impl Character_situation {
    fn creation {}
    fn level_up {}
    fn change_weapon {}
}



fn game(){
    let mut high_s = String::new();
    println!("deez");
    loop {
        println!("Enter a name for your character:");
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
}



fn combat {}

fn death {}

fn npc_dialogue {}

// fn hunt {}