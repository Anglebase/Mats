use crate::Mat;

/// A 2x2 matrix with elements of type `T`.
pub type Mat2<T = f32> = Mat<2, 2, T>;
/// A 3x3 matrix with elements of type `T`.
pub type Mat3<T = f32> = Mat<3, 3, T>;
/// A 4x4 matrix with elements of type `T`.
pub type Mat4<T = f32> = Mat<4, 4, T>;

/// A 2x3 matrix with elements of type `T`.
pub type Mat2x3<T = f32> = Mat<2, 3, T>;
/// A 2x4 matrix with elements of type `T`.
pub type Mat2x4<T = f32> = Mat<2, 4, T>;
/// A 3x2 matrix with elements of type `T`.
pub type Mat3x2<T = f32> = Mat<3, 2, T>;
/// A 3x4 matrix with elements of type `T`.
pub type Mat3x4<T = f32> = Mat<3, 4, T>;
/// A 4x2 matrix with elements of type `T`.
pub type Mat4x2<T = f32> = Mat<4, 2, T>;
/// A 4x3 matrix with elements of type `T`.
pub type Mat4x3<T = f32> = Mat<4, 3, T>;

/// A vector with `N` elements of type `T`.
pub type Vec<const N: usize, T = f32> = Mat<N, 1, T>;

/// A 2-dimensional vector with elements of type `T`.
pub type Vec2<T = f32> = Vec<2, T>;
/// A 3-dimensional vector with elements of type `T`.
pub type Vec3<T = f32> = Vec<3, T>;
/// A 4-dimensional vector with elements of type `T`.
pub type Vec4<T = f32> = Vec<4, T>;

/// If the `uniforms` feature is enabled, this module provides implementations of the `AsUniformValue`
/// trait for the types defined in this module.
/// 
/// This allows for the use of these types as uniform values in shaders (in crate `glium`).
#[cfg(feature = "uniforms")]
mod uniforms {
    use glium::uniforms::{AsUniformValue, UniformValue};

    use super::*;

    impl AsUniformValue for Vec2<f32> {
        fn as_uniform_value(&self) -> UniformValue<'_> {
            UniformValue::Vec2(self.data[0])
        }
    }

    impl AsUniformValue for Vec2<f64> {
        fn as_uniform_value(&self) -> UniformValue<'_> {
            UniformValue::DoubleVec2(self.data[0])
        }
    }

    impl AsUniformValue for Vec3<f32> {
        fn as_uniform_value(&self) -> UniformValue<'_> {
            UniformValue::Vec3(self.data[0])
        }
    }

    impl AsUniformValue for Vec3<f64> {
        fn as_uniform_value(&self) -> UniformValue<'_> {
            UniformValue::DoubleVec3(self.data[0])
        }
    }

    impl AsUniformValue for Vec4<f32> {
        fn as_uniform_value(&self) -> UniformValue<'_> {
            UniformValue::Vec4(self.data[0])
        }
    }

    impl AsUniformValue for Vec4<f64> {
        fn as_uniform_value(&self) -> UniformValue<'_> {
            UniformValue::DoubleVec4(self.data[0])
        }
    }

    impl AsUniformValue for Mat2<f32> {
        fn as_uniform_value(&self) -> UniformValue<'_> {
            UniformValue::Mat2(self.data)
        }
    }

    impl AsUniformValue for Mat2<f64> {
        fn as_uniform_value(&self) -> UniformValue<'_> {
            UniformValue::DoubleMat2(self.data)
        }
    }

    impl AsUniformValue for Mat3<f32> {
        fn as_uniform_value(&self) -> UniformValue<'_> {
            UniformValue::Mat3(self.data)
        }
    }

    impl AsUniformValue for Mat3<f64> {
        fn as_uniform_value(&self) -> UniformValue<'_> {
            UniformValue::DoubleMat3(self.data)
        }
    }

    impl AsUniformValue for Mat4<f32> {
        fn as_uniform_value(&self) -> UniformValue<'_> {
            UniformValue::Mat4(self.data)
        }
    }

    impl AsUniformValue for Mat4<f64> {
        fn as_uniform_value(&self) -> UniformValue<'_> {
            UniformValue::DoubleMat4(self.data)
        }
    }
}
