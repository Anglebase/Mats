use super::SetUniform;
use crate::Mat;

impl SetUniform for Mat<f32, 1, 2> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform2fv(location, 1, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f32, 1, 3> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform3fv(location, 1, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f32, 1, 4> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform4fv(location, 1, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f32, 2, 2> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix2fv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f32, 3, 3> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix3fv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f32, 4, 4> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix4fv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f32, 2, 3> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix2x3fv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f32, 2, 4> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix2x4fv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f32, 3, 2> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix3x2fv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f32, 3, 4> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix3x4fv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f32, 4, 2> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix4x2fv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f32, 4, 3> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix4x3fv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}

impl SetUniform for crate::dynamic::Mat<f32> {
    fn give(&self, location: i32) {
        unsafe {
            match (self.rows(), self.cols()) {
                (1, 2) => gl::Uniform2fv(location, 1, &self[(0, 0)]),
                (1, 3) => gl::Uniform3fv(location, 1, &self[(0, 0)]),
                (1, 4) => gl::Uniform4fv(location, 1, &self[(0, 0)]),
                (2, 2) => gl::UniformMatrix2fv(location, 1, gl::TRUE, &self[(0, 0)]),
                (3, 3) => gl::UniformMatrix3fv(location, 1, gl::TRUE, &self[(0, 0)]),
                (4, 4) => gl::UniformMatrix4fv(location, 1, gl::TRUE, &self[(0, 0)]),
                (2, 3) => gl::UniformMatrix2x3fv(location, 1, gl::TRUE, &self[(0, 0)]),
                (2, 4) => gl::UniformMatrix2x4fv(location, 1, gl::TRUE, &self[(0, 0)]),
                (3, 2) => gl::UniformMatrix3x2fv(location, 1, gl::TRUE, &self[(0, 0)]),
                (3, 4) => gl::UniformMatrix3x4fv(location, 1, gl::TRUE, &self[(0, 0)]),
                (4, 2) => gl::UniformMatrix4x2fv(location, 1, gl::TRUE, &self[(0, 0)]),
                (4, 3) => gl::UniformMatrix4x3fv(location, 1, gl::TRUE, &self[(0, 0)]),
                _ => panic!("Invalid matrix size for uniform"),
            }
        }
    }
}
