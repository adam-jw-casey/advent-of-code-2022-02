use advent_of_code_2022_02::calculate_score_from_move;
use advent_of_code_2022_02::calculate_score_from_outcome;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut score = calculate_score_from_move(&contents);
    println!("The expected score is: {score}!");

    score = calculate_score_from_outcome(&contents);
    println!("The outcome score is: {score}");
}
