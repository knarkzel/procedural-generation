//! Utility for creating procedurally generated maps
//!
//! # Quick Start
//!
//! ```rust
//! use procedural_generation::Generator;
//!
//! fn main() {
//!     Generator::new()
//!         .with_size(40, 10)
//!         .spawn_perlin(|value| {
//!             if value > 0.66 {
//!                 2
//!             } else if value > 0.33 {
//!                 1
//!             } else {
//!                 0
//!             }
//!         })
//!         .show();
//! }
//! ```
//!
//! Produces the following (prints with colors in terminal!):
//!
//! ```bash
//! 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
//! 1 1 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 1 1 1 1 2 2 2 2 2 2 2 2 2 2 2 2 2 2 1
//! 1 1 1 1 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2
//! 1 1 1 1 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2
//! 1 1 1 1 1 1 1 1 1 1 1 1 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2
//! 1 1 1 1 1 1 1 1 1 1 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2
//! 1 1 1 1 1 1 1 1 1 1 1 1 2 2 2 2 2 2 2 2 2 1 1 1 1 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2
//! 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 2 2 2 2 2 2 2 2 2 1 1 1 1
//! 0 0 0 0 0 0 0 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 2 2 2 2 2 2 2 2 2 2 1 1 1 1
//! 0 0 0 0 0 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 2 2 2 2 2 2 2 2 2 2 2 2 1 1 1
//! ```

use owo_colors::OwoColorize;
use rand::prelude::*;
use noise::{Perlin, NoiseFn, Seedable};
use smart_default::*;
use rayon::prelude::*;
use std::fmt;

/// Different options for defining how noise should behave. 
#[derive(Debug, SmartDefault)]
pub struct NoiseOptions {
    /// Higher frequency adds a zooming effect to the noise. Default is 1.0.
    #[default = 1.0]
    pub frequency: f64,
    #[default = 1.0]
    /// Higher redistribution exaggerates higher peaks and lower lows. Default is 1.0.
    pub redistribution: f64,
    /// More octaves increases variety. Default is 1.
    #[default = 1]
    pub octaves: usize,
}

impl NoiseOptions {
    /// Creates `NoiseOptions`. See [Making maps with noise functions](https://www.redblobgames.com/maps/terrain-from-noise/)
    /// for more detail on each option. Usage of `NoiseOptions` looks like this:
    ///
    /// ```rust
    /// use procedural_generation::*;
    ///
    /// fn main() {
    ///     let noise_options = NoiseOptions { frequency: 4., ..NoiseOptions::default() };
    ///     Generator::new()
    ///         .with_size(40, 10)
    ///         .with_options(noise_options)
    ///         .spawn_perlin(|value| {
    ///             if value > 0.5 {
    ///                 1
    ///             } else {
    ///                 0
    ///             }
    ///         })
    ///         .show();
    /// }
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
}

/// The foundation of this crate
#[derive(Debug, Default)]
pub struct Generator {
    pub map: Vec<usize>,
    pub width: usize,
    pub height: usize,
    pub noise_options: NoiseOptions,
    rooms: Vec<Room>,
    seed: u32,
}

