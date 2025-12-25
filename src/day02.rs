use std::fs;

pub fn day02_part1(filename: &str) -> i64 {
    let mut result = 0;
    // read the input file - it will be just one line
    let input = fs::read_to_string(filename).unwrap();
    // split the lines into pairs of numbers, separated by "," - the pair elements are separated by "-"
    let pairs = input.split(",").map(|pair| pair.split("-").map(|num| num.parse::<i64>().unwrap()).collect::<Vec<i64>>()).collect::<Vec<Vec<i64>>>();
    // iterate all the pairs
    for pair in pairs {
        // iterate from the first number to the second number, by one
        for num in pair[0]..=pair[1] {
            // if num has the uneven number of digits, continue to the next number
            if num.to_string().len() % 2 != 0 {
                continue;
            }
            // split the number into two halves
            let half = num.to_string().len() / 2;
            let first_half = num.to_string().chars().take(half).collect::<String>();
            let second_half = num.to_string().chars().skip(half).collect::<String>();
            // check if the first half is equal to the second half
            if first_half == second_half {
                result += num;
            }
        }
    }
    return result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02_part1_a() {
        let result = day02_part1("input/day02a.txt");
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn day02_part1_b() {
        let result = day02_part1("input/day02b.txt");
        assert_eq!(result, 21898734247);
    }
}