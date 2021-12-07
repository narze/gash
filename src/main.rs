use rand::prelude::*;
use std::process::Command;

fn main() {
    // TODO: Read dotfile for configuration

    // Random shell between fish or zsh
    let fish: bool = random();

    if fish {
        println!("Redirect to fish!");
        Command::new("fish")
            .status()
            .expect("failed to execute process");
    } else {
        println!("Redirect to zsh!");
        Command::new("zsh")
            .status()
            .expect("failed to execute process");
    }
}
