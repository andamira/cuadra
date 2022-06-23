// cuadra::grid::grid2d
//
//!
//

use crate::error::{Error, Result};
use core::ops::{Index, IndexMut};

/// Generic 2D grid. Simple & efficient.
///
/// # Table of Contents
///
/// [**Constructors from elements:**](#constructors-from-elements)
///
/// - [new][Grid2D#method.new]
/// - [from_rows][Grid2D#method.from_rows]
/// - [from_cols][Grid2D#method.from_cols]
/// - [from_row_major][Grid2D#method.from_row_major]
/// - [from_col_major][Grid2D#method.from_col_major]
/// - [from_fn_row_major][Grid2D#method.from_fn_row_major]
/// - [from_fn_col_major][Grid2D#method.from_fn_col_major]
/// - [from_iter_row_major][Grid2D#method.from_iter_row_major]
/// - [from_iter_col_major][Grid2D#method.from_iter_col_major]
///
/// [**Constructors from chunks:**](#constructors-from-chunks)
///
/// - [from_chunks][Grid2D#method.from_chunks]
///
/// [**General information**](#general-information)
///
/// - [capacity][Grid2D#method.capacity]
/// - [rows][Grid2D#method.rows]
/// - [cols][Grid2D#method.cols]
/// - [row_len][Grid2D#method.row_len]
/// - [col_len][Grid2D#method.col_len]
/// - [get_index][Grid2D#method.get_index]
/// - [get_index_unchecked][Grid2D#method.get_index_unchecked]
/// - [chunked_capacity][Grid2D#method.chunked_capacity]
/// - [chunked_cols][Grid2D#method.chunked_cols]
/// - [chunked_row_len][Grid2D#method.chunked_row_len]
/// - [get_chunk_index][Grid2D#method.get_index]
/// - [get_chunk_index_unchecked][Grid2D#method.get_index_unchecked]
///
/// [**Single element getters and setters:**](#get-or-set-a-single-element)
///
/// - [get][Grid2D#method.get]
/// - [get_unchecked][Grid2D#method.get_unchecked]
/// - [get_row_major][Grid2D#method.get_row_major]
/// - [get_col_major][Grid2D#method.get_col_major]
/// - [get_mut][Grid2D#method.get_mut]
/// - [get_mut_unchecked][Grid2D#method.get_mut_unchecked]
/// - [get_mut_row_major][Grid2D#method.get_mut_row_major]
/// - [get_mut_col_major][Grid2D#method.get_mut_col_major]
/// - [set][Grid2D#method.set]
/// - [set_unchecked][Grid2D#method.set_unchecked]
/// - [set_row_major][Grid2D#method.set_row_major]
/// - [set_col_major][Grid2D#method.set_col_major]
///
/// [**Iterators:**](#iterators)
///
/// - [row_major_iter][Grid2D#method.row_major_iter]
/// - [col_major_iter][Grid2D#method.col_major_iter]
/// - [row_iter][Grid2D#method.row_iter]
/// - [row_iter_mut][Grid2D#method.row_iter_mut]
/// - [col_iter][Grid2D#method.col_iter]
// - [col_iter_mut][Grid2D#method.col_iter_mut] // TODO
/// - [rows_iter][Grid2D#method.rows_iter]
/// - [cols_iter][Grid2D#method.cols_iter]
// - [diag1_iter][Grid2D#method.diag1_iter] // WIP
/// - [chunks_iter][Grid2D#method.chunks_iter]
/// - [chunks_iter_mut][Grid2D#method.chunks_iter_mut]
///
/// [**Collecting to Vec:**](#collecting-to-vec)
///
/// - [as_rows][Grid2D#method.as_rows]
/// - [as_cols][Grid2D#method.as_cols]
/// - [as_row_major][Grid2D#method.as_row_major]
/// - [as_col_major][Grid2D#method.as_col_major]
///
/// [**Exposing the underlying Vec:**](#exposing-the-underlying-vec)
///
/// - [vec][Grid2D#method.vec]
/// - [vec_ref][Grid2D#method.vec]
/// - [vec_ref_mut][Grid2D#method.vec]
///
/// [**Get or set a chunk:**](#get-or-set-a-chunk)
///
/// - [get_chunk][Grid2D#method.get_chunk]
/// - [get_chunk_unchecked][Grid2D#method.get_chunk_unchecked]
/// - [get_chunk_mut][Grid2D#method.get_chunk_mut]
/// - [get_chunk_mut_unchecked][Grid2D#method.get_chunk_mut_unchecked]
/// - [set_chunk][Grid2D#method.set_chunk]
/// - [set_chunk_unchecked][Grid2D#method.set_chunk_unchecked]
/// # Example
/// ```
/// # use cuadra::{Grid2D, Result};
/// # fn main() -> Result<()> {
/// let rows = vec![
///     vec![1, 2, 3],
///     vec![4, 5, 6]
/// ];
/// let cols = vec![
///     vec![1, 4],
///     vec![2, 5],
///     vec![3, 6]
/// ];
///
/// // construct
/// let grid = Grid2D::from_rows(&rows)?;
/// assert_eq!(grid, Grid2D::from_cols(&cols)?);
/// assert_eq!(grid.rows(), 2);
/// assert_eq!(grid.cols(), 3);
///
/// // index
/// assert_eq!(grid[(0, 0)], 1);
/// assert_eq!(grid[(1, 2)], 6);
/// assert_eq!(grid[(0, 2)], 3);
/// assert_eq!(grid[(1, 0)], 4);
///
/// // get
/// assert_eq!(grid.get(1, 2), Some(&6));
/// assert_eq!(grid.get_row_major(3), Some(&4));
/// assert_eq!(grid.get_col_major(3), Some(&5));
///
/// // get_mut
///
///
// /// assert_eq!(grid.as_rows(), rows);
/// # Ok(()) }
/// ```
///
#[derive(Clone, Eq, PartialEq)]
pub struct Grid2D<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

