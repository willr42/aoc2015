/// One million lights to a 1000x1000 grid.
/// Grid is numbered 0,999 on each dimension

/// Instruction format
/// turn on 0,0 through 999,999
/// toggle 0,0 through 999,0 - toggle first line of lights, turning off ones on, turning on ones off
/// turn off 499,499 through 500,500 - turn off & leave off the middle 4 lights.
use aoc2015::read_lines;

fn main() {
    if let Ok(lines) = read_lines("input/day3/day3input.txt") {
        for line in lines {
            // Each string
            if let Ok(current_line) = line {
                for char in current_line.chars() {}
            }
        }
    }
}
