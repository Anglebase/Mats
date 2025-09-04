# Mats

`Mats` is a matrix math library written in Rust, designed to provide lightweight, efficient, and easy-to-use matrix types and algorithms. It includes commonly used matrix algorithm encapsulation in fields such as linear algebra and computer graphics.

## Features

+ `uniforms`: If this feature is enabled, `Mats` will implement trait `AsUniformValue` for specific types of matrices, allowing them to interact with OpenGL.
+ `graphics`: If this feature is enabled, `Mats` will provide tools for computer graphics, such as coordinate transformation matrices.

## Example

```rust
use mats::Mat2;

fn main() {
    let a = Mat2::I();
    let b = Mat2::new([[1.0, 2.0], [3.0, 4.0]]);
    let c = a + b;
    println!("c = {:?}", c);
}
```

## License

Licensed under either of MIT license or Apache License, Version 2.0 at your option.

-------

# Release Notes

# Release v2.0.0

## Features
*A vector is a matrix with only one column.*
*The matrix is column-major*
+ Matrix:
    - Addition, subtraction, multiplication, division and their assignment of matrices.
    - Transposition of matrices.
    - Dot product of matrices.
    - Cross product of 3D vectors.
    - Like GLSL vector member access syntax.
    - Vector types alias.

+ GLSL Data Interface(with feature `uniforms`):
    - Implement trait `AsUniformValue`(from crate `glium`) for specific matrix types.

+ Computer Graphics(with feature `graphics`):
    - Coorinate transformations:
        - 2D/3D: Translation, rotation, scaling.
        - Projection: Orthographic, perspective.
        - Viewport transformation.


# Before v2.0.0

*This crate was originally born to solidify my Rust programming skills. Later, I tried some OpenGL tools and added matrix algorithms used in OpenGL to it. I think it could be more complete. So, starting from v2.0.0, I plan to refactor it based on my understanding of Rust updates, and make it a great choice for content related to matrix algorithms in Rust!*

## Futures

*I plan to refactor the project in the future to make it easier to use and maintain. In version 2.0.0, I will prioritize basic matrix operations and some basic operations, as well as parts related to computer graphics. More content related to linear algebra, as well as matrix calculations such as robot kinematics, robot inverse kinematics, and neural networks, will be added in the future.*