mod paper;
use std::fs::File;
use std::io::{BufRead, BufReader};
use sscanf::scanf;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut total_paper = 0;
    let mut total_ribbon = 0;
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let parsed = scanf!(line, "{u32}x{u32}x{u32}");
        let (l, w, h) = parsed.unwrap();
        total_paper += paper::calculate_wrapping_paper(l, w, h);
        total_ribbon += paper::calculate_ribbon(l, w, h);
    }

    println!("Total paper: {} square feet", total_paper);
    println!("Total ribbon: {} feet of ribbon", total_ribbon);
}