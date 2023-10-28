use std::collections::HashMap;

use aoc2015::read_lines;

/// infinite 2d grid
/// instructions as ><^V
/// delivers a present on moving (including his final move)
// map with cartesian coordinates as the keys
// Map {
//     (0, 0) : 1,

// }

#[derive(Debug, Eq, PartialEq, Hash)]
struct HouseLocation(isize, isize);
#[derive(Debug)]
struct HouseValue {
    count: usize,
}

fn main() {
    let mut santa_x_pos = 0;
    let mut santa_y_pos = 0;
    let mut robo_x_pos = 0;
    let mut robo_y_pos = 0;
    let mut map: HashMap<HouseLocation, HouseValue> = HashMap::new();
    let mut santas_turn = true;

    // initial location
    map.insert(HouseLocation(0, 0), HouseValue { count: 1 });

    if let Ok(lines) = read_lines("input/day3/day3input.txt") {
        for line in lines {
            // Each string
            if let Ok(ref test_str) = line {
                for char in test_str.chars() {
                    // Move location on grid

                    if santas_turn {
                        match char {
                            '>' => santa_x_pos += 1,
                            '<' => santa_x_pos -= 1,
                            '^' => santa_y_pos += 1,
                            'v' => santa_y_pos -= 1,
                            e => panic!("Wrong character, read in {e}"),
                        }

                        let map_key = HouseLocation(santa_x_pos, santa_y_pos);
                        // Update santa map
                        map.entry(map_key)
                            .and_modify(|house| house.count += 1)
                            .or_insert(HouseValue { count: 1 });
                    } else {
                        match char {
                            '>' => robo_x_pos += 1,
                            '<' => robo_x_pos -= 1,
                            '^' => robo_y_pos += 1,
                            'v' => robo_y_pos -= 1,
                            e => panic!("Wrong character, read in {e}"),
                        }

                        let map_key = HouseLocation(robo_x_pos, robo_y_pos);
                        map.entry(map_key)
                            .and_modify(|house| house.count += 1)
                            .or_insert(HouseValue { count: 1 });
                    }
                    santas_turn = !santas_turn;
                }
            };
        }
    }
    println!("{}", map.len());
}
