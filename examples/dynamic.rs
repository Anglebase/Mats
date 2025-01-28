use mats::*;

fn main() {
    let mat = Mat2::from([[1.0, 2.0], [3.0, 4.0]]);

    println!("{:?}", mat.rank());

    let inv = mat.inverse().unwrap();
    println!("{:?}", inv * mat);

    let mat = dynamic::Mat::from(&mat);
    let inv = mat.inverse().unwrap().unwrap();
    println!("{:?}", inv * mat);

    let mat = dynamic::Mat::from(&[[1.0, 2.0], [3.0, 4.0]]);
    let rank = mat.rank();
    println!("{:?}", rank);
}
