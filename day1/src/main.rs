use std::fs::read_to_string;

fn day1() {
    let path = "src/input-day1.txt";
    let input = read_to_string(path)
        .expect("Should have been able to read the file");

    let mut sum: i32 = 0;

    for line in input.split('\n') {
        let mut str = String::from("");

        for c in line.chars() {
            if c.is_digit(10) {
                str.push(c);
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_digit(10) {
                str.push(c);
                break;
            }
        }

        if str.is_empty() {
            continue;
        }

        sum += str.parse::<i32>().unwrap();
    }

    println!("Input: {sum}");
}

fn day1_part2() {
    let path = "src/input-day1.txt";
    let input = read_to_string(path)
        .expect("Should have been able to read the file");

    let words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut sum: i32 = 0;

    for line in input.split('\n') {
        let mut str = String::from("");

        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                str.push(c);
                continue;
            }

            let maybe_word = &line[i..if i+5 >= line.len() {line.len()} else { i+5}];

            for word in words {
                if maybe_word.starts_with(word) {
                    str.push(word_to_digit(word));
                    continue;
                }
            }
        }

        if str.is_empty() {
            continue;
        }

        let mut calibration_number: String = String::from("");
        calibration_number.push(str.chars().next().unwrap());
        calibration_number.push(str.chars().last().unwrap());
        sum += calibration_number.parse::<i32>().unwrap();
    }

    println!("Input: {sum}");
}

fn word_to_digit(string: &str) -> char {
    match string {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => '0'
    }
}

fn main() {
    println!("Hello, world!");

    use std::time::Instant;
    let now = Instant::now();

    day1();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let now = Instant::now();

    day1_part2();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
