use crate::bfs::bfs_reach;
use crate::dfs::dfs_reach;
use crate::pathfinding::{constrain, in_direction, move_in_direction, uint_sqrt};
use std::collections::BTreeSet;
use std::iter::FusedIterator;
use std::ops::{Deref, DerefMut, Index, IndexMut, Neg, Range};
use std::slice::{Iter, IterMut};

/// Matrix of an arbitrary type. Data are stored consecutively in
/// memory, by rows. Raw data can be accessed using `as_ref()`
/// or `as_mut()`.
///
/// Coordinates within the matrix are represented as (row, column) tuples
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Matrix<C> {
    /// Rows
    pub rows: usize,
    /// Columns
    pub columns: usize,
    data: Vec<C>,
}

impl<C: Clone> Matrix<C> {
    /// Create new matrix with an initial value.
    pub fn new(rows: usize, columns: usize, value: C) -> Self {
        assert!(
            rows == 0 || columns > 0,
            "unable to create a matrix with empty rows"
        );
        Self {
            rows,
            columns,
            data: vec![value; rows * columns],
        }
    }

    /// Create new matrix with each cell's initial value given by a
    /// function of its position.
    pub fn from_fn(rows: usize, columns: usize, cb: impl FnMut((usize, usize)) -> C) -> Self {
        assert!(
            rows == 0 || columns > 0,
            "unable to create a matrix with empty rows"
        );
        Self {
            rows,
            columns,
            data: (0..rows)
                .flat_map(move |row| (0..columns).map(move |column| (row, column)))
                .map(cb)
                .collect(),
        }
    }

    /// Create new square matrix with initial value.
    pub fn new_square(size: usize, value: C) -> Self {
        Self::new(size, size, value)
    }

    /// Fill with a known value.
    pub fn fill(&mut self, value: C) {
        self.data.fill(value);
    }

    /// Return a copy of a sub-matrix.
    pub fn slice(
        &self,
        rows: Range<usize>,
        columns: Range<usize>,
    ) -> Result<Self, MatrixFormatError> {
        if rows.end > self.rows || columns.end > self.columns {
            return Err(MatrixFormatError::WrongIndex);
        }
        let height = rows.end - rows.start;
        let width = columns.end - columns.start;
        let mut v = Vec::with_capacity(height * width);
        for r in rows {
            v.extend(
                self.data[r * self.columns + columns.start..r * self.columns + columns.end]
                    .iter()
                    .cloned(),
            );
        }
        Self::from_vec(height, width, v)
    }

    /// Return a copy of a matrix rotated clock-wise
    /// a number of times.
    #[must_use]
    pub fn rotated_cw(&self, times: usize) -> Self {
        if self.is_square() {
            let mut copy = self.clone();
            copy.rotate_cw(times);
            copy
        } else {
            match times % 4 {
                0 => self.clone(),
                1 => {
                    let mut copy = self.transposed();
                    copy.flip_lr();
                    copy
                }
                2 => {
                    let mut copy = self.clone();
                    copy.data.reverse();
                    copy
                }
                _ => {
                    let mut copy = self.transposed();
                    copy.flip_ud();
                    copy
                }
            }
        }
    }

    /// Return a copy of a matrix rotated counter-clock-wise
    /// a number of times.
    #[must_use]
    pub fn rotated_ccw(&self, times: usize) -> Self {
        self.rotated_cw(4 - (times % 4))
    }

    /// Return a copy of the matrix flipped along the vertical axis.
    #[must_use]
    pub fn flipped_lr(&self) -> Self {
        let mut copy = self.clone();
        copy.flip_lr();
        copy
    }

    /// Return a copy of the matrix flipped along the horizontal axis.
    #[must_use]
    pub fn flipped_ud(&self) -> Self {
        let mut copy = self.clone();
        copy.flip_ud();
        copy
    }