impl Generator {
    /// Create generator.
    pub fn new() -> Self {
        let seed: u32 = rand::thread_rng().gen();
        Self {
            seed,
            ..Self::default()
        }
    }
    fn spawn_room(&mut self, number: usize, size: &Size, rng: &mut StdRng) -> &mut Self {
        let mut x = rng.gen_range(0, self.width);
        let mut y = rng.gen_range(0, self.height);

        let width = rng.gen_range(size.min_size.0, size.max_size.0);
        let height = rng.gen_range(size.min_size.1, size.max_size.1);

        // shift room back on if it's off
        if x + width > self.width {
            x = self.width - width;
        }

        // shift room back on if it's off
        if y + height > self.height {
            y = self.height - height;
        }

        let mut collides = false;
        let room = Room::new(x, y, width, height);

        for other_room in &self.rooms {
            if room.intersects(&other_room) {
                collides = true;
                break;
            }
        }

        if !collides {
            for row in 0..height {
                for col in 0..width {
                    let pos = (room.x + col, room.y + row);
                    self.set(pos.0, pos.1, number);
                }
            }
            self.rooms.push(room);
        }
        self
    }
    /// Set seed for noise generation. Useful for reproducing results. Random otherwise.
    pub fn with_seed(mut self, seed: u32) -> Self {
        self.seed = seed;
        self
    }
    /// Changes how noise is generated. Different values make for much more interesting noise
    pub fn with_options(mut self, options: NoiseOptions) -> Self {
        self.noise_options = options;
        self
    }
    /// Prints the map to stdout with colors.
    pub fn show(&self) {
        println!("{}", self);
    }
    /// Sets size of map. This clears the map as well.
    pub fn with_size(mut self, width: usize, height: usize) -> Self {
        self.map = vec![0; width * height];
        self.width = width;
        self.height = height;
        self
    }
    /// Generates perlin noise over the entire map.
    /// For every coordinate, the closure `f(f64)` receives a value
    /// between 0 and 1. This closure must then return a usize
    /// accordingly to what value it receives, such as the following.
    /// You can also modify some options for how the noise should behave,
    /// see [NoiseOptions](struct.NoiseOptions.html).
    ///
    /// ```rust
    /// use procedural_generation::*;
    /// 
    /// fn main() {
    ///     Generator::new()
    ///         .with_size(40, 20)
    ///         .spawn_perlin(|value| {
    ///             if value > 0.66 {
    ///                 2
    ///             } else if value > 0.33 {
    ///                 1
    ///             } else {
    ///                 0
    ///             }
    ///         })
    ///         .show();
    /// }
    /// ```
    pub fn spawn_perlin<F: Fn(f64) -> usize + Sync>(mut self, f: F) -> Self {
        let perlin = Perlin::new().set_seed(self.seed);
        let redistribution = self.noise_options.redistribution;
        let freq = self.noise_options.frequency;
        let octaves = self.noise_options.octaves;
        let width = self.width;

        self.map.par_iter_mut().enumerate().for_each(|(pos, index)| {
            let x = pos % width;
            let y = pos / width;

            let nx = x as f64 / width as f64;
            let ny = y as f64 / width as f64;

            let value = (0..octaves).fold(0., |acc, n| {
                let power = 2.0f64.powf(n as f64);
                let modifier = 1. / power;
                acc + modifier * perlin.get([nx * freq * power, ny * freq * power])
            });

            // add redistribution, map range from -1, 1 to 0, 1 then parse
            // biome and set it
            *index = f((value.powf(redistribution) + 1.) / 2.);
        });
        self
    }
    /// Spawns rooms of varying sizes based on input `size`. `number` sets
    /// what number the rooms are represented with in the map, `rooms` is amount of rooms
    /// to generate and `size` specifies the minimum and maximum boundaries for each room.
    /// Shoutouts to this guy: [Procedural level generation with Rust](https://www.jamesbaum.co.uk/blether/procedural-level-generation-rust/).
    ///
    /// ```rust
    /// use procedural_generation::*;
    ///
    /// fn main() {
    ///     let size = Size::new((4, 4), (10, 10));
    ///     Generator::new()
    ///         .with_size(30, 20)
    ///         .spawn_rooms(2, 3, &size)
    ///         .show();
    /// }
    /// ```
    pub fn spawn_rooms(mut self, number: usize, rooms: usize, size: &Size) -> Self {
        let mut rng = SeedableRng::seed_from_u64(self.seed as u64);
        // let mut rng = rand::thread_rng();
        for _ in 0..rooms {
            self.spawn_room(number, size, &mut rng);
        }
        self
    }
    /// Returns value at (x, y) coordinate, useful since map is in 1d form
    /// but treated as 2d.
    pub fn get(&self, x: usize, y: usize) -> usize {
        self.map[x + y * self.width]
    }
    /// Same as `get(...)`, except sets value.
    pub fn set(&mut self, x: usize, y: usize, value: usize) {
        self.map[x + y * self.width] = value;
    }
    /// This is not recommended unless it's convenient or necessary,
    /// as 2d vectors are slow.
    pub fn get_2d_map(&self) -> Vec<Vec<usize>> {
        self.map.chunks(self.width).fold(vec![], |mut map, chunk| {
            map.push(chunk.into());
            map
        })
    }
}

