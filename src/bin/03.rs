fn bitmap(input: &str) -> u64 {
    input.bytes().fold(0, |acc, b| {
        acc | match b {
            b'a'..=b'z' => 1 << (b - 97),
            b'A'..=b'Z' => 1 << (b - 65 + 26),
            _ => panic!(),
        }
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let ans = input
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(l, r)| {
            let a = bitmap(l);
            let b = bitmap(r);

            (a & b).trailing_zeros() + 1
        })
        .sum();

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let one = input.lines().step_by(3);
    let two = input.lines().skip(1).step_by(3);
    let three = input.lines().skip(2).step_by(3);

    let ans = one
        .zip(two)
        .zip(three)
        .map(|((a, b), c)| {
            let x = bitmap(a);
            let y = bitmap(b);
            let z = bitmap(c);

            (x & y & z).trailing_zeros() + 1
        })
        .sum();

    Some(ans)
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
    fn test_bitmap() {
        assert_eq!(bitmap("a"), 1);
        assert_eq!(bitmap("z"), 1 << 25);
        assert_eq!(bitmap("A"), 1 << 26);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
