use regex::Regex;
use std::io::BufRead;

// TODO make tests module 
// TODO make tests for parse_calibration_line for no number found, first and last digit same, 
// and two digits.

// calibration data: file where each line contains a first and last digit
/// Given a regex that captures the first and last digit in a string and the string to be searched,
/// return a number composed of the two digits if they can be found.
///
/// The string represents a line of calibration data.
///
/// u8 was chosen because the maximum number possible from the output is 99,
/// which is less than the maximum value that can be represented within a u8 (i.e., 255).
///
/// The regex object is given as input because compilation is expensive.
///
/// Here is an example of how the function works.
/// ``` 
/// # use regex::Regex;
/// let re = Regex::new(r"^\D*(\d).*?(\d?)\D*$").unwrap();
/// assert_eq!(day1::parse_calibration_line(&re,String::from("1abc2")), Some(12))
/// ```
/// Here is an example where the string contains only a single digit.
/// ```
/// # use regex::Regex;
/// let re = Regex::new(r"^\D*(\d).*?(\d?)\D*$").unwrap();
/// assert_eq!(day1::parse_calibration_line(&re,String::from("treb7uchet")), Some(77))
/// ```
pub fn parse_calibration_line(re: &Regex, line: String) -> Option<u8> {
    // find the first match of the first and last digit
    let caps = re.captures(&line)?;
    let first: u8 = caps.get(1)?.as_str().parse().ok()?;
    let last: Option<u8> = caps.get(2)?.as_str().parse().ok();

    // return two digit number
    match last {
        Some(last) => Some(first * 10 + last),
        // first digit represents both first and last digit
        None => Some(first * 10 + first)
    }
}

/// Given a regex object and lines of calibration data, parse the calibration values
/// and sum them.
// TODO write a code example of this function
// TODO rewrite in a functional style that avoids mutation:w
pub fn sum_calibration_lines<T : BufRead>(re: &Regex, reader: T) -> Option<u64> {
    let mut sum : u64 = 0;
    for line in reader.lines() {
        let line = line.ok()?;
        let number = parse_calibration_line(&re,line)?;
        sum += number as u64;
    }
    Some(sum)
}
