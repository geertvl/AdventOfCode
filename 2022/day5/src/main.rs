use std::{fs::File, io::{BufReader, BufRead}, collections::VecDeque, fmt, vec};

use scanf::sscanf;

#[derive(Debug)]
struct Storage {
    stacks: Vec<Stack>,
}

struct Stack {
    name: u32,
    crates: VecDeque<char>,
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Stack")
            .field("name", &self.name)
            .finish()
    }
}

impl Storage {
    fn parse_movements(&mut self, movements: &Vec<String>) {
        for movement in movements {
            let mut quantity: u32 = 0;
            let mut stack_from: usize = 0;
            let mut stack_to: usize = 0;
            if sscanf!(movement.as_str(), "move {} from {} to {}", quantity, stack_from, stack_to).is_ok() {
                for _ in 0..quantity {
                    let crt = self.stacks[stack_from-1].crates.pop_front().unwrap();
                    self.stacks[stack_to-1].crates.push_front(crt);
                }
            }
        }
    }

    fn parse_movements_9001(&mut self, movements: &Vec<String>) {
        for movement in movements {
            let mut quantity: u32 = 0;
            let mut stack_from: usize = 0;
            let mut stack_to: usize = 0;
            if sscanf!(movement.as_str(), "move {} from {} to {}", quantity, stack_from, stack_to).is_ok() {
                let mut crane_stack: Vec<char> = vec![];
                for _ in 0..quantity {
                    let crt = self.stacks[stack_from-1].crates.pop_front().unwrap();
                    crane_stack.push(crt);
                }

                for stack in crane_stack.iter().rev() {
                    self.stacks[stack_to-1].crates.push_front(*stack);
                }
            }
        }
    }
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

fn parse_storage(input: &Vec<String>) -> Storage {
    // parse the stacks
    let mut stacks = input
        .last()
        .unwrap()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| Stack::new(c))
        .collect::<Vec<Stack>>();

    for line in input.iter().rev().skip(1) {
        for (i, j) in (2..=34).step_by(4).enumerate() {
            let ch = line.chars().nth(j-1).unwrap();
            if ch != ' ' {
                stacks[i].crates.push_front(ch);            
            }            
        }
    }

    Storage { stacks }
}

fn read_movements(buf: &mut BufReader<File>) -> Vec<String> {
    buf.lines().map(|line| line.unwrap()).collect::<Vec<String>>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = File::open("input.txt")?;
    let mut reader = BufReader::new(input);

    let storage_state = read_storage_state(&mut reader);
    let movements = read_movements(&mut reader);

    let mut storage = parse_storage(&storage_state);
    storage.parse_movements(&movements);    

    for mut stack in storage.stacks {
        let stack_top = stack.crates.pop_front().unwrap();
        println!("{}", stack_top);
    }

    println!("storage of the 9001");
    let mut storage_9001 = parse_storage(&storage_state);
    storage_9001.parse_movements_9001(&movements);

    for mut stack in storage_9001.stacks {
        let stack_top = stack.crates.pop_front().unwrap();
        println!("{}", stack_top);
    }

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

    #[test]
    fn parse_storage_complete() {
        let input: Vec<String> = vec![
            "    [H]         [D]     [P]        ".to_string(),
            "[W] [B]         [C] [Z] [D]        ".to_string(),
            "[T] [J]     [T] [J] [D] [J]        ".to_string(),
            "[H] [Z]     [H] [H] [W] [S]     [M]".to_string(),
            "[P] [F] [R] [P] [Z] [F] [W]     [F]".to_string(),
            "[J] [V] [T] [N] [F] [G] [Z] [S] [S]".to_string(),
            "[C] [R] [P] [S] [V] [M] [V] [D] [Z]".to_string(),
            "[F] [G] [H] [Z] [N] [P] [M] [N] [D]".to_string(),
            " 1   2   3   4   5   6   7   8   9 ".to_string(),
        ];

        let mut storage = parse_storage(&input);

        assert_eq!(9, storage.stacks.len());

        let movements: Vec<String> = vec![
            "move 2 from 8 to 2".to_string(),
            "move 3 from 9 to 2".to_string(),            
            // "move 1 from 3 to 8".to_string(),
        ];

        storage.parse_movements(&movements);

        assert_eq!(1, storage.stacks[7].crates.len());
        assert_eq!(13, storage.stacks[1].crates.len());
        assert_eq!(2, storage.stacks[8].crates.len());
    }
}
