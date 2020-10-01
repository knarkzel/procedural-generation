# Procedural Generation

This is a crate for for procedurally generating maps written in Rust.
It's very elegant to use, see the example below:

```rust
use procedural_generation::Generator;

fn main() {
    Generator::new()
        .with_size(5, 10)
        .spawn_repeated(1, 5)
        .spawn_repeated(2, 3)
        .show();
}
```

This produces the following:

```bash
[0, 0, 0, 0, 0]
[2, 2, 0, 0, 0]
[0, 0, 0, 0, 0]
[0, 0, 0, 0, 1]
[1, 0, 0, 0, 0]
[0, 0, 0, 0, 1]
[1, 2, 2, 0, 0]
[1, 2, 2, 2, 2]
[0, 0, 2, 2, 2]
[0, 0, 0, 2, 2]
```
