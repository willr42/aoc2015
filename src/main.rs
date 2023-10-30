use aoc2015::read_lines;

// One million lights to a 1000x1000 grid.
// Grid is numbered 0,999 on each dimension

/// From 0 to 999
#[derive(Debug, PartialEq)]
struct Coordinate(usize, usize);

#[derive(Debug, PartialEq)]
enum Direction {
    On,
    Off,
    Toggle,
}

/// parsed from a string
/// turn on 0,0 through 999,999
#[derive(Debug, PartialEq)]
struct Instruction {
    direction: Direction,
    start: Coordinate,
    stop: Coordinate,
}

fn parse_instruction(instruction_str: String) -> Instruction {
    let mut direction = Direction::Off;

    let instruction_vec: Vec<&str> = instruction_str.split(" ").collect();

    // Use index to track where we are in the 2x diff types of strings.
    // Not proud of this code but it works
    let mut index = 0;

    if instruction_vec[index] == "toggle" {
        direction = Direction::Toggle;
        index += 1;
    } else {
        index += 1;
        match instruction_vec[index] {
            "on" => direction = Direction::On,
            "off" => direction = Direction::Off,
            _ => panic!("Direction string should only be either on or off"),
        }
        index += 1;
    };

    let start_pos = instruction_vec[index].split_once(",").unwrap();
    let x_min: usize = start_pos.0.parse().unwrap();
    let y_min: usize = start_pos.1.parse().unwrap();
    index += 2;

    let end_pos = instruction_vec[index].split_once(",").unwrap();
    let x_max: usize = end_pos.0.parse().unwrap();
    let y_max: usize = end_pos.1.parse().unwrap();

    Instruction {
        direction,
        start: Coordinate(x_min, y_min),
        stop: Coordinate(x_max, y_max),
    }
}

fn main() {
    day6("test");
}

fn day6(test_input: &str) -> usize {
    let mut light_grid = [[false; 1000]; 1000];
    if let Ok(lines) = read_lines("input/day6/input.txt") {
        for line in lines {
            // Each string
            if let Ok(current_line) = line {
                let instruction = parse_instruction(current_line);
                if instruction.direction == Direction::On {
                    let rows = &mut light_grid[instruction.start.0..=instruction.stop.0];
                    for row in rows {
                        let cols = &mut row[instruction.start.1..=instruction.stop.1];
                        for light in cols {
                            *light = true;
                        }
                    }
                } else if instruction.direction == Direction::Off {
                    let rows = &mut light_grid[instruction.start.0..=instruction.stop.0];
                    for row in rows {
                        let cols = &mut row[instruction.start.1..=instruction.stop.1];
                        for light in cols {
                            *light = false;
                        }
                    }
                } else if instruction.direction == Direction::Toggle {
                    let rows = &mut light_grid[instruction.start.0..=instruction.stop.0];
                    for row in rows {
                        let cols = &mut row[instruction.start.1..=instruction.stop.1];
                        for light in cols {
                            if *light == true {
                                *light = false;
                            } else {
                                *light = true;
                            }
                        }
                    }
                }
            }
        }
    }

    let final_count = light_grid
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&light| light)
        .count();

    return 0;
}

mod tests {
    use crate::{day6, parse_instruction, Coordinate, Instruction};

    #[test]
    fn instruction_lights_on() {
        let result = parse_instruction("turn on 0,0 through 999,999".to_string());
        let expected = Instruction {
            direction: crate::Direction::On,
            start: Coordinate(0, 0),
            stop: Coordinate(999, 999),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn instruction_toggles_lights() {
        let result = parse_instruction("toggle 0,0 through 999,999".to_string());
        let expected = Instruction {
            direction: crate::Direction::Toggle,
            start: Coordinate(0, 0),
            stop: Coordinate(999, 999),
        };
        assert_eq!(result, expected)
    }
}
