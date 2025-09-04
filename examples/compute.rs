use mats::Mat2;

fn main() {
    let a = Mat2::I();
    let b = Mat2::new([[1.0, 2.0], [3.0, 4.0]]);
    let c = a + b;
    println!("c = {:?}", c);
}
