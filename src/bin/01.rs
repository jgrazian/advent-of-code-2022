pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut max = 0;
    for line in input.lines() {
        if line.is_empty() {
            max = max.max(sum);
            sum = 0;
            continue;
        }
        sum += line.trim().parse::<u32>().unwrap();
    }
    max = max.max(sum);

    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut max = [0; 3];
    for line in input.lines() {
        if line.is_empty() {
            max = match sum {
                x if x > max[0] => [x, max[0], max[1]],
                x if x > max[1] => [max[0], x, max[1]],
                x if x > max[2] => [max[0], max[1], x],
                _ => max,
            };
            sum = 0;
            continue;
        }
        sum += line.trim().parse::<u32>().unwrap();
    }
    max = match sum {
        x if x > max[0] => [x, max[0], max[1]],
        x if x > max[1] => [max[0], x, max[1]],
        x if x > max[2] => [max[0], max[1], x],
        _ => max,
    };

    Some(max.iter().sum())
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
