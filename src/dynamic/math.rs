use super::{Matrix, MatrixError};
use crate::Zero;

impl<T: Copy> Matrix<T> {
    /// Get the transpose of the matrix.
    ///
    /// # Examples
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let m = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    /// let mt = m.transpose();
    /// assert_eq!(mt, Matrix::new([[1, 4], [2, 5], [3, 6]]));
    /// ```
    #[inline]
    pub fn transpose(&self) -> Self {
        let mut result = unsafe { Matrix::uninit(self.cols, self.rows) };
        for row in 0..self.rows {
            for col in 0..self.cols {
                let si = self.at(row, col);
                let ri = result.at(col, row);
                result.data[ri] = self.data[si];
            }
        }
        result
    }

    /// Get the transpose of the matrix.
    ///
    /// # Note
    /// This is a shorthand for `transpose()`.
    ///
    /// # Examples
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let m = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    /// let mt = m.T();
    /// assert_eq!(mt, Matrix::new([[1, 4], [2, 5], [3, 6]]));
    /// ```
    #[allow(non_snake_case)]
    #[inline]
    pub fn T(&self) -> Self {
        self.transpose()
    }
}

impl<T: Copy> Matrix<T>
where
    T: std::ops::AddAssign + std::ops::Mul<Output = T> + Zero,
{
    /// Multiply two matrices.
    /// 
    /// # Errors
    /// If the number of columns of the first matrix is not equal
    /// to the number of rows of the second matrix.
    ///
    /// # Examples
    /// ```
    /// use mats::dynamic::Matrix;
    ///
    /// let m1 = Matrix::new([[1, 2], [3, 4]]);
    /// let m2 = Matrix::new([[5, 6], [7, 8]]);
    /// let m3 = m1.dot(&m2).unwrap();
    /// assert_eq!(m3, Matrix::new([[19, 22], [43, 50]]));
    /// ```
    pub fn dot(&self, other: &Self) -> super::Result<Self> {
        if self.cols != other.rows {
            return Err(MatrixError::DimensionsNotMatch {
                expected: format!("{}xN", self.cols,),
                actual: format!("{}x_", other.rows),
            });
        }
        let mut result = Matrix::fill(self.rows, other.cols, T::zero());
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    let ri = result.at(j, i);
                    let si = self.at(k, i);
                    let oi = other.at(j, k);
                    result.data[ri] += self.data[si] * other.data[oi];
                }
            }
        }
        Ok(result)
    }
}
