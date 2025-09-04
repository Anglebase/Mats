use crate::Mat;

/// A trait requires the type to have a unit one value.
pub trait UnitOne: Copy {
    /// Returns the unit one value.
    ///
    /// # Note
    /// For any type, the unit one value `I` should satisfy:
    /// + `I * I == I`
    /// + `I * A == A * I == A`
    /// + `A * A⁻¹ == I`
    ///
    /// Such as `1` for integers and `1.0` for floats.
    fn unit_one() -> Self;
}

macro_rules! impl_unit_one {
    ($($t:ty),*) => {
        $(
            impl UnitOne for $t {
                fn unit_one() -> Self {
                    1 as Self
                }
            }
        )*
    };
}
impl_unit_one!(
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64
);

/// A trait requires the type to have a zero value.
pub trait Zero: Copy {
    /// Returns the zero value.
    ///
    /// # Note
    /// For any type, the zero value `Z` should satisfy:
    /// + `Z + A == A + Z == A`
    /// + `Z * A == A * Z == Z`
    /// + 不存在 `Z⁻¹`
    fn zero() -> Self;
}

macro_rules! impl_zero {
    ($($t:ty), *) => {
        $(
            impl Zero for $t {
                fn zero() -> Self {
                    0 as Self
                }
            }
        )*
    };
}

impl_zero!(
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64
);

impl<T: Zero, const SIZE: usize> Zero for Mat<SIZE, SIZE, T> {
    /// Returns a zero matrix.
    ///
    /// # Example
    /// ```
    /// use mats::{Mat, Zero};
    ///
    /// let mat = Mat::<3, 3>::zero();
    ///
    /// assert_eq!(mat.raw_data(), &[[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]);
    /// ```
    #[inline]
    fn zero() -> Self {
        Self {
            data: [[T::zero(); SIZE]; SIZE],
        }
    }
}

impl<T, const SIZE: usize> UnitOne for Mat<SIZE, SIZE, T>
where
    T: UnitOne + Zero,
{
    /// Returns a identity matrix.
    ///
    /// # Example
    /// ```
    /// use mats::{Mat, UnitOne};
    ///
    /// let mat = Mat::<3, 3>::unit_one();
    ///
    /// assert_eq!(mat.raw_data(), &[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
    /// ```
    #[inline]
    fn unit_one() -> Self {
        let mut mat = Self::zero();
        for i in 0..SIZE {
            mat.data[i][i] = T::unit_one();
        }
        mat
    }
}

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
