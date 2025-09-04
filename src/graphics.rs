use crate::{Mat, Mat3, Mat4, UnitOne, Vec2, Vec3, Zero};

/// A trait for floating-point types.
pub trait Float:
    Copy
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Neg<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Add<Output = Self>
    + std::ops::DivAssign
    + std::ops::AddAssign
    + Zero
    + UnitOne
    + PartialOrd
{
    const PI: Self;
    const STRAIGHT: Self;
    const EPSILON: Self;

    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn sqrt(self) -> Self;
    fn abs(self) -> Self;
    fn tan(self) -> Self;
}

macro_rules! impl_float {
    ($($t:tt),*) => {
        $(
            impl Float for $t {
                const PI: Self = std::$t::consts::PI;
                const STRAIGHT: Self = 180.0 as _;
                const EPSILON: Self = std::$t::EPSILON;

                fn cos(self) -> Self {
                    self.cos()
                }

                fn sin(self) -> Self {
                    self.sin()
                }

                fn sqrt(self) -> Self {
                    self.sqrt()
                }

                fn abs(self) -> Self {
                    self.abs()
                }

                fn tan(self) -> Self {
                    self.tan()
                }
            }
        )*
    };
}

impl_float!(f32, f64);

/// Convert an angle in degrees to radians.
///
/// # Example
/// ```
/// use mats::graphics::radian;
///
/// let a = radian(45.0);
/// assert_eq!(a, std::f32::consts::PI / 4.0);
/// ```
#[inline]
pub fn radian<T: Float>(angle: T) -> T {
    angle * T::PI / T::STRAIGHT
}

/// Convert an angle in radians to degrees.
///
/// # Example
/// ```
/// use mats::graphics::degree;
///
/// let a = degree(std::f32::consts::PI / 4.0);
/// assert_eq!(a, 45.0);
/// ```
#[inline]
pub fn degree<T: Float>(angle: T) -> T {
    angle * T::STRAIGHT / T::PI
}

/// Create a 2D scaling matrix.
///
/// # Example
/// ```
/// use mats::{Vec3, Vec2, graphics::scale2d};
///
/// let v = Vec3::new([[1.0, 2.0, 1.0]]);
/// let scale = scale2d(Vec2::new([[2.0, 3.0]]));
/// let v2 = scale * v;
///
/// assert_eq!(v2.x(), 2.0);
/// assert_eq!(v2.y(), 6.0);
/// ```
#[inline]
pub fn scale2d<T: Float>(v: Vec2<T>) -> Mat3<T> {
    Mat {
        data: [
            [v.x(), T::zero(), T::zero()],
            [T::zero(), v.y(), T::zero()],
            [T::zero(), T::zero(), T::unit_one()],
        ],
    }
}

/// Create a 2D translation matrix.
///
/// # Example
/// ```
/// use mats::{Vec3, Vec2, graphics::translate2d};
///
/// let v = Vec3::new([[1.0, 2.0, 1.0]]);
/// let translate = translate2d(Vec2::new([[2.0, 3.0]]));
/// let v2 = translate * v;
///
/// assert_eq!(v2.x(), 3.0);
/// assert_eq!(v2.y(), 5.0);
/// ```
#[inline]
pub fn translate2d<T: Float>(v: Vec2<T>) -> Mat3<T> {
    Mat {
        data: [
            [T::unit_one(), T::zero(), T::zero()],
            [T::zero(), T::unit_one(), T::zero()],
            [v.x(), v.y(), T::unit_one()],
        ],
    }
}

/// Create a 2D rotation matrix.
///
/// # Example
/// ```
/// use mats::{Vec3, graphics::rotate2d};
///
/// let v = Vec3::new([[1.0, 0.0, 1.0]]);
/// let rotate = rotate2d(std::f32::consts::PI / 2.0);
/// let v2 = rotate * v;
///
/// let ais = v2 - Vec3::new([[0.0, 1.0, 0.0]]);
///
/// assert!(ais.x().abs() <= f32::EPSILON);
/// assert!(ais.y().abs() <= f32::EPSILON);
/// ```
#[inline]
pub fn rotate2d<T: Float>(angle: T) -> Mat3<T> {
    let c = angle.cos();
    let s = angle.sin();
    Mat {
        data: [
            [c, s, T::zero()],
            [-s, c, T::zero()],
            [T::zero(), T::zero(), T::unit_one()],
        ],
    }
}

