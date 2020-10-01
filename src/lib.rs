//! Utility for creating procedurally generated maps
//!
//! # Quick Start
//!
//! ```rust
//! use procedural_generation::Generator;
//!
//! fn main() {
//!     Generator::new()
//!         .with_size(5, 10)
//!         .spawn_repeated(1, 5)
//!         .spawn_repeated(2, 3)
//!         .show();
//! }
//! ```
//!
//! Produces the following:
//!
//! ```bash
//! [0, 0, 0, 0, 0]
//! [2, 2, 0, 0, 0]
//! [0, 0, 0, 0, 0]
//! [0, 0, 0, 0, 1]
//! [1, 0, 0, 0, 0]
//! [0, 0, 0, 0, 1]
//! [1, 2, 2, 0, 0]
//! [1, 2, 2, 2, 2]
//! [0, 0, 2, 2, 2]
//! [0, 0, 0, 2, 2]
//! ```

use rand::prelude::*;
use rand::rngs::ThreadRng;
use std::fmt;
use owo_colors::OwoColorize;

#[derive(Debug, Default)]
pub struct Generator {
    pub map: Vec<usize>,
    pub width: usize,
    pub height: usize,
    pub rng: ThreadRng,
}

impl Generator {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_size(mut self, width: usize, height: usize) -> Self {
        self.map = vec![0; width * height];
        self.width = width;
        self.height = height;
        self
    }
    pub fn show(&self) {
        println!("{}", self);
    }
    pub fn spawn(&mut self, number: usize) -> &mut Self {
        let start = self.rng.gen_range(0, self.map.len());
        self.map[start] = number;
        self.populate(start, 0.5);
        self
    }
    pub fn spawn_repeated(&mut self, number: usize, repeats: usize) -> &mut Self {
        for _ in 0..repeats {
            self.spawn(number);
        }
        self
    }
    pub fn populate(&mut self, start: usize, probability: f64) {
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
                let should_spawn = self.rng.gen_bool(probability);
                if should_spawn {
                    self.map[candidate] = number;
                    self.populate(candidate, probability / 2.);
                }
            }
        }
    }
}

impl fmt::Display for Generator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in 0..self.height {
            for y in 0..self.width {
                let value = self.map[x + y * self.height];
                let remainder = value % 8;
                match remainder {
                    1 => write!(f, "{:?} ", value.red())?,
                    2 => write!(f, "{:?} ", value.green())?,
                    3 => write!(f, "{:?} ", value.blue())?,
                    4 => write!(f, "{:?} ", value.cyan())?,
                    5 => write!(f, "{:?} ", value.magenta())?,
                    6 => write!(f, "{:?} ", value.white())?,
                    7 => write!(f, "{:?} ", value.yellow())?,
                    _ => write!(f, "{:?} ", value.black())?,
                }
                
            }
            if x < self.height - 1 {
                write!(f, "\n")?
            }
        }
        Ok(())
    }
}
