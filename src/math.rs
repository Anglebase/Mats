use crate::{Mat, Zero, types::Vec3};

impl<T, const ROWS: usize, const COLS: usize> Mat<ROWS, COLS, T>
where
    T: Copy + Zero + std::ops::AddAssign + std::ops::Mul<Output = T>,
{
    /// Computes the dot product of two matrices.
    ///
    /// The result is a new matrix with `ROWS` rows and `OTHER_COLS` columns, where
    /// `OTHER_COLS` is the number of columns in the second matrix.
    ///
    /// # Example
    ///
    /// ```
    /// use mats::Mat;
    ///
    /// let a = Mat::<3, 2, i32>::new([[1, 3, -4], [-2, 0, 5]]);
    /// let b = Mat::<2, 2, i32>::new([[-7, 9], [8, 10]]);
    /// let c = a.dot(&b);
    ///
    /// assert_eq!(c, Mat::new([[-25, -21, 73], [-12, 24, 18]]));
    /// ```
    pub fn dot<const OTHER_COLS: usize>(
        &self,
        other: &Mat<COLS, OTHER_COLS, T>,
    ) -> Mat<ROWS, OTHER_COLS, T> {
        let mut result = Mat {
            data: [[T::zero(); ROWS]; OTHER_COLS],
        };
        for i in 0..ROWS {
            for j in 0..OTHER_COLS {
                for k in 0..COLS {
                    result.data[j][i] += self.data[k][i] * other.data[j][k];
                }
            }
        }
        result
    }
}

impl<T> crate::types::Vec3<T>
where
    T: Copy + std::ops::Sub<Output = T> + std::ops::Mul<Output = T>,
{
    /// Computes the cross product of two vectors.
    ///
    /// The result is a new vector
    ///
    /// # Example
    /// ```
    /// use mats::Vec3;
    ///
    /// let a = Vec3::new([[1, 2, 3]]);
    /// let b = Vec3::new([[4, 5, 6]]);
    /// let c = a.cross(&b);
    ///
    /// assert_eq!(c, Vec3::new([[-3, 6, -3]]));
    /// ```
    pub fn cross(&self, other: &crate::types::Vec3<T>) -> crate::types::Vec3<T> {
        Vec3::new([[
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        ]])
    }
}

impl<T, const ROWS: usize, const COLS: usize> Mat<ROWS, COLS, T>
where
    T: Copy,
{
    /// Computes the transpose of the matrix.
    ///
    /// # Example
    /// ```
    /// use mats::Mat;
    ///
    /// let a = Mat::<3, 2, i32>::new([[1, 3, -4], [-2, 0, 5]]);
    /// let b = a.transpose();
    ///
    /// assert_eq!(b, Mat::new([[1, -2], [3, 0], [-4, 5]]));
    /// ```
    pub fn transpose(&self) -> Mat<COLS, ROWS, T> {
        let mut result: Mat<COLS, ROWS, T> =
            unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        for i in 0..ROWS {
            for j in 0..COLS {
                result.data[i][j] = self.data[j][i];
            }
        }
        result
    }

    /// Computes the transpose of the matrix.
    ///
    /// This is a shorthand for `Mat::transpose()`.
    ///
    /// # Example
    /// ```
    /// use mats::Mat;
    ///
    /// let a = Mat::<3, 2, i32>::new([[1, 3, -4], [-2, 0, 5]]);
    /// let b = a.T();
    ///
    /// assert_eq!(b, Mat::new([[1, -2], [3, 0], [-4, 5]]));
    /// ```
    #[allow(non_snake_case)]
    #[inline]
    pub fn T(&self) -> Mat<COLS, ROWS, T> {
        self.transpose()
    }
}
