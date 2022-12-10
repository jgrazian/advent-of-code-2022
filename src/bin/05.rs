fn parse(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let mut stacks = (0..10)
        .into_iter()
        .map(|_| Vec::with_capacity(32))
        .collect::<Vec<_>>();
    let mut instructions = Vec::new();

    let mut flag = false;

    'a: for line in input.lines() {
        if line.is_empty() {
            flag = true;
            continue;
        }

        if !flag {
            for (i, b) in line
                .bytes()
                .skip(1)
                .step_by(4)
                .map(|b| b as char)
                .enumerate()
            {
                match b {
                    '0'..='9' => continue 'a,
                    'A'..='Z' => stacks[i].push(b),
                    ' ' => (),
                    _ => panic!("Bad char {}", b),
                }
            }
        } else {
            let mut v = line
                .strip_prefix("move ")
                .unwrap()
                .split_whitespace()
                .step_by(2)
                .map(|s| s.parse::<usize>().unwrap());

            instructions.push((
                v.next().unwrap(),
                v.next().unwrap() - 1,
                v.next().unwrap() - 1,
            ));
        }
    }

    stacks.iter_mut().for_each(|v| v.reverse());
    (stacks, instructions)
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut crates, moves) = parse(input);

    for (n, from, to) in moves {
        for _ in 0..n {
            let c = crates[from].pop().unwrap();
            crates[to].push(c);
        }
    }

    let v = crates.iter().filter_map(|c| c.last()).collect::<String>();

    Some(v)
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut crates, moves) = parse(input);

    for (n, from, to) in moves {
        let range = crates[from].len() - n..;
        let tmp = crates[from].drain(range).collect::<Vec<_>>();
        crates[to].extend_from_slice(&tmp);
    }

    let v = crates.iter().filter_map(|c| c.last()).collect::<String>();

    Some(v)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
