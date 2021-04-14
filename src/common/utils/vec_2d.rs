use itertools::Itertools;
use std::iter::repeat;
use std::ops::Index;
use std::ops::IndexMut;

pub type Vec2dCoords = (usize, usize);
pub type Vec2dOffset = (isize, isize);

pub struct Vec2d<T> {
    data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Vec2d<T> {
    pub fn new(data: Vec<T>, width: usize, height: usize) -> Self {
        debug_assert_eq!(width * height, data.len(), "Failed to construct Vec2d");

        Vec2d {
            data,
            width,
            height,
        }
    }

    pub fn to_vec(self) -> Vec<T> {
        self.data
    }

    pub fn index_by_delta(&self, (x, y): Vec2dCoords, (dx, dy): Vec2dOffset) -> Option<&T> {
        let target_x = (x as isize + dx) as usize;
        let target_y = (y as isize + dy) as usize;
        let is_x_valid = target_x < self.width;
        let is_y_valid = target_y < self.height;

        iif!(
            is_x_valid && is_y_valid,
            Some(&self[(target_x, target_y)]),
            None
        )
    }

    pub fn iter_block(&self, (x, y): Vec2dCoords, range: usize) -> impl Iterator<Item = &T> {
        let (x, y, range) = (x as isize, y as isize, range as isize);

        let y_start = (y - range).max(0);
        let y_end = (y + range).min(self.height as isize - 1);
        let y_range = y_start..=y_end;

        y_range.flat_map(move |y| {
            let x_start = (x - range).max(0);
            let x_end = (x + range).min(self.width as isize - 1);

            let row_starting_idx = self.width * y as usize;
            let i_start = row_starting_idx + x_start as usize;
            let i_end = row_starting_idx + x_end as usize;
            let i_range = i_start..=i_end;

            self.data[i_range].iter()
        })
    }

    pub fn index_neighbours_range(
        &self,
        (x, y): Vec2dCoords,
        range: usize,
    ) -> impl Iterator<Item = &T> {
        let range = range as isize;

        let x_offsets_range = -range..=range;
        let y_offsets_range = -range..=range;

        let offsets = x_offsets_range
            .cartesian_product(y_offsets_range)
            .filter(|(x, y)| !(*x == 0 && *y == 0));

        offsets.flat_map(move |(dx, dy)| self.index_by_delta((x, y), (dx, dy)).into_iter())
    }

    pub fn index_neighbours_8(&self, (x, y): Vec2dCoords) -> impl Iterator<Item = &T> {
        self.index_neighbours_range((x, y), 1)
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.data.iter().zip(0..).map(move |(item, idx)| {
            let x = idx % self.width;
            let y = idx / self.width;

            (x, y, item)
        })
    }

    pub fn assign(mut self, (x, y): Vec2dCoords, item: T) -> Self {
        self[(x, y)] = item;
        self
    }
}

impl<T: Copy> Vec2d<T> {
    pub fn fill(item: T, width: usize, height: usize) -> Self {
        Vec2d {
            data: repeat(item).take(width * height).collect(),
            width,
            height,
        }
    }
}

impl<T> Index<Vec2dCoords> for Vec2d<T> {
    type Output = T;

    fn index(&self, (x, y): Vec2dCoords) -> &Self::Output {
        debug_assert!(x < self.width, "Vec2d indexed by invalid x coordinate");
        debug_assert!(y < self.height, "Vec2d indexed by invalid y coordinate");

        &self.data[self.width * y + x]
    }
}

impl<T> IndexMut<Vec2dCoords> for Vec2d<T> {
    fn index_mut(&mut self, (x, y): Vec2dCoords) -> &mut Self::Output {
        debug_assert!(x < self.width, "Vec2d indexed by invalid x coordinate");
        debug_assert!(y < self.height, "Vec2d indexed by invalid y coordinate");

        &mut self.data[self.width * y + x]
    }
}
