use std::{fs::{read_to_string}};

fn find_packet_marker(input: &str) -> usize {
    (5..input.len()).find(|c| {
        *(&input[c-4..*c].chars().all(|x| &input[c-4..*c].chars().filter(|y| x == *y).count() == &1))
    }).unwrap()    
}

fn find_message_marker(input: &str) -> usize {
    (15..input.len()).find(|c| {
        *(&input[c-14..*c].chars().all(|x| &input[c-14..*c].chars().filter(|y| x == *y).count() == &1))
    }).unwrap()    
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let input = read_to_string("input.txt")?;
    
    let packet_marker = find_packet_marker(input.as_str());
    println!("Packet marker: {}", packet_marker);
    let message_marker = find_message_marker(input.as_str());
    println!("Message marker: {}", message_marker);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marker_after_5() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let marker = find_packet_marker(input);
        
        assert_eq!(5, marker);
    }

    #[test]
    fn marker_after_6() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let marker = find_packet_marker(input);

        assert_eq!(6, marker);
    }

    #[test]
    fn marker_after_10() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let marker = find_packet_marker(input);

        assert_eq!(10, marker);
    }

    #[test]
    fn marker_after_11() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let marker = find_packet_marker(input);

        assert_eq!(11, marker);
    }
}