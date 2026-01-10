// Implementation Plan:
// The basic loop of a shell can be described by the following pseudocode:
//
// loop {
//     print_prompt();
//     let input = readline();
//     parse_and_execute(input);
// }
//
// The shell prompt is printed first ($, # etc.), then the shell reads a line
// of input. After that it parses and executes the input. When the execution
// of the input command is completed, the shell loop continues.

use std::io::Write;

fn main() {
    let mut prompt = "😇 ";
    loop {
        // Let's start by printing the prompt
        print!("{}", prompt);

        // We need to flush stdout to ensure the prompt is printed before read_line()
        let _ = std::io::stdout().flush();

        // Now we read a line of input from stdin
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        // trim() is used to remove the trailing newline
        let mut command_line = input.trim().split_whitespace();

        let command_name = command_line.next().unwrap();
        let args = command_line;

        match command_name {
            "cd" => {
                let mut args = args;

                // Default to root if directory is not provided
                let next_dir = args.next().unwrap_or("/");

                if let Err(e) = std::env::set_current_dir(next_dir) {
                    eprintln!("{}", e);
                }
            }
            "naughty" => prompt = "😈 ",
            "nice" => prompt = "😇 ",
            "exit" => return,
            _ => {
                let child = std::process::Command::new(command_name).args(args).spawn();

                match child {
                    Ok(mut child) => {
                        let _ = child.wait();
                    }
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
    }
}

// Notes
// Turns out Rust has a different mechanism for launching child processes
// So, instead of fork() and execve(), we use Command::spawn()
