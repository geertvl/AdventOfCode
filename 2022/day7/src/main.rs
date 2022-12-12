use std::iter::Scan;

trait Command {

}

struct Scanner {
    commands: Vec<Box<dyn Command>>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner {
            commands: vec![],
        }
    }

    fn parse_command(&mut self, input: &str) {
        match input {
            x if x.starts_with("$ cd") => println!("cd Command: {}", x),
            x if x.starts_with("$ ls") => println!("ls command: {}", x),
            _ => panic!("Not recognized command"),
        }

        // 1. start with $
        if input.starts_with("$") {
            //    a. cd
            //    b. ls   
        }
        // 2. start with dir
        if input.starts_with("dir") {
    
        }
        // 3. start with digits
        if input.chars().nth(0).unwrap().is_digit(10) {
    
        }
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_directory() {
        let input = "$ cd /";

        let mut scanner = Scanner::new();
        scanner.parse_command(input);
    }
}