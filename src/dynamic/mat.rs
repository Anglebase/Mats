/// A dynamic size matrix of row primary order for an M x N.
/// 
/// It has the same memory layout as `Mat<T, M, N>`, and can also
/// interact with the API by taking the address of the element `Mat[(0,0)]`.
///
/// # Generic Parameters
///
/// - `T` : The type of matrix element
#[derive(Clone, Eq)]
pub struct Mat<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Mat<T> {
    /// Get the number of rows of the matrix.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Get the number of columns for the matrix.
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Get the number of elements of the matrix.
    pub fn count(&self) -> usize {
        self.rows * self.cols
    }

    /// Determine if the matrix is a phalanx or not.
    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    /// Determine whether the two matrices are the same size.
    pub fn is_same_size_with(&self, other: &Mat<T>) -> bool {
        self.rows == other.rows && self.cols == other.cols
    }
}

impl<T> Mat<T> {
    // Creates a matrix of the specified size.
    pub fn new(rows: usize, cols: usize) -> Self {
        let data = Vec::with_capacity(rows * cols);
        Mat { data, rows, cols }
    }
}

impl<T: Copy> Mat<T> {
    // Create a matrix of the specified size and initialize it with the specified value.
    pub fn new_with_init(rows: usize, cols: usize, init: T) -> Self {
        let mut data = Vec::with_capacity(rows * cols);
        for _ in 0..rows * cols {
            data.push(init);
        }
        Mat { data, rows, cols }
    }
}

use std::fmt::Debug;
impl<T> Debug for Mat<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Mat<{}>({:?}, {:?}) {{",
            std::any::type_name::<T>(),
            self.rows,
            self.cols
        )?;
        for i in 0..self.rows {
            write!(f, "| ")?;
            for j in 0..self.cols {
                write!(f, "{:^9?}", self.data[i * self.cols + j])?;
            }
            writeln!(f, " |")?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T: PartialEq> PartialEq for Mat<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.rows != other.rows || self.cols != other.cols {
            return false;
        }
        for i in 0..self.count() {
            if self.data[i] != other.data[i] {
                return false;
            }
        }
        true
    }
}

impl<T> Mat<T>
where
    T: Copy + PartialOrd,
    T: Neg<Output = T> + Sub<Output = T>,
{
    /// Equality judgment function for allowable errors.
    pub fn eq_with_epsilon(&self, other: &Self, epsilon: T) -> bool {
        if self.rows != other.rows || self.cols != other.cols {
            return false;
        }
        for i in 0..self.count() {
            let diff = self.data[i] - other.data[i];
            if -epsilon > diff || diff > epsilon {
                return false;
            }
        }
        true
    }
}

use std::ops::AddAssign;
use std::ops::Index;
impl<'a, T> Index<(usize, usize)> for Mat<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.cols + index.1]
    }
}

use std::ops::IndexMut;
impl<'a, T> IndexMut<(usize, usize)> for Mat<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 * self.cols + index.1]
    }
}

use std::ops::Add;
impl<T: Copy + Add<Output = T>> Add for Mat<T> {
    type Output = Result<Mat<T>, &'static str>;

    fn add(self, other: Self) -> Self::Output {
        if self.rows != other.rows || self.cols != other.cols {
            return Err("矩阵的大小不匹配。");
        }
        let mut data = Vec::with_capacity(self.count());
        for i in 0..self.count() {
            data.push(self.data[i] + other.data[i]);
        }
        Ok(Mat {
            data,
            rows: self.rows,
            cols: self.cols,
        })
    }
}

use std::ops::Sub;
impl<T: Copy + Sub<Output = T>> Sub for Mat<T> {
    type Output = Result<Mat<T>, &'static str>;

    fn sub(self, other: Self) -> Self::Output {
        if self.rows != other.rows || self.cols != other.cols {
            return Err("矩阵的大小不匹配。");
        }
        let mut data = Vec::with_capacity(self.count());
        for i in 0..self.count() {
            data.push(self.data[i] - other.data[i]);
        }
        Ok(Mat {
            data,
            rows: self.rows,
            cols: self.cols,
        })
    }
}

use std::ops::Mul;
impl<T: Copy + Mul<Output = T>> Mul<T> for Mat<T> {
    type Output = Mat<T>;

