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

// Like GLSL syntax
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

/// (Vec2, T) -> Vec3
impl<T: Copy> From<(Vec2<T>, T)> for Vec3<T> {
    fn from((v, z): (Vec2<T>, T)) -> Self {
        Vec3::new([[v[0], v[1], z]])
    }
}

/// (T, Vec2) -> Vec3
impl<T: Copy> From<(T, Vec2<T>)> for Vec3<T> {
    fn from((x, v): (T, Vec2<T>)) -> Self {
        Vec3::new([[x, v[0], v[1]]])
    }
}

/// (Vec2, T, T) -> Vec4
impl<T: Copy> From<(Vec2<T>, T, T)> for Vec4<T> {
    fn from((v, z, w): (Vec2<T>, T, T)) -> Self {
        Vec4::new([[v[0], v[1], z, w]])
    }
}

/// (T, Vec2, T) -> Vec4
impl<T: Copy> From<(T, Vec2<T>, T)> for Vec4<T> {
    fn from((x, v, w): (T, Vec2<T>, T)) -> Self {
        Vec4::new([[x, v[0], v[1], w]])
    }
}

/// (T, T, Vec2) -> Vec4
impl<T: Copy> From<(T, T, Vec2<T>)> for Vec4<T> {
    fn from((x, y, v): (T, T, Vec2<T>)) -> Self {
        Vec4::new([[x, y, v[0], v[1]]])
    }
}

/// (Vec2, Vec2) -> Vec4
impl<T: Copy> From<(Vec2<T>, Vec2<T>)> for Vec4<T> {
    fn from((v1, v2): (Vec2<T>, Vec2<T>)) -> Self {
        Vec4::new([[v1[0], v1[1], v2[0], v2[1]]])
    }
}

/// (Vec3, T) -> Vec4
impl<T: Copy> From<(Vec3<T>, T)> for Vec4<T> {
    fn from((v, w): (Vec3<T>, T)) -> Self {
        Vec4::new([[v[0], v[1], v[2], w]])
    }
}

/// (T, Vec3) -> Vec4
impl<T: Copy> From<(T, Vec3<T>)> for Vec4<T> {
    fn from((x, v): (T, Vec3<T>)) -> Self {
        Vec4::new([[x, v[0], v[1], v[2]]])
    }
}
/// (T, T) -> Vec2
impl From<(i32, i32)> for Vec2<i32> {
    fn from((x, y): (i32, i32)) -> Self {
        Vec2::new([[x, y]])
    }
}
/// (T, T, T) -> Vec3
impl From<(i32, i32, i32)> for Vec3<i32> {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Vec3::new([[x, y, z]])
    }
}

/// (T, T, T, T) -> Vec4
impl<T: Copy> From<(T, T, T, T)> for Vec4<T> {
    fn from((x, y, z, w): (T, T, T, T)) -> Self {
        Vec4::new([[x, y, z, w]])
    }
}

/// [T;2] -> Vec2
impl<T: Copy> From<[T; 2]> for Vec2<T> {
    fn from(value: [T; 2]) -> Self {
        Vec2::new([[value[0], value[1]]])
    }
}

/// [T;3] -> Vec3
impl<T: Copy> From<[T; 3]> for Vec3<T> {
    fn from(value: [T; 3]) -> Self {
        Vec3::new([[value[0], value[1], value[2]]])
    }
}

/// [T;4] -> Vec4
impl<T: Copy> From<[T; 4]> for Vec4<T> {
    fn from(value: [T; 4]) -> Self {
        Vec4::new([[value[0], value[1], value[2], value[3]]])
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glsl() {
        let v1 = Vec3::new([[1.0, 2.0, 3.0]]);
        let v2 = Vec3::new([[4.0, 5.0, 6.0]]);

        assert_eq!(Vec3::from((v1.x(), v2.xy())), Vec3::new([[1.0, 4.0, 5.0]]));
        assert_eq!(Vec3::from((v1.yz(), v2.y())), Vec3::new([[2.0, 3.0, 5.0]]));
        assert_eq!(Vec4::from((v1.yy(), v2.zz())), Vec4::new([[2.0, 2.0, 6.0, 6.0]]));
    }
}
