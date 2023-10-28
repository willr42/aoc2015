fn part_one_main() {
    let mut nice_string_count = 0;
    let bad_patterns = [
        "ab".to_string(),
        "cd".to_string(),
        "pq".to_string(),
        "xy".to_string(),
    ];
    let vowel_str = "aeiou";

    if let Ok(lines) = read_lines("input/day5/day5input.txt") {
        for line in lines {
            // Each string
            if let Ok(ref test_str) = line {
                let mut repeated_chars = false;
                let mut bad_chars: bool = false;
                let mut vowel_count = 0;
                let mut prev_char = "".to_string();

                // Each character
                for char in test_str.chars() {
                    // Count vowel
                    if vowel_str.contains(char) {
                        vowel_count += 1;
                    };

                    if char.to_string() == prev_char {
                        repeated_chars = true;
                    };

                    if bad_patterns.iter().any(|pattern| {
                        return *pattern == prev_char.clone() + &char.to_string();
                    }) {
                        bad_chars = true;
                    }
                    prev_char = char.to_string();
                }
                // increment nice_string_count if it passes all 3 conditions
                if vowel_count >= 3 && repeated_chars == true && bad_chars == false {
                    println!("Nice {:?}", line);
                    nice_string_count += 1;
                }

                println!("Nice strings: {}", nice_string_count)
            };
        }
    }
}
