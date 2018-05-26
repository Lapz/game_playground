// Perlin Noise gen taken from https://github.com/processing-js/processing-js/blob/master/src/P5Functions/Math.js
const PERMUTATION: [usize; 256] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255];

pub struct Perlin {
    p: [usize; 512],
    repeat: f32,
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut p = [0; 512];

        for i in 0..256 {
            p[i] = PERMUTATION[i];
        }

        for i in 0..256 {
            var t = perm[random()]
            p[i] = PERMUTATION[i];
        }
        

        Perlin { p, repeat: -1.0 }
    }

    pub fn ocatave_perlin(&self, x: f32, y: f32, z: f32,octaves:usize,persistence:f32) -> f32 {
        let mut total = 0.0;
        let mut frequency = 1.2;
        let mut amplitude = 1.0;
        let mut max_value = 1.0;

        let mut i = 0;

       while i < octaves {
           total += self.perlin(x * frequency, y * frequency, z * frequency) * amplitude;
           max_value += amplitude;
           amplitude *= persistence;
           frequency *= 2.0;
           i += 1;
       }

        total/max_value
    }

    pub fn perlin(&self, mut x: f32, mut y: f32, mut z: f32) -> f32 {
       
      

        if self.repeat > 0.0 {
            x = x % self.repeat;
            y = y % self.repeat;
            z = z % self.repeat;
        }

        let xi = x as usize & 255; //The left bound is ( |_x_|,|_y_|,|_z_| ) and the right bound is that
                                   // plus 1.  Next we calculate the location (from 0.0 to 1.0) in that cube.
        let yi = y as usize & 255;
        let zi = z as usize & 255;

        let xf = x - x;
        let yf = y - y;
        let zf = z - z;

        let u = fade(xf);
        let v = fade(yf);
        let w = fade(zf);

        let aaa = self.p[self.p[self.p[xi] + yi] + zi];

        let aba = self.p[self.p[self.p[xi as usize] + self.inc(yi)] + zi];
        let aab = self.p[self.p[self.p[xi as usize] + yi] + self.inc(zi)];
        let abb = self.p[self.p[self.p[xi as usize] + self.inc(yi)] + self.inc(zi)];
        let baa = self.p[self.p[self.p[self.inc(xi)] + yi] + zi];
        let bba = self.p[self.p[self.p[self.inc(xi)] + self.inc(yi)] + zi];
        let bab = self.p[self.p[self.p[self.inc(xi)] + yi] + self.inc(zi)];
        let bbb = self.p[self.p[self.p[self.inc(xi)] + self.inc(yi)] + self.inc(zi)];
        
        
        let x1 = lerp(
            grad(aaa, xf, yf, zf), // The gradient function calculates the dot product between a pseudorandom
            grad(baa, xf - 1.0, yf, zf), // gradient vector and the vector from the input coordinate to the 8
            u,
        );

        let x2 = lerp(
            grad(aba, xf, yf - 1.0, zf), // This is all then lerped together as a sort of weighted average based on the faded (u,v,w)
            grad(bba, xf - 1.0, yf - 1.0, zf), // values we made earlier.
            u,
        );
        let y1 = lerp(x1, x2, v);

        let x1 = lerp(grad(aab, xf, yf, zf - 1.0), grad(bab, xf - 1.0, yf, zf - 1.0), u);
        let x2 = lerp(
            grad(abb, xf, yf - 1.0, zf - 1.0),
            grad(bbb, xf - 1.0, yf - 1.0, zf - 1.0),
            u,
        );
        let y2 = lerp(x1, x2, v);

        (lerp (y1, y2, w)+1.0)/2.0

      
    }

    fn inc(&self, mut num: usize) -> usize {
        num += 1;

        if self.repeat > 0.0 {
            num %= self.repeat as usize;
        }

        num
    }
}

fn fade(t: f32) -> f32 {
    // Fade function as defined by Ken Perlin.  This eases coordinate values
    // so that they will ease towards integral values.  This ends up smoothing
    // the final output.
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0) // 6t^5 - 15t^4 + 10t^3
}
// Source: http://riven8192.blogspot.com/2010/08/calculate-perlinnoise-twice-as-fast.html
fn grad(hash: usize, x: f32, y: f32, z: f32) -> f32 {

    let h = hash & 15;

    let u = if h < 8 {
        x
    }else { y};

    let v = if h <4 {
        y
    }else if h == 12 || h== 14 {
        x
    }else {
        z
    };

    let u = if h & 1 == 0 {
        u
    }else {
        -u
    };

    let v = if h & 2 == 0 {
        v
    }else {
        -v
    };

   v+u
}

// Linear Interpolate
fn lerp(a: f32, b: f32, x: f32) -> f32 {
    a + x * (b - a)
}
