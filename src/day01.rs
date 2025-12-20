use std::fs;

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
}

#[derive(PartialEq)]
struct Operation {
    direction: Direction,
    distance: i16,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.direction, self.distance)
    }
}

struct Dial {
    pos: i16,
    max: i16,
}

impl Dial {
    fn new(pos: i16, max: i16) -> Self {
        Self { pos, max }
    }

    fn turn(&mut self, operation: &Operation) -> i16 {
        let mut spills: i16 = 0;
        let size: i16 = self.max + 1;
        // if the direction is left, subtract the distance from the position
        if operation.direction == Direction::Left {
            let increased = self.pos - operation.distance;
            if self.pos != 0 && increased <= 0 {
                spills += 1;
            }
            spills += (increased / size).abs();
            self.pos = increased % size;
            while self.pos < 0 {
                self.pos += size
            }
        }
        // if the direction is right, add the distance to the position
        else {
            let increased = self.pos + operation.distance;
            spills += increased / size;
            self.pos = increased % size;
        }
        spills
    }

    fn inspect<F>(&self, f: F) -> bool
    where
        F: Fn(i16) -> bool,
    {
        f(self.pos)
    }
}

pub fn day01_part1(filename: &str) -> i16 {
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
            operations.push(Operation {
                direction: Direction::Left,
                distance: distance.parse::<i16>().unwrap(),
            });
        } else {
            operations.push(Operation {
                direction: Direction::Right,
                distance: distance.parse::<i16>().unwrap(),
            });
        }
    }

    // execute all the operations
    let mut counter = 0;
    for operation in operations {
        dial.turn(&operation);
        // inspect the position of the dial, if it's zero, increment the counter
        if dial.inspect(|pos| pos == 0) {
            counter += 1;
        }
    }
    counter
}

pub fn day01_part2(filename: &str) -> i16 {
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
            operations.push(Operation {
                direction: Direction::Left,
                distance: distance.parse::<i16>().unwrap(),
            });
        } else {
            operations.push(Operation {
                direction: Direction::Right,
                distance: distance.parse::<i16>().unwrap(),
            });
        }
    }

    // execute all the operations
    let mut counter = 0;
    for operation in operations {
        let spills = dial.turn(&operation);
        counter += spills;
        println!("Position: {}, Operation: {}, Spills: {}", dial.pos, operation, spills);
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

    #[test]
    fn day01_part2_a() {
        let result = day01_part2("input/day01a.txt");
        assert_eq!(result, 6);
    }

    #[test]
    fn day01_part2_b() {
        let result = day01_part2("input/day01b.txt");
        assert_eq!(result, 6860);
    }

    #[test]
    fn day01_left_no_spill() {
        let mut dial = Dial::new(50, 99);
        let operation = Operation {  direction: Direction::Left, distance: 30 };
        let spills = dial.turn(&operation);
        assert_eq!(dial.pos, 20);
        assert_eq!(spills, 0);
    }

    #[test]
    fn day01_left_one_spill_end_at_zero() {
        let mut dial = Dial::new(50, 99);
        let operation = Operation {  direction: Direction::Left, distance: 50 };
        let spills = dial.turn(&operation);
        assert_eq!(dial.pos, 0);
        assert_eq!(spills, 1);
    }

    #[test]
    fn day01_left_one_spill_end_below_zero() {
        let mut dial = Dial::new(50, 99);
        let operation = Operation {  direction: Direction::Left, distance: 60 };
        let spills = dial.turn(&operation);
        assert_eq!(dial.pos, 90);
        assert_eq!(spills, 1);
    }

    #[test]
    fn day01_left_two_spills_end_zero() {
        let mut dial = Dial::new(50, 99);
        let operation = Operation {  direction: Direction::Left, distance: 150 };
        let spills = dial.turn(&operation);
        assert_eq!(dial.pos, 0);
        assert_eq!(spills, 2);
    }

    #[test]
    fn day01_left_two_spills_end_below_zero() {
        let mut dial = Dial::new(50, 99);
        let operation = Operation {  direction: Direction::Left, distance: 160 };
        let spills = dial.turn(&operation);
        assert_eq!(dial.pos, 90);
        assert_eq!(spills, 2);
    }

    #[test]
    fn day01_right_no_spill() {}

    #[test]
    fn day01_right_one_spill_end_at_zero() {}

    #[test]
    fn day01_right_one_spill_end_after_zero() {}

    #[test]
    fn day01_right_two_spills_end_zero() {}

    #[test]
    fn day01_right_two_spills_end_above_zero() {}
}
