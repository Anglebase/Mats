use super::SetUniform;
use crate::Mat;

impl SetUniform for Mat<i32, 1, 2> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform2iv(location, 1, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<i32, 1, 3> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform3iv(location, 1, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<i32, 1, 4> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform4iv(location, 1, self[0].as_ptr());
        }
    }
}

impl SetUniform for crate::dynamic::Mat<i32> {
    fn give(&self, location: i32) {
        unsafe {
            match (self.rows(), self.cols()) {
                (1, 2) => gl::Uniform2iv(location, 1, &self[(0, 0)]),
                (1, 3) => gl::Uniform3iv(location, 1, &self[(0, 0)]),
                (1, 4) => gl::Uniform4iv(location, 1, &self[(0, 0)]),
                _ => panic!("Invalid matrix size for uniform"),
            }
        }
    }
}
