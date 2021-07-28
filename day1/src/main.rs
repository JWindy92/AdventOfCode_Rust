use std::fs;
use std::collections::HashMap;
use std::error::Error;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = climb_floors(&input, false).unwrap();
    let when_enter_basement = climb_floors(&input, true).unwrap();

    println!("{:?}", result);
    println!("{:?}", when_enter_basement);
}

fn climb_floors(input: &str, find_basement: bool) -> Result<i32, Box<dyn Error>> {

    let mut translation: HashMap<char, i32> = HashMap::new();
    translation.insert('(', 1);
    translation.insert(')', -1);
    
    let mut current_pos = 0;
    let mut step_count = 1;

    for char in input.chars() {
        current_pos += translation.get(&char).unwrap();
        if current_pos < 0 && find_basement == true {
            return Ok(step_count)
        }
        step_count += 1;
    }

    Ok(current_pos)
}


#[test]
fn check_part1() {
    let input = String::from("))(((((");
    let result = climb_floors(&input, false).unwrap();
    assert_eq!(result, 3)
}

#[test]
fn check_part2() {
    let input = String::from("(()))");
    let result = climb_floors(&input, true).unwrap();
    assert_eq!(result, 5)
}