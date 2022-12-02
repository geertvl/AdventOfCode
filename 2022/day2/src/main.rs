use std::{fs::File, io::{BufReader, BufRead}};

#[derive(Debug)]
struct Round {
  opponent: char,
  ours: char,
}

fn parse(buf: &mut BufReader<File>) -> Result<Vec<Round>, Box<dyn std::error::Error>> {
  let mut rounds: Vec<Round> = vec![];

  for line in buf.lines() {
    let line = line?;
    rounds.push(Round {
      opponent: line.chars().nth(0).unwrap(),
      ours: line.chars().nth(2).unwrap(),
    });
    
  }

  Ok(rounds)
}

// A = Rock, B = Paper, C = Scissors
// X = Rock, Y = Paper, Z = Scissors
// X = 1, Y = 2, Z = 3,
// Rock defeats Scissors
// Scissors defeats Paper
// Paper defeats Rock
fn round_score(round: &Round) -> u32 {
    let mut score = match round.ours {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    };

    if round.ours == 'X' && round.opponent == 'C' ||
       round.ours == 'Z' && round.opponent == 'B' ||
       round.ours == 'Y' && round.opponent == 'A' {
      score += 6;
    }

    if round.ours == 'X' && round.opponent == 'A' ||
       round.ours == 'Y' && round.opponent == 'B' ||
       round.ours == 'Z' && round.opponent == 'C' {
      score += 3;
    }

    score
}

fn round_new_score(round: &Round) -> u32 {
  let mut score = 0;

  if round.ours == 'X' {
    match round.opponent {
      'A' => score += 3,
      'B' => score += 1,
      'C' => score += 2,
      _ => score = 0,
    }
  }

  if round.ours == 'Y' {
    match round.opponent {
      'A' => score += 4,
      'B' => score += 5,
      'C' => score += 6,
      _ => score = 0,
    }
  }

  if round.ours == 'Z' {
    match round.opponent {
      'A' => score += 8,
      'B' => score += 9,
      'C' => score += 7,
      _ => score = 0,
    }
  }

  score
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let input = File::open("input.txt")?;      
  let mut reader = BufReader::new(input);
  let rounds = parse(&mut reader)?;

  let part1_total_score: u32 = rounds
    .iter()
    .map(|x| round_score(x))
    .sum();
  println!("Total score (part1): {}", part1_total_score);

  let part2_total_score: u32 = rounds
    .iter()
    .map(|x| round_new_score(x))
    .sum();

  println!("Total score (part2): {}", part2_total_score);    


  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_test_part1() {
    let rounds: Vec<Round> = vec![
      Round { opponent: 'A', ours: 'Y' },
      Round { opponent: 'B', ours: 'X' },
      Round { opponent: 'C', ours: 'Z' }
    ];

    let result: u32 = rounds
      .iter()
      .map(|x| round_score(x))
      .sum();

    assert_eq!(15, result);
  }

  #[test]
  fn sample_test_part2() {
    let rounds: Vec<Round> = vec![
      Round { opponent: 'A', ours: 'Y' },
      Round { opponent: 'B', ours: 'X' },
      Round { opponent: 'C', ours: 'Z' }
    ];

    let result: u32 = rounds
      .iter()
      .map(|x| round_new_score(x))
      .sum();

    assert_eq!(12, result);
  }
}

