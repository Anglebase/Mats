use crate::Mat;

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