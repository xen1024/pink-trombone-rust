use serde::{Serialize, Deserialize};
use serde_big_array::big_array;
use schemars::{JsonSchema};

big_array! { BigArray; N }

#[derive(Copy, Clone)]
#[cfg_attr(feature = "jsonse", derive(JsonSchema))]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "GradJSON")]
struct Grad {
    pub x: f32, pub y: f32, pub z: f32
}

impl Grad {
    pub fn dot2(&self, x: f32, y: f32) -> f32 { self.x * x + self.y * y }
}

const GRAD3: [Grad; 12] = [
    Grad{ x: 1.0, y: 1.0, z: 0.0}, Grad{x: -1.0, y: 1.0, z: 0.0}, Grad{x: 1.0, y: -1.0, z: 0.0}, Grad{x: -1.0, y: -1.0, z: 0.0 },
    Grad{ x: 1.0, y: 0.0, z: 1.0}, Grad{x: -1.0, y: 0.0, z: 1.0}, Grad{x: 1.0, y: 0.0, z: -1.0}, Grad{x:-1.0, y:0.0, z:-1.0},
    Grad{ x: 0.0, y: 1.0, z: 1.0 }, Grad{ x: 0.0, y: -1.0, z: 1.0 }, Grad{ x: 0.0, y: 1.0, z: -1.0}, Grad{x:0.0, y:-1.0, z:-1.0},
];

const P: [u8; 256] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36,
    103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0,
    26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56,
    87, 174, 20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77,
    146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245,
    40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89,
    18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173, 186, 3, 64,
    52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44,
    154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108,
    110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193,
    238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192,
    214, 31, 181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138,
    236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
];
/*
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoiseGenerator {
    #[serde(with = "BigArray")]
    grad_p: [Grad; 512],
    #[serde(with = "BigArray")]
    perm: [u8; 512],
}*/

#[cfg(feature = "jsonse")]
#[derive(JsonSchema)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "NoiseGeneratorJSON")]
pub struct NoiseGenerator {
    grad_p: [Grad; 16],
    perm: [u8; 16],
}

#[cfg(not(feature = "jsonse"))]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NoiseGenerator {
    #[serde(with = "BigArray")]
    grad_p: [Grad; 512],
    #[serde(with = "BigArray")]
    perm: [u8; 512],
}

impl NoiseGenerator {
    pub fn set_seed(&mut self, mut seed: u16) {
        if seed < 256 {
            seed |= seed << 8;
        }

        for i in 0..256 {
            let v = if (i & 1) == 1 {
                P[i] ^ (seed & 255) as u8
            } else {
                P[i] ^ ((seed >> 8) & 255) as u8
            };
            self.perm[i + 256] = v;
            self.perm[i] = v;
            let new_grad = GRAD3[v as usize % 12];
            self.grad_p[i + 256] = new_grad;
            self.grad_p[i] = new_grad;
        }
    }

    #[cfg(feature = "jsonse")]
    pub fn new(seed: u16) -> NoiseGenerator {
        let mut gen = NoiseGenerator {
            grad_p: [Grad{x:0.0, y: 0.0, z:0.0}; 16],
            perm: [0; 16],
        };
//        gen.set_seed(seed);
        gen
    }

    #[cfg(not(feature = "jsonse"))]
    pub fn new(seed: u16) -> NoiseGenerator {
        let mut gen = NoiseGenerator {
            grad_p: [Grad{x:0.0, y: 0.0, z:0.0}; 512],
            perm: [0; 512],
        };
        gen.set_seed(seed);
        gen
    }

    pub fn simplex(&mut self, x: f32) -> f32 { self.simplex2(x * 1.2, -x * 0.7) }

    fn simplex2(&mut self, xin: f32, yin: f32) -> f32 {
        // Skewing and unskewing factors for 2, 3, and 4 dimensions
        let f2: f32 = 0.5 * (3.0_f32.sqrt() - 1.0);
        let g2: f32 = (3.0 - 3.0_f32.sqrt()) / 6.0;
    
        // Skew the input space to determine which simplex cell we're in
        let s = (xin + yin) * f2; // Hairy factor for 2D
        let i = (xin + s).floor() as isize;
        let j = (yin + s).floor() as isize;
        let t = (i as f32 + j as f32) * g2;
        let x0 = xin - i as f32 + t; // The x,y distances from the cell origin, unskewed.
        let y0 = yin - j as f32 + t;
        // For the 2D case, the simplex shape is an equilateral triangle.
        // Determine which simplex we are in.
        let (i1, j1) = // Offsets for second (middle) corner of simplex in (i,j) coords
            if x0 > y0 { // lower triangle, XY order: (0,0)->(1,0)->(1,1)
                (1, 0)
            } else {    // upper triangle, YX order: (0,0)->(0,1)->(1,1)
                (0, 1)
            };
        // A step of (1,0) in (i,j) means a step of (1-c,-c) in (x,y), and
        // a step of (0,1) in (i,j) means a step of (-c,1-c) in (x,y), where
        // c = (3 - sqrt(3)) / 6
        let x1 = x0 - i1 as f32 + g2; // Offsets for middle corner in (x,y) unskewed coords
        let y1 = y0 - j1 as f32 + g2;
        let x2 = x0 - 1.0 + 2.0 * g2; // Offsets for last corner in (x,y) unskewed coords
        let y2 = y0 - 1.0 + 2.0 * g2;
        // Work out the hashed gradient indices of the three simplex corners
        let i_wrapped = i as u8 as usize;
        let j_wrapped = j as u8 as usize;
        let gi0 = self.grad_p[i_wrapped + self.perm[j_wrapped] as usize];
        let gi1 = self.grad_p[i_wrapped + i1 + self.perm[j_wrapped + j1] as usize];
        let gi2 = self.grad_p[i_wrapped + 1 + self.perm[j_wrapped + 1] as usize];
        // Noise contributions from the three corners
        let mut t0 = 0.5 - x0 * x0 - y0 * y0;
        let n0 = if t0 < 0.0 {
            0.0
        } else {
            t0 *= t0;
            t0 * t0 * gi0.dot2(x0, y0)  // (x,y) of grad3 used for 2D gradient
        };
        let mut t1 = 0.5 - x1 * x1 - y1 * y1;
        let n1 = if t1 < 0.0 {
            0.0
        } else {
            t1 *= t1;
            t1 * t1 * gi1.dot2(x1, y1)
        };
        let mut t2 = 0.5 - x2 * x2 - y2 * y2;
        let n2 = if t2 < 0.0 {
            0.0
        } else {
            t2 *= t2;
            t2 * t2 * gi2.dot2(x2, y2)
        };
        // Add contributions from each corner to get the final noise value.
        // The result is scaled to return values in the leterval [-1,1].
        70.0 * (n0 + n1 + n2)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn reproducible() {
        let mut generator = NoiseGenerator::new(15122);
        let vals: Vec<f32> = (0..112341).map(|i| generator.simplex(i as f32)).collect();
        assert_eq!(format!("{:.10}", vals.last().unwrap()), "0.8608906865");
    }
}