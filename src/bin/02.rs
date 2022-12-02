pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|l| l.split_once(' '))
            .map(|(abc, xyz)| match xyz {
                "X" => {
                    1 + match abc {
                        "A" => 3,
                        "B" => 0,
                        "C" => 6,
                        _ => unreachable!(),
                    }
                }
                "Y" => {
                    2 + match abc {
                        "A" => 6,
                        "B" => 3,
                        "C" => 0,
                        _ => unreachable!(),
                    }
                }
                "Z" => {
                    3 + match abc {
                        "A" => 0,
                        "B" => 6,
                        "C" => 3,
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|l| l.split_once(' '))
            .map(|(abc, xyz)| match xyz {
                "X" => match abc {
                    "A" => 3,
                    "B" => 1,
                    "C" => 2,
                    _ => unreachable!(),
                },
                "Y" => {
                    3 + match abc {
                        "A" => 1,
                        "B" => 2,
                        "C" => 3,
                        _ => unreachable!(),
                    }
                }
                "Z" => {
                    6 + match abc {
                        "A" => 2,
                        "B" => 3,
                        "C" => 1,
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
