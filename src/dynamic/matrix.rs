use crate::{Mat, UnitOne, Zero, dynamic::MatrixError};

/// A dynamic-size column-major matrix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T = f32> {
    pub(super) rows: usize,
    pub(super) cols: usize,
    pub(super) data: Vec<T>,
}

impl<T> Matrix<T> {
    /// Return the actual index position of the elements located in
    /// rows `row` and columns `col` of the matrix in the original data `self.data`.
    #[inline(always)]
    pub(super) const fn at(&self, row: usize, col: usize) -> usize {
        self.rows * col + row
    }

    /// Creates a new matrix from a given array of arrays.
    ///
    /// # Examples
    ///
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let data = [[1, 2, 3], [4, 5, 6]];
    /// let matrix = Matrix::new(data);
    /// ```
    pub fn new<const ROWS: usize, const COLS: usize>(data: [[T; ROWS]; COLS]) -> Self {
        let mut vec = Vec::with_capacity(ROWS * COLS);
        data.into_iter().for_each(|cols| {
            cols.into_iter().for_each(|e| vec.push(e));
        });
        Self {
            rows: ROWS,
            cols: COLS,
            data: vec,
        }
    }

    /// Creates a new matrix with uninitialized data.
    ///
    /// # Safety
    /// The caller must ensure that the data is initialized before using it.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let matrix = unsafe { Matrix::<f32>::uninit(2, 3) };
    /// assert_eq!(matrix.raw_data().len(), 6);
    /// ```
    #[inline]
    pub unsafe fn uninit(rows: usize, cols: usize) -> Self {
        let mut uninit = Vec::with_capacity(rows * cols);
        unsafe { uninit.set_len(rows * cols) };
        Self {
            rows,
            cols,
            data: uninit,
        }
    }

    /// Get the size of the matrix as a tuple of `(rows, cols)`.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!(matrix.size(), (3, 2));
    /// ```
    #[inline(always)]
    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    /// Get the raw data of the matrix as a slice of arrays.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!(matrix.raw_data(), &[1, 2, 3, 4, 5, 6]);
    /// ```
    #[inline(always)]
    pub fn raw_data(&self) -> &[T] {
        &self.data
    }
}

impl<T: Copy> Matrix<T> {
    /// Creates a new matrix filled with a given value.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let matrix = Matrix::fill(3, 2, 1);
    /// assert_eq!(matrix, Matrix::new([[1, 1, 1], [1, 1, 1]]));
    /// ```
    pub fn fill(rows: usize, cols: usize, value: T) -> Self {
        let vec = vec![value; rows * cols];
        Self {
            rows,
            cols,
            data: vec,
        }
    }
}

impl<T, const ROWS: usize, const COLS: usize> From<Mat<ROWS, COLS, T>> for Matrix<T> {
    /// Converts a static-size matrix `Mat` to a dynamic-size matrix `Matrix`.
    ///
    /// # Example
    /// ```
    /// use mats::Mat;
    /// use mats::dynamic::Matrix;
    ///
    /// let mat = Mat::new([[1, 2, 3], [4, 5, 6]]);
    /// let matrix = Matrix::from(mat);
    /// assert_eq!(matrix, Matrix::new([[1, 2, 3], [4, 5, 6]]));
    /// ```
    fn from(value: Mat<ROWS, COLS, T>) -> Self {
        Self::new(value.data)
    }
}

impl<T, const ROWS: usize, const COLS: usize> TryInto<Mat<ROWS, COLS, T>> for Matrix<T> {
    type Error = super::MatrixError;

    /// Try to convert a dynamic-size matrix `Matrix` to a static-size matrix `Mat`.
    ///
    /// # Errors
    /// If the dimensions of two matrices do not match, it will return
    /// a `MatrixError::DimensionsNotMatch` error.
    ///
    /// # Example
    /// ```
    /// use mats::Mat;
    /// use mats::dynamic::Matrix;
    ///
    /// let matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    /// let result = matrix.try_into();
    ///
    /// let result: Mat<3, 2, i32> = result.unwrap();
    /// assert_eq!(result, Mat::new([[1, 2, 3], [4, 5, 6]]));
    /// ```
    fn try_into(self) -> Result<Mat<ROWS, COLS, T>, Self::Error> {
        if self.rows != ROWS || self.cols != COLS {
            Err(MatrixError::DimensionsNotMatch {
                expected: format!("{ROWS}x{COLS}"),
                actual: format!("{}x{}", self.rows, self.cols),
            })
        } else {
            let mut result: Mat<ROWS, COLS, T> =
                unsafe { std::mem::MaybeUninit::uninit().assume_init() };
            self.data.into_iter().enumerate().for_each(|(i, e)| {
                result.data[i / ROWS][i % ROWS] = e;
            });
            Ok(result)
        }
    }
}

