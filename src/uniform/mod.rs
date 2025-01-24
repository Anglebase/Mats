/// 设置着色器uniform变量的trait
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
