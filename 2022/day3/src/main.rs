use std::{fs::File, io::{BufReader, BufRead}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let sums: u32 = calculate_total_score(&lines);

    let total_badge_score = calculate_badge_score(&lines);

    println!("Sum: {}", sums);
    println!("Badge: {}", total_badge_score);

    Ok(())
}


fn calculate_badge_score(lines: &Vec<String>) -> u32 {
    let badges = lines.chunks(3).map(|chunk| {
        find_same_in_group(chunk)
    }).collect::<Vec<char>>();

    calculate_item_score(badges)
}

fn find_same_in_group(group: &[String]) -> char {
    group[0]
        .chars()
        .filter(|c| group[1].chars().any(|d1| c == &d1) && group[2].chars().any(|d2| c == &d2))
        .nth(0)
        .unwrap()
}

fn calculate_total_score(lines: &Vec<String>) -> u32 {
    lines.iter().map(|line| {
        let same_items = find_same_items_in_compartments(line.as_str());
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
    fn chunk_3_items() {
        let input: Vec<&str> = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw"
        ];

        let split: Vec<&[&str]> = input
            .chunks(3).collect();

        assert_eq!(2, split.len());
    }

    #[test]
    fn chunk_mapping() {
        let input: Vec<&str> = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw"
        ];

        let chars: Vec<char> = input.chunks(3).map(|_| 'a').collect();

        assert_eq!(2, chars.len());
    }

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

    #[test]
    fn find_same_in_1_group() {
        let input: [String; 3] = [
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
        ];

        let result = find_same_in_group(&input);

        assert_eq!('r', result);
    }

    #[test]
    fn find_same_in_2_groups() {
        let input: [String; 6] = [
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string()
        ];

        let badges = input.chunks(3).map(|chunk| {
            find_same_in_group(chunk)
        }).collect::<Vec<char>>();

        assert_eq!('r', badges[0]);
        assert_eq!('Z', badges[1]);

        let score = calculate_item_score(badges);
        assert_eq!(70, score);
    }
}