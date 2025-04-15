/// A static size matrix of row primary order for an M x N.
///
/// # Generic Parameters
///
/// - `T` : The type of matrix element
/// - `M` : The number of rows of the matrix
/// - `N` : The number of columns in the matrix
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Mat<T, const M: usize, const N: usize> {
    data: [[T; N]; M],
}

impl<T, const M: usize, const N: usize> Mat<T, M, N>
where
    T: Copy + PartialOrd + From<f32>,
    T: Sub<Output = T> + Neg<Output = T>,
{
    /// Matrix equality comparisons that allow floating-point errors.
    pub fn eq_with_epsilon(&self, other: &Self, epsilon: T) -> bool {
        for (this, other) in self.data.iter().zip(other.data.iter()) {
            for (a, b) in this.iter().zip(other.iter()) {
                let diff = *a - *b;
                let diff = if diff > T::from(0.0) { diff } else { -diff };
                if diff > epsilon {
                    return false;
                }
            }
        }
        true
    }
}

use std::fmt::Debug;
impl<T, const M: usize, const N: usize> Debug for Mat<T, M, N>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Mat<{}, {:?}, {:?}> {{",
            std::any::type_name::<T>(),
            M,
            N
        )?;
        for i in 0..M {
            write!(f, "| ")?;
            for j in 0..N {
                write!(f, "{:^9?}", self.data[i][j])?;
            }
            writeln!(f, " |")?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T, const M: usize, const N: usize> From<[[T; N]; M]> for Mat<T, M, N> {
    fn from(value: [[T; N]; M]) -> Self {
        Self { data: value }
    }
}

impl<T, const M: usize, const N: usize> Into<[[T; N]; M]> for Mat<T, M, N> {
    fn into(self) -> [[T; N]; M] {
        self.data
    }
}

impl<T, const M: usize, const N: usize> Mat<T, M, N>
where
    T: Copy,
{
    /// Create a matrix of M x N with the given value
    ///
    /// # Example
    ///
    /// ```rust
    /// use mats::Mat;
    ///
    /// let mat1 = Mat::<f32, 2,3>::new_with_init(1.0);
    /// let mat2 = Mat::from([[1.0f32;3];2]);
    ///
    /// assert_eq!(mat1, mat2);
    /// ```
    pub fn new_with_init(init: T) -> Self {
        Self {
            data: [[init; N]; M],
        }
    }
}

impl<T, const M: usize, const N: usize> Mat<T, M, N>
where
    T: Default + Copy,
{
    /// Create a matrix of M x N with default values of T::default()
    ///
    /// # Example
    ///
    /// ```rust
    /// use mats::Mat;
    ///
    /// let mat1 = Mat::<f32, 2,3>::new();
    /// let mat2 = Mat::from([[0.0f32;3];2]);
    ///
    /// assert_eq!(mat1, mat2);
    /// ```
    pub fn new() -> Self {
        Self::new_with_init(T::default())
    }
}

impl<T, const M: usize, const N: usize> Default for Mat<T, M, N>
where
    T: Copy + Default,
{
    fn default() -> Self {
        Self {
            data: [[T::default(); N]; M],
        }
    }
}

use std::ops::Add;
impl<T, const M: usize, const N: usize> Add for Mat<T, M, N>
where
    T: Copy + Add<Output = T> + Default,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::new();
        result
            .data
            .iter_mut()
            .zip(self.data.iter().zip(rhs.data.iter()))
            .for_each(|(res, (a, b))| {
                res.iter_mut()
                    .zip(a.iter().zip(b.iter()))
                    .for_each(|(res, (a, b))| *res = *a + *b)
            });
        result
    }
}

use std::ops::Sub;
impl<T, const M: usize, const N: usize> Sub for Mat<T, M, N>
where
    T: Copy + Sub<Output = T> + Default,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Self::new();
        result
            .data
            .iter_mut()
            .zip(self.data.iter().zip(rhs.data.iter()))
            .for_each(|(res, (a, b))| {
                res.iter_mut()
                    .zip(a.iter().zip(b.iter()))
                    .for_each(|(res, (a, b))| *res = *a - *b)
            });
        result
    }
}

use std::ops::Mul;
impl<T, const M: usize, const N: usize> Mul<T> for Mat<T, M, N>
where
    T: Copy + Mul<Output = T> + Default,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut result = Self::new();
        result
            .data
            .iter_mut()
            .zip(self.data.iter())
            .for_each(|(res, a)| {
                res.iter_mut()
                    .zip(a.iter())
                    .for_each(|(res, a)| *res = *a * rhs)
            });
        result
    }
}

use std::ops::Div;
impl<T, const M: usize, const N: usize> Div<T> for Mat<T, M, N>
where
    T: Copy + Div<Output = T> + Default,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let mut result = Self::new();
        result
            .data
            .iter_mut()
            .zip(self.data.iter())
            .for_each(|(res, a)| {
                res.iter_mut()
                    .zip(a.iter())
                    .for_each(|(res, a)| *res = *a / rhs)
            });
        result
    }
}

