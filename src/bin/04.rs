fn bitmap(low: u32, high: u32) -> u128 {
    ((low - 1)..=(high - 1)).fold(0, |acc, v| acc | 1 << v)
}

pub fn part_one(input: &str) -> Option<u32> {
    let v = input
        .lines()
        .filter_map(|l| l.split_once(','))
        .map(|(l, r)| (l.split_once('-').unwrap(), r.split_once('-').unwrap()))
        .map(|(l, r)| {
            (
                (l.0.parse().unwrap(), l.1.parse().unwrap()),
                (r.0.parse().unwrap(), r.1.parse().unwrap()),
            )
        })
        .filter_map(|(l, r)| {
            let a = bitmap(l.0, l.1);
            let b = bitmap(r.0, r.1);

            let ab = a | b;
            if ab == a || ab == b {
                Some(1)
            } else {
                None
            }
        })
        .count();

    Some(v as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let v = input
        .lines()
        .filter_map(|l| l.split_once(','))
        .map(|(l, r)| (l.split_once('-').unwrap(), r.split_once('-').unwrap()))
        .map(|(l, r)| {
            (
                (l.0.parse().unwrap(), l.1.parse().unwrap()),
                (r.0.parse().unwrap(), r.1.parse().unwrap()),
            )
        })
        .filter_map(|(l, r)| {
            let a = bitmap(l.0, l.1);
            let b = bitmap(r.0, r.1);

            let ab = a & b;
            if ab > 0 {
                Some(1)
            } else {
                None
            }
        })
        .count();

    Some(v as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