/// Create a 3D translation matrix.
///
/// # Example
/// ```
/// use mats::{Vec4, Vec3, graphics::translate3d};
///
/// let v = Vec4::new([[1.0, 2.0, 3.0, 1.0]]);
/// let translate = translate3d(Vec3::new([[2.0, 3.0, 4.0]]));
/// let v2 = translate * v;
///
/// assert_eq!(v2.x(), 3.0);
/// assert_eq!(v2.y(), 5.0);
/// assert_eq!(v2.z(), 7.0);
/// ```
#[inline]
pub fn translate3d<T: Float>(v: Vec3<T>) -> Mat4<T> {
    Mat {
        data: [
            [T::unit_one(), T::zero(), T::zero(), T::zero()],
            [T::zero(), T::unit_one(), T::zero(), T::zero()],
            [T::zero(), T::zero(), T::unit_one(), T::zero()],
            [v.x(), v.y(), v.z(), T::unit_one()],
        ],
    }
}

/// Create a 3D scaling matrix.
///
/// # Example
/// ```
/// use mats::{Vec4, Vec3, graphics::scale3d};
///
/// let v = Vec4::new([[1.0, 2.0, 3.0, 1.0]]);
/// let scale = scale3d(Vec3::new([[2.0, 3.0, 4.0]]));
/// let v2 = scale * v;
///
/// assert_eq!(v2.x(), 2.0);
/// assert_eq!(v2.y(), 6.0);
/// assert_eq!(v2.z(), 12.0);
/// ```
#[inline]
pub fn scale3d<T: Float>(v: Vec3<T>) -> Mat4<T> {
    Mat {
        data: [
            [v.x(), T::zero(), T::zero(), T::zero()],
            [T::zero(), v.y(), T::zero(), T::zero()],
            [T::zero(), T::zero(), v.z(), T::zero()],
            [T::zero(), T::zero(), T::zero(), T::unit_one()],
        ],
    }
}

/// Create a 3D rotation matrix around the X axis.
///
/// # Example
/// ```
/// use mats::{Vec4, Vec3, graphics::rotate3d_x};
///
/// let v = Vec4::new([[0.0, 1.0, 0.0, 1.0]]);
/// let rotate = rotate3d_x(std::f32::consts::PI / 2.0);
/// let v2 = rotate * v;
///
/// let ais = v2 - Vec4::new([[0.0, 0.0, 1.0, 0.0]]);
///
/// assert!(ais.x().abs() <= f32::EPSILON);
/// assert!(ais.y().abs() <= f32::EPSILON);
/// assert!(ais.z().abs() <= f32::EPSILON);
/// ```
#[inline]
pub fn rotate3d_x<T: Float>(angle: T) -> Mat4<T> {
    let c = angle.cos();
    let s = angle.sin();
    Mat {
        data: [
            [T::unit_one(), T::zero(), T::zero(), T::zero()],
            [T::zero(), c, s, T::zero()],
            [T::zero(), -s, c, T::zero()],
            [T::zero(), T::zero(), T::zero(), T::unit_one()],
        ],
    }
}

/// Create a 3D rotation matrix around the Y axis.
///
/// # Example
/// ```
/// use mats::{Vec4, Vec3, graphics::rotate3d_y};
///
/// let v = Vec4::new([[1.0, 0.0, 0.0, 1.0]]);
/// let rotate = rotate3d_y(std::f32::consts::PI / 2.0);
/// let v2 = rotate * v;
///
/// let ais = v2 - Vec4::new([[0.0, 0.0, -1.0, 0.0]]);
///
/// assert!(ais.x().abs() <= f32::EPSILON);
/// assert!(ais.y().abs() <= f32::EPSILON);
/// assert!(ais.z().abs() <= f32::EPSILON);
/// ```
#[inline]
pub fn rotate3d_y<T: Float>(angle: T) -> Mat4<T> {
    let c = angle.cos();
    let s = angle.sin();
    Mat {
        data: [
            [c, T::zero(), -s, T::zero()],
            [T::zero(), T::unit_one(), T::zero(), T::zero()],
            [s, T::zero(), c, T::zero()],
            [T::zero(), T::zero(), T::zero(), T::unit_one()],
        ],
    }
}

/// Create a 3D rotation matrix around the Z axis.
///
/// # Example
/// ```
/// use mats::{Vec4, Vec3, graphics::rotate3d_z};
///
/// let v = Vec4::new([[1.0, 0.0, 0.0, 1.0]]);
/// let rotate = rotate3d_z(std::f32::consts::PI / 2.0);
/// let v2 = rotate * v;
///
/// let ais = v2 - Vec4::new([[0.0, 1.0, 0.0, 0.0]]);
///
/// assert!(ais.x().abs() <= f32::EPSILON);
/// assert!(ais.y().abs() <= f32::EPSILON);
/// assert!(ais.z().abs() <= f32::EPSILON);
/// ```
#[inline]
pub fn rotate3d_z<T: Float>(angle: T) -> Mat4<T> {
    let c = angle.cos();
    let s = angle.sin();
    Mat {
        data: [
            [c, s, T::zero(), T::zero()],
            [-s, c, T::zero(), T::zero()],
            [T::zero(), T::zero(), T::unit_one(), T::zero()],
            [T::zero(), T::zero(), T::zero(), T::unit_one()],
        ],
    }
}

