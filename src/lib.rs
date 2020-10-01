use rand::prelude::*;
use rand::rngs::ThreadRng;

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
        for i in 0..self.height {
            let anchor = i * self.width;
            let slice = &self.map[anchor..anchor + self.width];
            println!("{:?}", slice);
        }
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
        let candidates = vec![start.saturating_sub(1), start + 1, start.saturating_sub(self.width), start + self.width];
        for candidate in candidates {
            let remainder = candidate % self.width;
            if candidate > 0 && candidate < self.map.len() && remainder > 0 && remainder < self.width {
                let should_spawn = self.rng.gen_bool(probability);
                if should_spawn {
                    self.map[candidate] = number;
                    self.populate(candidate, probability / 2.);
                }
            }
        }
    }
}
