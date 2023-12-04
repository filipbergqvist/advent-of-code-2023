use std::fs;

fn main() {
    let input_path = "res/input.txt";
    let input = fs::read_to_string(input_path)
        .expect("Failed to read file");

    let mut points = 0;

    for line in input.lines() {
        println!("\nLine: {line}");
            
        let (mut _card, number) = line.split_once(':').unwrap();
        let (winning_numbers, your_numbers) = number.split_once('|').unwrap();

        let winning_numbers_vec: Vec<&str> = winning_numbers.split_whitespace().collect();

        let mut matches = 0;
        for your_number in your_numbers.split_whitespace() {
            println!("  Your number: {your_number}");
            if winning_numbers_vec.contains(&your_number) {
                matches += 1;
                println!("  It matches! Matches: {matches}");
            }
        }

        let points_from_card;
        match matches {
            0 => points_from_card = 0,
            1 => points_from_card = 1,
            _ => points_from_card = 1 << matches - 1,
        }

        println!("  Points from this card: {points_from_card}");
        points += points_from_card;
        println!("  Points after adding: {points}");
    }

    println!("Points: {points}");
}
