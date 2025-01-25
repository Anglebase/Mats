use mats::*;

fn main() {
    let vec = Vec4::from([1.0, 2.0, 3.0, 4.0]);
    let mat = Mat::from([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
        [13.0, 14.0, 15.0, 16.0],
    ]);
    println!("{:?}", vec);
    println!("{:?}", mat);
    let vec2 = vec * mat;
    println!("{:?}", vec2);
    let vec3 = Vec4::from((vec.xy(), vec2.zw()));
    println!("{:?}", vec3);
}
