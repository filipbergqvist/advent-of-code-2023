use std::fs;
use std::str;

const INPUT_PATH: &str = "res/input.txt";

fn main() {
    let input: String = fs::read_to_string(INPUT_PATH)
        .expect("Fail");

    // we want to collect the calibration value of each line
    let mut sum = 0;

    // read all of the lines in the input file
    let lines = input.lines();

    // iterate over the lines
    for line in lines {

        println!("\nLine: {line}");

        let characters_vector: Vec<char> = line.chars().collect();
        let mut first_digit = -1;
        let mut last_digit = -1;

        // for all the characters in the line
        for index in 0..characters_vector.len() {
            // check if a digit is starting at this character
            let digit = get_digit_at_index(&characters_vector, index);
            
            match digit {
                Some(digit) => {
                    // if it is, store it
                    if first_digit < 0 {
                        first_digit = digit;
                    }
                    
                    last_digit = digit
                }
                None => {
                    continue;
                }
            }
        }

        println!("  Found first digit {first_digit} and last digit {last_digit}");

        // parse the new string as an i32
        let calibration_values = (first_digit * 10) + last_digit;
        println!("  Calibration values: {calibration_values}");

        // add the parsed i32 to the sum
        sum += calibration_values;
    }

    println!("Sum: {sum}");
}

fn get_digit_at_index(characters: &Vec<char>, start_index: usize) -> Option<i32> {
    
    // this is very inefficient, but I decided to use the same function
    // for both digits and aliases...
    // this way, order is easily maintained
    let digital_aliases = [
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ];

    for alias in digital_aliases {
        let characters_left = characters.len() - start_index;
        if characters_left < alias.0.len() {
            // the current alias does not fit in the rest
            // of the line, so it can't be valid
            continue;
        }

        let end_index = start_index + alias.0.len();

        // get a slice of the characters to try matching
        let slice = &characters[start_index..end_index];
        let alias_vec: Vec<char> = alias.0.chars().collect();

        let mut alias_matches = true;
        // iterate over the characters in the alias
        for index in 0..alias_vec.len() {
            // if a character does not match, we are invalid
            if slice[index] != alias_vec[index] {
                alias_matches = false;
                break;
            }
        }

        if !alias_matches {
            continue;
        }

        println!("  Found digit {} at index {}", alias.0, start_index);
        return Some(alias.1);
    }

    None
}