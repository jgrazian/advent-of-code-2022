struct Grid {
    width: usize,
    height: usize,
    elements: Vec<u8>,
}

impl Grid {
    fn new(width: usize, height: usize, elements: Vec<u8>) -> Self {
        Self {
            width,
            height,
            elements,
        }
    }

    fn get_coord(&self, xy: (usize, usize)) -> u8 {
        self.elements[self.coord_to_index(xy)]
    }

    fn coord_to_index(&self, xy: (usize, usize)) -> usize {
        xy.1 * self.width + xy.0
    }

    fn index_to_coord(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = index / self.width;
        (x, y)
    }

    fn coord_values(&self) -> impl Iterator<Item = ((usize, usize), &u8)> + '_ {
        self.elements
            .iter()
            .enumerate()
            .map(|(i, v)| (self.index_to_coord(i), v))
    }

    fn row(&self, row: usize) -> impl Iterator<Item = &u8> + '_ {
        self.elements[row * self.width..row * self.width + self.width].iter()
    }

    fn rows(&self) -> impl Iterator<Item = &[u8]> + '_ {
        self.elements.chunks(self.width)
    }

    fn column(&self, column: usize) -> impl Iterator<Item = &u8> + '_ {
        self.elements.iter().skip(column).step_by(self.width)
    }

    fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = &u8>> + '_ {
        (0..self.width).into_iter().map(|c| self.column(c))
    }

    fn cross_from_coord(
        &self,
        xy: (usize, usize),
    ) -> impl Iterator<Item = [Option<(usize, usize)>; 4]> + '_ {
        let x = xy.0;
        let y = xy.1;

        (1..)
            .map(move |i| {
                [
                    if x >= i { Some((x - i, y)) } else { None },
                    if x < self.width - i {
                        Some((x + i, y))
                    } else {
                        None
                    },
                    if y >= i { Some((x, y - i)) } else { None },
                    if y < self.height - i {
                        Some((x, y + i))
                    } else {
                        None
                    },
                ]
            })
            .take_while(|arr| arr.iter().any(|v| v.is_some()))
    }
}

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
    fn test_grid_rows() {
        let elements = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let width = 3;
        let height = 3;

        let grid = Grid::new(width, height, elements);
        assert_eq!(grid.row(0).copied().collect::<Vec<_>>(), &[0, 1, 2]);
        assert_eq!(grid.row(1).copied().collect::<Vec<_>>(), &[3, 4, 5]);
        assert_eq!(grid.row(2).copied().collect::<Vec<_>>(), &[6, 7, 8]);
    }

    #[test]
    fn test_grid_columns() {
        let elements = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let width = 3;
        let height = 3;

        let grid = Grid::new(width, height, elements);
        assert_eq!(grid.column(0).copied().collect::<Vec<_>>(), &[0, 3, 6]);
        assert_eq!(grid.column(1).copied().collect::<Vec<_>>(), &[1, 4, 7]);
        assert_eq!(grid.column(2).copied().collect::<Vec<_>>(), &[2, 5, 8]);
    }

    #[test]
    fn test_grid_cross() {
        let elements = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let width = 3;
        let height = 3;

        let grid = Grid::new(width, height, elements);
        assert_eq!(
            grid.cross_from_coord((1, 1)).next().unwrap(),
            [Some((0, 1)), Some((2, 1)), Some((1, 0)), Some((1, 2))]
        );

        assert_eq!(
            grid.cross_from_coord((0, 0)).next().unwrap(),
            [None, Some((1, 0)), None, Some((0, 1))]
        );
        assert_eq!(
            grid.cross_from_coord((0, 0)).nth(1).unwrap(),
            [None, Some((2, 0)), None, Some((0, 2))]
        );
        assert_eq!(grid.cross_from_coord((0, 0)).nth(2), None);
    }

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
