# Mats

**WARN**: Better crate: `nalgebra`. This crate has stopped maintenance.

> **An interesting discovery**: The maximum version number for Rust is 1844674407309551615(2^64-1).

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