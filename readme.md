# cli_games_prepy

Basically a rework of some [Terminal Games](https://github.com/pabloqpacin/SLIT/tree/main/src/1-pythonTerminalGames) written in Python following a tutorial.

## 1. rock papers scissors

- add **rand** to Cargo.toml and `cargo build` straight away
- implement `mod otherfiles.rs`, a loop with error handling and a `match` construct
- experiment with `sudpawg.rs` -- looking good dawg!
- kinda stuck with user input for rock_paper_scissors: chars VS strings; also how to store the options? LEFT TODO
- ...
- start computer_tries_guessing: looking good, main game loop good, now just the logic left
  - WIP: MEMORY MANAGEMENT 
- complete CTG. I should have a review because I'm not sure about the datatypes/pointers (boxes or not)
  - also input 0 for `high_s` panics!