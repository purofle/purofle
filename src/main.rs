use std::io::{self, Write};
extern crate colored;
use colored::*;

fn main() {

    println!("meow! Welcome to purofleOS!");

    let mut input = String::new();

    while input != "\n" {

        print!("❯");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "who" => { println!("{}", "I'm purofle!".bright_cyan()) },
            "where" => { println!("{}", "Henan, China".bright_blue()) },
            "blog" => { println!("{}", "blog.archlinux.tech".bright_green()) },
            "status" => { println!("{}", "I'm a student, very busy :(".bright_yellow()) },
            "stack" => { println!("{}, {}, {}, {}, {}",
                "Kotlin".on_bright_purple().purple(),
                "Java".on_bright_red().red(),
                "Python".on_bright_green().green(),
                "TypeScript".on_bright_blue().blue(),
                "ShellScript".on_bright_black().black(),
            )},
            "email" => { println!("{}", "purofle at gmail.com".bold().bright_magenta()) },
            "telegram" => { println!("{}", "@purolle".bold().bright_cyan()) },
            "exit" => {
                println!("{}", "私は可愛いですから、お金をください！".italic().bright_blue());
                input = String::from("\n");
            }
            _ => println!("I don't know what you mean"),

        }
    }
}