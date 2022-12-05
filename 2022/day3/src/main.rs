use std::{fs::File, io::{BufReader, BufRead}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let sums: u32 = calculate_total_score(&mut reader);

    let total_badge_score = calculate_badge_score(&mut reader);

    println!("Sum: {}", sums);
    println!("Badge: {}", total_badge_score);

    Ok(())
}


fn calculate_badge_score(buf: &mut BufReader<File>) -> u32 {
    let group = buf.lines().take(3).;
    while group.is_none {


    }

    0
}

fn calculate_total_score(buf: &mut BufReader<File>) -> u32 {
    buf.lines().map(|line| {
        let same_items = find_same_items_in_compartments(line.unwrap().as_str());
        let score = calculate_item_score(same_items);
        score
    }).sum()
}

fn find_same_items_in_compartments(rucksack_content: &str) -> Vec<char> {
    let first = &rucksack_content[0..rucksack_content.len() / 2];
    let second = &rucksack_content[rucksack_content.len() / 2..];

    let mut doubles = first
        .chars()
        .filter(|c| second.chars().any(|d| c == &d))
        .collect::<Vec<char>>();
    doubles.dedup();

    doubles
}

fn calculate_item_score(doubles: Vec<char>) -> u32  {
    let total: u32 = doubles.iter().map(|double| {
        if double.is_lowercase() {
            (*double as u32) - 96
        } else {
            (*double as u32) - 38
        }
    }).sum();

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_identifier() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";

        let result = find_same_items_in_compartments(input);

        assert_eq!(1, result.len());
        assert_eq!('p', result[0]);
    }

    #[test]
    fn same_uppercase_identifier() {
        let input = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";

        let result = find_same_items_in_compartments(input);

        assert_eq!(1, result.len());
        assert_eq!('L', result[0]);
    }

    #[test]
    fn same_other_uppercase_identifier() {
        let input = "PmmdzqPrVvPwwTWBwg";

        let result = find_same_items_in_compartments(input);

        assert_eq!(1, result.len());
        assert_eq!('P', result[0]);
    }

    #[test]
    fn calculate_score() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";

        let double_items = find_same_items_in_compartments(input);
        let total = calculate_item_score(double_items);

        assert_eq!(16, total);
    }
}