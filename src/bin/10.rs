enum Op {
    Addx(i32),
    Noop,
}

impl Op {
    fn cycles(&self) -> i32 {
        match self {
            Self::Addx(_) => 2,
            Self::Noop => 1,
        }
    }

    fn apply(&self, reg: i32) -> i32 {
        match self {
            Self::Addx(v) => reg + v,
            Self::Noop => reg,
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut cycle = 0;
    let mut reg = 1;
    let mut out = 0;

    for op in input.lines().filter_map(|l| match &l[0..4] {
        "addx" => {
            let Some((_, v)) = l.split_once(' ') else {return None};
            Some(Op::Addx(v.parse().unwrap()))
        }
        "noop" => Some(Op::Noop),
        _ => None,
    }) {
        for _ in 0..op.cycles() {
            cycle += 1;
            if cycle == 20 || (cycle - 20) % 40 == 0 {
                out += cycle * reg;
            }
        }
        reg = op.apply(reg)
    }

    Some(out)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cycle: i32 = 0;
    let mut reg = 1;
    let mut line = [false; 40];

    for op in input.lines().filter_map(|l| match &l[0..4] {
        "addx" => {
            let Some((_, v)) = l.split_once(' ') else {return None};
            Some(Op::Addx(v.parse().unwrap()))
        }
        "noop" => Some(Op::Noop),
        _ => None,
    }) {
        for _ in 0..op.cycles() {
            cycle += 1;
            let sprite_pos = (cycle - 1) % 40;

            if sprite_pos == reg - 1 || sprite_pos == reg || sprite_pos == reg + 1 {
                line[sprite_pos as usize] = true;
            }

            if cycle % 40 == 0 {
                println!(
                    "{}",
                    line.iter()
                        .map(|v| if *v { '#' } else { '.' })
                        .collect::<String>()
                );
                line.fill(false);
            }
        }
        reg = op.apply(reg)
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