/// # Constructors
///
/// - [new][Grid2D#method.new]
/// - [from_rows][Grid2D#method.from_rows]
/// - [from_cols][Grid2D#method.from_cols]
/// - [from_row_major][Grid2D#method.from_row_major]
/// - [from_col_major][Grid2D#method.from_col_major]
/// - [from_iter_row_major][Grid2D#method.from_iter_row_major]
/// - [from_iter_col_major][Grid2D#method.from_iter_col_major]
/// - [from_fn_row_major][Grid2D#method.from_fn_row_major]
/// - [from_fn_col_major][Grid2D#method.from_fn_col_major]
///
impl<T: Clone> Grid2D<T> {
    /// Creates a new `Grid2D` with a size of `rows` and `cols`, filled with `element`.
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let grid = Grid2D::new(1, 2, 3);
    /// assert_eq!(grid.as_rows(),
    ///     vec![
    ///         vec![1, 1, 1],
    ///         vec![1, 1, 1],
    ///     ]
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(element: T, rows: usize, cols: usize) -> Self {
        let v = vec![element; rows * cols];
        Grid2D {
            grid: v,
            rows,
            cols,
        }
    }

    /// Creates a new `Grid2D` from a slice of rows.
    ///
    /// Returns an error if the rows are not all the same size.
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let rows = vec![
    ///     vec![1, 2, 3],
    ///     vec![4, 5, 6]
    /// ];
    /// let grid = Grid2D::from_rows(&rows)?;
    /// assert_eq!(grid[(1, 2)], 6);
    /// assert_eq!(grid.as_rows(), rows);
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_rows(elements: &[Vec<T>]) -> Result<Self>
    where
        T: Clone,
    {
        // MAYBE create unchecked version
        let row_len = elements.get(0).map(Vec::len).unwrap_or(0);
        if !elements.iter().all(|row| row.len() == row_len) {
            return Err(Error::DimensionMismatch);
        }
        Ok(Grid2D {
            grid: flatten(elements),
            rows: elements.len(),
            cols: row_len,
        })
    }

    /// Creates a new `Grid2D` from a slice of columns.
    ///
    /// Returns an error if the columns are not all the same size.
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let cols = vec![
    ///     vec![1, 4],
    ///     vec![2, 5],
    ///     vec![3, 6]
    /// ];
    /// let grid = Grid2D::from_cols(&cols)?;
    /// assert_eq!(grid[(1, 2)], 6);
    /// assert_eq!(grid.as_cols(), cols);
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_cols(elements: &[Vec<T>]) -> Result<Self>
    where
        T: Clone,
    {
        // MAYBE create unchecked version?
        let col_len = elements.get(0).map(Vec::len).unwrap_or(0);
        if !elements.iter().all(|col| col.len() == col_len) {
            return Err(Error::DimensionMismatch);
        }
        let rows = col_len;
        let cols = elements.len();
        let indices_row_major = (0..rows).flat_map(move |row| (0..cols).map(move |col| (row, col)));
        let grid = indices_row_major
            .map(|(row, col)| elements[col][row].clone())
            .collect();
        Ok(Grid2D { grid, rows, cols })
    }

    /// Creates a new `Grid2D` from the given flat slice in [row major
    /// order].
    ///
    /// Returns an error if the number of elements in `elements` is not the
    /// product of `rows` and `cols`, i.e. the dimensions do not
    /// match.
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let row_major = vec![
    ///     1, 2, 3,
    ///     4, 5, 6
    /// ];
    /// let grid = Grid2D::from_row_major(&row_major, 2, 3)?;
    /// assert_eq!(grid[(1, 2)], 6);
    /// assert_eq!(grid.as_rows(), vec![vec![1, 2, 3], vec![4, 5, 6]]);
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_row_major(elements: &[T], rows: usize, cols: usize) -> Result<Self>
    where
        T: Clone,
    {
        let total_len = rows * cols;
        if total_len != elements.len() {
            return Err(Error::DimensionMismatch);
        }
        Ok(Grid2D {
            grid: elements.to_vec(),
            rows,
            cols,
        })
    }

    /// Creates a new `Grid2D` from the given flat slice in [column major
    /// order].
    ///
    /// Return an error if the number of elements in `elements` is not the
    /// product of `rows` and `cols`, i.e. the dimensions do not
    /// match.
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let col_major = vec![
    ///     1, 4,
    ///     2, 5,
    ///     3, 6
    /// ];
    /// let grid = Grid2D::from_col_major(&col_major, 2, 3)?;
    /// assert_eq!(grid[(1, 2)], 6);
    /// assert_eq!(grid.as_rows(), vec![vec![1, 2, 3], vec![4, 5, 6]]);
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_col_major(elements: &[T], rows: usize, cols: usize) -> Result<Self>
    where
        T: Clone,
    {
        let total_len = rows * cols;
        if total_len != elements.len() {
            return Err(Error::DimensionMismatch);
        }
        let indices_row_major =
            (0..rows).flat_map(move |row| (0..cols).map(move |column| (row, column)));
        let grid = indices_row_major
            .map(|(row, column)| {
                let index = column * rows + row;
                elements[index].clone()
            })
            .collect();
        Ok(Grid2D { grid, rows, cols })
    }

    /// Creates a new `Grid2D` with the specified number of rows and columns
    /// and fills each element with the result of calling the given
    /// function. The function is called once for every location going in
    /// *row major order*.
    /// ```
    /// # use cuadra::Grid2D;
    /// let mut counter = 1;
    /// let increment = || { counter += 1; counter - 1 };
    /// let grid = Grid2D::from_fn_row_major(increment, 2, 3);
    /// assert_eq!(grid.as_rows(), vec![vec![1, 2, 3], vec![4, 5, 6]]);
    /// ```
    pub fn from_fn_row_major<F>(mut generator: F, rows: usize, cols: usize) -> Self
    where
        F: FnMut() -> T,
    {
        let total_len = rows * cols;
        let grid = (0..total_len).map(|_| generator()).collect();
        Grid2D { grid, rows, cols }
    }

    /// Creates a new `Grid2D` with the specified number of rows and columns
    /// and fills each element with the result of calling the given
    /// function. The function is called once for every location going in
    /// *column major order*.
    /// ```
    /// # use cuadra::Grid2D;
    /// let mut counter = 1;
    /// let increment = || { counter += 1; counter -1 };
    /// let grid = Grid2D::from_fn_col_major(increment, 2, 3);
    /// assert_eq!(grid.as_cols(), vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
    /// ```
    pub fn from_fn_col_major<F>(mut generator: F, rows: usize, cols: usize) -> Self
    where
        F: FnMut() -> T,
        T: Clone,
    {
        let total_len = rows * cols;
        let grid_col_major = (0..total_len).map(|_| generator()).collect::<Vec<_>>();
        Grid2D::from_col_major(&grid_col_major, rows, cols)
            .expect("from_fn_col_major should never fail")
    }

    /// Creates a new `Grid2D` with the specified number of rows and columns
    /// and fills each element with the elements produced from the provided
    /// iterator. If the iterator produces more than enough elements, the
    /// remaining are unused. Returns an error if the iterator does not produce
    /// enough elements.
    ///
    /// The elements are inserted into the grid in *row major order*.
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let iterator = (1..);
    /// let grid = Grid2D::from_iter_row_major(iterator, 2, 3)?;
    /// assert_eq!(grid.as_rows(), vec![vec![1, 2, 3], vec![4, 5, 6]]);
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_iter_row_major<I>(iterator: I, rows: usize, cols: usize) -> Result<Self>
    where
        I: Iterator<Item = T>,
    {
        let total_len = rows * cols;
        let grid = iterator.take(total_len).collect::<Vec<_>>();
        if grid.len() != total_len {
            return Err(Error::NotEnoughElements);
        }
        Ok(Grid2D { grid, rows, cols })
    }

    /// Creates a new `Grid2D` with the specified number of rows and columns
    /// and fills each element with the elements produced from the provided
    /// iterator. If the iterator produces more than enough elements, the
    /// remaining are unused. Returns an error if the iterator does not produce
    /// enough elements.
    ///
    /// The elements are inserted into the grid in *column major order*.
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let iterator = (1..);
    /// let grid = Grid2D::from_iter_col_major(iterator, 2, 3)?;
    /// assert_eq!(grid.as_rows(), vec![vec![1, 3, 5], vec![2, 4, 6]]);
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_iter_col_major<I>(iterator: I, rows: usize, cols: usize) -> Result<Self>
    where
        I: Iterator<Item = T>,
        T: Clone,
    {
        let total_len = rows * cols;
        let grid_col_major = iterator.take(total_len).collect::<Vec<_>>();
        Grid2D::from_col_major(&grid_col_major, rows, cols).map_err(|_| Error::NotEnoughElements)
    }
}

/// # Constructors from chunks
///
/// - [from_chunks][Grid2D#method.from_chunks]
//  - [from_iter_chunks][Grid2D#method.from_iter_chunks] // TODO
//  - [from_fn_chunks][Grid2D#method.from_fn_chunks] // MAYBE
impl<T: Copy> Grid2D<T> {
    /// Creates a new `Grid2D` by concatenating the elements inside `chunk`,
    /// with a total size of `rows` and `cols` * `chunk.len()`.
    ///
    /// `T` needs to be Copy.
    pub fn from_chunks(chunk: &[T], rows: usize, cols: usize) -> Self {
        let v = chunk.repeat(rows * cols);
        Grid2D {
            grid: v,
            rows,
            cols,
        }
    }
}

/// # General information
///
/// - [capacity][Grid2D#method.capacity]
/// - [rows][Grid2D#method.rows]
/// - [cols][Grid2D#method.cols]
/// - [row_len][Grid2D#method.row_len]
/// - [col_len][Grid2D#method.col_len]
/// - [get_index][Grid2D#method.get_index]
/// - [get_index_unchecked][Grid2D#method.get_index_unchecked]
/// - [chunked_capacity][Grid2D#method.chunked_capacity]
/// - [chunked_cols][Grid2D#method.chunked_cols]
/// - [chunked_row_len][Grid2D#method.chunked_row_len]
/// - [get_chunk_index][Grid2D#method.get_index]
/// - [get_chunk_index_unchecked][Grid2D#method.get_index_unchecked]
///
impl<T> Grid2D<T> {
    /// Returns the capacity of the grid (rows × cols).
    pub const fn capacity(&self) -> usize {
        self.rows * self.cols
    }

    /// Returns the number of rows.
    pub const fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns.
    pub const fn cols(&self) -> usize {
        self.cols
    }

    /// Returns the number of elements in each row, i.e. the number of columns.
    pub const fn row_len(&self) -> usize {
        self.cols
    }

    /// Returns the number of elements in each column, i.e. the number of rows.
    pub const fn col_len(&self) -> usize {
        self.rows
    }

    /// Translates 2D row,col coordinates into a 1D column index.
    ///
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let mut grid = Grid2D::from_rows(&rows).unwrap();
    ///
    /// assert_eq!(grid.get_index(0, 0), Some(0));
    /// assert_eq!(grid.get_index(1, 0), Some(3));
    /// assert_eq!(grid.get_index(1, 2), Some(5));
    /// assert_eq!(grid.get_index(1, 3), None);
    /// # Ok(())}
    /// ```
    pub const fn get_index(&self, row: usize, col: usize) -> Option<usize> {
        if row < self.rows && col < self.cols {
            Some(row * self.row_len() + col)
        } else {
            None
        }
    }

    /// Translates 2D row,col coordinates into a 1D column index, **unchecked**.
    ///
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let mut grid = Grid2D::from_rows(&rows).unwrap();
    ///
    /// assert_eq!(grid.get_index_unchecked(0, 0), 0);
    /// assert_eq!(grid.get_index_unchecked(1, 0), 3);
    /// assert_eq!(grid.get_index_unchecked(1, 2), 5);
    /// # Ok(())}
    /// ```
    pub const fn get_index_unchecked(&self, row: usize, col: usize) -> usize {
        row * self.row_len() + col
    }

    /// Returns the capacity of the grid in chunks (rows × cols / chunk_size).
    pub const fn chunked_capacity(&self, chunk_size: usize) -> usize {
        self.rows * self.cols / chunk_size
    }

    /// Returns the number of chunked columns.
    pub const fn chunked_cols(&self, chunk_size: usize) -> usize {
        self.cols / chunk_size
    }

    /// Returns the number of chunks in each row, i.e. the number of chunked columns.
    pub const fn chunked_row_len(&self, chunk_size: usize) -> usize {
        self.cols / chunk_size
    }

    /// Translates 2D row,col coordinates, with chunk length, into a 1D column index.
    ///
    /// - assumes the `row_len` to be an exact multiple of `chunk_len`.
    /// - only full chunks are allowed.
    ///
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let rows = vec![vec![0, 1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10, 11]];
    /// let mut grid = Grid2D::from_rows(&rows).unwrap();
    ///
    /// assert_eq!(grid.get_chunk_index(1, 0, 5), Some(5));
    /// assert_eq!(grid.get_chunk_index(2, 0, 2), Some(4));
    /// assert_eq!(grid.get_chunk_index(3, 0, 1), Some(3));
    /// assert_eq!(grid.get_chunk_index(3, 0, 2), None);
    ///
    /// assert_eq!(grid.get_chunk_index(2, 1, 2), Some(10));
    /// assert_eq!(grid.get_chunk_index(3, 1, 1), Some(9));
    /// # Ok(())}
    /// ```
    pub const fn get_chunk_index(&self, chunk_len: usize, row: usize, col: usize) -> Option<usize> {
        // if row < self.rows && self.cols % chunk_len == 0 { // doesn't work in all cases
        //
        // WIP:TODO find example this fixes.
        // if row < self.rows && col < (self.cols / chunk_len) && self.cols % chunk_len == 0 {

        if row < self.rows && col < (self.cols / chunk_len) {
            Some(row * self.row_len() + col * chunk_len)
        } else {
            None
        }
    }

    /// Translates 2D row,col coordinates, with chunk length, into a 1D column index,
    /// **unchecked**.
    ///
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let rows = vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7]];
    /// let mut grid = Grid2D::from_rows(&rows).unwrap();
    ///
    /// assert_eq!(grid.get_chunk_index_unchecked(2, 0, 1), 2);
    /// assert_eq!(grid.get_chunk_index_unchecked(3, 0, 1), 3); // chunk would be incomplete
    /// assert_eq!(grid.get_chunk_index_unchecked(4, 0, 2), 8); // doesn't check out of bounds
    ///
    /// assert_eq!(grid.get_chunk_index_unchecked(2, 1, 0), 4);
    /// assert_eq!(grid.get_chunk_index_unchecked(2, 1, 1), 6);
    /// # Ok(())}
    /// ```
    pub const fn get_chunk_index_unchecked(
        &self,
        chunk_len: usize,
        row: usize,
        col: usize,
    ) -> usize {
        row * self.row_len() + col * chunk_len
    }
}

/// # Get or set a single element
///
/// - [get][Grid2D#method.get]
/// - [get_unchecked][Grid2D#method.get_unchecked]
/// - [get_row_major][Grid2D#method.get_row_major]
/// - [get_col_major][Grid2D#method.get_col_major]
/// - [get_mut][Grid2D#method.get_mut]
/// - [get_mut_unchecked][Grid2D#method.get_mut_unchecked]
/// - [get_mut_row_major][Grid2D#method.get_mut_row_major]
/// - [get_mut_col_major][Grid2D#method.get_mut_col_major]
/// - [set][Grid2D#method.set]
/// - [set_unchecked][Grid2D#method.set_unchecked]
/// - [set_row_major][Grid2D#method.set_row_major]
/// - [set_col_major][Grid2D#method.set_col_major]
impl<T> Grid2D<T> {
    /// Returns a reference to the element at the given `row` and `col`umn.
    ///
    /// Returns `None` if the index is out of bounds.
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.get_index(row, col).map(|index| &self.grid[index])
    }

    /// Returns a reference to the element at the given `row` and `col`umn,
    /// **unchecked**.
    pub fn get_unchecked(&self, row: usize, col: usize) -> &T {
        &self.grid[self.get_index_unchecked(row, col)]
    }

    /// Returns a reference to the element at the given 1D index
    /// (in row major order).
    ///
    /// Returns `None` if the index is out of bounds.
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let grid = Grid2D::from_rows(&vec![vec![1, 2, 3], vec![4, 5, 6]])?;
    /// assert_eq!(grid.get_row_major(1), Some(&2));
    /// assert_eq!(grid.get_row_major(4), Some(&5));
    /// assert_eq!(grid.get_row_major(10000), None);
    /// # Ok(()) }
    /// ```
    pub fn get_row_major(&self, index: usize) -> Option<&T> {
        self.grid.get(index)
    }

    /// Returns a reference to the element at the given 1D index
    /// (in column major order).
    ///
    /// Returns `None` if the index is out of bounds.
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let grid = Grid2D::from_rows(&vec![vec![1, 2, 3], vec![4, 5, 6]])?;
    /// assert_eq!(grid.get_col_major(1), Some(&4));
    /// assert_eq!(grid.get_col_major(4), Some(&3));
    /// assert_eq!(grid.get_col_major(10000), None);
    /// # Ok(()) }
    /// ```
    pub fn get_col_major(&self, index: usize) -> Option<&T> {
        let col = dbg!(dbg!(index) / self.rows); // FIXME:DEBUG?
        let row = dbg!(index % self.rows);
        self.get(row, col)
    }

    /// Returns a mutable reference to the element at the given `row` and
    /// `col`umn if the index is in bounds (wrapped in `Some`).
    ///
    /// Returns `None` if the index is out of bounds.
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let mut grid = Grid2D::from_rows(&rows).unwrap();
    ///
    /// assert_eq!(grid.get_mut(0, 0), Some(&mut 1));
    /// assert_eq!(grid.get_mut(10, 10), None);
    ///
    /// grid.get_mut(0, 0).map(|x| *x = 100);
    /// assert_eq!(grid.get(0, 0), Some(&100));
    ///
    /// grid.get_mut(10, 10).map(|x| *x = 200);
    /// assert_eq!(grid.get(10, 10), None);
    /// # Ok(())}
    /// ```
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.get_index(row, col)
            .map(move |index| &mut self.grid[index])
    }

