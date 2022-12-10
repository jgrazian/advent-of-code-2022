pub fn part_one(input: &str) -> Option<u32> {
    let mut stack = vec![0];
    let mut total = 0;

    for l in input.lines() {
        match &l[0..3] {
            "$ c" => match &l[5..] {
                ".." => {
                    if let Some(v) = stack.pop() {
                        if v < 100000 {
                            total += v;
                        }
                    }
                }
                _ => stack.push(0),
            },
            "$ l" | "dir" => (),
            _ => {
                let (size_str, _) = l.split_once(' ').unwrap();
                let size: u32 = size_str.parse().unwrap();
                stack.iter_mut().for_each(|v| *v += size);
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut stack = vec![0];
    let mut dirs = Vec::with_capacity(64);

    for l in input.lines() {
        match &l[0..3] {
            "$ c" => match &l[5..] {
                ".." => {
                    if let Some(v) = stack.pop() {
                        dirs.push(v);
                    }
                }
                _ => stack.push(0),
            },
            "$ l" | "dir" => (),
            _ => {
                let (size_str, _) = l.split_once(' ').unwrap();
                let size: u32 = size_str.parse().unwrap();
                stack.iter_mut().for_each(|v| *v += size);
            }
        }
    }

    dirs.extend_from_slice(&stack);
    let free = 70000000 - stack[0];
    let needed = 30000000 - free;

    Some(dirs.iter().copied().filter(|d| d > &needed).min().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
