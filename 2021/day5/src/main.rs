use std::{str::FromStr, num::ParseIntError, fs::read_to_string};

const MAP_SIZE: usize = 10000;

fn take_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let extracted_end: usize = s
        .char_indices()
        .find_map(|(idx, c)| if accept(c) { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());

    let extracted = &s[..extracted_end];
    let remainder = &s[extracted_end..];
    (remainder, extracted)
}

fn extract_digits(s: &str) -> (&str, &str) {
    take_while(|c| c.is_ascii_digit(), s)
}

fn extract_rubble(s: &str) -> (&str, &str) {
    take_while(|c: char| c == ',' || c == ' ' || c == '-' || c == '>', s)
}

fn extract_vent_part(s: &str) -> Result<(&str, usize), ParseIntError> {
    let (s, _) = extract_rubble(s);
    let (s, digit) = extract_digits(s);

    let part = usize::from_str_radix(digit, 10)?;

    Ok((s, part))
}

#[derive(Debug, PartialEq)]
struct Vent {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl FromStr for Vent {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, x1) = extract_vent_part(s)?;
        let (s, y1) = extract_vent_part(s)?;
        let (s, x2) = extract_vent_part(s)?;
        let (_, y2) = extract_vent_part(s)?;

        Ok(Self {
            x1,
            y1,
            x2,
            y2,
        })
    }
}

impl Vent {
    fn covering_points(&self) -> Vec<(usize, usize)> {
        if self.x1 == self.x2 {    
            (self.y1.min(self.y2)..=self.y2.max(self.y1)).map(|y| (self.x1, y)).collect()
        } else if self.y1 == self.y2 {
            (self.x1.min(self.x2)..=self.x2.max(self.x1)).map(|x| (x, self.y1)).collect()
        } else if self.x1 <= self.x2 {
            if self.y1 <= self.y2 {
                (0..=(self.x2 - self.x1)).map(|inc| (self.x1 + inc, self.y1 + inc)).collect()
            } else {
                (0..=(self.x2 - self.x1)).map(|inc| (self.x1 + inc, self.y1 - inc)).collect()
            }
        } else if self.y1 <= self.y2 {
            (0..=(self.x1 - self.x2)).map(|inc| (self.x1 - inc, self.y1 + inc)).collect()
        } else {
            (0..=(self.x1 - self.x2)).map(|inc| (self.x1 - inc, self.y1 - inc)).collect()
        }
    }
}

fn calculate_horizontal_vertical(input: &str) -> usize {
    let mut map = vec![0;MAP_SIZE * MAP_SIZE];
    input
        .lines()
        .map(|line| Vent::from_str(line).unwrap())
        .filter(|vent| vent.x1 == vent.x2 || vent.y1 == vent.y2)
        .for_each(|vent| {
            vent.covering_points()
                .into_iter()
                .for_each(|(x,y)| map[y * MAP_SIZE + x] += 1);
        });

    map
        .into_iter()
        .filter(|&overlaps| overlaps >= 2)
        .count()
}

fn calculate_diagonal(input: &str) -> usize {
    let mut map = vec![0;MAP_SIZE * MAP_SIZE];
    input
        .lines()
        .map(|line| Vent::from_str(line).unwrap())
        .for_each(|vent| {
            vent.covering_points()
                .into_iter()
                .for_each(|(x, y)| map[y * MAP_SIZE + x] += 1);
        });

    map
        .into_iter()
        .filter(|&overlaps| overlaps >= 2)
        .count()
}

fn main() {
    let input = read_to_string("input.txt").expect("Input file not found");
    let hor_ver_count = calculate_horizontal_vertical(&input);
    println!("horizontal/vertical count: {}", hor_ver_count);
    let diag_count = calculate_diagonal(&input);
    println!("diagonal count: {}", diag_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_digit_vent() {
        let entry = Vent::from_str("1,2 -> 3,4").unwrap();
        assert_eq!(Vent { x1: 1, y1: 2, x2: 3, y2: 4 }, entry);
    }

    #[test]
    fn double_digit_vent() {
        let entry = Vent::from_str("12,23 -> 34,45").unwrap();
        assert_eq!(Vent { x1: 12, y1: 23, x2: 34, y2: 45 }, entry);
    }

    #[test]
    fn triple_digit_vent() {
        let entry = Vent::from_str("123,234 -> 345,456").unwrap();
        assert_eq!(Vent { x1: 123, y1: 234, x2: 345, y2: 456 }, entry);
    }

    #[test]
    fn covers_small_range() {
        let vent = Vent::from_str("1,1 -> 1,3").unwrap();
        assert_eq!([(1,1), (1,2), (1,3)], vent.covering_points()[..])
    }

    #[test]
    fn covers_downwards() {
        let vent = Vent::from_str("9,7 -> 7,7").unwrap();
        assert_eq!([(7,7), (8,7), (9,7)], vent.covering_points()[..])
    }

    #[test]
    fn rev_coords() {
        let vent = Vent::from_str("8,0 -> 0,8").unwrap();
        assert_eq!([(8, 0), (7, 1), (6, 2), (5, 3), (4, 4), (3, 5), (2, 6), (1, 7), (0, 8)], 
            vent.covering_points()[..])
    }
}