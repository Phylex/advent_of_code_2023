use std::{io::BufRead, io::BufReader, fs::File};

fn find_first_char_in_line(line: &str, reverse: bool) -> Option<char> {
    let char_iter: Box<dyn Iterator<Item=char>> = if !reverse {Box::new(line.chars())} else {Box::new(line.chars().rev())};
    for c in char_iter {
        if c.is_digit(10) {
            return Some(c);
        } 
    };
    None
}

fn replace_first_and_last_word_with_digit(line: &mut String) {
    const WRITTEN_DIGITS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut indices_of_digits: Vec<(usize, usize)> = Vec::new();
    for (i, &wd) in WRITTEN_DIGITS.iter().enumerate() {
        for (di, _) in line.match_indices(wd) {
            indices_of_digits.push((di, i));
        }
    }
    println!("{:?}", indices_of_digits);
    // replace the first letter of the last occurrence of any spelled out number with the digit as
    // string
    if let Some(max) = indices_of_digits.iter().max_by_key(|d| &d.0) {
        let replacement_digit = max.1.to_string();
        line.replace_range(max.0..=max.0, &replacement_digit);
    }
    // replace the first letter of the first occurrence of any spelled out digit in the string
    if let Some(min) = indices_of_digits.iter().min_by_key(|d| &d.0) {
        let replacement_digit = min.1.to_string();
        line.replace_range(min.0..=min.0, &replacement_digit);
    }
}


pub fn solve_day_1(freader: BufReader<File>, part_two: bool) {
    let mut sum = 0;
    for (i, line) in freader.lines().enumerate() {
        let mut line = match line {
            Ok(line) => line,
            Err(reason) => {
                println!("Failed to read line from file due to: {}", reason);
                return;
            }
        };
        print!("{} :", line);
        let line_content = if part_two {
            replace_first_and_last_word_with_digit(&mut line);
            line.clone()
        } else {
            line.clone()
        };
        print!(" {} : ", line_content);
        let first_digit = match find_first_char_in_line(&line_content, false) {
            Some(digit) => digit,
            None => {println!("No digit fond for line {}, aborting", i); return}
        };
        let second_digit = match find_first_char_in_line(&line_content, true) {
            Some(digit) => digit,
            None => {println!("No digit fond for line {}, aborting", i); return}
        };
        let mut complete_string = first_digit.to_string();
        complete_string.push(second_digit);
        let total = u32::from_str_radix(&complete_string, 10).unwrap();
        println!("Number of line {} is {}" , i, total);
        sum += total;
    }
    println!("The sum of all is {}", sum);
}
