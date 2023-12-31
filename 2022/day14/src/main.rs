use std::{
    cmp::{max, min},
    collections::{HashSet, vec_deque},
};

use itertools::Itertools;

const SAND_ORIGIN: Position = new_pos(500, 0);

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn create_line(&self, other: &Self) -> Vec<Self> {
        if self.x != other.x {
            let (min, max) = (min(self.x, other.x), max(self.x, other.x));
            let y = self.y;

            (min..=max).map(|x| new_pos(x, y)).collect()
        } else {
            let x = self.x;
            let (min, max) = (min(self.y, other.y), max(self.y, other.y));

            (min..=max).map(|y| new_pos(x, y)).collect()
        }
    }

    fn can_move_down(&self, set: &HashSet<Self>) -> bool {
        !set.contains(&Position {
            x: self.x - 1,
            y: self.y + 1,
        }) || !set.contains(&Position {
            x: self.x,
            y: self.y + 1,
        }) || !set.contains(&Position {
            x: self.x + 1,
            y: self.y + 1,
        })
    }

    fn move_down(&mut self, set: &HashSet<Self>) {
        if !set.contains(&Position {
            x: self.x,
            y: self.y + 1,
        }) {
            self.y += 1;
        } else if !set.contains(&Position {
            x: self.x - 1,
            y: self.y + 1,
        }) {
            self.x -= 1;
            self.y += 1;
        } else if !set.contains(&Position {
            x: self.x + 1,
            y: self.y + 1,
        }) {
            self.x += 1;
            self.y += 1;
        }
    }
}

const fn new_pos(x: u32, y: u32) -> Position {
    Position { x, y }
}

fn parse_positions(input: &str) -> HashSet<Position> {
    let mut positions = HashSet::new();

    for line in input.lines() {
        let mut past_pos = None;

        for element in line.split(" -> ") {
            let (x, y) = element
                .split(',')
                .map(|num| num.parse::<u32>())
                .collect_tuple()
                .unwrap();
            let x = x.unwrap();
            let y = y.unwrap();

            if let Some(past_pos) = past_pos {
                let cur_pos = new_pos(x, y);
                for pos in cur_pos.create_line(&past_pos) {
                    positions.insert(pos);
                }
            } else {
                positions.insert(new_pos(x, y));
            }

            past_pos = Some(new_pos(x, y));
        }
    }

    positions
}

fn find_lowest_point(point_set: &HashSet<Position>) -> u32 {
    point_set.iter().map(|point| point.y).max().unwrap()
}

fn print_map(point_set: &HashSet<Position>, floor: Option<u32>) {
    let left_bound = point_set.iter().map(|point| point.x).min().unwrap() - 1;
    let right_bound = point_set.iter().map(|point| point.x).max().unwrap() + 1;
    let _floor;

    if let Some(num) = floor {
        _floor = num;
    } else {
        _floor = find_lowest_point(point_set) + 2;
    }

    for y in 0..=_floor {
        print!("{:3}: ", y);
        for x in left_bound..=right_bound {
            let pos = new_pos(x, y);

            if y == _floor {
                print!("-");
            } else if point_set.contains(&pos) {
                print!("#");
            } else if x == 500 && y == 0 {
                print!("*");
            } else {
                print!(".");
            }
        }

        println!();
    }

    println!();
}

fn calc_sand_grain_count(point_set: &mut HashSet<Position>) -> u32 {
    let floor = find_lowest_point(point_set);
    let mut past_fell_into_abyss = false;

    dbg!(floor);

    for count in 0.. {
        // Create new grain
        let mut grain = SAND_ORIGIN;

        // Let it fall
        while grain.can_move_down(point_set) && grain.y <= floor {
            grain.move_down(point_set);
        }

        // Check if it's past the floor/it has gone into the abyss
        let fell_into_abyss = grain.y >= floor;

        // If it, and the previous grain fell, it means we achieved a loop
        if past_fell_into_abyss && fell_into_abyss {
            print_map(point_set, None);
            return count - 1;
        }

        past_fell_into_abyss = fell_into_abyss;
        point_set.insert(grain);

        if count % 1000 == 999 {
            dbg!(count);
        }
    }

    0
}

fn calc_sand_grain_count_until_filled(point_set: &HashSet<Position>) -> u32 {
    let floor = find_lowest_point(point_set) + 2;
    let mut point_set = point_set.clone();
    // A stack to contain the path of the previous grain of sand to fall
    // It's used so we don't need to recompute the path for each grain to fall
    let mut prev_path = vec_deque::VecDeque::new();
    prev_path.push_front(SAND_ORIGIN);

    dbg!(floor);

    for count in 1.. {
        // Create new grain
        let mut grain = prev_path.front().unwrap().clone();

        // Let it fall
        while grain.can_move_down(&point_set) && grain.y < floor - 1 {
            grain.move_down(&point_set);
            prev_path.push_front(grain.clone());
        }

        // If it, and the previous grain fell, it means we achieved a loop
        if grain == SAND_ORIGIN {
            print_map(&point_set, Some(floor));
            return count;
        }

        point_set.insert(grain);
        prev_path.pop_front();

        if count % 1000 == 999 {
            dbg!(count);
        }
    }

    0
}

fn main() {
    let input = include_str!("input.txt");
    let positions = parse_positions(input);

    print_map(&positions, None);
    println!("{}", calc_sand_grain_count(&mut positions.clone()));
    println!(
        "{}",
        calc_sand_grain_count_until_filled(&positions)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_half() {
        let input = include_str!("test_input.txt");
        let mut positions = parse_positions(input);

        print_map(&positions, None);

        assert_eq!(24, calc_sand_grain_count(&mut positions));
    }

    #[test]
    fn test_second_half() {
        let input = include_str!("test_input.txt");
        let mut positions = parse_positions(input);

        print_map(&positions, None);

        assert_eq!(93, calc_sand_grain_count_until_filled(&mut positions));
    }
}
