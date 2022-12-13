use std::collections::HashMap;

use advent_of_code::helpers::Grid;

fn make_grid(input: &str) -> Grid {
    let elements = input
        .bytes()
        .filter_map(|b| match b {
            b'a'..=b'z' => Some(b - b'a' + 1),
            b'S' => Some(0),
            b'E' => Some(b'z' - b'a' + 2),
            _ => None,
        })
        .collect();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    Grid::new(width, height, elements)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = make_grid(input);
    let start = grid.find(0).unwrap();

    let mut set = HashMap::from([(start, 0)]);

    let mut stack = Vec::from([(0, start)]);

    let mut min = u32::MAX;

    while let Some((path_score, xy)) = stack.pop() {
        let xy_value = grid.get_coord(xy);

        if xy_value == b'z' - b'a' + 2 {
            min = min.min(path_score);
            continue;
        }

        for uv in grid.manhattan_neighbors(xy) {
            if grid.get_coord(uv) > xy_value && grid.get_coord(uv) - xy_value > 1 {
                continue;
            }
            if set.contains_key(&uv) && set.get(&uv).unwrap() <= &path_score {
                continue;
            }

            set.insert(uv, path_score);
            stack.push((path_score + 1, uv))
        }
    }

    Some(min)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = make_grid(input);
    let mut sets = grid
        .coord_values()
        .filter_map(|(xy, v)| if *v == 1 { Some(((xy), 0)) } else { None })
        .map(|v| HashMap::from([v]))
        .collect::<Vec<_>>();

    let mut stack = Vec::new();

    for (set_idx, set) in sets.iter_mut().enumerate() {
        let xy = set.keys().next().unwrap();
        stack.push((0, *xy, set_idx));
    }

    let mut min = u32::MAX;

    while let Some((path_score, xy, set_idx)) = stack.pop() {
        let xy_value = grid.get_coord(xy);

        if xy_value == b'z' - b'a' + 2 {
            min = min.min(path_score);
            continue;
        }

        for uv in grid.manhattan_neighbors(xy) {
            if grid.get_coord(uv) > xy_value && grid.get_coord(uv) - xy_value > 1 {
                continue;
            }
            if sets[set_idx].contains_key(&uv) && sets[set_idx].get(&uv).unwrap() <= &path_score {
                continue;
            }

            sets[set_idx].insert(uv, path_score);
            stack.push((path_score + 1, uv, set_idx))
        }
    }

    Some(min)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
