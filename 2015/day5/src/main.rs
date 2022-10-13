<<<<<<< HEAD
fn main() {
    println!("Hello, world!");
}

fn contains_three_vowels()

#[allow(dead_code)]
fn is_nice_string(str: &str) -> bool {
    const VOWELS:[char;5] = ['a', 'e', 'i', 'o', 'u'];
    let s: String = str.to_owned();
    let mut chars: Vec<char> = s[..]
        .chars()
        .collect();
    chars
        .sort_by(|a, b| b.cmp(a));
    println!("test={:?}", chars);

    true
=======
use std::fs::File;
use std::io::{BufRead, BufReader};

fn has_three_vowels(input: &String) -> bool {
    let mut count = 0;
    for c in input.chars() {
        let contains = "aeiou".contains(c);
        if contains {
            count += 1;
        }
    }

    count >= 3
}

fn has_twice_in_row(input: &String) -> bool {
    let mut double = false;
    let mut prev: char = ' '; 
    for c in input.chars() {
        if c == prev {
            double = true;
        }
        prev = c;
    }

    double
}

fn has_twice_pair_in_row(input: &String) -> bool {
    let mut first: char = ' ';
    let mut sec: char = ' ';
    let mut found = false;
    for (i, c) in input.chars().enumerate() {
        if first != ' ' && sec != ' ' {
            let pat: String = format!("{}{}", first, sec);
            let vec: Vec<&str> = input[i..].matches(&pat).collect();
            if vec.len() > 0 {
                found = true;
            }
        }
    }

    found
}

fn containing_strings(input: &String) -> bool {
    input.contains("ab") || input.contains("cd") ||
        input.contains("pq") || input.contains("xy")
}

fn is_nice_string(line: &String) -> bool {
    let three_vowels = has_three_vowels(&line);
    println!("three vowels: {}", three_vowels);
    let multiples = has_twice_in_row(&line);
    println!("twice in a row: {}", multiples);
    let forbidden_string = containing_strings(&line);
    println!("forbidden string: {}", forbidden_string);

    three_vowels && multiples && !forbidden_string
}

fn is_improved_nice_string(line: &String) -> bool {

}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut counter = 0;
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if is_nice_string(&line) {
            counter += 1;
        }
    }

    println!("{} strings are nice.", counter);
>>>>>>> 62770baa624d963aa57017153481c6beb96e01f5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
<<<<<<< HEAD
    fn test_three_vowels() {
        assert!(is_nice_string("aei"));
        assert!(is_nice_string("xazegov"));
        assert!(is_nice_string("aeiouaeiouaeiou"));
    }
}
=======
    fn has_two_same_chars_in_a_row() {
        let input = "bbac";
        let result = has_twice_in_row(&input.to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn has_no_same_chars_in_a_row() {
        let input = "abcdb";
        let result = has_twice_in_row(&input.to_string());
        assert_eq!(result, false);
    }

    #[test]
    fn contains_forbidden_string() {
        let input = "haegwjzuvuyypxyu";
        let result = containing_strings(&input.to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn not_containing_forbidden_string() {
        let input = "haegwjzuvuyyp";
        let result = containing_strings(&input.to_string());
        assert_eq!(result, false);
    }

    #[test]
    fn nice_string() {
        let input = "ugknbfddgicrmopn";
        let result = is_nice_string(&input.to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn other_nice_string() {
        let input = "aaa";
        let result = is_nice_string(&input.to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn naughty_string() {
        let input = "jchzalrnumimnmhp";
        let result = is_nice_string(&input.to_string());
        assert_eq!(result, false);
    }

    #[test]
    fn other_naughty_string() {
        let input = "haegwjzuvuyypxyu";
        let result = is_nice_string(&input.to_string());
        assert_eq!(result, false);
    }

    #[test]
    fn another_naughty_string() {
        let input = "dvszwmarrgswjxmb";
        let result = is_nice_string(&input.to_string());
        assert_eq!(result, false);
    }

    #[test]
    fn 
}
>>>>>>> 62770baa624d963aa57017153481c6beb96e01f5
