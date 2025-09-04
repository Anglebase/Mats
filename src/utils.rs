use crate::Float;

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
