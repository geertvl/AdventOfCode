use std::fs::read_to_string;

fn parse_calories(input: &str) -> Vec<u32> {
    // let calories: Vec<u32> = input
    //     .lines()
    //     .filter(|&lines| !lines.is_empty())
    //     .map(|line| line.parse::<u32>().unwrap())
    //     .collect();

    let calories: Vec<u32> = input
        .lines()
        .fold(Vec::new(), |mut acc, x| {
            if x.is_empty() || acc.is_empty() {                
                acc.push(Vec::new());
            } 
              
            if !x.is_empty() {
                acc.last_mut().unwrap().push(x.parse::<u32>().unwrap());
            }
            acc
        })
        .into_iter()
        .map(|x| x.into_iter().reduce(|acc, item|  acc + item).unwrap())
        .collect(); 
       

        // .reduce(|acc, item| if acc >= item { acc } else { item })

    calories
}

fn calculate_highest_calories(calories: &Vec<u32>) -> &u32 {
    calories
        .into_iter()
        // .reduce(|acc, item| if acc >= item { acc } else { item })
        .max()
        .unwrap()
}

fn calculate_top_3_highest_calories(calories: &Vec<u32>) -> u32 {
    let mut calories = calories.clone();
    calories
        .sort_by(|a, b| b.cmp(a));
    let sum: u32 = calories
        .into_iter()
        .take(3)
        .collect::<Vec<u32>>()
        .iter()
        .sum();
    
    sum
}

fn main() {
    let input = read_to_string("input.txt").expect("Input file not found");
    let calories = parse_calories(&input);

    let highest_calories = calculate_highest_calories(&calories);
    println!("Highest calories: {}", highest_calories);
    let top_3_highest_calories = calculate_top_3_highest_calories(&calories);
    println!("Top 3 sum: {}", top_3_highest_calories);
}
