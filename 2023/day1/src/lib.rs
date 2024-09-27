use regex::Regex;
// TODO make tests module 
// TODO make tests for parse_calibration_line for no number found, first and last digit same, 
// and two digits.

// calibration data: file where each line contains a first and last digit
/// Given a compiled regex object and a string, return a two digit integer consisting of the first
/// and last digit within the string if the digits can be found.
///
/// The string represents a line of calibration data.
/// u8 chosen because the maximum number possible is 99, which is less than 255.
/// The regex object is given as input because compilation is expensive.
///
/// Here is an example of how the function works.
/// ``` 
/// # use regex::Regex;
/// let re = Regex::new(r"^\D*(\d)\D*(\d?)\D*$").unwrap();
/// assert_eq!(day1::parse_calibration_line(re,String::from("1abc2")), Some(12))
/// ```
pub fn parse_calibration_line(re: Regex, line: String) -> Option<u8> {
    // find the first match of the first and last digit
    let (_,[first,last]) = re.captures(&line)?.extract();
    // return two digit number
    let first : u8 = first.parse().ok()?;
    let last : u8 = last.parse().ok()?;
    Some(first * 10 + last)
}
