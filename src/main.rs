use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// returns the product of two numbers whose sum is 2020
fn read_input_to_map(filename: String) -> i64 {
    let file_input = File::open(filename).expect("file not found!");
    let file_reader = BufReader::new(file_input);
    let mut value = 0;
    let mut expense_report = HashMap::new();

    for line in file_reader.lines() {
        let entry = line.unwrap();
        expense_report.insert(entry.to_owned(), entry.parse::<i64>().unwrap());

        let search_key_num = 2020 - entry.parse::<i64>().unwrap();
        if expense_report.contains_key(&search_key_num.to_string()) {
            value = entry.parse::<i64>().unwrap() * search_key_num;
            break;
        }
    }

    value
}

fn main() {
    let product = read_input_to_map("./aoc-input/day_1_input.txt".to_string());
    println!("{}", product);
}
