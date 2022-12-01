pub fn part_one(input: &str) -> Option<u32> {
    Some(input.split("\n\n").fold(0, |acc, s| {
        acc.max(s.split('\n').map(|s| s.parse::<u32>().unwrap()).sum())
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n\n")
            .fold([0; 3], |acc, s| {
                match s.split('\n').map(|s| s.parse::<u32>().unwrap()).sum() {
                    x if x > acc[0] => [x, acc[0], acc[1]],
                    x if x > acc[1] => [acc[0], x, acc[1]],
                    x if x > acc[2] => [acc[0], acc[1], x],
                    _ => acc,
                }
            })
            .iter()
            .sum(),
    )
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
