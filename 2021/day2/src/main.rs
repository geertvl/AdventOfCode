use std::{fs::File, io::{BufReader, BufRead}};

#[derive(Debug)]
struct Command {
    action: String,
    steps: u32,
}

#[derive(Debug)]
struct Parser {
    input: String,
    read_pos: u32,
    ch: char,
}

impl Parser {
    fn new(input: String) -> Parser {
        Parser { input, read_pos: 0, ch: '\0' }
    }

    fn read_char(&mut self) {
        if self.read_pos as usize >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_pos as usize).unwrap();
        }
        self.read_pos = self.read_pos + 1;
    }

    fn skip_whitespace(&mut self) {
        while is_whitespace(self.ch) {
            self.read_char();
        }
    }

    fn read_action(&mut self) -> String {
        let position = self.read_pos;
        while is_letter(self.ch) {
            self.read_char();
        }
        let start = (position-1) as usize; 
        let end = (self.read_pos-1) as usize;

        self.input[start..end].to_string()
    }

    fn read_steps(&mut self) -> u32 {
        let position = self.read_pos;
        while is_digit(self.ch) {
            self.read_char();
        }
        let start = (position-1) as usize; 
        let end = (self.read_pos-1) as usize;

        self.input[start..end].parse().unwrap()
    }

    fn parse_line(&mut self) -> Command {
        self.read_char();
        self.skip_whitespace();
        let action = self.read_action();
        self.skip_whitespace();
        let steps = self.read_steps();

        Command { action, steps }
    }
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let mut commands: Vec<Command> = Vec::new();

    for line in reader.lines() {

        let mut parser = Parser::new(line?);
        let cmd = parser.parse_line();
        commands.push(cmd);
    }

    //println!("{:?}", commands);

    let mut horizontal_pos = 0;
    let mut depth = 0;

    for cmd in &commands {
        match cmd.action.as_str() {
            "forward" => horizontal_pos = horizontal_pos + cmd.steps,
            "down" => depth = depth + cmd.steps,
            "up" => depth = depth - cmd.steps,
            _ => panic!("action not recognized")
        }
    }
    println!("horizontal_pos: {}, depth: {} => {}", horizontal_pos, depth, horizontal_pos * depth);

    let mut horizontal_pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for cmd in &commands {
        match cmd.action.as_str() {
            "forward" => {
                horizontal_pos = horizontal_pos + cmd.steps;
                if aim > 0 {
                    depth = depth + (cmd.steps * aim);
                }
            },
            "down" => aim = aim + cmd.steps,
            "up" => aim = aim - cmd.steps,
            _ => panic!("action not recognized")
        }
    }
    println!("horizontal_pos: {}, depth: {} => {}", horizontal_pos, depth, horizontal_pos * depth);

    Ok(())
}
