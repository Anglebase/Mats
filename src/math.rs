use std::{fmt::Debug, ops::*};

use super::*;

impl<T, const M: usize> Vec<T, M>
where
    T: Copy + Default + Into<f32>,
    T: AddAssign + Mul<Output = T>,
{
    /// Calculate the modulus length of the vector.
    pub fn norm(&self) -> f32 {
        let s: f32 = (*self * self.transpose())[0][0].into();
        s.sqrt().into()
    }
}

impl<T, const M: usize> Vec<T, M>
where
    T: Copy + Default + Into<f32>,
    T: AddAssign + Mul<Output = T> + Div<Output = T>,
    T: From<f32>,
{
    /// Normalize vector.
    pub fn normalize(&self) -> Self {
        let norm = self.norm();
        if norm == 0.0 {
            return Self::new();
        }
        *self / norm.into()
    }
}

impl<T> Vec3<T>
where
    T: Copy + Default + Into<f32>,
    T: Mul<Output = T> + Sub<Output = T>,
{
    /// Calculate the cross product of two vectors.
    pub fn cross(&self, other: Self) -> Self {
        let mut result = Self::new();
        result.set_x(*self.y() * *other.z() - *self.z() * *other.y());
        result.set_y(*self.z() * *other.x() - *self.x() * *other.z());
        result.set_z(*self.x() * *other.y() - *self.y() * *other.x());
        result
    }
}

impl<T, const M: usize> Mat<T, M, M>
where
    T: Copy + From<f32> + PartialEq + Default,
    T: Mul<Output = T> + Sub<Output = T> + Div<Output = T>,
{
    /// LU decomposition is performed on the matrix.
    ///
    /// # Return
    /// It will return a tuple containing two matrices:
    ///     - The first matrix is the lower triangular matrix.
    ///     - The second matrix is the upper triangular matrix.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use mats::Mat;
    ///
    /// let a = Mat::from([
    ///     [1.0, 2.0, 3.0],
    ///     [4.0, 5.0, 6.0],
    ///     [7.0, 8.0, 9.0]
    /// ]);
    /// let (l, u) = a.lu();
    ///
    /// assert_eq!(l, Mat::from([
    ///     [1.0, 0.0, 0.0],
    ///     [4.0, 1.0, 0.0],
    ///     [7.0, 2.0, 1.0]
    /// ]));
    /// assert_eq!(u, Mat::from([
    ///     [1.0,  2.0,  3.0],
    ///     [0.0, -3.0, -6.0],
    ///     [0.0,  0.0,  0.0]
    /// ]));
    /// assert_eq!(l * u, a);
    /// ```
    pub fn lu(&self) -> (Mat<T, M, M>, Mat<T, M, M>) {
        let mut l = Mat::I();
        let mut u = *self;
        for i in 0..M {
            for j in i + 1..M {
                if u[i][i] == T::from(0.0) {
                    u.swap_row(i, M - 1);
                }
                let m = if u[i][i] == T::from(0.0) {
                    T::from(0.0)
                } else {
                    u[j][i] / u[i][i]
                };
                l[j][i] = m;
                for k in i..M {
                    u[j][k] = u[j][k] - m * u[i][k];
                }
            }
        }
        (l, u)
    }

    /// Calculate the determinant value of the matrix.
    ///
    /// # Return
    ///
    /// The determinant value of the matrix.
    ///
    /// # Example
    ///
    /// ```
    /// use mats::Mat;
    ///
    /// let a = Mat::from([
    ///     [1.0, 2.0, 3.0],
    ///     [4.0, 5.0, 6.0],
    ///     [7.0, 8.0, 9.0]
    /// ]);
    /// assert_eq!(a.det(), 0.0);
    /// ```
    pub fn det(&self) -> T {
        let (l, u) = self.lu();
        let mut det = T::from(1.0);
        for i in 0..M {
            det = det * l[i][i] * u[i][i];
        }
        det
    }
}