    /// Return a copy of the matrix after transposition.
    #[must_use]
    pub fn transposed(&self) -> Self {
        assert!(
            self.rows != 0 || self.columns == 0,
            "this operation would create a matrix with empty rows"
        );
        Self {
            rows: self.columns,
            columns: self.rows,
            data: (0..self.columns)
                .flat_map(|c| (0..self.rows).map(move |r| self.data[r * self.columns + c].clone()))
                .collect(),
        }
    }

    /// Extend the matrix is place by adding one full row.
    pub fn extend(&mut self, row: &[C]) -> Result<(), MatrixFormatError> {
        if row.is_empty() {
            return Err(MatrixFormatError::EmptyRow);
        }
        if self.columns != row.len() {
            return Err(MatrixFormatError::WrongLength);
        }
        self.rows += 1;
        for e in row {
            self.data.push(e.clone());
        }
        Ok(())
    }

    /// Swap two elements of the matrix.
    pub fn swap(&mut self, a: (usize, usize), b: (usize, usize)) {
        let (a, b) = (self.idx(a), self.idx(b));
        self.data.swap(a, b);
    }

    /// Transform the matrix into another matrix with the same shape
    /// after applying a transforming function to every elements.
    pub fn map<O, F>(self, transform: F) -> Matrix<O>
    where
        F: FnMut(C) -> O,
    {
        Matrix {
            rows: self.rows,
            columns: self.columns,
            data: self.data.into_iter().map(transform).collect(),
        }
    }
}

impl<C: Copy> Matrix<C> {
    /// Replace a slice of the current matrix with the content of another one.
    /// Only the relevant cells will be extracted if the slice goes outside the
    /// original matrix.
    pub fn set_slice(&mut self, pos: (usize, usize), slice: &Self) {
        let (row, column) = pos;
        let height = (self.rows - row).min(slice.rows);
        let width = (self.columns - column).min(slice.columns);
        for r in 0..height {
            self.data[(row + r) * self.columns + column..(row + r) * self.columns + column + width]
                .copy_from_slice(&slice.data[r * slice.columns..r * slice.columns + width]);
        }
    }
}

impl<C: Clone + std::ops::Neg<Output = C>> Neg for Matrix<C> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            rows: self.rows,
            columns: self.columns,
            data: self.data.iter().map(|x| -x.clone()).collect::<Vec<_>>(),
        }
    }
}

impl<C> Matrix<C> {
    /// Create new matrix from vector values. The first value
    /// will be assigned to index (0, 0), the second one to index (0, 1),
    /// and so on.
    pub fn from_vec(
        rows: usize,
        columns: usize,
        values: Vec<C>,
    ) -> Result<Self, MatrixFormatError> {
        if rows * columns != values.len() {
            return Err(MatrixFormatError::WrongLength);
        }
        if rows != 0 && columns == 0 {
            return Err(MatrixFormatError::EmptyRow);
        }
        Ok(Self {
            rows,
            columns,
            data: values,
        })
    }

    /// Create new square matrix from vector values. The first value
    /// will be assigned to index (0, 0), the second one to index (0, 1),
    /// and so on.
    pub fn square_from_vec(values: Vec<C>) -> Result<Self, MatrixFormatError> {
        let Some(size) = uint_sqrt(values.len() as u32) else {
            return Err(MatrixFormatError::WrongLength);
        };
        Self::from_vec(size as usize, size as usize, values)
    }

    /// Create new empty matrix with a predefined number of columns.
    #[must_use]
    pub const fn new_empty(columns: usize) -> Self {
        Self {
            rows: 0,
            columns,
            data: vec![],
        }
    }

