use std::fs;

pub fn day03_part1(filename: &str) -> u64 {
    let mut result = 0u64;
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
        let to_be_added = highest_digit.to_digit(10).unwrap() as u64 * 10 + highest_digit_in_range.to_digit(10).unwrap() as u64;
        result += to_be_added;
    }
    return result;
}

pub fn day03_part2(filename: &str) -> u64 {
    let mut result = 0;
    // read the input file
    let input = fs::read_to_string(filename).unwrap();
    // split the input into lines
    let lines = input.lines();
    // iterate over the lines
    for line in lines {
        let mut digits_left = 12;
        let mut to_be_added_total: u64 = 0;
        let mut searched_through = line.to_string();
        while digits_left > 0 {
            // find the highest digit in the line, ...
            // ... excluding the final digits_left characters (if it appears more than once, pick the first one)
            let effectively_searched_through = searched_through[..searched_through.len() - digits_left + 1].chars();
            let highest_digit = effectively_searched_through.max().unwrap();
            // add the number (highest_digit * 10^(digits_left - 1)) to the result
            let to_be_added: u64 = highest_digit.to_digit(10).unwrap() as u64 * 10_u64.pow((digits_left - 1) as u32);
            to_be_added_total += to_be_added;
            digits_left -= 1;
            // update the searched_through string to exclude evertything before and the highest_digit itself
            searched_through = searched_through[searched_through.find(highest_digit).unwrap() + 1..].to_string();
        }
        result += to_be_added_total;
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

    #[test]
    fn day03_part2_a() {
        let result = day03_part2("input/day03a.txt");
        assert_eq!(result, 3121910778619);
    }

    #[test]
    fn day03_part2_b() {
        let result = day03_part2("input/day03b.txt");
        assert_eq!(result, 170731717900423);
    }
}