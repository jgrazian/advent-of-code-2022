/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

#[derive(Debug, Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    elements: Vec<u8>,
}

impl Grid {
    pub fn new(width: usize, height: usize, elements: Vec<u8>) -> Self {
        Self {
            width,
            height,
            elements,
        }
    }

    pub fn get_coord(&self, xy: (usize, usize)) -> u8 {
        self.elements[self.coord_to_index(xy)]
    }

    pub fn coord_to_index(&self, xy: (usize, usize)) -> usize {
        xy.1 * self.width + xy.0
    }

    pub fn index_to_coord(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = index / self.width;
        (x, y)
    }

    pub fn coord_values(&self) -> impl Iterator<Item = ((usize, usize), &u8)> + '_ {
        self.elements
            .iter()
            .enumerate()
            .map(|(i, v)| (self.index_to_coord(i), v))
    }

    pub fn is_valid_coord(&self, xy: (isize, isize)) -> bool {
        let x = xy.0;
        let y = xy.1;

        x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize
    }

    pub fn find(&self, value: u8) -> Option<(usize, usize)> {
        self.elements.iter().enumerate().find_map(|(i, v)| {
            if v == &value {
                Some(self.index_to_coord(i))
            } else {
                None
            }
        })
    }

    pub fn row(&self, row: usize) -> impl Iterator<Item = &u8> + '_ {
        self.elements[row * self.width..row * self.width + self.width].iter()
    }

    pub fn rows(&self) -> impl Iterator<Item = &[u8]> + '_ {
        self.elements.chunks(self.width)
    }

    pub fn column(&self, column: usize) -> impl Iterator<Item = &u8> + '_ {
        self.elements.iter().skip(column).step_by(self.width)
    }

    pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = &u8>> + '_ {
        (0..self.width).into_iter().map(|c| self.column(c))
    }

    pub fn manhattan_neighbors(
        &self,
        xy: (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        let x = xy.0 as isize;
        let y = xy.1 as isize;
        [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .into_iter()
            .filter_map(|uv| {
                if self.is_valid_coord(uv) {
                    Some((uv.0 as usize, uv.1 as usize))
                } else {
                    None
                }
            })
    }

    pub fn cross_from_coord(
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
}
