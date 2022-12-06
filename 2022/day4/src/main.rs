use std::{fs::read_to_string, str::FromStr};

fn take_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let extracted_end: usize = s
        .char_indices()
        .find_map(|(idx, c)| if accept(c) { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());

    let extracted = &s[..extracted_end];
    let remainder = &s[extracted_end..];
    (remainder, extracted)
}

fn skip_unneeded(s: &str) -> (&str, &str) {
    take_while(|c| c == ',' || c == '-', s)
}

struct SectionGroup {
    first_elf: (u32, u32),
    last_elf: (u32, u32),
}

impl SectionGroup {
    fn get_first_section(&self) -> Vec<u32> {
        (self.first_elf.0..=self.first_elf.1).collect::<Vec<u32>>()  
    } 

    fn get_second_section(&self) -> Vec<u32> {
        (self.last_elf.0..=self.last_elf.1).collect::<Vec<u32>>()
    }
}

impl FromStr for SectionGroup {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rem, fstart) = take_while(|c| c.is_ascii_digit(), s);
        let (rem, _) = skip_unneeded(rem);
        let (rem, fend) = take_while(|c| c.is_ascii_digit(), rem);
        let (rem, _) = skip_unneeded(rem);
        let (rem, sstart) = take_while(|c| c.is_ascii_digit(), rem);
        let (rem, _) = skip_unneeded(rem);
        let (_, send) = take_while(|c| c.is_ascii_digit(), rem);

        Ok(SectionGroup {
            first_elf: (fstart.to_string().parse().unwrap(), fend.to_string().parse().unwrap()),
            last_elf: (sstart.to_string().parse().unwrap(), send.to_string().parse().unwrap()),
        })
    }
}

fn main() {
    let input = read_to_string("input.txt").expect("Input file not found");
    let groups = input
        .lines()
        .map(|line| SectionGroup::from_str(line).unwrap())
        .collect::<Vec<SectionGroup>>();

    let count = groups
        .iter()
        .fold(0, |accum, group| { 
            let section1 = group.get_first_section();
            let section2 = group.get_second_section();

            let intertwined = compare_sections(&section1, &section2);
            if intertwined {
                return accum + 1;
            }

            accum
         });

    let count2 = groups
         .iter()
         .fold(0, |accum, group| {            
            let section1 = group.get_first_section();
            let section2 = group.get_second_section();

            let overlapping = overlap_sections(&section1, &section2);
            if overlapping {
                return accum + 1;
            }

            accum
         });

    println!("Interwined section groups: {}", count);
    println!("Overlapping section groups: {}", count2);

}

fn compare_sections(sections1: &Vec<u32>, sections2: &Vec<u32>) -> bool {
    let mut result = section_in_other_section(&sections1, &sections2);
    if !result {
        result = section_in_other_section(&sections2, &sections1);
    }
    
    result
}

fn overlap_sections(sections1: &Vec<u32>, sections2: &Vec<u32>) -> bool {
    let mut result = section_overlap_other_section(sections1, sections2);
    if !result {
        result = section_overlap_other_section(sections2, sections1);
    }

    result
}

fn section_in_other_section(sections1: &Vec<u32>, sections2: &Vec<u32>) -> bool {
    sections1.iter().all(|c| sections2.iter().any(|d| d == c))
}

fn section_overlap_other_section(sections1: &Vec<u32>, sections2: &Vec<u32>) -> bool {
    sections1.iter().any(|c| sections2.iter().any(|d| d == c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_assigned_sections() {
        let elf1 = (2..=8).collect::<Vec<u32>>();
        let elf2 = (3..=7).collect::<Vec<u32>>();

        let mut result = section_in_other_section(&elf1, &elf2);
        if !result {
            result = section_in_other_section(&elf2, &elf1);
        }

        assert!(result);
    }

    #[test]
    fn create_section_group_single_digit() {
        let input = "2-4,6-8";
        let result = SectionGroup::from_str(input).unwrap();

        assert_eq!((2,4), result.first_elf);
        assert_eq!((6,8), result.last_elf);
    }

    #[test]
    fn create_section_group_double_digit() {
        let input = "12-24,36-48";
        let result = SectionGroup::from_str(input).unwrap();

        assert_eq!((12,24), result.first_elf);
        assert_eq!((36,48), result.last_elf);
    }

    #[test]
    fn overlap_sections_one() {
        let input = "5-7,7-9";
        let group = SectionGroup::from_str(input).unwrap();
        let result = overlap_sections(&group.get_first_section(), &group.get_second_section());

        assert!(result);
    }

    #[test]
    fn overlap_sections_more() {
        let input = "2-8,3-7";
        let group = SectionGroup::from_str(input).unwrap();
        let result = overlap_sections(&group.get_first_section(), &group.get_second_section());

        assert!(result);
    }

    #[test]
    fn overlap_sections_end() {
        let input = "2-6,4-8";
        let group = SectionGroup::from_str(input).unwrap();
        let result = overlap_sections(&group.get_first_section(), &group.get_second_section());

        assert!(result);
    }
}