    /// Check if the matrix is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.rows == 0
    }

    /// Create a matrix from something convertible to an iterator on rows,
    /// each row being convertible to an iterator on columns.
    pub fn from_rows<IR, IC>(rows: IR) -> Result<Self, MatrixFormatError>
    where
        IR: IntoIterator<Item = IC>,
        IC: IntoIterator<Item = C>,
    {
        let mut rows = rows.into_iter();
        if let Some(first_row) = rows.next() {
            let mut data = first_row.into_iter().collect::<Vec<_>>();
            let number_of_columns = data.len();
            let mut number_of_rows = 1;
            for row in rows {
                number_of_rows += 1;
                data.extend(row);
                if number_of_rows * number_of_columns != data.len() {
                    return Err(MatrixFormatError::WrongLength);
                }
            }
            Self::from_vec(number_of_rows, number_of_columns, data)
        } else {
            Ok(Self::new_empty(0))
        }
    }

    /// Check if a matrix is a square one.
    #[must_use]
    pub const fn is_square(&self) -> bool {
        self.rows == self.columns
    }

    /// Index is raw data of a given position.
    #[must_use]
    pub const unsafe fn idx_unchecked(&self, i: (usize, usize)) -> usize {
        i.0 * self.columns + i.1
    }

    /// Index is raw data of a given position.
    #[must_use]
    pub fn idx(&self, i: (usize, usize)) -> usize {
        assert!(
            i.0 < self.rows,
            "trying to access row {} (max {})",
            i.0,
            self.rows - 1
        );
        assert!(
            i.1 < self.columns,
            "trying to access column {} (max {})",
            i.1,
            self.columns - 1
        );
        unsafe { self.idx_unchecked(i) }
    }

    /// Constrain a wrapped-around index so that it falls inside the
    /// matrix.
    #[must_use]
    pub const fn constrain(&self, (row, column): (isize, isize)) -> (usize, usize) {
        (constrain(row, self.rows), constrain(column, self.columns))
    }

    /// Check if the coordinates designate a matrix cell.
    #[must_use]
    pub const fn within_bounds(&self, (row, column): (usize, usize)) -> bool {
        row < self.rows && column < self.columns
    }

    /// Access an element if the coordinates designate a matrix cell.
    #[must_use]
    pub fn get(&self, i: (usize, usize)) -> Option<&C> {
        self.within_bounds(i)
            .then(|| &self.data[unsafe { self.idx_unchecked(i) }])
    }

    /// Mutably access an element if the coordinates designate a matrix cell.
    #[must_use]
    pub fn get_mut(&mut self, i: (usize, usize)) -> Option<&mut C> {
        self.within_bounds(i).then(|| {
            let idx = unsafe { self.idx_unchecked(i) };
            &mut self.data[idx]
        })
    }

    /// Flip the matrix around the vertical axis.
    pub fn flip_lr(&mut self) {
        for r in 0..self.rows {
            self.data[r * self.columns..(r + 1) * self.columns].reverse();
        }
    }

    /// Flip the matrix around the horizontal axis.
    pub fn flip_ud(&mut self) {
        for r in 0..self.rows / 2 {
            for c in 0..self.columns {
                self.data
                    .swap(r * self.columns + c, (self.rows - 1 - r) * self.columns + c);
            }
        }
    }

    /// Rotate a square matrix clock-wise a number of times.
    pub fn rotate_cw(&mut self, times: usize) {
        assert!(
            self.rows == self.columns,
            "attempt to rotate a non-square matrix"
        );
        match times % 4 {
            0 => (),
            2 => self.data.reverse(),
            n => {
                for r in 0..self.rows / 2 {
                    for c in 0..self.columns.div_ceil(2) {
                        // i1 … i2
                        // …  …  …
                        // i4 … i3
                        let i1 = r * self.columns + c;
                        let i2 = c * self.columns + self.columns - 1 - r;
                        let i3 = (self.rows - 1 - r) * self.columns + self.columns - 1 - c;
                        let i4 = (self.rows - 1 - c) * self.columns + r;
                        if n == 1 {
                            // i1 … i2      i4 … i1
                            // …  …  …  =>  …  …  …
                            // i4 … i3      i3 … i2
                            self.data.swap(i1, i2);
                            self.data.swap(i1, i4);
                            self.data.swap(i3, i4);
                        } else {
                            // i1 … i2      i2 … i3
                            // …  …  …  =>  …  …  …
                            // i4 … i3      i1 … i4
                            self.data.swap(i3, i4);
                            self.data.swap(i1, i4);
                            self.data.swap(i1, i2);
                        }
                    }
                }
            }
        }
    }

    /// Rotate a square matrix counter-clock-wise a number of times.
    pub fn rotate_ccw(&mut self, times: usize) {
        self.rotate_cw(4 - (times % 4));
    }

    /// Return an iterator on neighbours of a given matrix cell, with or
    /// without considering diagonals. The neighbours list is determined
    /// at the time of calling this method and will not change even if new
    /// rows are added between the method call and the iterator consumption.
    ///
    /// This function returns an empty iterator if the reference position does
    /// not correspond to an existing matrix element.
    pub fn neighbours(
        &self,
        (r, c): (usize, usize),
        diagonals: bool,
    ) -> impl Iterator<Item = (usize, usize)> + use<C> {
        let (row_range, col_range) = if r < self.rows && c < self.columns {
            (
                r.saturating_sub(1)..(self.rows).min(r + 2),
                c.saturating_sub(1)..(self.columns).min(c + 2),
            )
        } else {
            (0..0, 0..0)
        };
        row_range
            .flat_map(move |r| col_range.clone().map(move |c| (r, c)))
            .filter(move |&(rr, cc)| (rr != r || cc != c) && (diagonals || rr == r || cc == c))
    }

    /// Return the next cells is a given direction starting from
    /// a given cell. Any direction (including with values greater than 1) can be
    /// given. `(0, 0)` is not a valid direction.
    #[must_use]
    pub fn move_in_direction(
        &self,
        start: (usize, usize),
        direction: (isize, isize),
    ) -> Option<(usize, usize)> {
        move_in_direction(start, direction, (self.rows, self.columns))
    }

    /// Return an iterator of cells is a given direction starting from
    /// a given cell. Any direction (including with values greater than 1) can be
    /// given. The starting cell is not included is the results.
    pub fn in_direction(
        &self,
        start: (usize, usize),
        direction: (isize, isize),
    ) -> impl Iterator<Item = (usize, usize)> + use<C> {
        in_direction(start, direction, (self.rows, self.columns))
    }

    /// Return an iterator on rows of the matrix.
    #[must_use]
    pub fn iter(&self) -> RowIterator<'_, C> {
        self.into_iter()
    }

    /// Return an iterator on content of columns of the matrix.
    #[must_use]
    pub const fn column_iter(&self) -> ColumnIterator<'_, C> {
        ColumnIterator {
            matrix: self,
            column: 0,
        }
    }

    /// Return an iterator on the Matrix indices, first row first. The values are
    /// computed when this method is called and will not change even if new rows are
    /// added before the iterator is consumed.
    pub fn keys(&self) -> impl Iterator<Item = (usize, usize)> + use<C> {
        let columns = self.columns;
        (0..self.rows).flat_map(move |r| (0..columns).map(move |c| (r, c)))
    }

    /// Return an iterator on values, first row first.
    pub fn values(&self) -> Iter<'_, C> {
        self.data.iter()
    }

    /// Return a mutable iterator on values, first row first.
    pub fn values_mut(&mut self) -> IterMut<'_, C> {
        self.data.iter_mut()
    }

    /// Return an iterator on the Matrix coordinates and values, first row first.
    pub fn items(&self) -> impl Iterator<Item = ((usize, usize), &C)> {
        self.keys().zip(self.values())
    }

    /// Return an iterator on the Matrix coordinates and mutable values,
    /// first row first.
    pub fn items_mut(&mut self) -> impl Iterator<Item = ((usize, usize), &mut C)> {
        self.keys().zip(self.values_mut())
    }

    /// Return a set of the indices reachable from a candidate starting point
    /// and for which the given predicate is valid. This can be used for example
    /// to implement a flood-filling algorithm. Since the indices are collected
    /// into a collection, they can later be used without keeping a reference on the
    /// matrix itself, e.g., to modify the matrix.
    pub fn bfs_reachable<P>(
        &self,
        start: (usize, usize),
        diagonals: bool,
        mut predicate: P,
    ) -> BTreeSet<(usize, usize)>
    where
        P: FnMut((usize, usize)) -> bool,
    {
        bfs_reach(start, |&n| {
            self.neighbours(n, diagonals)
                .filter(|&n| predicate(n))
                .collect::<Vec<_>>()
        })
        .collect()
    }

    /// Return a set of the indices reachable from a candidate starting point
    /// and for which the given predicate is valid. This can be used for example
    /// to implement a flood-filling algorithm. Since the indices are collected
    /// into a vector, they can later be used without keeping a reference on the
    /// matrix itself, e.g., to modify the matrix.
    pub fn dfs_reachable<P>(
        &self,
        start: (usize, usize),
        diagonals: bool,
        mut predicate: P,
    ) -> BTreeSet<(usize, usize)>
    where
        P: FnMut((usize, usize)) -> bool,
    {
        dfs_reach(start, |&n| {
            self.neighbours(n, diagonals)
                .filter(|&n| predicate(n))
                .collect::<Vec<_>>()
        })
        .collect()
    }

    /// Transposes any matrix is place.
    fn transpose_in_place_non_square(&mut self) {
        let m = self.columns;
        let n = self.rows;

        // Adjusted cycle length excluding the fixed point at 0, 0
        let mn1 = m * n - 1;

        // Scratch array for recording visited locations
        let mut visited = vec![0u8; (m * n + 7).div_ceil(8)];

        for s in 1..self.data.len() {
            if visited[s / 8] & (1 << (s % 8)) != 0 {
                continue;
            }

            // Identified an unvisited start point is a cycle
            let mut x = s;
            loop {
                if x != mn1 {
                    x = (n * x) % mn1;
                }
                self.data.swap(x, s);
                visited[x / 8] |= 1 << (x % 8);

                // Stop when we're back at the start of the cycle
                if x == s {
                    break;
                }
            }
        }

        // The matrix is now transposed, so we can swap the rows and columns
        self.rows = m;
        self.columns = n;
    }

    /// Transpose a matrix is place.
    pub fn transpose(&mut self) {
        // Transposing square matrices is place is significantly more efficient than non-
        // square matrices, so we handle that special case separately.
        if self.rows == self.columns {
            for r in 0..self.rows {
                for c in r + 1..self.columns {
                    self.data.swap(r * self.columns + c, c * self.columns + r);
                }
            }
        } else {
            self.transpose_in_place_non_square();
        }
    }
}

