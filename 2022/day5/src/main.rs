use std::{fs::File, io::{BufReader, BufRead}, collections::VecDeque};

#[derive(Debug)]
struct Storage {
    stack: Vec<Stack>,
}

#[derive(Debug)]
struct Stack {
    name: u32,
    crates: VecDeque<char>,
}

impl Stack {
    fn new(name: char) -> Stack {
        Self {
            name: name.to_digit(10).unwrap(),
            crates: VecDeque::new(),
        }
    }
}

fn read_storage_state(buf: &mut BufReader<File>) -> Vec<String> {
    buf.lines().take_while(|line| {
        match line {
            Ok(l) => !l.is_empty(),
            Err(_) => panic!("not a line")
        }
    })
    .map(|line| line.unwrap())
    .collect::<Vec<String>>()
}

fn parse_storage(input: Vec<String>) -> Storage {
    // parse the stacks
    let stacks = input
        .last()
        .unwrap()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| Stack::new(c))
        .collect::<Vec<Stack>>();

    for line in input[input.len()..0] {
        
    }

    Storage { stack: stacks }
}

fn read_movements(buf: &mut BufReader<File>) -> Vec<String> {
    buf.lines().map(|line| line.unwrap()).collect::<Vec<String>>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = File::open("input.txt")?;
    let mut reader = BufReader::new(input);

    let storage_state = read_storage_state(&mut reader);
    let movements = read_movements(&mut reader);

    let storage = parse_storage(storage_state);

    println!("Storage");
    println!("{:?}", storage);

    // println!("Movements");
    // println!("{:?}", movements);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_stack()
    {
        let input = '1';
        let stack = Stack::new(input);

        assert_eq!(1, stack.name);
    }
}
