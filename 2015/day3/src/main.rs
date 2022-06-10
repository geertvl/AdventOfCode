use std::fs;

struct House {
    x: i32,
    y: i32,
    presents: u32,
}

impl House {
    fn increment(&mut self) {
        self.presents = self.presents + 1;
    }
}

struct Houses(Vec<House>);

impl Houses {
    fn new() -> Self {
        Houses(Vec::new())
    }

    fn push(&mut self, x: i32, y: i32) {
        let found_house = self.0.iter_mut().find(
            |h| h.x == x && h.y == y
        );
        match found_house {
            Some(house) => house.increment(),
            None => self.0.push(House { x: x, y: y, presents: 1 }),
        }
    }

    fn push_with_presents(&mut self, x: i32, y: i32, presents: u32) {
        self.0.push(House { x: x, y: y, presents: presents });
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

fn step(direction: char) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;
    match direction {
        '>' => x = x + 1,
        '<' => x = x - 1,
        '^' => y = y + 1,
        'v' => y = y - 1,
        _ => panic!("Direction is not known"),
    }

    (x, y)
}

fn process_santa(houses: &mut Houses, input: &String) {
    let mut pos_x = 0;
    let mut pos_y = 0;

    houses.push(pos_x, pos_y);

    for c in input.chars() {
        let (x, y) = step(c);
        pos_x += x;
        pos_y += y;
        houses.push(pos_x, pos_y);
    }
}

fn process_santa_robosanta(houses: &mut Houses, input: &String) {
    let mut santa_pos_x = 0;
    let mut santa_pos_y = 0;
    let mut robo_pos_x = 0;
    let mut robo_pos_y = 0;

    houses.push_with_presents(0, 0, 2);

    for (idx, c) in input.chars().enumerate() {
        if idx % 2 == 0 {
            let (x, y) = step(c);
            santa_pos_x += x;
            santa_pos_y += y;
            houses.push(santa_pos_x, santa_pos_y);
        } else {
            let (x, y) = step(c);
            robo_pos_x += x;
            robo_pos_y += y;
            houses.push(robo_pos_x, robo_pos_y);
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt");

    let contents = contents.unwrap();
    let mut santa_only_houses = Houses::new();
    process_santa(&mut santa_only_houses, &contents);
    println!("Santa delivered to {} houses at least 1 present", santa_only_houses.len());

    let mut santa_robo_houses = Houses::new();
    process_santa_robosanta(&mut santa_robo_houses, &contents);
    println!("Santa and Robo-Santa delivered to {} houses at least 1 present", santa_robo_houses.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_house(houses: &Houses, index: usize, presents: u32) {
        assert_eq!(houses.0[index].presents, presents);
    }

    #[test]
    fn deliver_two_houses() {
        let input = ">";
        let mut houses = Houses::new();
        process_santa(&mut houses, &input.to_string());
        assert_eq!(houses.len(), 2);
        assert_house(&houses, 0, 1);
        assert_house(&houses, 1, 1);
    }

    #[test]
    fn deliver_four_houses() {
        let input = "^>v<";
        let mut houses = Houses::new();
        process_santa(&mut houses, &input.to_string());
        assert_eq!(houses.len(), 4);
        assert_house(&houses, 0, 2);
        assert_house(&houses, 1, 1);
        assert_house(&houses, 2, 1);
        assert_house(&houses, 3, 1);
    }

    #[test]
    fn deliver_up_down() {
        let input = "^v^v^v^v^v";
        let mut houses = Houses::new();
        process_santa(&mut houses, &input.to_string());
        assert_eq!(houses.len(), 2);
        assert_house(&houses, 0, 6);
        assert_house(&houses, 1, 5);
    }

    #[test]
    fn deliver_by_santa_robo() {
        let input = "^v";
        let mut houses = Houses::new();
        process_santa_robosanta(&mut houses, &input.to_string());
    }
}