impl<T: Copy> Matrix<T>
where
    T: std::ops::Add<Output = T>,
{
    /// Compute the addition of two matrices.
    ///
    /// # Errors
    /// If the dimensions of two matrices do not match, it will return
    /// a `MatrixError::DimensionsNotMatch` error.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let matrix1 = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    /// let matrix2 = Matrix::new([[7, 8, 9], [10, 11, 12]]);
    /// let result = matrix1.add(&matrix2).unwrap();
    /// assert_eq!(result, Matrix::new([[8, 10, 12], [14, 16, 18]]));
    /// ```
    pub fn add(&self, other: &Self) -> super::Result<Self> {
        if self.size() != other.size() {
            Err(MatrixError::DimensionsNotMatch {
                expected: format!("{}x{}", self.rows, self.cols),
                actual: format!("{}x{}", other.rows, other.cols),
            })
        } else {
            let mut result = unsafe { Self::uninit(self.rows, self.cols) };
            result
                .data
                .iter_mut()
                .enumerate()
                .for_each(|(i, e)| *e = self.data[i] + other.data[i]);
            Ok(result)
        }
    }
}

impl<T: Copy> Matrix<T>
where
    T: std::ops::Sub<Output = T>,
{
    /// Compute the subtraction of two matrices.
    ///
    /// # Errors
    /// If the dimensions of two matrices do not match, it will return
    /// a `MatrixError::DimensionsNotMatch` error.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let matrix1 = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    /// let matrix2 = Matrix::new([[7, 8, 9], [10, 11, 12]]);
    /// let result = matrix1.sub(&matrix2).unwrap();
    /// assert_eq!(result, Matrix::new([[-6, -6, -6], [-6, -6, -6]]));
    /// ```
    pub fn sub(&self, other: &Self) -> super::Result<Self> {
        if self.size() != other.size() {
            Err(MatrixError::DimensionsNotMatch {
                expected: format!("{}x{}", self.rows, self.cols),
                actual: format!("{}x{}", other.rows, other.cols),
            })
        } else {
            let mut result = unsafe { Self::uninit(self.rows, self.cols) };
            result
                .data
                .iter_mut()
                .enumerate()
                .for_each(|(i, e)| *e = self.data[i] - other.data[i]);
            Ok(result)
        }
    }
}

impl<T: Copy> Matrix<T>
where
    T: std::ops::Mul<Output = T>,
{
    /// Compute the multiplication of a matrix and a scalar.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    /// let result = matrix.mul(2);
    /// assert_eq!(result, Matrix::new([[2, 4, 6], [8, 10, 12]]));
    /// ```
    pub fn mul(&self, other: T) -> Self {
        let mut result = unsafe { Self::uninit(self.rows, self.cols) };
        result
            .data
            .iter_mut()
            .zip(self.data.iter())
            .for_each(|(r, e)| *r = *e * other);
        result
    }
}

impl<T: Copy> Matrix<T>
where
    T: std::ops::Div<Output = T>,
{
    /// Compute the multiplication of a matrix and a scalar.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let matrix = Matrix::new([[2, 4, 6], [8, 10, 12]]);
    /// let result = matrix.div(2);
    /// assert_eq!(result, Matrix::new([[1, 2, 3], [4, 5, 6]]));
    /// ```
    pub fn div(&self, other: T) -> Self {
        let mut result = unsafe { Self::uninit(self.rows, self.cols) };
        result
            .data
            .iter_mut()
            .zip(self.data.iter())
            .for_each(|(r, e)| *r = *e / other);
        result
    }
}

impl<T: Copy> Matrix<T>
where
    T: std::ops::Neg<Output = T>,
{
    /// Compute the negation of a matrix.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    /// let result = matrix.neg();
    /// assert_eq!(result, Matrix::new([[-1, -2, -3], [-4, -5, -6]]));
    /// ```
    pub fn neg(&self) -> Self {
        let mut result = unsafe { Self::uninit(self.rows, self.cols) };
        result
            .data
            .iter_mut()
            .zip(self.data.iter())
            .for_each(|(r, e)| *r = -*e);
        result
    }
}

