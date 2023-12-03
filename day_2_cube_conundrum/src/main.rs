use std::collections::HashMap;
use std::fs;
use std::str;

struct Game {
    id: u32,
    color_counts: HashMap<String, u32>,
}

impl Game {
    fn is_possible_with_counts(&self, red_count: u32, green_count: u32, blue_count: u32) -> bool {
        let color_counts = &self.color_counts;
        
        if color_counts.get("red").unwrap() > &red_count {
            return false;
        }
        if color_counts.get("green").unwrap() > &green_count {
            return false;
        }
        if color_counts.get("blue").unwrap() > &blue_count {
            return false;
        }

        true
    }
}

fn main() {
    let _result = part_1();

    //println!("{result}");
}

fn part_1() {
    const INPUT_PATH: &str = "res/input.txt";
    let input: String = fs::read_to_string(INPUT_PATH)
        .expect("Fail");
    
    let mut sum: u32 = 0;

    for line in input.lines() {
        println!("\nLine:  {line}");

        let mut parts = line.split(':');
        
        let mut game_id_str = parts.next().unwrap();
        let subsets = parts.next().unwrap().split(';');

        game_id_str = game_id_str.split_whitespace().last().unwrap();
        let game_id = game_id_str.parse::<u32>().unwrap();
        let mut color_value_map = HashMap::new();

        println!("  Game ID:  {game_id}");

        for subset in subsets {
            println!("  Subset:  {subset}");
            let subset_parts = subset.split(",");

            for subset_part in subset_parts {
                let subset_part_trimmed = subset_part.trim();
                //println!("      Part:  {subset_part_trimmed}");

                let mut parts = subset_part_trimmed.split_whitespace();

                let count = parts.next().unwrap().parse::<u32>().unwrap();
                let color = parts.next().unwrap().to_string();

                //println!("      Color:  {color}, Count: {count}");

                if color_value_map.contains_key(&color) {
                    let current_color_count = color_value_map.get(&color).unwrap();
                    if *current_color_count < count {
                        color_value_map.insert(color, count);
                    }
                }
                else {
                    color_value_map.insert(color, count);
                }
            }
        }

        let game = Game {
            id: game_id,
            color_counts: color_value_map,
        };

        println!("Game {} Total: Red: {}, Green: {}, Blue: {}",
            game.id, 
            game.color_counts.get("red").unwrap(),
            game.color_counts.get("green").unwrap(),
            game.color_counts.get("blue").unwrap()
        );

        if game.is_possible_with_counts(12, 13, 14) {
            println!("Game {} is Possible!", game.id);
            sum += game.id;
            println!("Sum after adding: {}", sum);
        }
        else {
            println!("Game {} is NOT Possible!", game.id);
        }
    }

    println!("\nSum: {sum}");
}