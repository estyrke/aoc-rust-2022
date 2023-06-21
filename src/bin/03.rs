use std::collections::HashSet;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split_terminator('\n')
            .map(|items| {
                let comps = items.split_at(items.len() / 2);
                let c1: HashSet<u8> = HashSet::from_iter(comps.0.bytes());
                let c2 = HashSet::from_iter(comps.1.bytes());
                (match c1.intersection(&c2).next().unwrap() {
                    &c if b'a' <= c && c <= b'z' => c - b'a' + 1,
                    &c if b'A' <= c && c <= b'Z' => c - b'A' + 27,
                    &c => panic!("Unexpected item {}", c),
                }) as u32
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split_terminator('\n')
            .tuples()
            .map(|(r1, r2, r3)| {
                let c1: HashSet<u8> = HashSet::from_iter(r1.bytes());
                let c2 = HashSet::from_iter(r2.bytes());
                let c3 = HashSet::from_iter(r3.bytes());
                (match c1
                    .intersection(&c2)
                    .collect::<HashSet<&u8>>()
                    .intersection(c3)
                    .next()
                    .unwrap()
                {
                    &c if b'a' <= c && c <= b'z' => c - b'a' + 1,
                    &c if b'A' <= c && c <= b'Z' => c - b'A' + 27,
                    &c => panic!("Unexpected item {}", c),
                }) as u32
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
