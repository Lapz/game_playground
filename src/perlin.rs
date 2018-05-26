// PerlinNoiseNoise gen taken from https://github.com/processing-js/processing-js/blob/master/src/P5Functions/Math.js
const PERMUTATION: [usize; 256] = [0; 256];

use rand::prelude::*;

pub struct PerlinNoise {
    perm: [usize; 512],
    octaves: usize,
    fallout:f64,
}

impl PerlinNoise {
    pub fn new() -> PerlinNoise {
        let mut perm = [0; 512];
        let mut rng = thread_rng();

        for i in 0..256 {
            perm[i] = i;
        }

        for i in 0..256 {
            let j = rng.gen_range(0, 256) & 0xFF;
            let t = perm[j];

            perm[j] = perm[i];
            perm[i] = t;
        }

        for i in 0..256 {
            perm[i + 256] = perm[i];
        }

        
        PerlinNoise { perm, octaves:4,fallout:0.5  }
    }

    pub fn get3d(&self,args:[f64;3]) -> f64 {
        let mut effect =1.0;
        let mut  k =1.0;
        let mut sum = 0.0;

        for _ in 0..self.octaves {
            effect *= self.fallout;
            sum += effect * (1.0 + self.noise3d(k*args[0], k*args[1], k*args[2]))/2.0;
            k*= 2.0

        }

        sum
    }

    pub fn get2d(&self,args:[f64;2]) -> f64 {
        let mut effect =1.0;
        let mut  k =1.0;
        let mut sum = 0.0;

        for _ in 0..self.octaves {
            effect *= self.fallout;
            sum += effect * ( (1.0 + self.noise2d(k*args[0], k*args[1]))/2.0);
           
            k*= 2.0

        }

        sum
    }

    pub fn get(&self,x:f64) -> f64 {
        let mut effect = 1.0;
        let mut  k =1.0;
        let mut sum = 0.0;

        for _ in 0..self.octaves {
            effect *= self.fallout;
            sum += effect * ((1.0 + self.noise1d(k*x,))/2.0);
            k *= 2.0
        }

        sum
    }

    fn noise3d(&self, mut x: f64, mut y: f64, mut z: f64) -> f64 {
        let X = (x as usize & 255);
        let Y = (y as usize & 255);
        let Z = (z as usize & 255);

        x -= x;
        y -= y;
        z -= z;

        let fx = (3.0 - 2.0 * x) * x * x;
        let fy = (3.0 - 2.0 * y) * y * y;
        let fz = (3.0 - 2.0 * z) * z * z;

        let p0 = self.perm[X] + Y;
        let p00 = self.perm[p0] + Z;
        let p01 = self.perm[p0 + 1] + Z;
        let p1 = self.perm[X + 1] + Y;
        let p10 = self.perm[p1] + Z;
        let p11 = self.perm[p1 + 1] + Z;

        lerp(
            fz,
            lerp(
                fy,
                lerp(
                    fx,
                    grad3d(self.perm[p00], x, y, z),
                    grad3d(self.perm[p10], x - 1.0, y, z),
                ),
                lerp(
                    fx,
                    grad3d(self.perm[p01], x, y - 1.0, z),
                    grad3d(self.perm[p11], x - 1.0, y - 1.0, z),
                ),
            ),
            lerp(
                fy,
                lerp(
                    fx,
                    grad3d(self.perm[p00 + 1], x, y, z - 1.0),
                    grad3d(self.perm[p10 + 1], x - 1.0, y, z - 1.0),
                ),
                lerp(
                    fx,
                    grad3d(self.perm[p01 + 1], x, y - 1.0, z - 1.0),
                    grad3d(self.perm[p11 + 1], x - 1.0, y - 1.0, z - 1.0),
                ),
            ),
        )
    }

    fn noise2d(&self,mut x: f64, mut y: f64) -> f64 {
        let X = (x as usize & 255);
        let Y = (y as usize & 255);

        x -= x;
        y -= y;

        let fx = (3.0 - 2.0 * x) * x * x;
        let fy = (3.0 - 2.0 * y) * y * y;
        let p0 = self.perm[X] + Y;
        let p1 = self.perm[X + 1] + Y;

        lerp(
            fy,
            lerp(fx, grad2d(self.perm[p0], x, y), grad2d(self.perm[p1], x - 1.0, y)),
            lerp(
                fx,
                grad2d(self.perm[p0 + 1], x, y - 1.0),
                grad2d(self.perm[p1 + 1], x - 1.0, y - 1.0),
            ),
        )
    }

    fn noise1d(&self,mut x:f64) -> f64 {
           let X = x.floor() as usize & 255;
           println!("before {:?}",x);
            let x = x.floor();
           println!("after {:?}",x);
           let fx = (3.0 - 2.0 * x) * x * x;
           lerp(fx, grad1d(self.perm[X], x), grad1d(self.perm[X+1], x-1.0))
    }
}

// Source: http://riven8192.blogspot.com/2010/08/calculate-perlinnoise-twice-as-fast.html
fn grad3d(hash: usize, x: f64, y: f64, z: f64) -> f64 {
    let h = hash & 15;

    let u = if h < 8 { x } else { y };

    let v = if h < 4 {
        y
    } else if h == 12 || h == 14 {
        x
    } else {
        z
    };

    let u = if h & 1 == 0 { u } else { -u };

    let v = if h & 2 == 0 { v } else { -v };

    v + u
}

fn grad2d(hash: usize, x: f64, y: f64) -> f64 {
    let v = if hash & 1 == 0 { x } else { y };

    if (hash & 1) == 0 {
        -v
    } else {
        v
    }
}

fn grad1d(hash: usize, x: f64) -> f64 {
    if (hash & 1) == 0 {
        -x
    } else {
        x
    }
}

// Linear Interpolate
fn lerp(a: f64, b: f64, x: f64) -> f64 {
    a + x * (b - a)
}