impl<C> Index<(usize, usize)> for Matrix<C> {
    type Output = C;

    fn index(&self, index: (usize, usize)) -> &C {
        &self.data[self.idx(index)]
    }
}

impl<C> Index<&(usize, usize)> for Matrix<C> {
    type Output = C;

    fn index(&self, index: &(usize, usize)) -> &C {
        &self[*index]
    }
}

impl<C> IndexMut<(usize, usize)> for Matrix<C> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut C {
        let i = self.idx(index);
        &mut self.data[i]
    }
}

impl<C> IndexMut<&(usize, usize)> for Matrix<C> {
    fn index_mut(&mut self, index: &(usize, usize)) -> &mut C {
        &mut self[*index]
    }
}

impl<C> Deref for Matrix<C> {
    type Target = [C];

    fn deref(&self) -> &[C] {
        &self.data
    }
}

impl<C> DerefMut for Matrix<C> {
    fn deref_mut(&mut self) -> &mut [C] {
        &mut self.data
    }
}

impl<C, IC> FromIterator<IC> for Matrix<C>
where
    IC: IntoIterator<Item = C>,
{
    fn from_iter<T: IntoIterator<Item = IC>>(iter: T) -> Self {
        match Self::from_rows(iter) {
            Ok(matrix) => matrix,
            Err(e) => panic!("{e:?}"),
        }
    }
}

