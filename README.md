# Procedural Generation

This is a crate for for procedurally generating maps written in Rust.
It's very elegant to use and creates nice results, see the example below:

```rust
use procedural_generation::Generator;

fn main() {
    Generator::new()
        .with_size(40, 10)
        .spawn_perlin(|value| {
            if value > 0.66 {
                2
            } else if value > 0.33 {
                1
            } else {
                0
            }
        })
        .show();
}
```

Produces the following (prints with colors in terminal!):

![map](https://i.imgur.com/12OKFbC.png)