    /// Returns a mutable reference to the element at the given `row` and
    /// `col`umn, **unchecked**.
    pub fn get_mut_unchecked(&mut self, row: usize, col: usize) -> &mut T {
        let index = self.get_index_unchecked(row, col);
        &mut self.grid[index]
    }

    /// Returns a mutable reference to the element at the given index in row
    /// major order. Returns [`None`] if the index is out of bounds.
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let mut grid = Grid2D::from_rows(&rows)?;
    ///
    /// assert_eq!(grid.get_mut_row_major(1), Some(&mut 2));
    /// assert_eq!(grid.get_mut_row_major(10), None);
    ///
    /// grid.get_mut_row_major(3).map(|x| *x = 100);
    /// assert_eq!(grid.get(1, 0), Some(&100));
    ///
    /// grid.get_mut_row_major(10).map(|x| *x = 200);
    /// assert_eq!(grid.get(10, 10), None);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_mut_row_major(&mut self, index: usize) -> Option<&mut T> {
        self.grid.get_mut(index)
    }

    /// Returns a mutable reference to the element at the given index in row
    /// major order. Returns [`None`] if the index is out of bounds.
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let mut grid = Grid2D::from_rows(&rows)?;
    ///
    /// assert_eq!(grid.get_mut_col_major(1), Some(&mut 4));
    /// assert_eq!(grid.get_mut_col_major(10), None);
    ///
    /// grid.get_mut_col_major(4).map(|x| *x = 100);
    /// assert_eq!(grid.get(0, 2), Some(&100));
    ///
    /// grid.get_mut_col_major(10).map(|x| *x = 200);
    /// assert_eq!(grid.get(10, 10), None);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_mut_col_major(&mut self, index: usize) -> Option<&mut T> {
        let col = index / self.rows;
        let row = index % self.rows;
        self.get_mut(row, col)
    }

    /// Changes the element at given `row` and `column` to `element`. Returns
    /// [`Ok(())`] if the indices were in bounds and returns an [`Err`]
    /// otherwise.
    /// ```
    /// # use cuadra::{Grid2D, Error};
    /// let mut grid = Grid2D::new(42, 2, 3);
    ///
    /// let result = grid.set(0, 0, 100);
    /// assert_eq!(result, Ok(()));
    /// assert_eq!(grid.get(0, 0), Some(&100));
    ///
    /// let result = grid.set(10, 20, 200);
    /// assert_eq!(result, Err(Error::IndicesOutOfBounds(10, 20)));
    /// ```
    pub fn set(&mut self, row: usize, col: usize, element: T) -> Result<()> {
        self.get_mut(row, col)
            .map(|location| {
                *location = element;
            })
            .ok_or(Error::IndicesOutOfBounds(row, col))
    }

    /// Changes the element at given `row` and `column` to `element`, **unchecked**.
    pub fn set_unchecked(&mut self, row: usize, col: usize, element: T) {
        let location = self.get_mut_unchecked(row, col);
        *location = element;
    }

    /// Changes the element at the given `index` to `element`, in row major
    /// order. Returns [`Ok(())`] if the index is in bounds and returns an
    /// [`Err`] otherwise.
    /// ```
    /// # use cuadra::{Grid2D, Error};
    /// let mut grid = Grid2D::new(42, 2, 3);
    ///
    /// let result = grid.set_row_major(4, 100);
    /// assert_eq!(result, Ok(()));
    /// assert_eq!(grid.get(1, 1), Some(&100));
    ///
    /// let result = grid.set_row_major(10, 200);
    /// assert_eq!(result, Err(Error::IndexOutOfBounds(10)));
    /// ```
    pub fn set_row_major(&mut self, index: usize, element: T) -> Result<()> {
        self.get_mut_row_major(index)
            .map(|location| {
                *location = element;
            })
            .ok_or(Error::IndexOutOfBounds(index))
    }

    /// Changes the element at the given `index` to `element`, in column major
    /// order. Returns [`Ok(())`] if the index is in bounds and returns an
    /// [`Err`] otherwise.
    /// ```
    /// # use cuadra::{Grid2D, Error};
    /// let mut grid = Grid2D::new(42, 2, 3);
    ///
    /// let result = grid.set_col_major(4, 100);
    /// assert_eq!(result, Ok(()));
    /// assert_eq!(grid.get(0, 2), Some(&100));
    ///
    /// let result = grid.set_col_major(10, 200);
    /// assert_eq!(result, Err(Error::IndexOutOfBounds(10)));
    /// ```
    pub fn set_col_major(&mut self, index: usize, element: T) -> Result<()> {
        self.get_mut_col_major(index)
            .map(|location| {
                *location = element;
            })
            .ok_or(Error::IndexOutOfBounds(index))
    }
}

