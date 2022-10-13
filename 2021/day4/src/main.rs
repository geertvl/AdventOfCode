use std::{fs::File, io::{BufReader, BufRead}, slice::Iter};

#[derive(Debug, Copy, Clone)]
struct Cell {
    number: u8,
    hit: bool,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            number: 0,
            hit: false,
        }
    }
}

struct Board {
    cells: Box<[Cell;5*5]>,
}

impl Board {
    fn new() -> Board {
        Board { cells: Box::new([Cell::new();5*5]) }
    }

    fn mark(&mut self, mark_number: &u8) {
        let _ = self.cells.iter_mut().map(|cell| {
            if cell.number == *mark_number {
                cell.hit = true;
            }
        }).collect::<Vec<_>>();
    }

    fn horizontal_line_complete(&self) -> bool {
        let mut start = 0;
        let mut end = start + 5;
        for _ in 0..5 {
            let row = self.cells[start..end].iter().all(|cell| cell.hit);
            start = end;
            end = start + 5;
        }

        true
    }

    fn vertical_line_complete(&self) -> bool {
        let mut step = 0;
        for i in 0..5 {
            for j in 0..5 {
                self.cells[j+step].hit
            }
            step += 5;
        }

        true
    }
}

fn parse(buf: &mut BufReader<File>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut lines: Vec<String> = vec![];

    for line in buf.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

fn parse_numbers(line: &str) -> Vec<u8> {
    line.trim()
        .split_whitespace()
        .map(|c| c.parse::<u8>().unwrap())
        .collect()
}

fn parse_random_numbers(line: &str) -> Vec<u8> {
    line.trim()
        .split(',')
        .map(|c| c.parse::<u8>().unwrap())
        .collect()
}

fn parse_board(lines: &mut Iter<String>) -> Option<Board> {
    let mut board = Board::new();
    let mut count = 0;
    for _ in 1..=5 {
        let current_line = lines.next().unwrap();
        let board_line = parse_numbers(current_line.as_str());
        for number in board_line.iter() {
            board.cells[count].number = *number;
            count += 1;
        }
    }

    match lines.next() {   // empty line
        Some(_) => Some(board),
        None => None,
    }
}

fn parse_boards(mut lines: Iter<String>) -> Vec<Board> {
    let mut boards: Vec<Board> = vec![];

    while let Some(board) = parse_board(&mut lines) {
        boards.push(board);
    }

    boards
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = File::open("input.txt")?;
    let mut reader = BufReader::new(input);

    let lines = parse(&mut reader)?;

    let mut lines_iter = lines.iter();

    let random_numbers = match lines_iter.next() {
        Some(random_line) => parse_random_numbers(random_line.as_str()),
        None => panic!("Input file is not in the correct form"),
    };
    println!("random {:?}", random_numbers);

    lines_iter.next(); // empty line

    let mut boards = parse_boards(lines_iter);
    if boards.len() != 0 {
        println!("TEXT");
    }
    for number in random_numbers.iter() {
        for board in boards.iter_mut() {
            board.mark(&number);
        }
    }

    Ok(())
}
