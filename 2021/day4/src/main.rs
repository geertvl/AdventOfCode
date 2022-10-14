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

#[derive(Debug, Clone)]
struct Board {
    cells: Box<[Cell;5*5]>,
}

impl Board {
    fn new() -> Board {
        Board { cells: Box::new([Cell::new();5*5]) }
    }

    fn mark(&mut self, mark_number: u8) {
        let _ = self.cells.iter_mut().map(|cell| {
            if cell.number == mark_number {
                cell.hit = true;
            }
        }).collect::<Vec<_>>();
    }

    fn row_winner(&self) -> bool {
        self.cells.chunks(5).any(|chunk| chunk.iter().all(|n| n.hit))
    }

    fn col_winner(&self) -> bool {
        let flattened: Vec<&Cell> = (0..5)
            .flat_map(|col| self.cells.iter().skip(col).step_by(5))
            .collect();
        let col = flattened
            .chunks(5)
            .into_iter()
            .any(|chunk| chunk.into_iter().all(|cell| cell.hit));
        col
    }

    fn sum_of_unmarked_cells(&self) -> u32 {
        self.cells.iter().fold(0, |accum, cell| {
            if !cell.hit {
                accum + cell.number as u32
            } else {
                accum
            }
        })
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

    lines_iter.next(); // empty line

    let boards = parse_boards(lines_iter);
    let mut final_score_boards = boards.clone();
    let final_score = random_numbers.iter().find_map(|nbr| {
        final_score_boards
            .iter_mut()
            .map(|board| {
                board.mark(*nbr);
                board
            })
            .filter(|b| b.col_winner() || b.row_winner())
            .map(|winner| winner.sum_of_unmarked_cells() * *nbr as u32)
            .next()
    }).unwrap();

    println!("Final score: {}", final_score);

    let mut last_score_boards = boards.clone();
    let last_score = random_numbers.iter().filter_map(|nbr| {
        last_score_boards
            .iter_mut()
            .filter(|b| !b.col_winner() && !b.row_winner())
            .map(|board| {
                board.mark(*nbr);
                board
            })
            .filter(|b| b.col_winner() || b.row_winner())
            .map(|winner| winner.sum_of_unmarked_cells() * *nbr as u32)
            .last()
    }).last().unwrap();

    println!("Last score: {}", last_score);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_board(hits: Vec<usize>) -> Board {
        let mut board = Board {
            cells: Box::new([
                Cell { number: 1, hit: false },
                Cell { number: 2, hit: false },
                Cell { number: 3, hit: false },
                Cell { number: 4, hit: false },
                Cell { number: 5, hit: false },

                Cell { number: 6, hit: false },
                Cell { number: 7, hit: false },
                Cell { number: 8, hit: false },
                Cell { number: 9, hit: false },
                Cell { number: 10, hit: false },

                Cell { number: 11, hit: false },
                Cell { number: 12, hit: false },
                Cell { number: 13, hit: false },
                Cell { number: 14, hit: false },
                Cell { number: 15, hit: false },
                
                Cell { number: 16, hit: false },
                Cell { number: 17, hit: false },
                Cell { number: 18, hit: false },
                Cell { number: 19, hit: false },
                Cell { number: 20, hit: false },

                Cell { number: 21, hit: false },
                Cell { number: 22, hit: false },
                Cell { number: 23, hit: false },
                Cell { number: 24, hit: false },
                Cell { number: 25, hit: false },
            ])
        };

        for hit in hits {
            board.cells[hit].hit = true;
        }

        board
    }

    #[test]
    fn col_winner_first_col() {
        let board = setup_board(vec![0, 5, 10, 15, 20]);
        let result = board.col_winner();

        assert!(result);
    }

    #[test]
    fn col_winner_last_col() {
        let board = setup_board(vec![4, 9, 14, 19, 24]);
        assert!(board.col_winner());
    }

    #[test]
    fn col_winner_middle_col() {
        let board = setup_board(vec![3, 8, 13, 18, 23]);
        assert!(board.col_winner());
    }

    #[test]
    fn col_loser() {
        let board = setup_board(vec![2, 9, 12, 19, 20]);
        assert!(board.col_winner());
    }

    #[test]
    fn row_winner_first() {
        let board = setup_board(vec![0,1,2,3,4]);
        assert!(board.row_winner());
    }

    #[test]
    fn row_winner_last() {
        let board = setup_board(vec![20,21,22,23,24]);
        assert!(board.row_winner());
    }

    #[test]
    fn mark_number_on_board() {
        let mut board = setup_board(vec![]);
        let i: u8 = 2;
        board.mark(i);

        assert!(board.cells[1].hit);
    }
}