impl fmt::Display for Generator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let value = self.get(x, y);
                let remainder = value % 7;
                match remainder {
                    1 => write!(f, "{:?} ", value.red())?,
                    2 => write!(f, "{:?} ", value.green())?,
                    3 => write!(f, "{:?} ", value.cyan())?,
                    4 => write!(f, "{:?} ", value.magenta())?,
                    5 => write!(f, "{:?} ", value.white())?,
                    6 => write!(f, "{:?} ", value.yellow())?,
                    _ => write!(f, "{:?} ", value.blue())?,
                }
            }
            if y < self.height - 1 {
                write!(f, "\n")?
            }
        }
        Ok(())
    }
}

/// Size constraints for spawning rooms
pub struct Size {
    /// First option is width, second option is height
    pub min_size: (usize, usize),
    /// First option is width, second option is height
    pub max_size: (usize, usize),
}

impl Size {
    pub fn new(min_size: (usize, usize), max_size: (usize, usize)) -> Self {
        Self { min_size, max_size }
    }
}

#[derive(Debug, Default)]
struct Room {
    x: usize,
    y: usize,
    x2: usize,
    y2: usize,
    width: usize,
    height: usize,
}

impl Room {
    fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Room {
            x,
            y,
            x2: x + width,
            y2: y + height,
            width,
            height,
        }
    }
    fn intersects(&self, other: &Self) -> bool {
        self.x <= other.x2 && self.x2 >= other.x && self.y <= other.y2 && self.y2 >= other.y
    }
}

#[cfg(test)]
mod tests {
    // Test for both seeding and implementation changes.
    #[test]
    fn perlin() {
        use super::*;
        let generator = Generator::new()
            .with_size(40, 10)
            .with_seed(0)
            .spawn_perlin(|value| {
                if value > 0.66 {
                    2
                } else if value > 0.33 {
                    1
                } else {
                    0
                }
            });
        let output = vec![
            1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
            1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
            1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
            1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
            1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
            2,2,2,2,2,2,2,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,
            2,2,2,2,2,2,2,2,2,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,
            2,2,2,2,2,2,2,2,2,2,2,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,
            2,2,2,2,2,2,2,2,2,2,2,2,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,
            2,2,2,2,2,2,2,2,2,2,2,2,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,
        ];
        assert_eq!(generator.map, output);
    }
    #[test]
    fn rooms() {
        use super::*;
        let size = Size::new((4, 4), (10, 10));
        let generator = Generator::new()
            .with_size(40, 10)
            .with_seed(0)
            .spawn_rooms(1, 5, &size);
        let output = vec![
            0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,0,0,1,1,1,1,1,
            0,0,0,0,0,0,0,0,0,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,0,0,1,1,1,1,1,
            0,0,0,0,0,0,0,0,0,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,0,0,1,1,1,1,1,
            0,0,0,0,0,0,0,0,0,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,0,0,1,1,1,1,1,
            0,0,0,0,0,0,0,0,0,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,0,0,1,1,1,1,1,
            0,0,0,0,0,0,0,0,0,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,0,0,1,1,1,1,1,
            0,0,0,0,0,0,0,0,0,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,0,0,1,1,1,1,1,
            0,0,0,0,0,0,0,0,0,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,0,0,1,1,1,1,1,
            0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,
        ];
        assert_eq!(generator.map, output);
    }
}