/// # Iterators
///
/// - [row_major_iter][Grid2D#method.row_major_iter]
/// - [col_major_iter][Grid2D#method.col_major_iter]
/// - [row_iter][Grid2D#method.row_iter]
/// - [row_iter_mut][Grid2D#method.row_iter_mut]
/// - [col_iter][Grid2D#method.col_iter]
// - [col_iter_mut][Grid2D#method.col_iter_mut]
/// - [rows_iter][Grid2D#method.rows_iter]
/// - [cols_iter][Grid2D#method.cols_iter]
///
/// - [chunks_iter][Grid2D#method.chunks_iter]
/// - [chunks_iter_mut][Grid2D#method.chunks_iter_mut]
///
impl<T> Grid2D<T> {
    /// Returns an iterator over references to all elements in *row major order*.
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let grid = Grid2D::from_rows(&rows)?;
    /// let row_major = grid.row_major_iter();
    /// assert_eq!(row_major.cloned().collect::<Vec<_>>(), vec![1, 2, 3, 4, 5, 6]);
    /// # Ok(()) }
    /// ```
    pub fn row_major_iter(&self) -> impl DoubleEndedIterator<Item = &T> {
        self.grid.iter()
    }

    /// Returns an iterator over references to all elements in *column major order*.
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let grid = Grid2D::from_rows(&rows)?;
    /// let col_major = grid.col_major_iter();
    /// assert_eq!(col_major.cloned().collect::<Vec<_>>(), vec![1, 4, 2, 5, 3, 6]);
    /// # Ok(()) }
    /// ```
    pub fn col_major_iter(&self) -> impl DoubleEndedIterator<Item = &T> {
        (0..self.cols).flat_map(move |col| (0..self.rows).map(move |row| &self[(row, col)]))
    }

