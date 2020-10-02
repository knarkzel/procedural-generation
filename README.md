# Procedural Generation

This is a crate for for procedurally generating maps written in Rust.
It's very elegant to use, see the example below:

```rust
use procedural_generation::Generator;

fn main() {
    let size = Size::new((4, 4), (10, 10));
    Generator::new()
        .with_size(30, 10)
        .spawn_terrain(1, 50)
        .spawn_rooms(2, 3, &size)
        .show();
}
```

Produces the following (prints with colors in terminal!):

![map](https://i.imgur.com/N37VPat.png)
