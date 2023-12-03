use std::fs;

struct Number {
    number: u32,
    start_x: i32,
    end_x: i32,
    y: i32
}

impl Number {
    fn to_string(&self) -> String {
        return format!("Number: {}, Start X: {}, End X: {}, Y: {}",
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
    x: usize,
    y: usize,
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

    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    let mut y = 0;
    for line in input.lines() {
       
        println!("Line: {line}");
       
        let char_vec: Vec<char> = line.chars().collect();

        let mut number_string: String = Default::default();
        for x in 0..char_vec.len() {
            let character = char_vec[x];

            if character.is_numeric() {
                number_string.push(character);
            }
            else {
                if !number_string.is_empty() {
                    // Store the number string we found so far

                    let start_x = x - number_string.len();
                    let end_x = x - 1;
                    consume_and_write_number_to_vec(
                        &mut number_string,
                        start_x,
                        end_x,
                        y,
                        &mut numbers
                    );
                }

                if character != '.' {
                    let symbol = Symbol {
                        character,
                        x: x,
                        y: y,
                    };

                    symbols.push(symbol);
                }
            }
        }

        if !number_string.is_empty() {
            let start_x = char_vec.len() - number_string.len();
            let end_x = char_vec.len() - 1;
            consume_and_write_number_to_vec(
                &mut number_string,
                start_x,
                end_x,
                y,
                &mut numbers
            );
        }

        y += 1;
    }

    let mut sum = 0;

    for number in numbers {
        println!("{}", number.to_string());

        let mut had_adjacent_symbol = false;
        for i in 0..symbols.len() {
            let symbol = &symbols[i];

            if number.is_adjacent_to(symbol.x.try_into().unwrap(), symbol.y.try_into().unwrap()) {
                had_adjacent_symbol = true;
                println!("  Is adjacent to {}", symbol.to_string());
                break;
            }
        }

        if had_adjacent_symbol {
            sum += number.number;
        }
        else {
            println!("  Is NOT adjacent to a symbol!");
        }
    }

    // for symbol in symbols {
    //     println!("  {}", symbol.to_string());
    // }

    println!("Sum: {sum}");
}

fn consume_and_write_number_to_vec(number_string: &mut String, start_x: usize, end_x: usize, y:usize, numbers: &mut Vec<Number>) {
    // Store the number string we found so far
    let number = Number {
        number: number_string.parse::<u32>().unwrap(),
        start_x: start_x.try_into().unwrap(),
        end_x: end_x.try_into().unwrap(),
        y: y.try_into().unwrap(),
    };

    numbers.push(number);
    number_string.clear();
}