    /// Returns an iterator over references to all elements in the given row.
    ///
    /// Returns an error if the index is out of bounds.
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let grid = Grid2D::from_rows(&rows)?;
    /// let mut row_iter = grid.row_iter(1)?;
    /// assert_eq!(row_iter.next(), Some(&4));
    /// assert_eq!(row_iter.next(), Some(&5));
    /// assert_eq!(row_iter.next(), Some(&6));
    /// assert_eq!(row_iter.next(), None);
    /// # Ok(()) }
    /// ```
    pub fn row_iter(&self, row_index: usize) -> Result<impl DoubleEndedIterator<Item = &T>> {
        let start = self
            .get_index(row_index, 0)
            .ok_or(Error::IndicesOutOfBounds(row_index, 0))?;
        let end = start + self.row_len();
        Ok(self.grid[start..end].iter())
    }

    /// Returns a mutable iterator over references to all elements in the given row.
    ///
    /// Returns an error if the index is out of bounds.
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let mut grid = Grid2D::from_rows(&rows)?;
    /// let mut row_iter = grid.row_iter_mut(1)?;
    /// assert_eq!(row_iter.next(), Some(&mut 4));
    /// assert_eq!(row_iter.next(), Some(&mut 5));
    /// assert_eq!(row_iter.next(), Some(&mut 6));
    /// assert_eq!(row_iter.next(), None);
    /// # Ok(()) }
    /// ```
    pub fn row_iter_mut(
        &mut self,
        row_index: usize,
    ) -> Result<impl DoubleEndedIterator<Item = &mut T>> {
        let start = self
            .get_index(row_index, 0)
            .ok_or(Error::IndicesOutOfBounds(row_index, 0))?;
        let end = start + self.row_len();
        Ok(self.grid[start..end].iter_mut())
    }