use std::ops::AddAssign;
impl<T, const M: usize, const N: usize> AddAssign for Mat<T, M, N>
where
    T: Copy + Default + AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.data
            .iter_mut()
            .zip(rhs.data.iter())
            .for_each(|(lhs, rhs)| {
                lhs.iter_mut()
                    .zip(rhs.iter())
                    .for_each(|(lhs, rhs)| *lhs += *rhs);
            });
    }
}

use std::ops::SubAssign;
impl<T, const M: usize, const N: usize> SubAssign for Mat<T, M, N>
where
    T: Copy + Default + SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.data
            .iter_mut()
            .zip(rhs.data.iter())
            .for_each(|(lhs, rhs)| {
                lhs.iter_mut()
                    .zip(rhs.iter())
                    .for_each(|(lhs, rhs)| *lhs -= *rhs);
            });
    }
}

use std::ops::MulAssign;
impl<T, const M: usize, const N: usize> MulAssign<T> for Mat<T, M, N>
where
    T: Copy + Default + MulAssign,
{
    fn mul_assign(&mut self, rhs: T) {
        self.data.iter_mut().for_each(|lhs| {
            lhs.iter_mut().for_each(|lhs| *lhs *= rhs);
        });
    }
}

use std::ops::DivAssign;
impl<T, const M: usize, const N: usize> DivAssign<T> for Mat<T, M, N>
where
    T: Copy + Default + DivAssign,
{
    fn div_assign(&mut self, rhs: T) {
        self.data.iter_mut().for_each(|lhs| {
            lhs.iter_mut().for_each(|lhs| *lhs /= rhs);
        });
    }
}

use std::ops::Index;
impl<T, const M: usize, const N: usize> Index<usize> for Mat<T, M, N> {
    type Output = [T; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

use std::ops::IndexMut;
impl<T, const M: usize, const N: usize> IndexMut<usize> for Mat<T, M, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

use std::ops::Neg;
impl<T, const M: usize, const N: usize> Neg for Mat<T, M, N>
where
    T: Copy + Neg<Output = T> + Default,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut result = Self::new();
        for i in 0..M {
            for j in 0..N {
                result.data[i][j] = -self.data[i][j];
            }
        }
        result
    }
}

impl<T, const M: usize, const K: usize, const N: usize> Mul<Mat<T, K, N>> for Mat<T, M, K>
where
    T: Copy + Default,
    T: Mul<Output = T> + AddAssign,
{
    type Output = Mat<T, M, N>;

    fn mul(self, rhs: Mat<T, K, N>) -> Self::Output {
        let mut result = Mat::<T, M, N>::new();
        for i in 0..M {
            for j in 0..N {
                for k in 0..K {
                    result.data[i][j] += self.data[i][k] * rhs.data[k][j];
                }
            }
        }
        result
    }
}

impl<T, const M: usize, const N: usize> Mat<T, M, N>
where
    T: Copy + Default,
{
    /// Generates a transpose matrix for the current matrix
    #[allow(non_snake_case)]
    pub fn T(&self) -> Mat<T, N, M> {
        let mut result = Mat::<T, N, M>::new();
        result.data.iter_mut().enumerate().for_each(|(i, r)| {
            r.iter_mut().enumerate().for_each(|(j, c)| {
                *c = self.data[j][i];
            });
        });
        result
    }

    /// It is same as `T()` method.
    pub fn transpose(&self) -> Mat<T, N, M> {
        self.T()
    }
}

impl<T, const M: usize> Mat<T, M, M>
where
    T: Copy + From<f32>,
{
    /// Generate an identity matrix
    #[allow(non_snake_case)]
    pub fn I() -> Self {
        let mut result = Self {
            data: [[0.0.into(); M]; M],
        };
        result.data.iter_mut().enumerate().for_each(|(i, v)| {
            v.iter_mut().enumerate().for_each(|(j, v)| {
                if i == j {
                    *v = T::from(1.0);
                }
            });
        });
        result
    }

    /// It is same as `I()` method.
    pub fn identity() -> Self {
        Self::I()
    }
}

impl<T, const M: usize, const N: usize> Mat<T, M, N> {
    /// Get the number of rows of the matrix.
    pub const fn rows(&self) -> usize {
        M
    }
    /// Get the number of columns of the matrix.
    pub const fn cols(&self) -> usize {
        N
    }
    /// Get the number of elements of the matrix.
    pub const fn count(&self) -> usize {
        M * N
    }
}

impl<T, const M: usize, const N: usize> Mat<T, M, N>
where
    T: Copy + Default,
{
    /// Swap two rows of the matrix.
    ///
    /// # Parameters
    /// - `i`,`j` : The row index which to be swapped.
    pub fn swap_row(&mut self, i: usize, j: usize) {
        self.data.swap(i, j);
    }

    /// Swap two columns of the matrix.
    ///
    /// # Parameters
    /// - `i`,`j` : The column index which to be swapped.
    pub fn swap_col(&mut self, i: usize, j: usize) {
        for row in self.data.iter_mut() {
            row.swap(i, j);
        }
    }
}