    fn mul(self, other: T) -> Self::Output {
        let mut data = Vec::with_capacity(self.count());
        for i in 0..self.count() {
            data.push(self.data[i] * other);
        }
        Mat {
            data,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

use std::ops::Div;
impl<T: Copy + Div<Output = T>> Div<T> for Mat<T> {
    type Output = Mat<T>;

    fn div(self, other: T) -> Self::Output {
        let mut data = Vec::with_capacity(self.count());
        for i in 0..self.count() {
            data.push(self.data[i] / other);
        }
        Mat {
            data,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

use std::ops::Neg;
use std::ops::SubAssign;
impl<T: Copy + Neg<Output = T>> Neg for Mat<T> {
    type Output = Mat<T>;

    fn neg(self) -> Self::Output {
        let mut data = Vec::with_capacity(self.count());
        for i in 0..self.count() {
            data.push(-self.data[i]);
        }
        Mat {
            data,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl<T: Copy + AddAssign> AddAssign for Mat<T> {
    fn add_assign(&mut self, rhs: Self) {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!("矩阵的大小不匹配。");
        }
        for i in 0..self.count() {
            self.data[i] += rhs.data[i];
        }
    }
}

impl<T: Copy + SubAssign> SubAssign for Mat<T> {
    fn sub_assign(&mut self, rhs: Self) {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!("矩阵的大小不匹配。");
        }
        for i in 0..self.count() {
            self.data[i] -= rhs.data[i];
        }
    }
}

use std::ops::MulAssign;
impl<T: Copy + MulAssign> MulAssign<T> for Mat<T> {
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..self.count() {
            self.data[i] *= rhs;
        }
    }
}

use std::ops::DivAssign;
impl<T: Copy + DivAssign> DivAssign<T> for Mat<T> {
    fn div_assign(&mut self, rhs: T) {
        for i in 0..self.count() {
            self.data[i] /= rhs;
        }
    }
}

impl<T> Mul for Mat<T>
where
    T: Copy + Default,
    T: Mul<Output = T> + AddAssign,
{
    type Output = Result<Mat<T>, &'static str>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.cols != rhs.rows {
            return Err("矩阵的大小不匹配。");
        }
        let mut data = Vec::with_capacity(self.rows * rhs.cols);
        for i in 0..self.rows {
            for j in 0..rhs.cols {
                let mut sum = T::default();
                for k in 0..self.cols {
                    sum += self[(i, k)] * rhs[(k, j)];
                }
                data.push(sum);
            }
        }
        Ok(Mat {
            data,
            rows: self.rows,
            cols: rhs.cols,
        })
    }
}

impl<T, const M: usize, const N: usize> From<&crate::Mat<T, M, N>> for Mat<T>
where
    T: Copy,
{
    fn from(value: &crate::Mat<T, M, N>) -> Self {
        let mut data = Vec::with_capacity(value.count());
        for i in 0..M {
            for j in 0..N {
                data.push(value[i][j]);
            }
        }
        Mat {
            data,
            rows: M,
            cols: N,
        }
    }
}

impl<T, const M: usize, const N: usize> From<&[[T; N]; M]> for Mat<T>
where
    T: Copy,
{
    fn from(value: &[[T; N]; M]) -> Self {
        let mut data = Vec::with_capacity(M * N);
        for i in 0..M {
            for j in 0..N {
                data.push(value[i][j]);
            }
        }
        Mat {
            data,
            rows: M,
            cols: N,
        }
    }
}

impl<T, const M: usize, const N: usize> TryInto<crate::Mat<T, M, N>> for Mat<T>
where
    T: Copy,
{
    type Error = &'static str;

    fn try_into(self) -> Result<crate::Mat<T, M, N>, Self::Error> {
        if self.rows != M || self.cols != N {
            return Err("矩阵的大小不匹配。");
        }
        let mut data = [[self.data[0]; N]; M];
        for i in 0..self.rows {
            for j in 0..self.cols {
                data[i][j] = self.data[i * self.cols + j];
            }
        }
        Ok(crate::Mat::from(data))
    }
}

impl<T: Copy> TryFrom<Vec<Vec<T>>> for Mat<T> {
    type Error = &'static str;

    fn try_from(value: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let rows = value.len();
        let cols = value[0].len();
        for i in 1..rows {
            if value[i].len() != cols {
                return Err("每个动态数组必须具有相同个数的元素。");
            }
        }
        let mut data = Vec::with_capacity(rows * cols);
        for i in 0..rows {
            for j in 0..cols {
                data.push(value[i][j]);
            }
        }
        Ok(Mat { data, rows, cols })
    }
}

impl<T: Copy> Mat<T> {
    /// Get a transpose matrix for the matrix.
    pub fn transpose(&self) -> Mat<T> {
        let mut data = Vec::with_capacity(self.cols * self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                data.push(self[(j, i)]);
            }
        }
        Mat {
            data,
            rows: self.cols,
            cols: self.rows,
        }
    }

    /// It is an alias of transpose().
    #[allow(non_snake_case)]
    pub fn T(&self) -> Mat<T> {
        self.transpose()
    }
}

impl<T: Copy + From<f32>> Mat<T> {
    /// Create an identity matrix of the specified size.
    pub fn identity(size: usize) -> Self {
        let mut data = Vec::with_capacity(size * size);
        for i in 0..size {
            for j in 0..size {
                if i == j {
                    data.push(T::from(1.0));
                } else {
                    data.push(T::from(0.0));
                }
            }
        }
        Mat {
            data,
            rows: size,
            cols: size,
        }
    }

    /// It is an alias of identity().
    #[allow(non_snake_case)]
    pub fn I(size: usize) -> Self {
        Self::identity(size)
    }
}

impl<T: Copy> Mat<T> {
    /// Swap two rows of the matrix.
    pub fn swap_row(&mut self, i: usize, j: usize) -> Result<(), &'static str> {
        if i >= self.rows || j >= self.rows {
            return Err("行索引超出范围。");
        }
        for k in 0..self.cols {
            let temp = self[(i, k)];
            self[(i, k)] = self[(j, k)];
            self[(j, k)] = temp;
        }
        Ok(())
    }

    /// Swap two columns of the matrix.
    pub fn swap_col(&mut self, i: usize, j: usize) -> Result<(), &'static str> {
        if i >= self.cols || j >= self.cols {
            return Err("列索引超出范围。");
        }
        for k in 0..self.rows {
            let temp = self[(k, i)];
            self[(k, i)] = self[(k, j)];
            self[(k, j)] = temp;
        }
        Ok(())
    }

    /// Get a copy of the elements of a row of the matrix.
    pub fn get_row(&self, i: usize) -> Result<Mat<T>, &'static str> {
        if i >= self.rows {
            return Err("行索引超出范围。");
        }
        let mut row = Vec::with_capacity(self.cols);
        for j in 0..self.cols {
            row.push(self[(i, j)]);
        }
        Ok(Mat {
            data: row,
            rows: 1,
            cols: self.cols,
        })
    }