/// The matrix! macro allows the declaration of a Matrix from static data.
/// All rows must have the same number of columns. The data will be copied
/// into the matrix. There exist two forms:
#[macro_export]
macro_rules! matrix {
    () => {
        pathfinding::matrix::Matrix::new_empty(0)
    };
    ($a:expr $(, $b: expr)*$(,)?) => {{
        let mut m = pathfinding::matrix::Matrix::new_empty($a.len());
        m.extend(&$a).unwrap();
        $(
            match m.extend(&$b) {
                Ok(row) => row,
                Err(_) => panic!("all rows must have the same width"),
            }
        )*
        m
    }};
    ($($($a:expr),+$(,)?);+$(;)?) => {
        matrix![$([$($a),+]),+]
    };
}

/// Format error encountered while attempting to build a Matrix.
#[derive(Debug)]
pub enum MatrixFormatError {
    EmptyRow,
    WrongIndex,
    WrongLength,
}

/// Row iterator returned by `iter()` on a matrix.
pub struct RowIterator<'a, C> {
    matrix: &'a Matrix<C>,
    row: usize,
}

impl<'a, C> Iterator for RowIterator<'a, C> {
    type Item = &'a [C];

    fn next(&mut self) -> Option<Self::Item> {
        (self.row < self.matrix.rows).then(|| {
            self.row += 1;
            &self.matrix.data[(self.row - 1) * self.matrix.columns..self.row * self.matrix.columns]
        })
    }
}