    /// Returns an iterator over references to all elements in the given column.
    /// Returns an error if the index is out of bounds.
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let grid = Grid2D::from_rows(&rows)?;
    /// let mut col_iter = grid.col_iter(1)?;
    /// assert_eq!(col_iter.next(), Some(&2));
    /// assert_eq!(col_iter.next(), Some(&5));
    /// assert_eq!(col_iter.next(), None);
    /// # Ok(()) }
    /// ```
    pub fn col_iter(&self, col_index: usize) -> Result<impl DoubleEndedIterator<Item = &T>> {
        if col_index >= self.cols {
            return Err(Error::IndicesOutOfBounds(0, col_index));
        }
        Ok((0..self.col_len()).map(move |row_index| &self[(row_index, col_index)]))
    }

    // TODO:FIXME
    // /// Returns a mutable iterator over references to all elements in the given
    // /// column. Returns an error if the index is out of bounds.
    // // pub fn col_iter_mut(&mut self, col_index: usize) -> Result<impl DoubleEndedIterator<Item = &mut T>> {
    // pub fn col_iter_mut(&mut self, col_index: usize) -> Result<impl Iterator<Item = &mut T>> {
    //     if col_index >= self.cols {
    //         return Err(Error::IndicesOutOfBounds(0, col_index));
    //     }
    //     Ok((0..self.col_len()).map(move |row_index| &mut self[(row_index, col_index)]))
    // }
    // }

    /// Returns an `Iterator` over all rows. Each `Item` is itself another
    /// `Iterator` over references to the elements in that row.
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let grid = Grid2D::from_rows(&rows)?;
    /// for row_iter in grid.rows_iter() {
    ///     for element in row_iter {
    ///         print!("{} ", element);
    ///     }
    ///     println!();
    /// }
    ///
    /// let mut rows_iter = grid.rows_iter();
    ///
    /// let mut first_row_iter = rows_iter.next().unwrap();
    /// assert_eq!(first_row_iter.next(), Some(&1));
    /// assert_eq!(first_row_iter.next(), Some(&2));
    /// assert_eq!(first_row_iter.next(), Some(&3));
    /// assert_eq!(first_row_iter.next(), None);
    ///
    /// let mut second_row_iter = rows_iter.next().unwrap();
    /// assert_eq!(second_row_iter.next(), Some(&4));
    /// assert_eq!(second_row_iter.next(), Some(&5));
    /// assert_eq!(second_row_iter.next(), Some(&6));
    /// assert_eq!(second_row_iter.next(), None);
    ///
    /// assert!(rows_iter.next().is_none());
    /// # Ok(())
    /// # }
    /// ```
    pub fn rows_iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &T>> {
        (0..self.rows()).map(move |row_index| {
            self.row_iter(row_index)
                .expect("rows_iter should never fail")
        })
    }

