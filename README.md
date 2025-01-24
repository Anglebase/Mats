# Mats

这个 crate 包含了矩阵及其常用数学操作的实现

如果启用特性 `glsl` ，那么可以通过它的 trait `SetUniform` 与 GLSL 中的 uniform 变量交互

# 示例

这是一个简单的示例
```rust
use mats::*;

fn main() {
    let vec = Vec4::<f32>::from([1.0, 2.0, 3.0, 4.0]);
    let mat = Mat::<f32, 4, 4>::from([
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
```