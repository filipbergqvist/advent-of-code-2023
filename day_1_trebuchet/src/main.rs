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
        let mut digit_string = String::default();

        // for all the characters in the line
        for index in 0..line.len() {
            // check if a digit is starting at this character
            let digit_char = get_digit_at_index(&characters_vector, index);
            
            match digit_char {
                Some(digit_char) => {
                    // if it is, add that digit to a string
                    digit_string.push(digit_char);
                }
                None => {
                    continue;
                }
            }
        }

        println!("  Parsed into {digit_string}");

        // convert the line to a char iterator
        // we filter out every character that is not numeric
        // collect the digits into a vector to allow indexing
        let digits_vector: Vec<char> = digit_string
            .chars()
            .collect();

        if digits_vector.is_empty() {
            println!("  Line has no digits");
            continue;
        }

        // copy the first and last digits from the vector
        let first_digit = digits_vector.first().unwrap().clone();
        let last_digit = digits_vector.last().unwrap().clone();
        
        // put the digits in a new string
        let mut first_and_last_digit: String = Default::default();
        first_and_last_digit.push(first_digit);
        first_and_last_digit.push(last_digit);

        // parse the new string as an i32
        let calibration_values = first_and_last_digit.parse::<i32>().unwrap();
        println!("  Calibration values: {calibration_values}");

        // add the parsed i32 to the sum
        sum += calibration_values;
    }

    println!("Sum: {sum}");
}

fn get_digit_at_index(characters: &Vec<char>, start_index: usize) -> Option<char> {
    
    // this is very inefficient, but I decided to use the same function
    // for both digits and aliases...
    // this way, order is easily maintained
    let digital_aliases = [
        ("1", '1'),
        ("2", '2'),
        ("3", '3'),
        ("4", '4'),
        ("5", '5'),
        ("6", '6'),
        ("7", '7'),
        ("8", '8'),
        ("9", '9'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9')
    ];

    for alias in digital_aliases {
        let characters_left = characters.len() - start_index;
        if characters_left < alias.0.len() {
            // the current alias does not fit in the rest
            // of the line, so it can't be valid
            continue;
        }

        let max_index = start_index + alias.0.len();

        // get a slice of the characters to try matching
        let slice = &characters[start_index..max_index];
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