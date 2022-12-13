use advent_of_code::helpers::Grid;

pub fn part_one(input: &str) -> Option<u32> {
    let elements = input.bytes().filter(|b| matches!(b, b'0'..=b'9')).collect();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let grid = Grid::new(width, height, elements);

    let mut count = 0;
    'a: for (xy, v) in grid.coord_values() {
        let mut mask = [true; 4];

        for cross in grid.cross_from_coord(xy) {
            if cross.iter().zip(mask).any(|(c, m)| c.is_none() && m) {
                count += 1;
                continue 'a;
            }

            cross.iter().zip(mask.iter_mut()).for_each(|(c, m)| {
                if let Some(xy) = c {
                    if grid.get_coord(*xy) >= *v {
                        *m = false;
                    }
                }
            })
        }

        if mask.iter().copied().any(|v| v) {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let elements = input.bytes().filter(|b| matches!(b, b'0'..=b'9')).collect();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let grid = Grid::new(width, height, elements);

    let mut max_dist = 0;
    for (xy, v) in grid.coord_values() {
        let mut mask = [true; 4];
        let mut dist = [0; 4];

        for cross in grid.cross_from_coord(xy) {
            cross
                .iter()
                .zip(mask.iter_mut())
                .zip(dist.iter_mut())
                .filter_map(|((c, m), d)| {
                    if c.is_none() || !*m {
                        None
                    } else {
                        Some((c.unwrap(), m, d))
                    }
                })
                .for_each(|(xy, m, d)| {
                    if grid.get_coord(xy) >= *v {
                        *m = false;
                    }
                    *d += 1;
                })
        }

        max_dist = max_dist.max(dist.iter().product());
    }

    Some(max_dist)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
