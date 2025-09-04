/// A column-major matrix with `ROWS` rows and `COLS` columns, and elements of type `T`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mat<const ROWS: usize, const COLS: usize, T = f32> {
    data: [[T; ROWS]; COLS],
}

impl<T, const ROWS: usize, const COLS: usize> Mat<ROWS, COLS, T> {
    /// Create a new matrix with the given data.
    ///
    /// # Example
    /// ```
    /// use mats::Mat;
    ///
    /// let m = Mat::new([[1.0, 2.0], [3.0, 4.0]]);
    /// ```
    #[inline]
    pub const fn new(data: [[T; ROWS]; COLS]) -> Self {
        Self { data }
    }

    /// Return a reference to the raw data of the matrix.
    ///
    /// # Example
    /// ```
    /// use mats::Mat;
    ///
    /// let m = Mat::new([[1.0, 2.0], [3.0, 4.0]]);
    /// let data = m.raw_data();
    /// assert_eq!(data, &[[1.0, 2.0], [3.0, 4.0]]);
    /// ```
    #[inline]
    pub const fn raw_data(&self) -> &[[T; ROWS]; COLS] {
        &self.data
    }
}

impl<T, const ROWS: usize, const COLS: usize> Mat<ROWS, COLS, T>
where
    T: Copy,
{
    /// Create a new matrix with all elements initialized to the given value.
    ///
    /// # Example
    /// ```
    /// use mats::Mat;
    ///
    /// let m = Mat::<2, 2>::init(0.0);
    ///
    /// assert_eq!(m.raw_data(), &[[0.0, 0.0], [0.0, 0.0]]);
    /// ```
    #[inline]
    pub const fn init(value: T) -> Self {
        Self {
            data: [[value; ROWS]; COLS],
        }
    }

    /// Fill the matrix with the given value.
    ///
    /// # Example
    /// ```
    /// use mats::Mat;
    ///
    /// let mut m = Mat::new([[1.0, 2.0], [3.0, 4.0]]);
    /// m.fill(1.0);
    ///
    /// assert_eq!(m.raw_data(), &[[1.0, 1.0], [1.0, 1.0]]);
    /// ```
    #[inline]
    pub fn fill(&mut self, value: T) {
        for i in 0..COLS {
            for j in 0..ROWS {
                self.data[i][j] = value;
            }
        }
    }
}

// Implement trait Default for Mat
impl<T, const ROWS: usize, const COLS: usize> Default for Mat<ROWS, COLS, T>
where
    T: Default + Copy,
{
    /// Create a new matrix with all elements initialized to the default value of `T`.
    ///
    /// # Example
    /// ```
    /// use mats::Mat;
    ///
    /// let m = Mat::<2, 2, f32>::default();
    ///
    /// assert_eq!(m.raw_data(), &[[f32::default(), f32::default()], [f32::default(), f32::default()]]);
    /// ```
    #[inline]
    fn default() -> Self {
        Self {
            data: [[T::default(); ROWS]; COLS],
        }
    }
}

impl<T, const ROWS: usize, const COLS: usize> From<[[T; ROWS]; COLS]> for Mat<ROWS, COLS, T> {
    /// Convert a 2D array into a matrix.
    ///
    /// # Example
    /// ```
    /// use mats::Mat;
    ///
    /// let arr = [[1.0, 2.0], [3.0, 4.0]];
    /// let m: Mat<2, 2, f32> = arr.into();
    ///
    /// assert_eq!(m.raw_data(), &arr);
    /// ```
    fn from(value: [[T; ROWS]; COLS]) -> Self {
        Self { data: value }
    }
}

impl<T, const SIZE: usize> From<[T; SIZE]> for Mat<SIZE, 1, T> {
    /// Convert a 1D array into a column vector.
    ///
    /// # Example
    /// ```
    /// use mats::Mat;
    ///
    /// let arr = [1.0, 2.0, 3.0, 4.0];
    /// let m: Mat<4, 1, f32> = arr.into();
    ///
    /// assert_eq!(m.raw_data(), &[[1.0, 2.0, 3.0, 4.0]]);
    /// ```
    fn from(value: [T; SIZE]) -> Self {
        Self { data: [value] }
    }
}

mod base;
mod math;
mod ops;
mod traits;
mod types;
mod utils;

pub use traits::*;
pub use types::*;
pub use utils::*;

/// `#[cfg(feature = "graphics")]`: Module `graphics` provides a range
/// of facilities or tools for computer graphics.
/// 
/// When the `graphics` feature is enabled, this module will be available.
#[cfg(feature = "graphics")]
pub mod graphics;