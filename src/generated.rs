use crate::types::*;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mat() {
        let v = Vec4::from([[1, 2, 3, 4]]);
        assert_eq!(v.xy(), Vec2::from([1, 2]));
        assert_eq!(v.xyz(), Vec3::from([1, 2, 3]));
        assert_eq!(v.xwzz(), Vec4::from([1, 4, 3, 3]));
    }
}
