use std::fs;

pub fn day03_part1(filename: &str) -> u32 {
    let mut result = 0;
    // read the input file
    let input = fs::read_to_string(filename).unwrap();
    // split the input into lines
    let lines = input.lines();
    // iterate over the lines
    for line in lines {
        // find the highest digit in the line, excluding the final character (if it appears more than once, pick the first one)
        let highest_digit = line[..line.len() - 1].chars().max().unwrap();
        // select the range between the character after the highest_digit and the end of the line
        let range = line[line.find(highest_digit).unwrap() + 1..].to_string();
        // find the highest digit in the range (if it appears more than once, pick the first one)
        let highest_digit_in_range = range.chars().max().unwrap();
        // add the number (highest_digit * 10 + highest_digit_in_range) to the result
        let to_be_added = highest_digit.to_digit(10).unwrap() * 10 + highest_digit_in_range.to_digit(10).unwrap();
        result += to_be_added;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day03_part1_a() {
        let result = day03_part1("input/day03a.txt");
        assert_eq!(result, 357);
    }

    #[test]
    fn day03_part1_b() {
        let result = day03_part1("input/day03b.txt");
        assert_eq!(result, 17263);
    }
}