use std::fs;

struct PartNumber {
    number: u32,
    start_x: i32,
    end_x: i32,
    y: i32
}

impl PartNumber {
    fn to_string(&self) -> String {
        return format!("Part Number: {}, Start X: {}, End X: {}, Y: {}",
            self.number,
            self.start_x,
            self.end_x,
            self.y
        );
    }

    fn is_adjacent_to(&self, x: i32, y:i32) -> bool {
        if x > (self.end_x + 1) || x < (self.start_x - 1) {
            return false;
        }
        if y > (self.y + 1) || y < (self.y - 1) {
            return false;
        }
        true
    }
}

struct Symbol {
    character: char,
    x: i32,
    y: i32,
    is_gear: bool,
    adjacent_part_numbers: Vec<u32>

}

impl Symbol {
    fn to_string(&self) -> String {
        return format!("Symbol: {}, X: {}, Y: {}",
            self.character,
            self.x,
            self.y
        );
    }
}

fn main() {
    let input_path = "res/input.txt";
    let input = fs::read_to_string(input_path)
        .expect("Failed to read file");

    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    let mut y = 0;
    for line in input.lines() {
       
        println!("Line: {line}");
       
        let char_vec: Vec<char> = line.chars().collect();
        let mut part_number_string: String = Default::default();

        // iterate over all characters in a line
        for x in 0..char_vec.len() {
            let character = char_vec[x];

            if character.is_numeric() {
                // add numbers to a string
                part_number_string.push(character);
            }
            else {

                if !part_number_string.is_empty() {
                    // if we found a non-number, but we found numbers before, then this is the end of a number
                    // consume the part number string, and add it to the part numbers
                    let start_x = x - part_number_string.len();
                    let end_x = x - 1;
                    consume_and_write_number_to_vec(
                        &mut part_number_string,
                        start_x,
                        end_x,
                        y,
                        &mut part_numbers
                    );
                }

                if character != '.' {
                    // if we found a non-number that is not a ., it is a symbol
                    let symbol = Symbol {
                        character,
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),

                        // part 2: gears are * symbols
                        is_gear: character == '*',
                        adjacent_part_numbers: Vec::new()
                    };

                    // store all symbols for later
                    symbols.push(symbol);
                }
            }
        }

        // if we reached the end of a line but have a non-consumed part number string, we must consume it
        if !part_number_string.is_empty() {
            let start_x = char_vec.len() - part_number_string.len();
            let end_x = char_vec.len() - 1;
            consume_and_write_number_to_vec(
                &mut part_number_string,
                start_x,
                end_x,
                y,
                &mut part_numbers
            );
        }

        // count the current line being iterated over
        y += 1;
    }

    let mut part_number_sum = 0;

    for part_number in part_numbers {
        println!("{}", part_number.to_string());

        let mut had_adjacent_symbol = false;
        for i in 0..symbols.len() {
            let symbol = &mut symbols[i];

            if part_number.is_adjacent_to(symbol.x.try_into().unwrap(), symbol.y.try_into().unwrap()) {
                println!("  Is adjacent to: {}", symbol.to_string());
                had_adjacent_symbol = true;

                // part 2: store what part numbers a gear was next to
                if symbol.is_gear {
                    symbol.adjacent_part_numbers.push(part_number.number);
                }
                break;
            }
        }

        // if we had an adjacent symbol, add us to the sum
        if had_adjacent_symbol {
            part_number_sum += part_number.number;
        }
        else {
            println!("  Is NOT adjacent to a symbol!");
        }
    }

    // part 2: find gear ratio sum
    let mut gear_ratio_sum = 0;

    for symbol in symbols {
        if symbol.is_gear && symbol.adjacent_part_numbers.len() == 2 {
            // for each valid gear, multiply together the two part numbers and add it to the sum
            let gear_ratio = symbol.adjacent_part_numbers[0] * symbol.adjacent_part_numbers[1];
            gear_ratio_sum += gear_ratio;
            println!("  Gear {} with Gear Ratio: {gear_ratio}", symbol.to_string());
        }
    }

    // for symbol in symbols {
    //     println!("  {}", symbol.to_string());
    // }

    println!("Part 1: Sum: {part_number_sum}");
    println!("Part 2: Gear Ratio Sum: {gear_ratio_sum}");
}

fn consume_and_write_number_to_vec(number_string: &mut String, start_x: usize, end_x: usize, y:usize, numbers: &mut Vec<PartNumber>) {
    // Store the number string we found so far
    let number = PartNumber {
        number: number_string.parse::<u32>().unwrap(),
        start_x: start_x.try_into().unwrap(),
        end_x: end_x.try_into().unwrap(),
        y: y.try_into().unwrap(),
    };

    numbers.push(number);
    number_string.clear();
}