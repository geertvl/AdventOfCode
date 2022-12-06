fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_assigned_sections() {
        let elf1 = (2..=4).collect::<Vec<u32>>();
        let elf2 = (6..=8).collect::<Vec<u32>>();

        for i in elf1 {
            println!("{:?}", i);
        }
    }
}