    /// Get a copy of the elements of a column of the matrix.
    pub fn get_col(&self, j: usize) -> Result<Mat<T>, &'static str> {
        if j >= self.cols {
            return Err("列索引超出范围。");
        }
        let mut col = Vec::with_capacity(self.rows);
        for i in 0..self.rows {
            col.push(self[(i, j)]);
        }
        Ok(Mat {
            data: col,
            rows: self.rows,
            cols: 1,
        })
    }

    /// Set the elements of a row of the matrix to specified vector.
    pub fn set_row(&mut self, i: usize, row: &Mat<T>) -> Result<(), &'static str> {
        if i >= self.rows {
            return Err("行索引超出范围。");
        }
        if row.rows != 1 {
            return Err("输入不是行向量。");
        }
        if row.cols != self.cols {
            return Err("向量的维度与矩阵的列数不匹配。");
        }
        for j in 0..self.cols {
            self[(i, j)] = row[(0, j)];
        }
        Ok(())
    }

    /// Set the elements of a column of the matrix to specified vector.
    pub fn set_col(&mut self, j: usize, col: &Mat<T>) -> Result<(), &'static str> {
        if j >= self.cols {
            return Err("列索引超出范围。");
        }
        if col.cols != 1 {
            return Err("输入不是列向量。");
        }
        if col.rows != self.rows {
            return Err("向量的维度与矩阵的行数不匹配。");
        }
        for i in 0..self.rows {
            self[(i, j)] = col[(i, 0)];
        }
        Ok(())
    }
}

