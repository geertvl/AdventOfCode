use std::vec;

use scanf::sscanf;

enum Command {
    ChangeDir(String),
    ListFiles,
    Directory(String),
    File(u32, String),
    Unknown,
}

struct Directory {
    size: u32,
    dirs: Vec<Directory>,
}

impl Directory {
    fn new(name: String) -> Directory {
        Directory { size: 0, dirs: vec![] }
    }
}

struct Scanner {
    commands: Vec<Command>,
    root: Directory,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner {
            commands: vec![],
            root: Directory::new("/".to_string()),
        }
    }

    fn parse_command(&mut self, input: &str) {
        let command = match input {
            x if x.starts_with("$ cd") => Scanner::parse_cd_command(x),
            x if x.starts_with("$ ls") => Command::ListFiles,
            x if x.starts_with("dir") => Scanner::parse_dir_command(x),
            x if input.chars().nth(0).unwrap().is_digit(10) => Scanner::parse_file_command(x),
            _ => Command::Unknown,
        };

        self.commands.push(command);
    }

    fn parse_cd_command(input: &str) -> Command {
        let mut dir = String::new();
        let _ = sscanf!(input, "$ cd {}", dir);

        Command::ChangeDir(dir)
    }

    fn parse_dir_command(input: &str) -> Command {
        let mut dir = String::new();
        let _ = sscanf!(input, "dir {}", dir);

        Command::Directory(dir)
    }

    fn parse_file_command(input: &str) -> Command {
        let mut bytes: u32 = 0;
        let mut name = String::new();

        let _ = sscanf!(input, "{} {}", bytes, name);

        Command::File(bytes, name)
    }

    fn create_tree(&mut self) {
        let current_dir = "/".to_string();
        let mut iter = self.commands.iter_mut();
        let step = iter.next().unwrap();

        while let Some(cmd) = iter.next() {
            match cmd {
                Command::ChangeDir(dir) => println!("changedir: {}", dir),
                Command::ListFiles => println!("list ")
            }
        }
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn change_directory() {
        let input = "$ cd /";

        let mut scanner = Scanner::new();
        scanner.parse_command(input);

        assert_eq!(1, scanner.commands.len());
        assert!(if let Command::ChangeDir(_) = &scanner.commands[0] { true } else { false });
    }

    #[test]
    fn file_name_command() {
        let input = "12345 name.txt";

        let mut scanner = Scanner::new();
        scanner.parse_command(input);

        assert_eq!(1, scanner.commands.len());
        assert!(if let Command::File(_, _) = &scanner.commands[0] { true } else { false });  
        if let Command::File(size, name) = &scanner.commands[0] {
            assert_eq!(12345, *size);
            assert_eq!("name.txt".to_string(), *name);
        }    
    }

    #[test]
    fn sample_input() {
        let lines = vec![
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k"
        ];

        let mut scanner = Scanner::new();
        for line in lines {
            scanner.parse_command(line);
        }

        assert_eq!(23, scanner.commands.len());

        scanner.create_tree();
    }
}