impl<T, const M: usize> Mat<T, M, M>
where
    T: Copy + Default + From<f32> + PartialOrd + Debug,
    T: DivAssign + SubAssign + Mul<Output = T>,
{
    /// Calculate the inverse matrix of the matrix
    ///
    /// # Return
    ///
    /// The function returns an `Option`:
    /// - If the matrix is invertible, it will return a `Some(x)` value containing the inverse matrix.
    /// - If the matrix is irreversible, it will return `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use mats::Mat;
    ///
    /// let a = Mat::from([
    ///     [1.0, 2.0, 3.0],
    ///     [4.0, 5.0, 6.0],
    ///     [7.0, 8.0, 9.0]
    /// ]);
    /// assert_eq!(a.inverse(), None);
    ///
    /// let mat = Mat::from([
    ///     [1.0, 2.0, 3.0],
    ///     [4.0, 7.0, 6.0],
    ///     [7.0, 8.0, 9.0]
    /// ]);
    /// assert!(Mat::<f64,3,3>::I().eq_with_epsilon(&(mat * mat.inverse().unwrap()), 1e-6));
    /// ```
    pub fn inverse(&self) -> Option<Self> {
        let mut it = *self;
        let mut ext = Self::I();
        // Gauss-Jordan elimination method
        for i in 0..M {
            let mut it_row_i = Vec::from([it[i]]);
            let mut ext_row_i = Vec::from([ext[i]]);
            if T::from(-1e-10) < it[i][i] && it[i][i] < T::from(1e-10) {
                return None;
            }
            it_row_i /= it[i][i];
            ext_row_i /= it[i][i];
            it[i] = it_row_i[0];
            ext[i] = ext_row_i[0];
            for j in 0..M {
                if i != j {
                    let mut it_row_j = Vec::from([it[j]]);
                    let mut ext_row_j = Vec::from([ext[j]]);
                    it_row_j -= it_row_i * it[j][i];
                    ext_row_j -= ext_row_i * it[j][i];
                    it[j] = it_row_j[0];
                    ext[j] = ext_row_j[0];
                }
            }
        }

        Some(ext)
    }
}

impl<T, const M: usize, const N: usize> Mat<T, M, N>
where
    T: Copy + Default + PartialEq,
    T: SubAssign + Mul<Output = T> + Div<Output = T>,
{
    /// Calculate the rank of the matrix
    ///
    /// # Return
    ///
    /// The rank of the matrix
    ///
    /// # Example
    ///
    /// ```rust
    /// use mats::{Mat3, Mat4};
    /// let mat = Mat3::from([
    ///     [1.0, 2.0, 3.0],
    ///     [4.0, 5.0, 6.0],
    ///     [7.0, 8.0, 9.0]
    /// ]);
    /// assert_eq!(mat.rank(), 2);
    ///
    /// assert_eq!(Mat4::<f32>::new().rank(), 0);
    /// ```
    pub fn rank(&self) -> usize {
        let mut it = *self;
        for i in 0..M {
            let row_i = Vec::from([it[i]]);
            if it[i][i] == T::default() {
                continue;
            }
            for j in i + 1..M {
                let mut row_j = Vec::from([it[j]]);
                row_j -= row_i * (it[j][i] / it[i][i]);
                it[j] = row_j[0];
            }
        }

        let mut rank = 0;
        for i in 0..M {
            for j in 0..N {
                if it[i][j] != T::default() {
                    rank += 1;
                    break;
                }
            }
        }
        rank
    }
}

/// Generate a 2D rotation transformation matrix.
///
/// # Parameters
///
/// - `angle` : The angle of rotation in radians.
///
/// # Return
///
/// Rotation matrix
pub fn rotate2(angle: f32) -> Mat3<f32> {
    let c = angle.cos();
    let s = angle.sin();
    Mat3::from([[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]])
}

/// Generate a 2D translational transformation matrix.
///
/// # Parameters
///
/// - `v` : Pan vector.
///
/// # Return
///
/// Translation matrix.
pub fn translate2(v: Vec2<f32>) -> Mat3<f32> {
    Mat3::from([[1.0, 0.0, *v.x()], [0.0, 1.0, *v.y()], [0.0, 0.0, 1.0]])
}

/// Generate a 2D scaled transformation matrix.
///
/// # Parameter
///
/// - `v` : Scale factor.
///
/// # Return
///
/// Scale matrix.
pub fn scale2(v: Vec2<f32>) -> Mat3<f32> {
    Mat3::from([[*v.x(), 0.0, 0.0], [0.0, *v.y(), 0.0], [0.0, 0.0, 1.0]])
}

