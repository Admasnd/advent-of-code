use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use day1::sum_calibration_lines;

fn main() {
    let num_def = r"one|two|three|four|five|six|seven|eight|nine|\d";
    let re = Regex::new(&num_def).expect("regex should compile");
    let f = File::open("input.txt").expect("calibration data should be available on filesystem");
    let reader = BufReader::new(f);
    let sum = sum_calibration_lines(&re,reader);
    match sum {
        Some(sum) => println!("The sum is {}", sum),
        None => println!("Issue parsing calibration lines"),
    }
}
