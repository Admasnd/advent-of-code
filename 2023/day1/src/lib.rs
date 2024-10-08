use fancy_regex::Regex;
use std::io::BufRead;

pub const PART2_REGEX: &str = "(?<!tw)one|\
    (?<!eigh)two|\
    (?<!eigh)three|\
    four|\
    five|\
    six|\
    seven|\
    (?<!on|thre|fiv|nin)eight|\
    (?<!seve)nine|\
    \\d";

// If the given string that contains the spelling of a number, return the 
// corresponding digit as string. Otherwise, return the string as is.
fn convert_num_str(s: &str) -> &str {
    match s {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => s,
    }
}

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
/// # use fancy_regex::Regex;
/// let re = Regex::new(r"\d").unwrap();
/// assert_eq!(day1::parse_calibration_line(&re,String::from("1abc2")), Some(12))
/// ```
/// Here is an example where the string contains only a single digit.
/// ```
/// # use fancy_regex::Regex;
/// let re = Regex::new(r"\d").unwrap();
/// assert_eq!(day1::parse_calibration_line(&re,String::from("treb7uchet")), Some(77))
/// ```
pub fn parse_calibration_line(re: &Regex, line: String) -> Option<u8> {
    // find the first match of the first and last digit
    let mut caps = re.find_iter(&line);
    let first = caps.next()?.ok()?.as_str();
    let first : u8 = convert_num_str(first)
        .parse()
        .ok()?;
    let last: Option<u8> = caps.last()
        .and_then(|m| m.ok())
        .and_then(|m| convert_num_str(m.as_str()).parse().ok());

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
pub fn sum_calibration_lines<T : BufRead>(re: &Regex, reader: T) -> Option<u64> {
    reader.lines()
        .map(|line| parse_calibration_line(re,line.ok()?))
        .fold(Some(0 as u64), |sum, num| Some(sum? + (num? as u64)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn par2_test_given_examples() {
        let re = Regex::new(PART2_REGEX).expect("regex should compile");
        let test_cases = [
            ("two1nine",29),
            ("eightwothree",83),
            ("abcone2threexyz",13),
            ("xtwone3four",24),
            ("4nineeightseven2",42),
            ("zoneight234",14),
            ("7pqrstsixteen",76)];

        for test_case in test_cases {
            assert_eq!(parse_calibration_line(&re,String::from(test_case.0)), Some(test_case.1))
        }
    }
}
