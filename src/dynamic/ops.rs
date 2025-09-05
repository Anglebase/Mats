use super::Matrix;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

impl<T: Copy + Add<Output = T>> Add for Matrix<T> {
    type Output = Self;

    /// Operator overloading for addition of two matrices.
    ///
    /// # Panics
    /// If the dimensions of two matrices do not match, it will panic.
    ///
    /// # Note
    /// This method is same as `Matrix::add` method. But it is more recommended
    /// to use the latter. It will acquire ownership of the data.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let matrix1 = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    /// let matrix2 = Matrix::new([[7, 8, 9], [10, 11, 12]]);
    /// let result = matrix1 + matrix2;
    /// assert_eq!(result, Matrix::new([[8, 10, 12], [14, 16, 18]]));
    /// ```
    fn add(self, other: Self) -> Self::Output {
        (&self).add(&other).unwrap()
    }
}

impl<T: Copy + AddAssign> AddAssign for Matrix<T> {
    /// Operator overloading for addition of two matrices and assign the result to the left-hand matrix.
    ///
    /// # Panics
    /// If the dimensions of two matrices do not match, it will panic.
    ///
    /// # Note
    /// This method is same as `Matrix::add_assign` method. But it is more recommended
    /// to use the latter. It will acquire ownership of the data.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let mut matrix1 = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    /// let matrix2 = Matrix::new([[7, 8, 9], [10, 11, 12]]);
    /// matrix1 += matrix2;
    /// assert_eq!(matrix1, Matrix::new([[8, 10, 12], [14, 16, 18]]));
    /// ```
    fn add_assign(&mut self, other: Self) {
        self.add_assign(&other).unwrap()
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Matrix<T> {
    type Output = Self;
    /// Operator overloading for subtraction of two matrices.
    ///
    /// # Panics
    /// If the dimensions of two matrices do not match, it will panic.
    ///
    /// # Note
    /// This method is same as `Matrix::sub` method. But it is more recommended
    /// to use the latter. It will acquire ownership of the data.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let matrix1 = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    /// let matrix2 = Matrix::new([[7, 8, 9], [10, 11, 12]]);
    /// let result = matrix1 - matrix2;
    /// assert_eq!(result, Matrix::new([[-6, -6, -6], [-6, -6, -6]]));
    /// ```
    fn sub(self, other: Self) -> Self::Output {
        (&self).sub(&other).unwrap()
    }
}

impl<T: Copy + SubAssign> SubAssign for Matrix<T> {
    /// Operator overloading for subtraction of two matrices and assign the result to the left-hand matrix.
    ///
    /// # Panics
    /// If the dimensions of two matrices do not match, it will panic.
    ///
    /// # Note
    /// This method is same as `Matrix::sub_assign` method. But it is more recommended
    /// to use the latter. It will acquire ownership of the data.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let mut matrix1 = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    /// let matrix2 = Matrix::new([[7, 8, 9], [10, 11, 12]]);
    /// matrix1 -= matrix2;
    /// assert_eq!(matrix1, Matrix::new([[-6, -6, -6], [-6, -6, -6]]));
    /// ```
    fn sub_assign(&mut self, other: Self) {
        self.sub_assign(&other).unwrap()
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Matrix<T> {
    type Output = Self;
    /// Operator overloading for multiplication of a matrix and a scalar.
    ///
    /// # Note
    /// This method is same as `Matrix::mul` method. But it is more recommended
    /// to use the latter. It will acquire ownership of the data.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    /// let result = matrix * 2;
    /// assert_eq!(result, Matrix::new([[2, 4, 6], [8, 10, 12]]));
    /// ```
    fn mul(self, other: T) -> Self::Output {
        (&self).mul(other)
    }
}

impl<T: Copy + MulAssign> MulAssign<T> for Matrix<T> {
    /// Operator overloading for multiplication of a matrix and a scalar and assign the result to the left-hand matrix.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let mut matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    /// matrix *= 2;
    /// assert_eq!(matrix, Matrix::new([[2, 4, 6], [8, 10, 12]]));
    /// ```
    fn mul_assign(&mut self, other: T) {
        self.mul_assign(other)
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for Matrix<T> {
    type Output = Self;
    /// Operator overloading for division of a matrix and a scalar.
    ///
    /// # Note
    /// This method is same as `Matrix::div` method. But it is more recommended
    /// to use the latter. It will acquire ownership of the data.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let matrix = Matrix::new([[2, 4, 6], [8, 10, 12]]);
    /// let result = matrix / 2;
    /// assert_eq!(result, Matrix::new([[1, 2, 3], [4, 5, 6]]));
    /// ```
    fn div(self, other: T) -> Self::Output {
        (&self).div(other)
    }
}

impl<T: Copy + DivAssign> DivAssign<T> for Matrix<T> {
    /// Operator overloading for division of a matrix and a scalar and assign the result to the left-hand matrix.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let mut matrix = Matrix::new([[2, 4, 6], [8, 10, 12]]);
    /// matrix /= 2;
    /// assert_eq!(matrix, Matrix::new([[1, 2, 3], [4, 5, 6]]));
    /// ```
    fn div_assign(&mut self, other: T) {
        self.div_assign(other)
    }
}

impl<T: Copy + Neg<Output = T>> Neg for Matrix<T> {
    type Output = Self;
    /// Operator overloading for negation of a matrix.
    ///
    /// # Note
    /// This method is same as `Matrix::neg` method. But it is more recommended
    /// to use the latter. It will acquire ownership of the data.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    /// let result = -matrix;
    /// assert_eq!(result, Matrix::new([[-1, -2, -3], [-4, -5, -6]]));
    /// ```
    fn neg(self) -> Self::Output {
        (&self).neg()
    }
}

use std::ops::{Index, IndexMut};

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    /// Operator overloading for indexing a matrix.
    ///
    /// # Panics
    /// If the index is out of bounds, it will panic.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!(matrix[(0, 0)], 1);
    /// assert_eq!(matrix[(2, 1)], 6);
    /// ```
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        self.get(row, col).unwrap()
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    /// Operator overloading for indexing a mutable matrix.
    ///
    /// # Panics
    /// If the index is out of bounds, it will panic.
    ///
    /// # Example
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let mut matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    /// matrix[(0, 0)] = 10;
    /// matrix[(2, 1)] = 60;
    /// assert_eq!(matrix, Matrix::new([[10, 2, 3], [4, 5, 60]]));
    /// ```
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        self.get_mut(row, col).unwrap()
    }
}
