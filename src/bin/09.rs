use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut h = (0, 0);
    let mut t = (0, 0);
    let mut t_pos = HashSet::new();

    for (instr, steps) in input
        .lines()
        .filter_map(|l| l.split_once(' '))
        .map(|(s, v)| (s, v.parse::<u8>().unwrap()))
    {
        for _ in 0..steps {
            h = match instr {
                "L" => (h.0 - 1, h.1),
                "R" => (h.0 + 1, h.1),
                "U" => (h.0, h.1 + 1),
                "D" => (h.0, h.1 - 1),
                x => panic!("Bad instr: {}", x),
            };

            let dx = h.0 - t.0;
            let dy = h.1 - t.1;
            if i32::abs(dx) > 1 || i32::abs(dy) > 1 {
                t = (t.0 + i32::signum(dx), t.1 + i32::signum(dy));
            }

            t_pos.insert(t);
        }
    }

    Some(t_pos.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut knots = [(0, 0); 10];
    let mut t_pos = HashSet::new();

    for (instr, steps) in input
        .lines()
        .filter_map(|l| l.split_once(' '))
        .map(|(s, v)| (s, v.parse::<u8>().unwrap()))
    {
        for _ in 0..steps {
            knots[0] = match instr {
                "L" => (knots[0].0 - 1, knots[0].1),
                "R" => (knots[0].0 + 1, knots[0].1),
                "U" => (knots[0].0, knots[0].1 + 1),
                "D" => (knots[0].0, knots[0].1 - 1),
                x => panic!("Bad instr: {}", x),
            };

            for i in 1..knots.len() {
                let prev = knots[i - 1];
                let knot = &mut knots[i];

                let dx = prev.0 - knot.0;
                let dy = prev.1 - knot.1;
                if i32::abs(dx) > 1 || i32::abs(dy) > 1 {
                    *knot = (knot.0 + i32::signum(dx), knot.1 + i32::signum(dy));
                }
            }

            t_pos.insert(knots[9]);
        }
    }

    Some(t_pos.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
