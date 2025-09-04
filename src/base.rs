use crate::{Mat, UnitOne, Zero};

impl<T: Default, const SIZE: usize> Mat<SIZE, SIZE, T>
where
    T: UnitOne + Zero,
{
    /// Returns a new identity matrix.
    ///
    /// # Example
    /// ```
    /// use mats::Mat;
    ///
    /// let mat = Mat::<3, 3>::new_identity();
    ///
    /// assert_eq!(mat.raw_data(), &[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
    /// ```
    #[inline]
    pub fn new_identity() -> Self {
        Self::unit_one()
    }

    /// Returns the identity matrix.
    ///
    /// This is a shorthand for `Mat::new_identity()`.
    ///
    /// # Example
    /// ```
    /// use mats::Mat;
    ///
    /// let mat = Mat::<3, 3>::I();
    ///
    /// assert_eq!(mat.raw_data(), &[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
    /// ```
    #[allow(non_snake_case)]
    #[inline]
    pub fn I() -> Self {
        Self::unit_one()
    }
}
