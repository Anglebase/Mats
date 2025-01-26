/// This trait is a unified interface for setting shader variables.
///
/// To enable this feature, you need to add the following dependencies
/// to your Cargo.toml:
///
/// ```toml
/// [dependencies]
/// mats = { version = "...", features = ["glsl"] }
/// ```
///
/// The matrix memory layout in Mats is continuous, and it is also possible
/// to interact directly by calling the API from the address of the element
/// `[0][0]`, this trait just provides a unified interface.
/// It is important to note that **Mats's matrix is a row primary order matrix
/// and needs to be transposed when using API calls**.
pub trait SetUniform {
    fn give(&self, location: i32);
}

impl SetUniform for f32 {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform1f(location, *self);
        }
    }
}

impl SetUniform for f64 {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform1d(location, *self);
        }
    }
}

impl SetUniform for i32 {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform1i(location, *self);
        }
    }
}

impl SetUniform for u32 {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform1ui(location, *self);
        }
    }
}

mod uniform_f32;
mod uniform_f64;
mod uniform_i32;
mod uniform_u32;
