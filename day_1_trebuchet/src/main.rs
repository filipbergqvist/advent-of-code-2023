use std::fs;
use std::str;

fn main() {
    let path: &str = "res/input.txt";
    let input: String = fs::read_to_string(path)
        .expect("Fail");

    // we want to collect the calibration value of each line
    let mut sum = 0;

    // read all of the lines in the input file
    let lines = input.lines();

    // iterate over the lines
    for line in lines {

        // convert the line to a char iterator
        // we filter out every character that is not numeric
        let digits = line
            .chars()
            .filter(|character| character.is_numeric());

        // collect the digits into a vector to allow indexing
        let digits_vector: Vec<char> = digits.collect();

        if digits_vector.is_empty() {
            println!("Line: {line}: Line has no digits");
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
        println!("Line: {line}: Calibration values: {calibration_values}");

        // add the parsed i32 to the sum
        sum += calibration_values;
    }

    println!("Sum: {sum}");
}
