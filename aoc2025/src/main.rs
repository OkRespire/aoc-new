use std::{
    fs::File,
    io::{Error, Read},
};

struct AOC {}

impl AOC {
    pub fn day_one_part_one(contents: &String) -> i32 {
        let contents = contents.replace("R", "").replace("L", "-");
        let mut dial = 50;

        let mut count = 0;

        let contents_vec: Vec<i32> = contents
            .lines()
            .filter(|x| !x.is_empty())
            .map(|x| -> i32 { x.parse().expect("fuck") })
            .collect();

        for turn in contents_vec {
            dial = (dial + turn) % 100;
            if dial == 0 {
                count += 1;
            }
        }

        count
    }

    pub fn day_one_part_two(contents: &String) -> i32 {
        let contents = contents.replace("R", "").replace("L", "-");
        let mut dial = 50;

        let mut count = 0;

        let contents_vec: Vec<i32> = contents
            .lines()
            .filter(|x| !x.is_empty())
            .map(|x| -> i32 { x.parse().expect("fuck") })
            .collect();

        for turn in contents_vec {
            let div_val = if turn < 0 { turn / -100 } else { turn / 100 };
            let mod_val = if turn < 0 { turn % -100 } else { turn % 100 };
            count += div_val;

            if (dial != 0 && dial + mod_val <= 0) || (dial + mod_val >= 100) {
                count += 1
            }

            dial = ((dial + turn) % 100 + 100) % 100;
        }

        count
    }

    pub fn day_two_part_one(contents: &String) -> i128 {
        let mut total_invalid: i128 = 0;
        let range_vec: Vec<&str> = contents.split(",").collect();

        for range in range_vec {
            let parts: Vec<&str> = range.split("-").collect();

            let start_str = parts[0].trim();
            let end_str = parts[1].trim();

            let start = match start_str.parse::<i128>() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!(
                        "Warning: Could not parse start number from '{}'. Skipping.",
                        start_str
                    );
                    continue;
                }
            };

            let end = match end_str.parse::<i128>() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!(
                        "Warning: Could not parse end number from '{}'. Skipping.",
                        end_str
                    );
                    continue;
                }
            };

            for i in start..=end {
                let i_str = i.to_string();
                let mid_point = i_str.len() / 2;
                if i_str.len() % 2 == 0 && i_str[0..mid_point].eq(&i_str[mid_point..]) {
                    total_invalid += i;
                }
            }
        }

        total_invalid
    }

    // Used https://www.youtube.com/watch?v=LTT93lHogRM for this solution
    pub fn day_two_part_two(contents: &String) -> i128 {
        let mut total_invalid: i128 = 0;
        let range_vec: Vec<&str> = contents.split(",").collect();

        for range in range_vec {
            let parts: Vec<&str> = range.split("-").collect();

            let start_str = parts[0].trim();
            let end_str = parts[1].trim();

            let start = match start_str.parse::<i128>() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!(
                        "Warning: Could not parse start number from '{}'. Skipping.",
                        start_str
                    );
                    continue;
                }
            };

            let end = match end_str.parse::<i128>() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!(
                        "Warning: Could not parse end number from '{}'. Skipping.",
                        end_str
                    );
                    continue;
                }
            };

            for i in start..=end {
                let i_str = i.to_string();
                let mid_point = i_str.len() / 2;
                for j in 0..mid_point {
                    if i_str.len().rem_euclid(j + 1) == 0 {
                        let all_match = i_str[0..=j]
                            .chars()
                            .cycle()
                            .zip(i_str.chars())
                            .all(|(a, b)| a == b);
                        if all_match {
                            total_invalid += i;
                            break;
                        }
                    }
                }
            }
        }

        total_invalid
    }
}
fn day1() -> Result<(), Error> {
    let mut day1_file =
        File::open("/home/respire/Desktop/advent-of-code/aoc2025/input/dayone.txt")?;
    let mut contents = String::new();
    day1_file.read_to_string(&mut contents)?;
    let day1_answer_part1 = AOC::day_one_part_one(&contents);
    let day1_answer_part2 = AOC::day_one_part_two(&contents);
    println!(
        "---Day 1 Answers ---\nPart1: {}\nPart2: {}",
        day1_answer_part1, day1_answer_part2,
    );

    Ok(())
}

fn day2() -> Result<(), Error> {
    let mut day2_file =
        File::open("/home/respire/Desktop/advent-of-code/aoc2025/input/daytwo.txt")?;
    let mut contents = String::new();
    day2_file.read_to_string(&mut contents)?;
    let day2_answer_part1 = AOC::day_two_part_one(&contents);
    let day2_answer_part2 = AOC::day_two_part_two(&contents);

    println!(
        "---Day 2 Answers ---\nPart1: {}\nPart2: {}",
        day2_answer_part1, day2_answer_part2,
    );
    Ok(())
}

fn main() -> Result<(), Error> {
    day1()?;
    day2()?;

    Ok(())
}