    /// Returns an `Iterator` over all columns. Each `Item` is itself
    /// another `Iterator` over references to the elements in that column.
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let grid = Grid2D::from_rows(&rows)?;
    /// for col_iter in grid.cols_iter() {
    ///     for element in col_iter {
    ///         print!("{} ", element);
    ///     }
    ///     println!();
    /// }
    ///
    /// let mut cols_iter = grid.cols_iter();
    ///
    /// let mut first_col_iter = cols_iter.next().unwrap();
    /// assert_eq!(first_col_iter.next(), Some(&1));
    /// assert_eq!(first_col_iter.next(), Some(&4));
    /// assert_eq!(first_col_iter.next(), None);
    ///
    /// let mut second_col_iter = cols_iter.next().unwrap();
    /// assert_eq!(second_col_iter.next(), Some(&2));
    /// assert_eq!(second_col_iter.next(), Some(&5));
    /// assert_eq!(second_col_iter.next(), None);
    ///
    /// let mut third_col_iter = cols_iter.next().unwrap();
    /// assert_eq!(third_col_iter.next(), Some(&3));
    /// assert_eq!(third_col_iter.next(), Some(&6));
    /// assert_eq!(third_col_iter.next(), None);
    ///
    /// assert!(cols_iter.next().is_none());
    /// # Ok(())
    /// # }
    /// ```
    pub fn cols_iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &T>> {
        (0..self.cols).map(move |col_index| {
            self.col_iter(col_index)
                .expect("cols_iter should never fail")
        })
    }

    /// Returns an iterator over `chunk_size` elements of the grid
    /// in *row major order*.
    ///
    /// # Panics
    /// Panics if `chunk_size` is 0.
    pub fn chunks_iter(&self, chunk_size: usize) -> impl DoubleEndedIterator<Item = &[T]> {
        self.grid.chunks(chunk_size)
    }

    /// Returns a mutable iterator over `chunk_size` elements of the grid
    /// in *row major order*.
    ///
    /// # Panics
    /// Panics if `chunk_size` is 0.
    pub fn chunks_iter_mut(
        &mut self,
        chunk_size: usize,
    ) -> impl DoubleEndedIterator<Item = &mut [T]> {
        self.grid.chunks_mut(chunk_size)
    }
}

/// # Collecting to Vec
///
/// - [as_rows][Grid2D#method.as_rows]
/// - [as_cols][Grid2D#method.as_cols]
/// - [as_row_major][Grid2D#method.as_row_major]
/// - [as_col_major][Grid2D#method.as_col_major]
///
impl<T> Grid2D<T> {
    /// Collects the `Grid2D` into a `Vec` of rows.
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let grid = Grid2D::from_rows(&rows)?;
    /// assert_eq!(grid.as_rows(), rows);
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_rows(&self) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        self.rows_iter()
            .map(|row_iter| row_iter.cloned().collect())
            .collect()
    }

    /// Collects the `Grid2D` into a `Vec` of columns.
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let cols = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
    /// let grid = Grid2D::from_cols(&cols)?;
    /// assert_eq!(grid.as_cols(), cols);
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_cols(&self) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        self.cols_iter()
            .map(|col_iter| col_iter.cloned().collect())
            .collect()
    }

    /// Collects the `Grid2D` into a `Vec` of elements in *row major order*.
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let grid = Grid2D::from_rows(&rows)?;
    /// assert_eq!(grid.as_row_major(), vec![1, 2, 3, 4, 5, 6]);
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_row_major(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.row_major_iter().cloned().collect()
    }

    /// Collects the `Grid2D` into a `Vec` of elements in *column major order*.
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let grid = Grid2D::from_rows(&rows)?;
    /// assert_eq!(grid.as_col_major(), vec![1, 4, 2, 5, 3, 6]);
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_col_major(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.col_major_iter().cloned().collect()
    }
}

/// # Exposing the underlying Vec
///
/// - [vec][Grid2D#method.vec]
/// - [vec_ref][Grid2D#method.vec]
/// - [vec_ref_mut][Grid2D#method.vec]
///
impl<T> Grid2D<T> {
    /// Returns the underlying Vec.
    pub fn vec(self) -> Vec<T> {
        self.grid
    }

    /// Returns a reference to the underlying Vec.
    pub fn vec_ref(&self) -> &Vec<T> {
        &self.grid
    }

    /// Returns a mutable reference to the underlying Vec
    pub fn vec_ref_mut(&mut self) -> &mut Vec<T> {
        &mut self.grid
    }
}

/// # Get or set a chunk.
///
/// - [get_chunk][Grid2D#method.get_chunk]
/// - [get_chunk_unchecked][Grid2D#method.get_chunk_unchecked]
/// - [get_chunk_mut][Grid2D#method.get_chunk_mut]
/// - [get_chunk_mut_unchecked][Grid2D#method.get_chunk_mut_unchecked]
/// - [set_chunk][Grid2D#method.set_chunk]
/// - [set_chunk_unchecked][Grid2D#method.set_chunk_unchecked]
///
impl<T: Clone> Grid2D<T> {
    /// Returns a Vec of references for the chunk of elements at the given `row` and `col`umn.
    ///
    /// Returns `None` if the index is out of bounds.
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let rows = vec![vec![0, 1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10, 11]];
    /// let mut grid = Grid2D::from_rows(&rows).unwrap();
    ///
    /// assert_eq!(grid.get_chunk(1, 0, 5), Some(vec![5].as_slice()));
    /// assert_eq!(grid.get_chunk(2, 0, 2), Some(vec![4,5].as_slice()));
    /// assert_eq!(grid.get_chunk(3, 0, 1), Some(vec![3,4,5].as_slice()));
    /// assert_eq!(grid.get_chunk(3, 0, 2), None);
    /// assert_eq!(grid.get_chunk(2, 1, 2), Some(vec![10,11].as_slice()));
    /// assert_eq!(grid.get_chunk(2, 0, 4), None);
    /// # Ok(())}
    /// ```
    pub fn get_chunk(&self, chunk_len: usize, row: usize, col: usize) -> Option<&[T]> {
        self.get_chunk_index(chunk_len, row, col)
            .map(|index| &self.grid[index..index + chunk_len])
    }

