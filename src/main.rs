use aoc2015::read_lines;

/// One million lights to a 1000x1000 grid.
/// Grid is numbered 0,999 on each dimension

/// turn on 0,0 through 999,999
/// turn off 499,499 through 500,500 - turn off & leave off the middle 4 lights.
/// toggle 0,0 through 999,0 - toggle first line of lights, turning off ones on, turning on ones off

/// from 0 to 999
struct Coordinate(usize, usize);

enum Direction {
    On,
    Off,
    Toggle,
}

/// parsed from a string
/// turn on 0,0 through 999,999
struct Instruction {
    direction: Direction,
    start: Coordinate,
    stop: Coordinate,
}

fn parse_instruction(instruction_str: String) -> Instruction {
    let mut direction = Direction::Off;

    for str in instruction_str.split(' ') {
        match str {
            "toggle" => direction = Direction::Toggle,
            "off" => direction = Direction::Off,
            "on" => direction = Direction::On,
            _ => (),
        };
    }

    Instruction {
        direction,
        start: Coordinate(0, 0),
        stop: Coordinate(999, 999),
    }
}

fn main() {
    day6("test");
}

fn day6(test_input: &str) -> usize {
    let light_grid = [[false; 999]; 999];
    dbg!(light_grid);
    if let Ok(lines) = read_lines("input/day3/day3input.txt") {
        for line in lines {
            // Each string
            if let Ok(current_line) = line {}
        }
    }
    return 0;
}

mod tests {
    use crate::day6;

    #[test]
    fn all_lights() {
        let result = day6("turn on 0,0 through 999,999");
    }
}
