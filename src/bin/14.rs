use std::cmp;
use std::collections::HashMap;

use itertools::Itertools;

#[allow(dead_code)]
fn print_cave(cave: &HashMap<(u32, u32), char>) {
    let (min_x, max_x) = cave.iter().fold((1000, 0), |(min, max), ((x, _), _)| {
        (cmp::min(min, *x), cmp::max(max, *x))
    });
    let (min_y, max_y) = cave.iter().fold((1000, 0), |(min, max), ((_, y), _)| {
        (cmp::min(min, *y), cmp::max(max, *y))
    });

    for y in min_y - 1..max_y + 1 {
        for x in min_x - 1..max_x + 1 {
            print!("{}", cave.get(&(x, y)).unwrap_or(&'.'));
        }
        println!();
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut cave = parse_cave(input);
    //print_cave(&cave);

    let (_, max_y) = cave.iter().fold((1000, 0), |(min, max), ((_, y), _)| {
        (cmp::min(min, *y), cmp::max(max, *y))
    });

    let mut i = 0u32;

    'outer: loop {
        let mut sand = (500u32, 0u32);
        i += 1;
        loop {
            if sand.1 > max_y {
                break 'outer;
            }
            if let None = cave.get(&(sand.0, sand.1 + 1)) {
                sand = (sand.0, sand.1 + 1);
            } else if let None = cave.get(&(sand.0 - 1, sand.1 + 1)) {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if let None = cave.get(&(sand.0 + 1, sand.1 + 1)) {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                cave.insert(sand, 'o');
                break;
            };
        }
    }
    //print_cave(&cave);
    Some(i - 1)
}

fn parse_cave(input: &str) -> HashMap<(u32, u32), char> {
    let mut cave: HashMap<(u32, u32), char> = HashMap::new();
    let segments = input
        .split("\n")
        .map(|path| path.split(" -> "))
        .map(|points| {
            points
                .filter_map(|point| {
                    point
                        .split(",")
                        .filter_map(|i| i.parse::<u32>().ok())
                        .collect_tuple::<(u32, u32)>()
                })
                .collect_vec()
        })
        .filter(|seg| seg.len() > 0)
        .collect_vec();
    segments.iter().for_each(|seg| {
        seg.iter().reduce(|&(x1, y1), p2 @ &(x2, y2)| {
            if x1 == x2 {
                (cmp::min(y1, y2)..cmp::max(y1, y2) + 1).for_each(|y| {
                    cave.insert((x1, y), '#');
                });
            } else {
                (cmp::min(x1, x2)..cmp::max(x1, x2) + 1).for_each(|x| {
                    cave.insert((x, y1), '#');
                });
            };
            p2
        });
    });
    cave
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cave = parse_cave(input);

    let (_, max_y) = cave.iter().fold((1000, 0), |(min, max), ((_, y), _)| {
        (cmp::min(min, *y), cmp::max(max, *y))
    });

    let mut i = 0u32;

    'outer: loop {
        let mut sand = (500u32, 0u32);
        i += 1;
        loop {
            if cave.get(&sand) != None {
                break 'outer;
            }
            if sand.1 > max_y {
                cave.insert(sand, 'o');
                break;
            }
            if let None = cave.get(&(sand.0, sand.1 + 1)) {
                sand = (sand.0, sand.1 + 1);
            } else if let None = cave.get(&(sand.0 - 1, sand.1 + 1)) {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if let None = cave.get(&(sand.0 + 1, sand.1 + 1)) {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                cave.insert(sand, 'o');
                break;
            };
        }
    }
    //print_cave(&cave);
    Some(i - 1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