    /// Returns a slice for the chunk of elements at the given `row` and `col`umn, **unchecked**.
    ///
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let rows = vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7]];
    /// let mut grid = Grid2D::from_rows(&rows).unwrap();
    ///
    /// assert_eq!(grid.get_chunk_unchecked(2, 0, 1), vec![2, 3]);
    /// assert_eq!(grid.get_chunk_unchecked(3, 0, 1), vec![3, 4, 5]); // overlap
    /// assert_eq!(grid.get_chunk_unchecked(6, 0, 0), vec![0, 1, 2, 3, 4, 5]); // overlap
    /// assert_eq!(grid.get_chunk_unchecked(2, 1, 0), vec![4, 5]);
    /// assert_eq!(grid.get_chunk_unchecked(2, 0, 2), vec![4, 5]); // overlap
    /// # Ok(())}
    /// ```
    pub fn get_chunk_unchecked(&self, chunk_len: usize, row: usize, col: usize) -> &[T] {
        let index = self.get_chunk_index_unchecked(chunk_len, row, col);
        &self.grid[index..index + chunk_len]
    }

    ///
    pub fn get_chunk_mut(&mut self, chunk_len: usize, row: usize, col: usize) -> Option<&mut [T]> {
        self.get_chunk_index(chunk_len, row, col)
            .map(move |index| &mut self.grid[index..index + chunk_len])
    }

    ///
    pub fn get_chunk_mut_unchecked(
        &mut self,
        chunk_len: usize,
        row: usize,
        col: usize,
    ) -> &mut [T] {
        let index = self.get_chunk_index_unchecked(chunk_len, row, col);
        &mut self.grid[index..index + chunk_len]
    }

    /// Sets the elements on a chunk.
    ///
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let rows = vec![vec![0, 1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10, 11]];
    /// let mut grid = Grid2D::from_rows(&rows)?;
    /// grid.set_chunk(2, 0, 1, &[222, 333])?;
    /// assert_eq!(grid.get_chunk(4, 0, 0), Some(vec![0, 1, 222, 333].as_slice()));
    /// # Ok(()) }
    /// ```

    pub fn set_chunk(
        &mut self,
        chunk_len: usize,
        row: usize,
        col: usize,
        elements: &[T],
    ) -> Result<()> {
        let chunk = self
            .get_chunk_mut(chunk_len, row, col)
            .ok_or(Error::ChunkIndicesOutOfBounds(row, col, chunk_len))?;
        for (n, element) in chunk.iter_mut().enumerate() {
            *element = elements[n].clone();
        }
        Ok(())
    }

    /// Sets the elements on a chunk, **unchecked**.
    ///
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let rows = vec![vec![0, 1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10, 11]];
    /// let mut grid = Grid2D::from_rows(&rows)?;
    /// grid.set_chunk_unchecked(2, 0, 1, &[222, 333]);
    /// assert_eq!(grid.get_chunk_unchecked(4, 0, 0), vec![0, 1, 222, 333]);
    /// # Ok(()) }
    /// ```
    /// # Panics
    /// Panics if the indices are out of bounds.
    pub fn set_chunk_unchecked(
        &mut self,
        chunk_len: usize,
        row: usize,
        col: usize,
        elements: &[T],
    ) {
        let chunk = self.get_chunk_mut_unchecked(chunk_len, row, col);
        for (n, element) in chunk.iter_mut().enumerate() {
            *element = elements[n].clone();
        }
    }
}

fn flatten<T: Clone>(nested: &[Vec<T>]) -> Vec<T> {
    nested.iter().flat_map(|row| row.clone()).collect()
}

impl<T> Index<(usize, usize)> for Grid2D<T> {
    type Output = T;

    /// Returns the element at the given indices, given as `(row, col)`.
    /// ```
    /// # use cuadra::{Grid2D, Result};
    /// # fn main() -> Result<()> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let grid = Grid2D::from_rows(&rows)?;
    /// assert_eq!(grid[(0, 2)], 3);
    /// assert_eq!(grid[(1, 0)], 4);
    /// # Ok(()) }
    /// ```
    ///
    /// # Panics
    /// Panics if the indices are out of bounds.
    ///
    /// ```rust,should_panic
    /// # use cuadra::Grid2D;
    /// let grid = Grid2D::new(32, 2, 3);
    /// let element = grid[(10, 10)];
    /// ```
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        self.get(row, col)
            .unwrap_or_else(|| panic!("Index indices {}, {} out of bounds", row, col))
    }
}

impl<T> IndexMut<(usize, usize)> for Grid2D<T> {
    /// Returns a mutable version of the element at the given indices, given as
    /// `(row, col)`.
    /// ```
    /// # use cuadra::Grid2D;
    /// let mut grid = Grid2D::new(32, 2, 3);
    /// assert_eq!(grid[(0, 0)], 32);
    /// grid[(0, 0)] = 64;
    /// assert_eq!(grid[(0, 0)], 64);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the indices are out of bounds.
    ///
    /// ```rust,should_panic
    /// # use cuadra::Grid2D;
    /// let mut grid = Grid2D::new(32, 2, 3);
    /// grid[(10, 10)] = 16;
    /// ```
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        self.get_mut(row, col)
            .unwrap_or_else(|| panic!("Index mut indices {}, {} out of bounds", row, col))
    }
}

mod std_impls {
    use super::Grid2D;
    use std::{any::type_name, fmt};

    // IMPROVE
    // impl<T> fmt::Display for Grid2D<T> {
    // impl<T: fmt::Debug> fmt::Debug for Grid2D<T> {
    impl<T> fmt::Debug for Grid2D<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Grid2D {{ {}×{}, {} }}",
                self.rows,
                self.cols,
                type_name::<T>()
            )
        }
    }
}
