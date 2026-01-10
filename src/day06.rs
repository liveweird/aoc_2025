use std::fs;

enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn new(operation: &str) -> Self {
        match operation {
            "+" => Self::Add,
            "*" => Self::Multiply,
            _ => panic!("Invalid operation: {}", operation),
        }
    }
}

fn day06_part1(filename: &str) -> u64 {
    let mut result = 0;

    // read the input file
    let input = fs::read_to_string(filename).unwrap();

    // split the input into lines
    let lines = input.lines().collect::<Vec<&str>>();

    // separate the last line from the rest
    let last_line = lines.last().unwrap();
    let initial_lines = &lines[..lines.len() - 1];

    // create the 2D matrix of numbers
    let mut matrix = Vec::new();

    // iterate over the initial lines
    for line in initial_lines {
        // split the line into numbers
        let numbers = line.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
        // add the vector to the matrix
        matrix.push(numbers);
    }

    // transpose the matrix
    let transposed_matrix = (0..matrix[0].len())
        .map(|col_idx| matrix.iter().map(|row| row[col_idx]).collect::<Vec<u64>>())
        .collect::<Vec<Vec<u64>>>();
    
    // parse the last line into operations
    let operations = last_line.split_whitespace().map(|x| Operation::new(x)).collect::<Vec<_>>();

    // iterate throught the transposed matrix row by row
    for (i, row) in transposed_matrix.iter().enumerate() {
        let operation = &operations[i];

        // iterate through the row
        let mut row_result = match operation {
            Operation::Add => 0,
            Operation::Multiply => 1,
        };

        for number in row {
            match operation {
                Operation::Add => row_result += number,
                Operation::Multiply => row_result *= number,
            }
        }
        
        result += row_result;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day06_part1a_test() {
        assert_eq!(day06_part1("input/day06a.txt"), 4277556);
    }

    #[test]
    fn day06_part1b_test() {
        assert_eq!(day06_part1("input/day06b.txt"), 6417439773370);
    }
}
