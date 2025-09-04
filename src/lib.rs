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

mod base;
mod ops;
mod math;
mod types;
pub mod graphics;

pub use base::*;
pub use types::*;