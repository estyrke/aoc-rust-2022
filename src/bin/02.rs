fn score(other: u32, me: u32) -> u32 {
    match (other, me) {
        (x, y) if x == y => y + 1 + 3,             // Draw
        (x, y) if ((x + 2) % 3) == y => y + 1 + 0, // Loss
        (x, y) if ((x + 1) % 3) == y => y + 1 + 6, // Win
        _ => 1000,
    }
}

fn score2(other: u32, me: u32) -> u32 {
    match (other, me) {
        (x, 1) => x + 1 + 3,             // Draw
        (x, 0) => ((x + 2) % 3) + 1 + 0, // Loss
        (x, 2) => ((x + 1) % 3) + 1 + 6, // Win
        _ => 1000,
    }
}

fn char_to_int(c: &str) -> u32 {
    match c {
        "A" | "X" => 0,
        "B" | "Y" => 1,
        "C" | "Z" => 2,
        _ => 1000,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = input
        .split('\n')
        .filter_map(|pair| pair.split_once(' '))
        .map(|(other, me)| (char_to_int(other), char_to_int(me)))
        .map(|(other, me)| score(other, me));
    Some(map.sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .filter_map(|pair| pair.split_once(' '))
            .map(|(other, me)| (char_to_int(other), char_to_int(me)))
            .map(|(other, me)| score2(other, me))
            .sum::<u32>(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
