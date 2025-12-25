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


fn compare_texts_in_array(texts: Vec<String>) -> bool {
    let to_compare = texts[0].clone();
    for text in texts {
        if text != to_compare {
            return false;
        }
    }
    return true;
}

pub fn day02_part2(filename: &str) -> i64 {
    let mut result = 0;
    // read the input file - it will be just one line
    let input = fs::read_to_string(filename).unwrap();
    // split the lines into pairs of numbers, separated by "," - the pair elements are separated by "-"
    let pairs = input.split(",").map(|pair| pair.split("-").map(|num| num.parse::<i64>().unwrap()).collect::<Vec<i64>>()).collect::<Vec<Vec<i64>>>();
    // iterate all the pairs
    for pair in pairs {
        // iterate from the first number to the second number, by one
        for num in pair[0]..=pair[1] {
            let mut divisor = 2;
            // maximum divisor is a square root of the number of digits, rounded down to the nearest integer
            let max_divisor = (num.to_string().len() as f64).sqrt().floor() as usize;

            'the_outer_loop: while divisor <= max_divisor {
                if num.to_string().len() % divisor != 0 {
                    continue;
                }

                // split the number into divisor parts
                let size_of_piece = num.to_string().len() / divisor;
                // create an array of the pieces
                let pieces = num.to_string().chars().collect::<Vec<char>>().chunks(size_of_piece).map(|chunk| chunk.iter().collect::<String>()).collect::<Vec<String>>();
                // check if the first half is equal to the second half
                if compare_texts_in_array(pieces) {
                    result += num;
                    continue 'the_outer_loop;
                }

                divisor += 1;
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

    #[test]
    fn day02_part2_a() {
        let result = day02_part2("input/day02a.txt");
        assert_eq!(result, 4174379265);
    }

    #[test]
    fn day02_part2_b() {
        let result = day02_part2("input/day02b.txt");
        assert_eq!(result, 0);
    }
}