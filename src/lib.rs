//! Utility for creating procedurally generated maps
//!
//! # Quick Start
//!
//! ```rust
//! use procedural_generation::Generator;
//!
//! fn main() {
//!     let size = Size::new((4, 4), (10, 10));
//!     Generator::new()
//!         .with_size(30, 10)
//!         .spawn_terrain(1, 50)
//!         .spawn_rooms(2, 3, &size)
//!         .show();
//! }
//! ```
//!
//! Produces the following (prints with colors in terminal!):
//!
//! ```bash
//! 0 0 0 0 0 1 1 0 0 0 1 1 1 0 0 0 1 0 1 1 1 1 1 1 0 0 0 1 1 0
//! 0 0 0 1 1 1 1 1 1 1 1 1 1 0 0 1 1 1 1 1 1 1 1 0 0 0 1 1 1 0
//! 0 0 0 0 1 1 0 1 1 1 0 0 1 1 0 1 1 0 0 1 0 2 2 2 2 2 2 0 0 0
//! 0 1 1 0 0 0 0 1 1 1 0 0 1 1 0 1 1 0 0 1 1 2 2 2 2 2 2 0 0 0
//! 0 0 0 0 0 1 1 0 1 1 0 1 1 1 1 0 0 0 0 1 1 2 2 2 2 2 2 0 0 0
//! 0 0 1 1 1 1 2 2 2 2 2 2 2 2 2 1 1 0 1 1 1 2 2 2 2 2 2 1 0 0
//! 0 1 1 1 1 1 2 2 2 2 2 2 2 2 2 1 0 0 0 1 0 2 2 2 2 2 2 1 1 0
//! 0 1 1 1 1 1 2 2 2 2 2 2 2 2 2 0 0 0 0 1 1 2 2 2 2 2 2 1 1 1
//! 0 1 1 1 1 1 2 2 2 2 2 2 2 2 2 1 1 1 0 1 0 2 2 2 2 2 2 0 1 1
//! 0 1 1 1 1 1 2 2 2 2 2 2 2 2 2 1 1 1 0 1 1 2 2 2 2 2 2 0 1 1
//! ```

use owo_colors::OwoColorize;
use rand::prelude::*;
use rand::rngs::ThreadRng;
use std::fmt;

/// The foundation of this crate
#[derive(Debug, Default)]
pub struct Generator {
    pub map: Vec<usize>,
    pub width: usize,
    pub height: usize,
    rooms: Vec<Room>,
}

impl Generator {
    pub fn new() -> Self {
        Self::default()
    }
    fn spawn_base(&mut self, number: usize, rng: &mut ThreadRng) -> usize {
        let start = rng.gen_range(0, self.map.len());
        self.map[start] = number;
        start
    }
    fn populate(&mut self, start: usize, probability: f64, rng: &mut ThreadRng) {
        let number = self.map[start];
        let candidates = vec![
            start.saturating_sub(1),
            start + 1,
            start.saturating_sub(self.width),
            start + self.width,
        ];
        for candidate in candidates {
            let remainder = candidate % self.width;
            if candidate > 0
                && candidate < self.map.len()
                && remainder > 0
                && remainder < self.width
            {
                let should_spawn = rng.gen_bool(probability);
                if should_spawn {
                    self.map[candidate] = number;
                    self.populate(candidate, probability / 2., rng);
                }
            }
        }
    }
    fn spawn_room(&mut self, number: usize, size: &Size, rng: &mut ThreadRng) -> &mut Self {
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
                    self.map[pos.0 + pos.1 * self.width] = number;
                }
            }
            self.rooms.push(room);
        }
        self
    }
    pub fn show(&self) {
        println!("{}", self);
    }
    pub fn get_2d_map(&self) -> Vec<Vec<usize>> {
        self.map.chunks(self.width).fold(vec![], |mut map, chunk| {
            map.push(chunk.into());
            map
        })
    }
    pub fn with_size(&mut self, width: usize, height: usize) -> &mut Self {
        self.map = vec![0; width * height];
        self.width = width;
        self.height = height;
        self
    }
    pub fn spawn_terrain(&mut self, number: usize, repeats: usize) -> &mut Self {
        let mut rng = rand::thread_rng();
        for _ in 0..repeats {
            let start = self.spawn_base(number, &mut rng);
            self.populate(start, 0.5, &mut rng);
        }
        self
    }
    pub fn spawn_rooms(&mut self, number: usize, rooms: usize, size: &Size) -> &mut Self {
        let mut rng = rand::thread_rng();
        for _ in 0..rooms {
            self.spawn_room(number, size, &mut rng);
        }
        self
    }
}

impl fmt::Display for Generator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let value = self.map[x + y * self.width];
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
        Self {
            min_size,
            max_size,
        }
    }
}

#[derive(Debug)]
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
