pub fn part_one(input: &str) -> Option<u32> {
    Some(input.split("\n\n").fold(0, |acc, s| {
        acc.max(s.split('\n').map(|s| s.parse::<u32>().unwrap()).sum())
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut snacks = input
        .split("\n\n")
        .map(|s| s.split('\n').map(|s| s.parse::<u32>().unwrap()).sum())
        .collect::<Vec<_>>();

    snacks.sort_unstable();
    Some(snacks[snacks.len() - 3..].iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
