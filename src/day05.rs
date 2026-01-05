use std::fs;
use regex::Regex;

struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }
}

impl std::fmt::Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

pub fn day05_part1(filename: &str) -> u64 {
    // read the input file
    let input = fs::read_to_string(filename).unwrap();

    let mut ranges: Vec<Range> = Vec::new();
    let mut numbers: Vec<u64> = Vec::new();

    // regex that represents two integers seperated with a hyphen
    let re_range = Regex::new(r"(\d+)-(\d+)").unwrap();
    // regex that represents a single integer
    let re_number = Regex::new(r"(\d+)").unwrap();

    // keep reading lines from the file, using an iterator until the end of the file
    for line in input.lines() {
        // if the line matches the range regex, add the range to the ranges vector
        if re_range.is_match(line) {
            let caps = re_range.captures(line).unwrap();
            ranges.push(Range::new(caps[1].parse::<u64>().unwrap(), caps[2].parse::<u64>().unwrap()));
        }
        // if the line matches the number regex, add the number to the numbers vector
        else if re_number.is_match(line) {
            numbers.push(line.parse::<u64>().unwrap());
        }
    }

    let mut result = 0;

    // iterate through all the numbers
    for number in numbers {
        // iterate through all the ranges
        'the_inner_loop: for range in &ranges {
            // if the number is within the range, increment the counter, jump to the next number
            if number >= range.start && number <= range.end {
                result += 1;
                break 'the_inner_loop;
            }
        }
    }
    return result;
}

pub fn day05_part2(filename: &str) -> u64 {
    // read the input file
    let input = fs::read_to_string(filename).unwrap();

    let mut ranges: Vec<Range> = Vec::new();
    let mut numbers: Vec<u64> = Vec::new();

    // regex that represents two integers seperated with a hyphen
    let re_range = Regex::new(r"(\d+)-(\d+)").unwrap();
    // regex that represents a single integer
    let re_number = Regex::new(r"(\d+)").unwrap();

    // keep reading lines from the file, using an iterator until the end of the file
    for line in input.lines() {
        // if the line matches the range regex, add the range to the ranges vector
        if re_range.is_match(line) {
            let caps = re_range.captures(line).unwrap();
            ranges.push(Range::new(caps[1].parse::<u64>().unwrap(), caps[2].parse::<u64>().unwrap()));
        }
        // if the line matches the number regex, add the number to the numbers vector
        else if re_number.is_match(line) {
            numbers.push(line.parse::<u64>().unwrap());
        }
    }

    // sort the ranges by start
    ranges.sort_by_key(|range| range.start);

    let mut merged_ranges: Vec<Range> = Vec::new();
    let mut result = 0;

    // iterate through all the ranges
    for range in ranges {
        if merged_ranges.is_empty() {
            merged_ranges.push(range);
            continue;
        }

        // get the last merged ranges
        let last_merged_range = merged_ranges.last_mut().unwrap();

        // if the current range is shifted right of the merged range, but overlaps with it, merge them
        if range.start <= last_merged_range.end && range.end > last_merged_range.end {
            last_merged_range.end = range.end;
        }
        // if the current range includes the merged range, merge them
        else if range.start > last_merged_range.end {
            merged_ranges.push(range);
        }
    }

    // iterate through all the merged ranges
    for merged_range in merged_ranges {
        result += merged_range.end - merged_range.start + 1;
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day05_part1_a() {
        let result = day05_part1("input/day05a.txt");
        assert_eq!(result, 3);
    }

    #[test]
    fn day05_part1_b() {
        let result = day05_part1("input/day05b.txt");
        assert_eq!(result, 770);
    }

    #[test]
    fn day05_part2_a() {
        let result = day05_part2("input/day05a.txt");
        assert_eq!(result, 14);
    }

    #[test]
    fn day05_part2_b() {
        let result = day05_part2("input/day05b.txt");
        assert_eq!(result, 357674099117260);
    }
}