/// Create a 3D rotation matrix around `axis`.
///
/// # Example
/// ```
/// use mats::{Vec4, Vec3, graphics::rotate3d};
///
/// let v = Vec4::new([[1.0, 0.0, 0.0, 1.0]]);
/// let rotate = rotate3d(Vec3::new([[0.0, 1.0, 0.0]]), std::f32::consts::PI / 2.0);
/// let v2 = rotate * v;
///
/// let ais = v2 - Vec4::new([[0.0, 0.0, -1.0, 0.0]]);
///
/// assert!(ais.x().abs() <= f32::EPSILON);
/// assert!(ais.y().abs() <= f32::EPSILON);
/// assert!(ais.z().abs() <= f32::EPSILON);
/// ```
#[inline]
pub fn rotate3d<T: Float>(axis: Vec3<T>, angle: T) -> Mat4<T> {
    // Diary: Fix this function's BUG.
    //
    // For performance reasons, I try to minimize multiplication
    // calculations as much as possible. But during testing,
    // cubes always produce some inexplicable terrain changes.
    // I have checked the formula several times and have not
    // found any errors. When I had no choice, I threw it to AI.
    // AI told me that I used the value before normalization.
    //
    // AI is really powerful.

    // let xx = axis.x() * axis.x();
    // let yy = axis.y() * axis.y();
    // let zz = axis.z() * axis.z();
    // // Normalize the axis vector.
    // let m = xx + yy + zz;
    // axis /= m.sqrt();
    // // values
    // let c = angle.cos();
    // let s = angle.sin();
    // let c1 = T::unit_one() - c;
    // let xyc1 = axis.x() * axis.y() * c1;
    // let yzc1 = axis.y() * axis.z() * c1;
    // let xzc1 = axis.x() * axis.z() * c1;
    // let xs = axis.x() * s;
    // let ys = axis.y() * s;
    // let zs = axis.z() * s;
    // let xxc1 = xx * c1;
    // let yyc1 = yy * c1;
    // let zzc1 = zz * c1;
    // // result
    // Mat {
    //     data: [
    //         [c + xxc1, xyc1 + zs, xzc1 - ys, T::zero()],
    //         [xyc1 - zs, c + yyc1, yzc1 + xs, T::zero()],
    //         [xzc1 + ys, yzc1 - xs, c + zzc1, T::zero()],
    //         [T::zero(), T::zero(), T::zero(), T::unit_one()],
    //     ],
    // }

    // So now it's implemented like this.
    let axis = axis / axis.T().dot(&axis)[0].sqrt();
    unsafe { rotate3d_no_norm(axis, angle) }
}

/// Create a 3D rotation matrix around `axis` without normalizing it.
///
/// # Safety
/// This function does not check if `axis` is normalized.
///
/// # Example
/// ```
/// use mats::{Vec4, Vec3, graphics::rotate3d_no_norm};
///
/// let v = Vec4::new([[1.0, 0.0, 0.0, 1.0]]);
/// let rotate = unsafe { rotate3d_no_norm(Vec3::new([[0.0, 1.0, 0.0]]), std::f32::consts::PI / 2.0) };
/// let v2 = rotate * v;
///
/// let ais = v2 - Vec4::new([[0.0, 0.0, -1.0, 0.0]]);
///
/// assert!(ais.x().abs() <= f32::EPSILON);
/// assert!(ais.y().abs() <= f32::EPSILON);
/// assert!(ais.z().abs() <= f32::EPSILON);
/// ```
#[inline]
pub unsafe fn rotate3d_no_norm<T: Float>(axis: Vec3<T>, angle: T) -> Mat4<T> {
    let xx = axis.x() * axis.x();
    let yy = axis.y() * axis.y();
    let zz = axis.z() * axis.z();
    // values
    let c = angle.cos();
    let s = angle.sin();
    let c1 = T::unit_one() - c;
    let xyc1 = axis.x() * axis.y() * c1;
    let yzc1 = axis.y() * axis.z() * c1;
    let xzc1 = axis.x() * axis.z() * c1;
    let xs = axis.x() * s;
    let ys = axis.y() * s;
    let zs = axis.z() * s;
    let xxc1 = xx * c1;
    let yyc1 = yy * c1;
    let zzc1 = zz * c1;
    // result
    Mat {
        data: [
            [c + xxc1, xyc1 + zs, xzc1 - ys, T::zero()],
            [xyc1 - zs, c + yyc1, yzc1 + xs, T::zero()],
            [xzc1 + ys, yzc1 - xs, c + zzc1, T::zero()],
            [T::zero(), T::zero(), T::zero(), T::unit_one()],
        ],
    }
}