impl<T: Copy> Matrix<T>
where
    T: std::ops::AddAssign,
{
    /// Compute the addition of two matrices and assign the result to the left-hand matrix.
    ///
    /// # Errors
    /// If the dimensions of two matrices do not match, it will return
    /// a `MatrixError::DimensionsNotMatch` error.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let mut matrix1 = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    /// let matrix2 = Matrix::new([[7, 8, 9], [10, 11, 12]]);
    /// matrix1.add_assign(&matrix2).unwrap();
    /// assert_eq!(matrix1, Matrix::new([[8, 10, 12], [14, 16, 18]]));
    /// ```
    pub fn add_assign(&mut self, other: &Self) -> super::Result<()> {
        if self.size() != other.size() {
            Err(MatrixError::DimensionsNotMatch {
                expected: format!("{}x{}", self.rows, self.cols),
                actual: format!("{}x{}", other.rows, other.cols),
            })
        } else {
            self.data
                .iter_mut()
                .zip(other.data.iter())
                .for_each(|(r, e)| *r += *e);
            Ok(())
        }
    }
}

impl<T: Copy> Matrix<T>
where
    T: std::ops::SubAssign,
{
    /// Compute the subtraction of two matrices and assign the result to the left-hand matrix.
    ///
    /// # Errors
    /// If the dimensions of two matrices do not match, it will return
    /// a `MatrixError::DimensionsNotMatch` error.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let mut matrix1 = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    /// let matrix2 = Matrix::new([[7, 8, 9], [10, 11, 12]]);
    /// matrix1.sub_assign(&matrix2).unwrap();
    /// assert_eq!(matrix1, Matrix::new([[-6, -6, -6], [-6, -6, -6]]));
    /// ```
    pub fn sub_assign(&mut self, other: &Self) -> super::Result<()> {
        if self.size() != other.size() {
            Err(MatrixError::DimensionsNotMatch {
                expected: format!("{}x{}", self.rows, self.cols),
                actual: format!("{}x{}", other.rows, other.cols),
            })
        } else {
            self.data
                .iter_mut()
                .zip(other.data.iter())
                .for_each(|(r, e)| *r -= *e);
            Ok(())
        }
    }
}

impl<T: Copy> Matrix<T>
where
    T: std::ops::MulAssign,
{
    /// Compute the multiplication of a matrix and a scalar and assign the result to the left-hand matrix.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let mut matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    /// matrix.mul_assign(2);
    /// assert_eq!(matrix, Matrix::new([[2, 4, 6], [8, 10, 12]]));
    /// ```
    #[inline]
    pub fn mul_assign(&mut self, other: T) {
        self.data.iter_mut().for_each(|e| *e *= other);
    }
}

impl<T: Copy> Matrix<T>
where
    T: std::ops::DivAssign,
{
    /// Compute the division of a matrix and a scalar and assign the result to the left-hand matrix.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let mut matrix = Matrix::new([[2, 4, 6], [8, 10, 12]]);
    /// matrix.div_assign(2);
    /// assert_eq!(matrix, Matrix::new([[1, 2, 3], [4, 5, 6]]));
    /// ```
    #[inline]
    pub fn div_assign(&mut self, other: T) {
        self.data.iter_mut().for_each(|e| *e /= other);
    }
}

impl<T> Matrix<T> {
    /// Get the reference of the element at the specified row and column.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!(matrix.get(2, 1), Some(&6));
    /// ```
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows || col < self.cols {
            Some(&self.data[self.rows * col + row])
        } else {
            None
        }
    }

    /// Get the mutable reference of the element at the specified row and column.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let mut matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    /// if let Some(elem) = matrix.get_mut(2, 1) {
    ///     *elem = 10;
    /// }
    /// assert_eq!(matrix, Matrix::new([[1, 2, 3], [4, 5, 10]]));
    /// ```
    #[inline]
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row < self.rows || col < self.cols {
            Some(&mut self.data[self.rows * col + row])
        } else {
            None
        }
    }
}

impl<T: Copy + UnitOne + Zero> Matrix<T> {
    /// Create a new identity matrix with the specified dimensions.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let matrix = Matrix::new_identity(3, 3);
    /// assert_eq!(matrix, Matrix::new([[1, 0, 0], [0, 1, 0], [0, 0, 1]]));
    /// ```
    #[inline]
    pub fn new_identity(size: usize) -> Self {
        let mut result = Self::fill(size, size, T::zero());
        for i in 0..size {
            result.data[size * i + i] = T::unit_one();
        }
        result
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn I(size: usize) -> Self {
        Self::new_identity(size)
    }
}
