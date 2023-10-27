// They have a list of the dimensions (length l, width w, and height h)
// 2*l*w + 2*w*h + 2*h*l
//A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
// A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet of wrapping paper plus 1 square foot of slack, for a total of 43 square feet.

use aoc2015::{self, read_lines};
use itertools::Itertools;

#[derive(Debug)]
struct Present {
    length: usize,
    width: usize,
    height: usize,
}

impl Present {
    fn new(length: usize, width: usize, height: usize) -> Present {
        return Present {
            length,
            width,
            height,
        };
    }

    fn required_wrapping(&self) -> usize {
        let initial_required = (2 * self.length * self.width)
            + (2 * self.width * self.height)
            + (2 * self.height * self.length);

        let min_dimension = *vec![
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ]
        .iter()
        .min()
        .unwrap();

        return initial_required + min_dimension;
    }

    fn required_ribbon(&self) -> usize {
        // Initial ribbon
        let perimeters = vec![
            2 * (self.length + self.width),
            2 * (self.width + self.height),
            2 * (self.height + self.length),
        ];

        let min_perimeter = *perimeters.iter().min().unwrap();
        dbg!(min_perimeter);
        //Bow = cubic feet of volume of present
        let bow_length = self.length * self.width * self.height;
        dbg!(bow_length);

        return min_perimeter + bow_length;
    }
}

impl From<String> for Present {
    /// input_str must be in format of `lxwxh`
    fn from(input_str: String) -> Self {
        let (length, width, height) = input_str
            .split("x")
            .map(|s| s.parse::<usize>().unwrap())
            .collect_tuple()
            .expect("Should fit in a tuple");

        return Present {
            length,
            width,
            height,
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::Present;

    #[test]
    fn create_present_from_string() {
        let test_str = "1x2x3".to_string();
        let test_present = Present::from(test_str);
        assert_eq!(
            (1usize, 2usize, 3usize),
            (test_present.length, test_present.width, test_present.height)
        );
    }

    #[test]
    fn correct_wrapping_size_with_diff_dimensions() {
        let test_present = Present::new(2, 3, 4);
        assert_eq!(58, test_present.required_wrapping())
    }
    #[test]
    fn correct_wrapping_size_with_similar_dimensions() {
        let test_present = Present::new(1, 1, 10);
        assert_eq!(43, test_present.required_wrapping())
    }
    #[test]
    fn correct_bow_length_with_diff_dimensions() {
        let test_present = Present::new(2, 3, 4);
        assert_eq!(34, test_present.required_ribbon())
    }
    #[test]
    fn correct_bow_length_with_similar_dimensions() {
        let test_present = Present::new(1, 1, 10);
        assert_eq!(14, test_present.required_ribbon())
    }
}

fn main() {
    if let Ok(lines) = read_lines("input/day3/day3input.txt") {
        for line in lines {
            if let Ok(present_str) = line {
                // some logic here
            }
        }
    };
    println!("Result: {}", result)
}
