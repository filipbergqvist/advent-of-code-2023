use std::fs;

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
    matches: u32,
}

impl Card {
    fn count_matches(&self) -> u32 {
        let mut matches = 0;

        for number in self.numbers.iter() {
            if self.winning_numbers.contains(&number) {
                matches += 1;
            }
        }

        matches
    }

    fn to_string(&self) -> String {
        return format!("Card: {}, Matches: {}: {}| {}",
            self.id,
            self.matches,
            u32_vec_to_string(&self.winning_numbers),
            u32_vec_to_string(&self.numbers),
        );
    }
}

fn main() {
    let input_path = "res/input.txt";
    let input = fs::read_to_string(input_path)
        .expect("Failed to read file");

    let mut points = 0;

    for line in input.lines() {
        //println!("\nLine: {line}");
            
        let (card, number) = line.split_once(':').unwrap();
        let (_card_str, card_id) = card.split_once(' ').unwrap();
        let card_id_i = card_id.to_string().trim().parse::<u32>().unwrap();
        let (winning_numbers, your_numbers) = number.split_once('|').unwrap();

        let winning_numbers_vec: Vec<u32> = str_vec_to_u32_vec(winning_numbers.split_whitespace().collect());
        let numbers_vec: Vec<u32> = str_vec_to_u32_vec(your_numbers.split_whitespace().collect());

        let mut card = Card {
            id: card_id_i,
            winning_numbers: winning_numbers_vec,
            numbers: numbers_vec,
            matches: 0,
        };

        card.matches = card.count_matches();

        println!("{}", card.to_string());

        let points_from_card;
        match card.matches {
            0 => points_from_card = 0,
            1 => points_from_card = 1,
            _ => points_from_card = 1 << card.matches - 1,
        }

        println!("  Points from this card: {points_from_card}");
        points += points_from_card;
        println!("  Points after adding: {points}");
    }

    println!("Points: {points}");
}

fn str_vec_to_u32_vec(str_vec: Vec<&str>) -> Vec<u32> {
    let mut u32_vec: Vec<u32> = Vec::new();
    str_vec.into_iter().for_each(|str| u32_vec.push(str.parse::<u32>().unwrap()));
    return u32_vec;
}

fn u32_vec_to_string(u32_vec : &Vec<u32>) -> String {
    let mut string: String = Default::default();
    for u32 in u32_vec {

        string.push_str(&u32.to_string());
        string.push(' ');
    }
    string
}