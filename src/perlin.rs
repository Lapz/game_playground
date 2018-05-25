// Perlin Noise gen taken from http://flafla2.github.io/2014/08/09/perlinnoise.html
const PERMUTATION: [usize; 256] = [
    151,
    160,
    137,
    91,
    90,
    15, // Hash lookup table as defined by Ken Perlin.  This is a randomly
    131,
    13,
    201,
    95,
    96,
    53,
    194,
    233,
    7,
    225,
    140,
    36,
    103,
    30,
    69,
    142,
    8,
    99,
    37,
    240,
    21,
    10,
    23, // arranged array of all numbers from 0-255 self.inclusive.
    190,
    6,
    148,
    247,
    120,
    234,
    75,
    0,
    26,
    197,
    62,
    94,
    252,
    219,
    203,
    117,
    35,
    11,
    32,
    57,
    177,
    33,
    88,
    237,
    149,
    56,
    87,
    174,
    20,
    125,
    136,
    171,
    168,
    68,
    175,
    74,
    165,
    71,
    134,
    139,
    48,
    27,
    166,
    77,
    146,
    158,
    231,
    83,
    111,
    229,
    122,
    60,
    211,
    133,
    230,
    220,
    105,
    92,
    41,
    55,
    46,
    245,
    40,
    244,
    102,
    143,
    54,
    65,
    25,
    63,
    161,
    1,
    216,
    80,
    73,
    209,
    76,
    132,
    187,
    208,
    89,
    18,
    169,
    200,
    196,
    135,
    130,
    116,
    188,
    159,
    86,
    164,
    100,
    109,
    198,
    173,
    186,
    3,
    64,
    52,
    217,
    226,
    250,
    124,
    123,
    5,
    202,
    38,
    147,
    118,
    126,
    255,
    82,
    85,
    212,
    207,
    206,
    59,
    227,
    47,
    16,
    58,
    17,
    182,
    189,
    28,
    42,
    223,
    183,
    170,
    213,
    119,
    248,
    152,
    2,
    44,
    154,
    163,
    70,
    221,
    153,
    101,
    155,
    167,
    43,
    172,
    9,
    129,
    22,
    39,
    253,
    19,
    98,
    108,
    110,
    79,
    113,
    224,
    232,
    178,
    185,
    112,
    104,
    218,
    246,
    97,
    228,
    251,
    34,
    242,
    193,
    238,
    210,
    144,
    12,
    191,
    179,
    162,
    241,
    81,
    51,
    145,
    235,
    249,
    14,
    239,
    107,
    49,
    192,
    214,
    31,
    181,
    199,
    106,
    157,
    184,
    84,
    204,
    176,
    115,
    121,
    50,
    45,
    127,
    4,
    150,
    254,
    138,
    236,
    205,
    93,
    222,
    114,
    67,
    29,
    24,
    72,
    243,
    141,
    128,
    195,
    78,
    66,
    215,
    61,
    156,
    180,
];

pub struct Perlin {
    p: [usize; 512],
    repeat: f32,
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut p = [0; 512];

        for i in 0..512 {
            p[i] = PERMUTATION[i % 256];
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
