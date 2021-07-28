use std::collections::HashMap;

fn main() {
    let file = std::fs::read_to_string("input.txt").expect("Failed to read file");

    let mut santa = Santa::new();
    let mut robo_santa = Santa::new();

    let mut step_count = 1;
    for char in file.chars() {
        match step_count % 2 {
            1 => santa.make_move(char),
            0 => robo_santa.make_move(char),
            _ => (),
        }
        step_count += 1;
    }

    santa.combine_maps(&mut robo_santa.houses_visited);
    println!("Number of houses visited: {}", santa.get_number_unique_houses());
}

struct Santa {
    x: i32,
    y: i32,
    houses_visited: HashMap<(i32, i32), i32>
}

impl Santa {

    fn new() -> Santa {
        Santa {
            x: 0,
            y: 0,
            houses_visited: HashMap::new(),
        }
    }

    fn combine_maps(&mut self, other_map: &mut HashMap<(i32, i32), i32>) {
        
        for house in other_map.keys() {
            if !self.houses_visited.contains_key(&house) {
                self.houses_visited.insert(house.clone(), other_map[house]);
            } else {
                self.houses_visited.insert(*house, self.houses_visited[&house] + other_map[house]);
            }
        }
    }

    fn make_move(&mut self, input: char) -> () {
        match input {
            '^' => self.y += 1,
            'v' => self.y -= 1,
            '>' => self.x += 1,
            '<' => self.x -= 1,
            _ => (),
        }

        let current_house = self.get_location();

        if !self.houses_visited.contains_key(&current_house) {
            self.houses_visited.insert(current_house, 0);
        }
        self.houses_visited.insert(current_house, self.houses_visited[&current_house] + 1);
    }
    
    fn get_location(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn get_number_unique_houses(&self) -> usize {
        self.houses_visited.keys().len()
    }

}

#[test]
fn test_move() {
    let mut santa = Santa::new();

    santa.make_move('^');
    santa.make_move('>');
    santa.make_move('>');

    let new_loc = santa.get_location();

    assert_eq!((2,1), new_loc)
}

#[test]
fn test_house_visits() {
    let mut santa = Santa::new();

    santa.make_move('^');
    assert_eq!(1,santa.houses_visited[&santa.get_location()]);
    santa.make_move('>');
    assert_eq!(1,santa.houses_visited[&santa.get_location()]);
    santa.make_move('>');
    assert_eq!(1,santa.houses_visited[&santa.get_location()]);
    santa.make_move('<');
    assert_eq!(2,santa.houses_visited[&santa.get_location()]);
}

#[test]
fn test_house_count() {
    let mut santa = Santa::new();

    santa.make_move('^');
    santa.make_move('>');
    santa.make_move('>');
    santa.make_move('<');

    assert_eq!(3, santa.get_number_unique_houses())
}
