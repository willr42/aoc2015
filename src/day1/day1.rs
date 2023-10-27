#[derive(Debug)]
struct Count {
    up_a_floor: isize,
    down_a_floor: isize,
    current: isize,
}

impl Count {
    fn new() -> Count {
        Count {
            up_a_floor: 0,
            down_a_floor: 0,
            current: 0,
        }
    }
}

fn day1() {
    let content = read_file("input/day1/day1input.txt").unwrap();
    let mut counter = Count::new();
    for (i, char) in content.chars().enumerate() {
        if char == '(' {
            counter.up_a_floor += 1;
        };

        if char == ')' {
            counter.down_a_floor += 1;
        }

        counter.current = counter.up_a_floor - counter.down_a_floor;

        if counter.current == -1 {
            println!("Entered basement at {}", i + 1);
        }
    }

    println!("Destination floor: {}", counter.current);
}
