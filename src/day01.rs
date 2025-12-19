use std::fs;

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(PartialEq)]
struct Operation {
    direction: Direction,
    distance: i16
}

struct Dial {
    pos: i16,
    max: i16
}

impl Dial {
    fn new(pos: i16, max: i16) -> Self {
        Self { pos, max }
    }

    fn turn(&mut self, operation: Operation) {
        // if the direction is left, subtract the distance from the position
        if operation.direction == Direction::Left {
            self.pos = (self.max + 1 + self.pos - operation.distance) % (self.max + 1);
        }
        // if the direction is right, add the distance to the position
        else {
            self.pos = (self.pos + operation.distance) % (self.max + 1);
        }
    }

    fn inspect<F>(&self, f: F) -> bool where F: Fn(i16) -> bool {
        f(self.pos)
    }
}

pub fn day01_part1(filename: &str) -> u64 {
    let mut dial = Dial::new(50, 99);

    // create a collection to keep all the operations
    let mut operations = Vec::new();

    // read the input file
    let input = fs::read_to_string(filename).unwrap();
    let lines = input.lines();

    for line in lines {
        // each line is in a format: "DN" where D just the first char and N is a remainder parsed as a number
        let (direction, distance) = line.split_at(1);
        
        if direction == "L" {
            operations.push(Operation { direction: Direction::Left, distance: distance.parse::<i16>().unwrap() });
        } else {
            operations.push(Operation { direction: Direction::Right, distance: distance.parse::<i16>().unwrap() });
        }
    }

    // execute all the operations
    let mut counter = 0;
    for operation in operations {
        dial.turn(operation);
        // inspect the position of the dial, if it's zero, increment the counter
        if dial.inspect(|pos| pos == 0) {
            counter += 1;
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_part1_a() {
        let result = day01_part1("input/day01a.txt");
        assert_eq!(result, 3);
    }

    #[test]
    fn day01_part1_b() {
        let result = day01_part1("input/day01b.txt");
        assert_eq!(result, 1158);
    }
}
