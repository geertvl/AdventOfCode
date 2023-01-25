use scanf::sscanf;

struct Scanner {
    commands: Vec<Command>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner {
            commands: vec![],
        }
    }

    fn parse_command(&mut self, input: &str) {
        let command = match input {
            x if x.starts_with("$ cd") => self.parse_cd_command(x),
            x if x.starts_with("$ ls") => Command::ListFiles,
            x if x.starts_with("dir") => self.parse_dir_command(x),
            x if !x.is_empty() && x.chars().nth(0).unwrap().is_digit(10) => self.parse_file_command(x),
            _ => Command::Unknown,
        };

        self.commands.push(command);
    }

    fn parse_cd_command(&mut self, input: &str) -> Command {
        let mut dir = String::new();
        let _ = sscanf!(input, "$ cd {}", dir);

        Command::ChangeDir(dir)
    }

    fn parse_dir_command(&mut self, input: &str) -> Command {
        let mut dir = String::new();
        let _ = sscanf!(input)
    }

    fn parse_file_command(&mut self, input: &str) -> Command {
        Command::Unknown
    }
}

struct FileSystem {

}

enum Command {
    ChangeDir(String),
    ListFiles,
    File(u32, String),
    Directory(String),
    Unknown,
}

fn main() {
    // cd x : moves one level in
    // cd ..: go up a level.
    // cd / : go to the root
    // ls   : list of all files/dirs in current dir.
    // 123 abc : file abc with size 123
    // dir xyz : dir xyz in current directory

    let scanner = Scanner::new();

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_directory() {
        let input = "$ cd /";

        let mut scanner = Scanner::new();
        scanner.parse_command(input);

        assert_eq!(1, scanner.commands.len());
        let command = &scanner.commands[0];

        assert!(command is ChangeDir(x))
    }
}