/// Generate a 3D rotation transformation matrix.
///
/// # Parameters
///
/// - `angle` : The angle of rotation in radians.
/// - `axis` : Rotary shaft.
///
/// # Return
///
/// Rotation matrix.
pub fn rotate3(angle: f32, axis: Vec3<f32>) -> Mat4<f32> {
    let mut result = Mat4::I();
    let axis = axis.normalize();
    let c = angle.cos();
    let s = angle.sin();
    let k = Mat::from([
        [0.0, -*axis.z(), *axis.y()],
        [*axis.z(), 0.0, -*axis.x()],
        [-*axis.y(), *axis.x(), 0.0],
    ]);
    let r = Mat3::I() + k * s + (k * k) * (1.0 - c);
    for i in 0..3 {
        for j in 0..3 {
            result[i][j] = r[i][j];
        }
    }
    result
}

/// Generate a 3D translational transformation matrix.
///
/// # Parameters
///
/// - `v` : Pan vector.
///
/// # Return
///
/// Translation matrix.
pub fn translate3(v: Vec3<f32>) -> Mat4<f32> {
    Mat4::from([
        [1.0, 0.0, 0.0, *v.x()],
        [0.0, 1.0, 0.0, *v.y()],
        [0.0, 0.0, 1.0, *v.z()],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

/// Generate a 3D scale transformation matrix
///
/// # Parameters
///
/// - `v` : Scale factor
///
/// # Return
///
/// Scale the matrix
pub fn scale3(v: Vec3<f32>) -> Mat4<f32> {
    Mat4::from([
        [*v.x(), 0.0, 0.0, 0.0],
        [0.0, *v.y(), 0.0, 0.0],
        [0.0, 0.0, *v.z(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

/// A 3D rotation transformation matrix around the x-axis is generated
///
/// # Parameters
///
/// - `angle` : The angle of rotation in radians
///
/// # Return
///
/// Rotation matrix
pub fn rotate3_x(angle: f32) -> Mat4<f32> {
    let c = angle.cos();
    let s = angle.sin();
    Mat4::from([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, c, -s, 0.0],
        [0.0, s, c, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

/// A 3D rotation transformation matrix around the y-axis is generated
///
/// # Parameters
///
/// - `angle` : The angle of rotation in radians
///
/// # Return
///
/// Rotation matrix
pub fn rotate3_y(angle: f32) -> Mat4<f32> {
    let c = angle.cos();
    let s = angle.sin();
    Mat4::from([
        [c, 0.0, s, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-s, 0.0, c, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

/// A 3D rotation transformation matrix is generated about the z-axis
///
/// # Parameters
///
/// - `angle` : The angle of rotation in radians
///
/// # Return
///
/// Rotation matrix
pub fn rotate3_z(angle: f32) -> Mat4<f32> {
    let c = angle.cos();
    let s = angle.sin();
    Mat4::from([
        [c, -s, 0.0, 0.0],
        [s, c, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

/// Angle system to radian system
///
/// # Parameters
///
/// - `degrees` : Angle system angle
///
/// # Return
///
/// Radian angles
///
/// # Example
///
/// ```
/// use mats::radian;
///
/// let radian = radian(90.0);
/// assert_eq!(radian, std::f32::consts::PI / 2.0);
/// ```
pub fn radian(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

/// Generate a matrix of camera position transformations
///
/// # Parameters
///
/// - `eye` : Camera position
/// - `target` : Target position
/// - `up` : Upper vector
///
/// # Return
///
/// Positional Transformation Matrix
///
/// # Example
///
/// ```
/// use mats::{look_at,Vec3};
///
/// let eye = Vec3::from([0.0, 0.0, 1.0]);
/// let target = Vec3::from([0.0, 0.0, 0.0]);
/// let up = Vec3::from([0.0, 1.0, 0.0]);
/// let matrix = look_at(eye, target, up);
/// ```
pub fn look_at(eye: Vec3<f32>, target: Vec3<f32>, up: Vec3<f32>) -> Mat4<f32> {
    let z = (eye - target).normalize(); // 计算z轴方向向量
    let x = up.cross(z).normalize(); // 计算x轴方向向量
    let y = z.cross(x); // 计算y轴方向向量

    let translation = [
        [1.0, 0.0, 0.0, -eye.x()],
        [0.0, 1.0, 0.0, -eye.y()],
        [0.0, 0.0, 1.0, -eye.z()],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let rotation = [
        [*x.x(), *x.y(), *x.z(), 0.0],
        [*y.x(), *y.y(), *y.z(), 0.0],
        [*z.x(), *z.y(), *z.z(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    Mat4::from(rotation) * Mat4::from(translation)
}

/// Generate a perspective projection matrix
///
/// # Parameters
///
/// - `fov` : Filed of view
/// - `aspect` Aspect ratio
/// - `z_near` Near cut plane
/// - `z_far` Far cut face
///
/// # Return
///
/// Perspective projection matrix
///
/// # Example
///
/// ```
/// use mats::perspective;
///
/// let matrix = perspective(45.0, 1.0, 0.1, 100.0);
/// ```
pub fn perspective(fov: f32, aspect: f32, z_near: f32, z_far: f32) -> Mat4<f32> {
    let f = 1.0 / (fov / 2.0).tan();
    let mut result = [[0.0; 4]; 4];

    result[0][0] = f / aspect;
    result[1][1] = f;
    result[2][2] = (z_far + z_near) / (z_near - z_far);
    result[2][3] = (2.0 * z_far * z_near) / (z_near - z_far);
    result[3][2] = -1.0;

    Mat4::from(result)
}

/// Generate a orthographic projection matrix
///
/// # Parameters
///
/// - `left` : Left
/// - `right` : Right
/// - `top` : Top
/// - `botton` : Bottom
/// - `z_near` : Near
/// - `z_far` : Far
///
/// # Return
///
/// Orthographic projection matrix
///
/// # Example
///
/// ```
/// use mats::ortho;
///
/// let matrix = ortho((0.0, 800.0), (0.0, 600.0), 0.1, 100.0);
/// ```
pub fn ortho(
    (left, right): (f32, f32),
    (top, botton): (f32, f32),
    z_near: f32,
    z_far: f32,
) -> Mat4<f32> {
    Mat4::from([
        [
            2.0 / (right - left),
            0.0,
            0.0,
            (left + right) / (left - right),
        ],
        [
            0.0,
            2.0 / (top - botton),
            0.0,
            (botton + top) / (botton - top),
        ],
        [
            0.0,
            0.0,
            2.0 / (z_near - z_far),
            (z_near + z_far) / (z_near - z_far),
        ],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;
    const SQRT_3: f32 = 1.732_050_807_568_877_2;

    #[test]
    fn test_norm_happy_path() {
        let v = Vec3::from([3.0, 4.0, 0.0]);
        assert_eq!(v.norm(), 5.0);

        let v = Vec3::from([1.0, 1.0, 1.0]);
        assert_eq!(v.norm(), SQRT_3);

        let v = Vec3::from([0.0, 0.0, 5.0]);
        assert_eq!(v.norm(), 5.0);
    }

    #[test]
    fn test_norm_zero_vector() {
        let v = Vec3::<f32>::new();
        assert_eq!(v.norm(), 0.0);
    }

    #[test]
    fn test_normalize_happy_path() {
        let v = Vec3::from([3.0, 4.0, 0.0]);
        let normalized = v.normalize();
        assert_eq!(normalized, Vec3::from([0.6, 0.8, 0.0]));

        let v = Vec3::from([1.0, 1.0, 1.0]);
        let normalized = v.normalize();
        assert_eq!(
            normalized,
            Vec3::from([1.0 / SQRT_3, 1.0 / SQRT_3, 1.0 / SQRT_3])
        );

        let v = Vec3::from([0.0, 0.0, 5.0]);
        let normalized = v.normalize();
        assert_eq!(normalized, Vec3::from([0.0, 0.0, 1.0]));
    }

    #[test]
    fn test_normalize_zero_vector() {
        let v = Vec3::<f32>::new();
        let normalized = v.normalize();
        assert_eq!(normalized, Vec3::new());
    }

    #[test]
    fn test_cross_happy_path() {
        let v1 = Vec3::from([1.0, 0.0, 0.0]);
        let v2 = Vec3::from([0.0, 1.0, 0.0]);
        let cross = v1.cross(v2);
        assert_eq!(cross, Vec3::from([0.0, 0.0, 1.0]));

        let v1 = Vec3::from([1.0, 2.0, 3.0]);
        let v2 = Vec3::from([4.0, 5.0, 6.0]);
        let cross = v1.cross(v2);
        assert_eq!(cross, Vec3::from([-3.0, 6.0, -3.0]));
    }

    #[test]
    fn test_cross_parallel_vectors() {
        let v1 = Vec3::from([1.0, 2.0, 3.0]);
        let v2 = Vec3::from([2.0, 4.0, 6.0]);
        let cross = v1.cross(v2);
        assert_eq!(cross, Vec3::new());
    }

    #[test]
    fn test_rotate2_happy_path() {
        let angle = PI / 2.0;
        let matrix = rotate2(angle);
        assert!(matrix.eq_with_epsilon(
            &Mat3::from([[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]]),
            1e-6
        ));
    }

    #[test]
    fn test_rotate2_zero_angle() {
        let angle = 0.0;
        let matrix = rotate2(angle);
        assert_eq!(
            matrix,
            Mat3::from([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
        );
    }

    #[test]
    fn test_rotate2_full_rotation() {
        let angle = 2.0 * PI;
        let matrix = rotate2(angle);
        assert!(matrix.eq_with_epsilon(
            &Mat3::from([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]),
            1e-6
        ));
    }

    #[test]
    fn test_translate2_happy_path() {
        let v = Vec2::from([1.0, 2.0]);
        let matrix = translate2(v);
        assert_eq!(
            matrix,
            Mat3::from([[1.0, 0.0, 1.0], [0.0, 1.0, 2.0], [0.0, 0.0, 1.0]])
        );
    }

    #[test]
    fn test_translate2_zero_vector() {
        let v = Vec2::new();
        let matrix = translate2(v);
        assert_eq!(
            matrix,
            Mat3::from([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
        );
    }

    #[test]
    fn test_scale2_happy_path() {
        let v = Vec2::from([2.0, 3.0]);
        let matrix = scale2(v);
        assert_eq!(
            matrix,
            Mat3::from([[2.0, 0.0, 0.0], [0.0, 3.0, 0.0], [0.0, 0.0, 1.0]])
        );
    }

    #[test]
    fn test_scale2_zero_vector() {
        let v = Vec2::from([0.0, 0.0]);
        let matrix = scale2(v);
        assert_eq!(
            matrix,
            Mat3::from([[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]])
        );
    }

    #[test]
    fn test_rotate3_happy_path() {
        let angle = PI / 2.0;
        let axis = Vec3::from([0.0, 0.0, 1.0]);
        let matrix = rotate3(angle, axis);
        assert_eq!(
            matrix,
            Mat4::from([
                [0.0, -1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ])
        );
    }

    #[test]
    fn test_rotate3_zero_angle() {
        let angle = 0.0;
        let axis = Vec3::from([1.0, 0.0, 0.0]);
        let matrix = rotate3(angle, axis);
        assert!(matrix.eq_with_epsilon(&Mat4::I(), 1e-6));
    }

    #[test]
    fn test_rotate3_full_rotation() {
        let angle = 2.0 * PI;
        let axis = Vec3::from([1.0, 0.0, 0.0]);
        let matrix = rotate3(angle, axis);
        assert!(matrix.eq_with_epsilon(&Mat4::I(), 1e-6));
    }

    #[test]
    fn test_rotate3_x_happy_path() {
        let angle = PI / 2.0;
        let matrix = rotate3_x(angle);
        assert!(matrix.eq_with_epsilon(
            &Mat4::from([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, -1.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]),
            1e-6
        ));
    }

    #[test]
    fn test_rotate3_x_zero_angle() {
        let angle = 0.0;
        let matrix = rotate3_x(angle);
        assert!(matrix.eq_with_epsilon(&Mat4::I(), 1e-6));
    }

    #[test]
    fn test_rotate3_x_full_rotation() {
        let angle = 2.0 * PI;
        let matrix = rotate3_x(angle);
        assert!(matrix.eq_with_epsilon(&Mat4::I(), 1e-6));
    }

    #[test]
    fn test_rotate3_y_happy_path() {
        let angle = PI / 2.0;
        let matrix = rotate3_y(angle);
        assert!(matrix.eq_with_epsilon(
            &Mat4::from([
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]),
            1e-6
        ));
    }

    #[test]
    fn test_rotate3_y_zero_angle() {
        let angle = 0.0;
        let matrix = rotate3_y(angle);
        assert!(matrix.eq_with_epsilon(&Mat4::I(), 1e-6));
    }

    #[test]
    fn test_rotate3_y_full_rotation() {
        let angle = 2.0 * PI;
        let matrix = rotate3_y(angle);
        assert!(matrix.eq_with_epsilon(&Mat4::I(), 1e-6));
    }

    #[test]
    fn test_rotate3_z_happy_path() {
        let angle = PI / 2.0;
        let matrix = rotate3_z(angle);
        assert!(matrix.eq_with_epsilon(
            &Mat4::from([
                [0.0, -1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]),
            1e-6
        ));
    }

    #[test]
    fn test_rotate3_z_zero_angle() {
        let angle = 0.0;
        let matrix = rotate3_z(angle);
        assert!(matrix.eq_with_epsilon(&Mat4::I(), 1e-6));
    }

    #[test]
    fn test_rotate3_z_full_rotation() {
        let angle = 2.0 * PI;
        let matrix = rotate3_z(angle);
        assert!(matrix.eq_with_epsilon(&Mat4::I(), 1e-6));
    }

    #[test]
    fn test_translate3_happy_path() {
        let v = Vec3::from([1.0, 2.0, 3.0]);
        let matrix = translate3(v);
        assert_eq!(
            matrix,
            Mat4::from([
                [1.0, 0.0, 0.0, 1.0],
                [0.0, 1.0, 0.0, 2.0],
                [0.0, 0.0, 1.0, 3.0],
                [0.0, 0.0, 0.0, 1.0],
            ])
        );
    }

    #[test]
    fn test_translate3_zero_vector() {
        let v = Vec3::new();
        let matrix = translate3(v);
        assert!(matrix.eq_with_epsilon(&Mat4::I(), 1e-6));
    }

    #[test]
    fn test_scale3_happy_path() {
        let v = Vec3::from([2.0, 3.0, 4.0]);
        let matrix = scale3(v);
        assert_eq!(
            matrix,
            Mat4::from([
                [2.0, 0.0, 0.0, 0.0],
                [0.0, 3.0, 0.0, 0.0],
                [0.0, 0.0, 4.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ])
        );
    }

    #[test]
    fn test_scale3_zero_vector() {
        let v = Vec3::from([0.0, 0.0, 0.0]);
        let matrix = scale3(v);
        assert_eq!(
            matrix,
            Mat4::from([
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ])
        );
    }

    #[test]
    fn lu_1() {
        let mat = Mat4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let (l, u) = mat.lu();
        assert_eq!(
            l,
            Mat4::from([
                [1.0, 0.0, 0.0, 0.0],
                [5.0, 1.0, 0.0, 0.0],
                [9.0, 2.0, 1.0, 0.0],
                [13.0, 3.0, 0.0, 1.0],
            ])
        );
        assert_eq!(
            u,
            Mat4::from([
                [1.0, 2.0, 3.0, 4.0],
                [0.0, -4.0, -8.0, -12.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ])
        );
        assert_eq!(l * u, mat);
    }

    #[test]
    fn lu_2() {
        use crate::*;
        let mat = Mat4::from([
            [3.0, 9.0, 4.0, 7.0],
            [8.0, 2.0, 9.0, 10.0],
            [4.0, 1.0, 3.0, 10.0],
            [7.0, 6.0, 6.0, 1.0],
        ]);
        let (l, u) = mat.lu();
        assert!(mat.eq_with_epsilon(&(l * u), 1e-6))
    }

    #[test]
    fn inverse() {
        let mat = Mat3::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        assert_eq!(mat.inverse(), None);

        let mat = Mat3::from([[1.0, 2.0, 3.0], [4.0, 7.0, 6.0], [7.0, 8.0, 9.0]]);
        assert!(Mat3::<f64>::I().eq_with_epsilon(&(mat * mat.inverse().unwrap()), 1e-6));

        let mat = Mat4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        assert_eq!(mat.inverse(), None);

        let mat = Mat4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 9.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 1.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        assert!(Mat4::<f64>::I().eq_with_epsilon(&(mat * mat.inverse().unwrap()), 1e-6));
    }

    #[test]
    fn rank() {
        let mat = Mat3::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        assert_eq!(mat.rank(), 2);

        let mat = Mat4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        assert_eq!(mat.rank(), 2);

        assert_eq!(Mat4::<f32>::new().rank(), 0);
    }
}
