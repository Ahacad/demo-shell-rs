use std::io::{stdin, stdout};
use std::process::{Child, Command};

fn main() {
    loop {
        print!("> ");
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        Command::new(command).spawn().unwrap();

        child.wait();
    }
}
