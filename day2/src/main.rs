use std::str::FromStr;
//use std::fs;

fn main() {
    let file = std::fs::read_to_string("input.txt").expect("Failed to read file");

    let mut total_paper = 0;
    let mut total_ribbon = 0;

    for line in file.lines() {
        let some_box = Box::new(line);
        total_paper += some_box.calculate_paper();
        total_ribbon += some_box.calculate_ribbon();
    }

    println!("Total sq ft paper: {}", total_paper);
    println!("Total ft ribbon: {}", total_ribbon);
}

struct Box {
    length: i32,
    width: i32,
    height: i32,
}

impl Box {
    fn new(input: &str) -> Box {
        let mut split = input.split('x');
        
        Box { 
            length: FromStr::from_str(split.next().unwrap()).unwrap(),
            width: FromStr::from_str(split.next().unwrap()).unwrap(),
            height: FromStr::from_str(split.next().unwrap()).unwrap(),
        }
    }

    fn calculate_paper(&self) -> i32 {
        // 2*l*w + 2*w*h + 2*h*l
        let sides = vec![
            2 * (&self.length * &self.width),
            2 * (&self.width * &self.height),
            2 * (&self.height * &self.length),
        ];
        let slack = sides.iter().min().unwrap() / 2;
        let sum_of_sides: i32 = sides.iter().sum();

        sum_of_sides + slack
    }

    fn calculate_bow(&self) -> i32 {
        &self.length * &self.width * &self.height
    }

    fn get_smaller_sides(&self) -> Vec<&i32> {
        let mut sides = vec![&self.length, &self.width, &self.height];
        let max = sides.iter().max().unwrap();

        for (idx, side) in sides.iter().enumerate() {
            if side == max {
                sides.remove(idx);
                break;
            }
        }
        sides
    }

    fn calculate_ribbon(&self) -> i32 {
        let mut ribbon_len = 0;
        let sides = &self.get_smaller_sides();
        
        for side in sides {
            ribbon_len += *side * 2
        };

        ribbon_len += &self.calculate_bow();
        ribbon_len
    }
}

#[test]
fn ribbon_test() {
    let input = String::from("1x10x10");
    let test_box = Box::new(&input);

    assert_eq!(122, test_box.calculate_ribbon())
}