use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct PerlinNoise {
    perm: Vec<usize>,
    octaves: usize,
    fallout: f64,
}

impl PerlinNoise {
    pub fn new() -> PerlinNoise {
        let mut rng = thread_rng();
        let mut perm: Vec<_> = (0..256).collect();

        perm.shuffle(&mut rng);
        perm.extend(perm.clone());

        PerlinNoise {
            perm,
            octaves: 4,
            fallout: 0.5,
        }
    }
    /// Return value is normalized to the range 0 -> 1 for 4 octaves
    pub fn get(&self, args: [f64; 2]) -> f64 {
        let mut effect = 1.0;
        let mut k = 1.0;
        let mut sum = 0.0;

        for _ in 0..self.octaves {
            effect *= self.fallout;
            sum += effect * ((1.0 + self.noise(k * args[0], k * args[1])) / 2.0);

            k *= 2.0
        }
        let normalized = (sum - 0.35) * (1. / 0.25);
        if normalized > 1. {
            1.
        } else if normalized < 0. {
            0.
        } else {
            normalized
        }
    }
    fn noise(&self, mut x: f64, mut y: f64) -> f64 {
        let x0 = (x.floor() as usize) & 255;
        let y0 = (y.floor() as usize) & 255;

        x -= x.floor();
        y -= y.floor();

        let fx = fade(x);
        let fy = fade(y);
        let p0 = self.perm[x0] + y0;
        let p1 = self.perm[x0 + 1] + y0;

        lerp(
            fy,
            lerp(
                fx,
                grad(self.perm[p0], x, y),
                grad(self.perm[p1], x - 1.0, y),
            ),
            lerp(
                fx,
                grad(self.perm[p0 + 1], x, y - 1.0),
                grad(self.perm[p1 + 1], x - 1.0, y - 1.0),
            ),
        )
    }
}

fn grad(hash: usize, x: f64, y: f64) -> f64 {
    let v = if hash & 1 == 0 { x } else { y };

    if (hash & 1) == 0 {
        -v
    } else {
        v
    }
}

fn lerp(t: f64, a: f64, b: f64) -> f64 {
    a + t * (b - a)
}

fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6. - 15.) + 10.)
}