impl<T> Mat<T>
where
    T: Copy + From<f32> + PartialEq,
    T: Div<Output = T> + Mul<Output = T> + Sub<Output = T>,
{
    /// LU decomposition is performed on the matrix.
    ///
    /// # Return
    /// If it returns `Ok(x)`:
    /// It will return a tuple containing two matrices:
    ///     - The first matrix is the lower triangular matrix.
    ///     - The second matrix is the upper triangular matrix.
    pub fn lu(&self) -> Result<(Self, Self), &'static str> {
        if self.rows != self.cols {
            return Err("矩阵必须是方阵。");
        }
        let mut l = Self::I(self.rows);
        let mut u = self.clone();
        for i in 0..self.rows {
            for j in i + 1..self.cols {
                if u[(i, i)] == T::from(0.0) {
                    u.swap_row(i, self.rows - 1)?;
                }
                let m = if u[(i, i)] == T::from(0.0) {
                    T::from(0.0)
                } else {
                    u[(j, i)] / u[(i, i)]
                };
                l[(j, i)] = m;
                for k in i..self.rows {
                    u[(j, k)] = u[(j, k)] - m * u[(i, k)];
                }
            }
        }
        Ok((l, u))
    }

    /// Calculate the determinant value of the matrix.
    ///
    /// # Return
    ///
    /// If it returns `Ok(x)`:
    /// The determinant value of the matrix.
    pub fn det(&self) -> Result<T, &'static str> {
        let (l, u) = self.lu()?;
        let mut det = T::from(1.0);
        for i in 0..self.rows {
            det = det * u[(i, i)] * l[(i, i)];
        }
        Ok(det)
    }
}

impl<T> Mat<T>
where
    T: Copy + From<f32> + PartialOrd + Debug,
    T: DivAssign + SubAssign + Mul<Output = T>,
{
    /// Calculate the inverse matrix of the matrix
    ///
    /// # Return
    /// If it returns `Ok(x)`:
    /// The function returns an `Option`:
    /// - If the matrix is invertible, it will return a `Some(x)` value containing the inverse matrix.
    /// - If the matrix is irreversible, it will return `None`.
    pub fn inverse(&self) -> Result<Option<Self>, &'static str> {
        if self.rows != self.cols {
            return Err("矩阵必须是方阵。");
        }
        let mut it = self.clone();
        let mut ext = Self::I(self.rows);
        // Gauss-Jordan elimination method
        for i in 0..self.rows {
            let mut it_row_i = it.get_row(i)?;
            let mut ext_row_i = ext.get_row(i)?;
            if T::from(-1e-10) < it[(i, i)] && it[(i, i)] < T::from(1e-10) {
                return Ok(None);
            }
            it_row_i /= it[(i, i)];
            ext_row_i /= it[(i, i)];
            it.set_row(i, &it_row_i)?;
            ext.set_row(i, &ext_row_i)?;
            for j in 0..self.rows {
                if i != j {
                    let mut it_row_j = it.get_row(j)?;
                    let mut ext_row_j = ext.get_row(j)?;
                    it_row_j -= it.get_row(i)? * it[(j, i)];
                    ext_row_j -= ext.get_row(i)? * it[(j, i)];
                    it.set_row(j, &it_row_j)?;
                    ext.set_row(j, &ext_row_j)?;
                }
            }
        }
        Ok(Some(ext))
    }
}

impl<T> Mat<T>
where
    T: Copy + Default + PartialEq,
    T: SubAssign + Mul<Output = T> + Div<Output = T>,
{
    /// Calculate the rank of the matrix
    ///
    /// # Return
    /// If it returns `Ok(x)`:
    /// The rank of the matrix
    pub fn rank(&self) -> Result<usize, &'static str> {
        let mut it = self.clone();
        for i in 0..self.rows {
            let row_i = it.get_row(i)?;
            if it[(i, i)] == T::default() {
                continue;
            }
            for j in i + 1..self.rows {
                let mut row_j = it.get_row(j)?;
                row_j -= row_i.clone() * (it[(j, i)] / it[(i, i)]);
                it.set_row(j, &row_j)?;
            }
        }

        let mut rank = 0;
        for i in 0..self.rows {
            for j in 0..self.cols {
                if it[(i, j)] != T::default() {
                    rank += 1;
                    break;
                }
            }
        }
        Ok(rank)
    }
}