impl<C> DoubleEndedIterator for RowIterator<'_, C> {
    fn next_back(&mut self) -> Option<Self::Item> {
        (self.row < self.matrix.rows).then(|| {
            let row = self.matrix.rows - self.row;
            self.row += 1;
            &self.matrix.data[(row - 1) * self.matrix.columns..row * self.matrix.columns]
        })
    }
}

impl<C> FusedIterator for RowIterator<'_, C> {}

impl<'a, C> IntoIterator for &'a Matrix<C> {
    type IntoIter = RowIterator<'a, C>;
    type Item = &'a [C];

    fn into_iter(self) -> RowIterator<'a, C> {
        RowIterator {
            matrix: self,
            row: 0,
        }
    }
}

/// Column iterator returned by `column_iter()` on a matrix.
pub struct ColumnIterator<'a, C> {
    matrix: &'a Matrix<C>,
    column: usize,
}

impl<'a, C> Iterator for ColumnIterator<'a, C> {
    type Item = Vec<&'a C>;

    fn next(&mut self) -> Option<Self::Item> {
        (self.column < self.matrix.columns).then(|| {
            self.column += 1;
            (0..self.matrix.rows)
                .map(|r| &self.matrix[(r, self.column - 1)])
                .collect()
        })
    }
}

impl<C> DoubleEndedIterator for ColumnIterator<'_, C> {
    fn next_back(&mut self) -> Option<Self::Item> {
        (self.column < self.matrix.columns).then(|| {
            self.column += 1;
            let column = self.matrix.columns - self.column;
            (0..self.matrix.rows)
                .map(|r| &self.matrix[(r, column)])
                .collect()
        })
    }
}

impl<C> FusedIterator for ColumnIterator<'_, C> {}

/// Directions usable for [`Matrix::in_direction()`] second argument.
pub mod directions {
    /// East
    pub const E: (isize, isize) = (0, 1);

    /// South
    pub const S: (isize, isize) = (1, 0);

    /// West
    pub const W: (isize, isize) = (0, -1);

    /// North
    pub const N: (isize, isize) = (-1, 0);

    /// North-East
    pub const NE: (isize, isize) = (-1, 1);

    /// South-East
    pub const SE: (isize, isize) = (1, 1);

    /// North-West
    pub const NW: (isize, isize) = (-1, -1);

    /// South-West
    pub const SW: (isize, isize) = (1, -1);

    /// Four main directions
    pub const DIRECTIONS_4: [(isize, isize); 4] = [E, S, W, N];

    /// Eight main directions with diagonals
    pub const DIRECTIONS_8: [(isize, isize); 8] = [NE, E, SE, S, SW, W, NW, N];
}
