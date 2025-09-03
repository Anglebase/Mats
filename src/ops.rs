use crate::{Mat, Zero};

impl<T, const ROWS: usize, const COLS: usize> std::ops::Add<Self> for Mat<ROWS, COLS, T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Output = Self;

    /// Matrix addition.
    ///
    /// # Examples
    /// ```
    /// use mats::Mat;
    ///
    /// let a = Mat::new([[1.0, 2.0], [3.0, 4.0]]);
    /// let b = Mat::new([[5.0, 6.0], [7.0, 8.0]]);
    /// let c = a + b;
    ///
    /// assert_eq!(c, Mat::new([[6.0, 8.0], [10.0, 12.0]]));
    /// ```
    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..COLS {
            for j in 0..ROWS {
                self.data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }
        self
    }
}

impl<T, const ROWS: usize, const COLS: usize> std::ops::Sub<Self> for Mat<ROWS, COLS, T>
where
    T: std::ops::Sub<Output = T> + Copy,
{
    type Output = Self;

    /// Matrix subtraction.
    ///
    /// # Examples
    /// ```
    /// use mats::Mat;
    ///
    /// let a = Mat::new([[1.0, 2.0], [3.0, 4.0]]);
    /// let b = Mat::new([[5.0, 6.0], [7.0, 8.0]]);
    /// let c = a - b;
    ///
    /// assert_eq!(c, Mat::new([[-4.0, -4.0], [-4.0, -4.0]]));
    /// ```
    fn sub(mut self, rhs: Self) -> Self::Output {
        for i in 0..COLS {
            for j in 0..ROWS {
                self.data[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }
        self
    }
}

impl<T, const ROWS: usize, const COLS: usize> std::ops::Mul<T> for Mat<ROWS, COLS, T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Self;

    /// Matrix multiplication by a scalar.
    ///
    /// # Examples
    /// ```
    /// use mats::Mat;
    ///
    /// let a = Mat::new([[1.0, 2.0], [3.0, 4.0]]);
    /// let b = 2.0;
    /// let c = a * b;
    ///
    /// assert_eq!(c, Mat::new([[2.0, 4.0], [6.0, 8.0]]));
    /// ```
    fn mul(mut self, rhs: T) -> Self::Output {
        for i in 0..COLS {
            for j in 0..ROWS {
                self.data[i][j] = self.data[i][j] * rhs;
            }
        }
        self
    }
}

impl<T, const ROWS: usize, const COLS: usize> std::ops::Div<T> for Mat<ROWS, COLS, T>
where
    T: std::ops::Div<Output = T> + Copy,
{
    type Output = Self;

    /// Matrix division by a scalar.
    ///
    /// # Examples
    /// ```
    /// use mats::Mat;
    ///
    /// let a = Mat::new([[1.0, 2.0], [3.0, 4.0]]);
    /// let b = 2.0;
    /// let c = a / b;
    ///
    /// assert_eq!(c, Mat::new([[0.5, 1.0], [1.5, 2.0]]));
    /// ```
    fn div(mut self, rhs: T) -> Self::Output {
        for i in 0..COLS {
            for j in 0..ROWS {
                self.data[i][j] = self.data[i][j] / rhs;
            }
        }
        self
    }
}

impl<T, const ROWS: usize, const COLS: usize> std::ops::Neg for Mat<ROWS, COLS, T>
where
    T: std::ops::Neg<Output = T> + Copy,
{
    type Output = Self;

    /// Matrix negation.
    ///
    /// # Examples
    /// ```
    /// use mats::Mat;
    ///
    /// let a = Mat::new([[1.0, 2.0], [3.0, 4.0]]);
    /// let b = -a;
    ///
    /// assert_eq!(b, Mat::new([[-1.0, -2.0], [-3.0, -4.0]]));
    /// ```
    fn neg(mut self) -> Self::Output {
        for i in 0..COLS {
            for j in 0..ROWS {
                self.data[i][j] = -self.data[i][j];
            }
        }
        self
    }
}

impl<T, const ROWS: usize, const COLS: usize, const OTHER_COLS: usize>
    std::ops::Mul<Mat<COLS, OTHER_COLS, T>> for Mat<ROWS, COLS, T>
where
    T: Copy + Zero + std::ops::AddAssign + std::ops::Mul<Output = T>,
{
    type Output = Mat<ROWS, OTHER_COLS, T>;

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
    ///
    /// let c = a * b;
    ///
    /// assert_eq!(c, Mat::new([[-25, -21, 73], [-12, 24, 18]]));
    /// ```
    fn mul(self, rhs: Mat<COLS, OTHER_COLS, T>) -> Self::Output {
        self.dot(&rhs)
    }
}

impl<T, const ROWS: usize, const COLS: usize> std::ops::Index<(usize, usize)>
    for Mat<ROWS, COLS, T>
{
    type Output = T;

    /// Indexing into the matrix.
    ///
    /// # Examples
    /// ```
    /// use mats::Mat;
    ///
    /// let a = Mat::new([[1.0, 2.0], [3.0, 4.0]]);
    ///
    /// assert_eq!(a[(0, 0)], 1.0);
    /// assert_eq!(a[(1, 0)], 3.0);
    /// assert_eq!(a[(0, 1)], 2.0);
    /// assert_eq!(a[(1, 1)], 4.0);
    /// ```
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row][col]
    }
}

impl<T, const SIZE: usize> std::ops::Index<usize> for Mat<SIZE, 1, T> {
    type Output = T;

    /// Indexing into the vector.
    /// 
    /// # Examples
    /// ```
    /// use mats::Mat;
    ///
    /// let a = Mat::new([[1.0, 2.0, 3.0]]);
    ///
    /// assert_eq!(a[0], 1.0);
    /// assert_eq!(a[1], 2.0);
    /// assert_eq!(a[2], 3.0);
    /// ```
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[0][index]
    }
}