/// Create a view transformation matrix.
///
/// # Example
/// ```
/// use mats::{Vec3, Vec4, graphics::look_at};
///
/// let eye = Vec3::new([[1.0, 0.0, 1.0]]);
/// let center = Vec3::new([[0.0, 0.0, 1.0]]);
/// let up = Vec3::new([[0.0, 1.0, 0.0]]);
/// let view = look_at(eye, center, up);
///
/// let v = Vec4::new([[1.0, 0.0, 0.0, 1.0]]);
/// let v2 = view * v;
///
/// assert_eq!(v2.x(), 1.0);
/// assert_eq!(v2.y(), 0.0);
/// assert_eq!(v2.z(), 0.0);
/// ```
#[inline]
pub fn look_at<T: Float>(eye: Vec3<T>, center: Vec3<T>, up: Vec3<T>) -> Mat4<T> {
    let z = eye - center;
    let x = up.cross(&z);
    let y = z.cross(&x);

    // Normalize the vectors.
    let xx = z.x() * z.x();
    let yy = z.y() * z.y();
    let zz = z.z() * z.z();
    let z = z / (xx + yy + zz).sqrt();

    let xx = x.x() * x.x();
    let yy = x.y() * x.y();
    let zz = x.z() * x.z();
    let x = x / (xx + yy + zz).sqrt();

    let xx = y.x() * y.x();
    let yy = y.y() * y.y();
    let zz = y.z() * y.z();
    let y = y / (xx + yy + zz).sqrt();

    let a = -x.T().dot(&eye)[0];
    let b = -y.T().dot(&eye)[0];
    let c = -z.T().dot(&eye)[0];
    Mat {
        data: [
            [x.x(), y.x(), z.x(), T::zero()],
            [x.y(), y.y(), z.y(), T::zero()],
            [x.z(), y.z(), z.z(), T::zero()],
            [a, b, c, T::unit_one()],
        ],
    }
}

/// Create a perspective projection matrix.
///
/// # Example
/// ```
/// use mats::{Vec4, graphics::perspective};
///
/// let fov = std::f32::consts::PI / 2.0;
/// let aspect = 16.0 / 9.0;
/// let z_near = 0.1;
/// let z_far = 100.0;
/// let proj = perspective(fov, aspect, z_near, z_far);
///
/// let v = Vec4::new([[1.0, 0.0, 0.0, 1.0]]);
/// let v2 = proj * v;
///
/// assert_eq!(v2.x(), 9.0 / 16.0);
/// assert_eq!(v2.y(), 0.0);
/// assert_eq!(v2.z(), 200.0 / 999.0);
/// ```
pub fn perspective<T: Float>(fov: T, aspect: T, z_near: T, z_far: T) -> Mat4<T> {
    let two = T::unit_one() + T::unit_one();
    let f = T::unit_one() / (fov / two).tan();
    let ratio = T::unit_one() / (z_near - z_far);

    let m = (z_far + z_near) * ratio;
    let n = (two * z_far * z_near) * ratio;
    Mat {
        data: [
            [f / aspect, T::zero(), T::zero(), T::zero()],
            [T::zero(), f, T::zero(), T::zero()],
            [T::zero(), T::zero(), m, -T::unit_one()],
            [T::zero(), T::zero(), n, T::zero()],
        ],
    }
}

/// Create an orthographic projection matrix.
///
/// # Example
/// ```
/// use mats::{Vec4, graphics::orthographic};
///
/// let (left, top, right, bottom) = (-1.0, 1.0, 1.0, -1.0);
/// let z_near = -10.0;
/// let z_far = 10.0;
/// let proj = orthographic((left, top, right, bottom), z_near, z_far);
///
/// let v = Vec4::new([[1.0, 2.0, 2.0, 1.0]]);
/// let v2 = proj * v;
///
/// assert_eq!(v2.x(), 1.0);
/// assert_eq!(v2.y(), 2.0);
/// assert_eq!(v2.z(), -0.2);
/// ```
pub fn orthographic<T: Float>(
    (left, top, right, bottom): (T, T, T, T),
    z_near: T,
    z_far: T,
) -> Mat4<T> {
    let two = T::unit_one() + T::unit_one();
    let rl = T::unit_one() / (right - left);
    let tb = T::unit_one() / (top - bottom);
    let nf = T::unit_one() / (z_near - z_far);

    Mat {
        data: [
            [two * rl, T::zero(), T::zero(), -rl * (right + left)],
            [T::zero(), two * tb, T::zero(), -tb * (top + bottom)],
            [T::zero(), T::zero(), two * nf, -nf * (z_near + z_far)],
            [T::zero(), T::zero(), T::zero(), T::unit_one()],
        ],
    }
}
