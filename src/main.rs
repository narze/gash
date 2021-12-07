use rand::prelude::*;
use std::process::Command;
// use std::{thread, time};
// use std::os::unix::process::CommandExt;
// use std::io::Error;
use exec;

fn main() {
    // TODO: Read dotfile for configuration

    // Random shell between fish or zsh
    let mut rng = thread_rng();
    let shell_str = ["/opt/homebrew/bin/fish", "/bin/zsh"]
        .choose(&mut rng)
        .unwrap();
    // println!("Redirect to Shell: {}", shell);
    // let dur = time::Duration::from_millis(1000);
    // thread::sleep(dur);

    // let shell_path = Command::new("which").arg(shell).output().expect("failed to execute process");
    // let mut shell_str = String::from_utf8(shell_path.stdout).expect("error");
    // let len = shell_str.len();
    // shell_str.truncate(len - 1);
    println!("shell path: {}", shell_str);

    let err = exec::Command::new(shell_str).exec();
    println!("Error: {}", err);
}
