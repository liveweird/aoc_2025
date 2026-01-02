use std::fs;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: u8,
    y: u8,
}

impl Position {
    fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

struct Tile {
    is_filled: bool,
    neighbors: u16
}

impl Tile {
    fn new(is_filled: bool, neighbors: u16) -> Self {
        Self { is_filled, neighbors }
    }
}

struct Floor {
    floor_map: HashMap<Position, Tile>,
}

impl Floor {
    fn new(size: u8) -> Self {
        let mut floor_map = HashMap::new();
        for x in 0..size {
            for y in 0..size {
                floor_map.insert(Position::new(x, y), Tile::new(false, 0));
            }
        }
        Self { floor_map }
    }

    fn count_neighbors(position: &Position, floor_map: &HashMap<Position, Tile>) -> u16 {
        let mut neighbors = 0;
        for x in -1i8..=1i8 {
            for y in -1i8..=1i8 {
                if x == 0 && y == 0 {
                    continue;
                }
                let new_x = position.x as i16 + x as i16;
                let new_y = position.y as i16 + y as i16;
                if new_x >= 0 && new_y >= 0 {
                    let neighbor_position = Position::new(new_x as u8, new_y as u8);
                    if floor_map.contains_key(&neighbor_position) && floor_map.get(&neighbor_position).unwrap().is_filled {
                        neighbors += 1;
                    }
                }
            }
        }
        neighbors
    }

    fn inform_neighbors(position: &Position, floor_map: &mut HashMap<Position, Tile>) {
        for x in -1i8..=1i8 {
            for y in -1i8..=1i8 {
                if x == 0 && y == 0 {
                    continue;
                }

                let new_x = position.x as i16 + x as i16;
                let new_y = position.y as i16 + y as i16;
                if new_x >= 0 && new_y >= 0 {
                    let neighbor_position = Position::new(new_x as u8, new_y as u8);
                    if floor_map.contains_key(&neighbor_position) && floor_map.get(&neighbor_position).unwrap().is_filled {
                        floor_map.get_mut(&neighbor_position).unwrap().neighbors -= 1;
                    }
                }
            }
        }
    }

    fn _print(&self, size: u8) {
        for y in 0..size {
            for x in 0..size {
                let tile = self.floor_map.get(&Position::new(x, y)).unwrap();
                if tile.is_filled {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn _print_neighbors(&self, size: u8) {
        for y in 0..size {
            for x in 0..size {
                let tile = self.floor_map.get(&Position::new(x, y)).unwrap();
                print!("{}", tile.neighbors);
            }
            println!();
        }
    }
}

pub fn day04_part1(filename: &str, size: u8) -> u64 {
    let mut result = 0;
    // create the map, indexed with u8 pair, the value is a pair: boolean and u16
    let mut floor = Floor::new(size);
    // read the file, named 'filename'
    let input = fs::read_to_string(filename).unwrap();
    // for every line ...
    for (line_idx, line) in input.lines().enumerate() {
        // ... and the intersection with every column ...
        for (column_idx, char) in line.chars().enumerate() {
            // ... mark the boolean as full/empty
            if char == '.' {
                floor.floor_map.get_mut(&Position::new(column_idx as u8, line_idx as u8)).unwrap().is_filled = false;
            } else {
                floor.floor_map.get_mut(&Position::new(column_idx as u8, line_idx as u8)).unwrap().is_filled = true;
            }
        }
    }
    //floor.print(size);
    // iterate through the whole floor map
    let positions_to_update: Vec<(Position, u16)> = floor.floor_map.iter()
        .filter(|(_, tile)| tile.is_filled)
        .map(|(position, _)| {
            let neighbors = Floor::count_neighbors(position, &floor.floor_map);
            (*position, neighbors)
        })
        .collect();
    
    for (position, neighbors) in positions_to_update {
        floor.floor_map.get_mut(&position).unwrap().neighbors = neighbors;
    }
    // floor.print_neighbors(size);

    // iterate through the whole floor map, again
    for (_position, tile) in floor.floor_map.iter() {
        // count only the items that have at least 4 neighbors
        if tile.is_filled && tile.neighbors < 4 {
            result += 1;
        }
    }
    result
}

pub fn day04_part2(filename: &str, size: u8) -> u64 {
    let mut result = 0;
    // create the map, indexed with u8 pair, the value is a pair: boolean and u16
    let mut floor = Floor::new(size);
    // read the file, named 'filename'
    let input = fs::read_to_string(filename).unwrap();
    // for every line ...
    for (line_idx, line) in input.lines().enumerate() {
        // ... and the intersection with every column ...
        for (column_idx, char) in line.chars().enumerate() {
            // ... mark the boolean as full/empty
            if char == '.' {
                floor.floor_map.get_mut(&Position::new(column_idx as u8, line_idx as u8)).unwrap().is_filled = false;
            } else {
                floor.floor_map.get_mut(&Position::new(column_idx as u8, line_idx as u8)).unwrap().is_filled = true;
            }
        }
    }
    //floor.print(size);
    // iterate through the whole floor map
    let positions_to_update: Vec<(Position, u16)> = floor.floor_map.iter()
        .filter(|(_, tile)| tile.is_filled)
        .map(|(position, _)| {
            let neighbors = Floor::count_neighbors(position, &floor.floor_map);
            (*position, neighbors)
        })
        .collect();
    
    for (position, neighbors) in positions_to_update {
        floor.floor_map.get_mut(&position).unwrap().neighbors = neighbors;
    }
    // floor.print_neighbors(size);

    loop {
        // iterate through the whole floor map, again
        let positions_to_clear: Vec<Position> = floor.floor_map.iter()
            .filter(|(_, tile)| tile.is_filled && tile.neighbors < 4)
            .map(|(position, _)| *position)
            .collect();
        
        if positions_to_clear.is_empty() {
            break;
        }

        for position in positions_to_clear {
            result += 1;
            // clear this tile
            floor.floor_map.get_mut(&position).unwrap().is_filled = false;
            floor.floor_map.get_mut(&position).unwrap().neighbors = 0;
            Floor::inform_neighbors(&position, &mut floor.floor_map);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day04_part1_a() {
        let result = day04_part1("input/day04a.txt", 10);
        assert_eq!(result, 13);
    }

    #[test]
    fn day04_part1_b() {
        let result = day04_part1("input/day04b.txt", 139);
        assert_eq!(result, 1474);
    }

    #[test]
    fn day04_part2_a() {
        let result = day04_part2("input/day04a.txt", 10);
        assert_eq!(result, 43);
    }

    #[test]
    fn day04_part2_b() {
        let result = day04_part2("input/day04b.txt", 139);
        assert_eq!(result, 8910);
    }
}
