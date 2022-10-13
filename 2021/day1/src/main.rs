use std::{fs::File, io::{BufReader, BufRead}};

fn parse(buf: &mut BufReader<File>) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    let mut depths: Vec<u64> = vec![];

    for line in buf.lines() {
        depths.push(line?.parse::<u64>()?);
    }

    Ok(depths)
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let input = File::open("input.txt")?;
    let mut reader = BufReader::new(input);

    let depths = parse(&mut reader)?;

    let depth_pairs = depths
        .iter()
        .zip(depths.iter().skip(1));

    let increases = depth_pairs
        .clone()
        .filter(|(x, y)| x < y)
        .count();

    println!("{}", increases);

    let depth_triplets = depth_pairs.zip(depths.iter().skip(2));
    let windows: Vec<u64> = depth_triplets.map(|((x, y), z)| x + y + z).collect();
    let window_pairs = windows.iter().zip(windows.iter().skip(1));

    println!("{}", window_pairs.filter(|(x, y)| x < y).count());

    Ok(())
}
