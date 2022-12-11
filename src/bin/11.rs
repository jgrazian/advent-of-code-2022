use std::collections::VecDeque;

#[derive(Debug, Clone)]
enum Op {
    Add(u64),
    Mul(u64),
    SelfMul,
}

impl Op {
    fn apply(&self, v: u64) -> u64 {
        match self {
            Op::Add(x) => x + v,
            Op::Mul(x) => x * v,
            Op::SelfMul => v * v,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    op: Op,
    test: (u64, usize, usize),
}

fn parse(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let mut items = VecDeque::new();
    let mut op = Op::SelfMul;
    let mut test_value = 0;
    let mut true_rule = 0;
    let mut false_rule;

    for line in input.lines().filter(|l| !l.is_empty()) {
        match &line[0..6] {
            "Monkey" => (),
            "  Star" => {
                let (_, item_list) = line.split_once(": ").unwrap();
                items = item_list
                    .split(", ")
                    .map(|v| v.parse())
                    .collect::<Result<VecDeque<_>, _>>()
                    .unwrap();
            }
            "  Oper" => {
                let v = line[25..].parse();
                op = if line.contains('*') {
                    if let Ok(v) = v {
                        Op::Mul(v)
                    } else {
                        Op::SelfMul
                    }
                } else {
                    Op::Add(v.unwrap())
                }
            }
            "  Test" => test_value = line[21..].parse().unwrap(),
            "    If" => match &line[7..11] {
                "true" => true_rule = line[29..].parse().unwrap(),
                "fals" => {
                    false_rule = line[30..].parse().unwrap();
                    let monk = Monkey {
                        items: items.clone(),
                        op: op.clone(),
                        test: (test_value, true_rule, false_rule),
                    };
                    monkeys.push(monk);
                    items.clear();
                }
                x => panic!("Unknown: {}", x),
            },
            x => panic!("Unknown: {}", x),
        }
    }

    monkeys
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut monkeys = parse(input);
    let mut inspect_count = vec![0; monkeys.len()];

    for _ in 0..20 {
        for m in 0..monkeys.len() {
            let op = monkeys[m].op.clone();
            let mut new_values = monkeys[m]
                .items
                .drain(..)
                .map(|item| {
                    inspect_count[m] += 1;
                    op.apply(item) / 3
                })
                .collect::<Vec<_>>();

            let (div, t, f) = monkeys[m].test;
            new_values.drain(..).for_each(|new_val| {
                if new_val % div == 0 {
                    monkeys[t].items.push_back(new_val);
                } else {
                    monkeys[f].items.push_back(new_val);
                }
            });
        }
    }

    inspect_count.sort_unstable();
    Some(inspect_count[inspect_count.len() - 2..].iter().product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut monkeys = parse(input);
    let mut inspect_count = vec![0; monkeys.len()];

    let divisor = monkeys
        .iter()
        .map(|m| m.test.0)
        .reduce(|acc, m| acc * m)
        .unwrap();

    for _ in 0..10000 {
        for m in 0..monkeys.len() {
            let op = monkeys[m].op.clone();
            let (div, t, f) = monkeys[m].test;

            let mut new_values = monkeys[m]
                .items
                .drain(..)
                .map(|item| {
                    inspect_count[m] += 1;
                    op.apply(item) % divisor
                })
                .collect::<Vec<_>>();

            new_values.drain(..).for_each(|new_val| {
                if new_val % div == 0 {
                    monkeys[t].items.push_back(new_val);
                } else {
                    monkeys[f].items.push_back(new_val);
                }
            });
        }
    }

    inspect_count.sort_unstable();
    Some(inspect_count[inspect_count.len() - 2..].iter().product())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
