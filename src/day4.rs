// multithread this?
use md5;

fn main() {
    let input_str = "iwrupvqb".to_string();
    let mut counter: i128 = 0;

    loop {
        counter = counter + 1;
        let input = input_str.clone() + &counter.to_string();
        let hash = format!("{:x}", md5::compute(input));
        let first_five = &hash[0..5];

        if first_five == "000000" {
            println!("The count is {}", counter);
            break;
        }
    }
}
