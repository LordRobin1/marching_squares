use std::ops::Add;

use crate::balls::*;
use pixel_lib::*;

pub struct Square {
    pub origin: Point,
    pub dimension: f32,
}

struct WeightedPoint {
    x: f32,
    y: f32,
    weight: f32,
}

/// T: top, B: bottom, l: left, r: right
enum Case {
    Full,
    Empty,
    Tl,
    Tr,
    Bl,
    Br,
    TlBl,
    TrBr,
    TlTr,
    BlBr,
    TlBr,
    TrBl,
    TlTrBl,
    TlTrBr,
    TrBlBr,
    TlBrBl,
}

impl Square {
    pub fn march(&mut self, buffer: &mut [u32], balls: &[Ball], width: u32) {
        let formula = |p: Point| -> f32 {
            let mut sum = 0.;
            for ball in balls {
                let influence = (ball.radius.powi(2) / p.sq_distance(&ball.position)).clamp(0., 2.);
                sum += influence.clamp(0., 1.);
            }
            sum
        };
        let tl = formula(self.origin);
        let tr = formula(self.origin.add(&Point {
            x: self.dimension,
            y: 0.,
        }));
        let bl = formula(self.origin.add(&Point {
            x: 0.,
            y: self.dimension,
        }));
        let br = formula(self.origin.add(&Point {
            x: self.dimension,
            y: self.dimension,
        }));
        let case = self.shade(tl, tr, bl, br);
        if let Case::Full = case {
            let y_range = self.origin.y as u32..(self.origin.y + self.dimension) as u32;

            for mut y in y_range {
                let x_range = self.origin.x as u32..(self.origin.x + self.dimension) as u32;

                for mut x in x_range {
                    let i = (x + y * width) as usize;
                    if i >= buffer.len() {
                        break;
                    }
                    buffer[i] = Color {
                        r: 0.,
                        g: 0.8,
                        b: 0.0,
                        a: 1.,
                    }
                    .as_u32();
                }
            }
        }
    }

    fn shade(&self, tl: f32, tr: f32, bl: f32, br: f32) -> Case {
        let tl_point = WeightedPoint {
            x: self.origin.x,
            y: self.origin.y,
            weight: tl,
        };
        let tr_point = WeightedPoint {
            x: self.origin.x + self.dimension,
            y: self.origin.y,
            weight: tr,
        };
        let bl_point = WeightedPoint {
            x: self.origin.x,
            y: self.origin.y + self.dimension,
            weight: bl,
        };
        let br_point = WeightedPoint {
            x: self.origin.x + self.dimension,
            y: self.origin.y + self.dimension,
            weight: br,
        };
        if tl > 1. || tr > 1. || bl > 1. || br > 1. {
            return Case::Full;
        } else {
            return Case::Empty;
        }
        match (tl > 1., tr > 1., bl > 1., br > 1.) {
            (true, true, true, true) => Case::Full,
            (false, false, false, false) => Case::Empty,
            (true, false, false, false) => {
                // Tl inside, rest outside
                self.compute_intersection((&tl_point, &tr_point), (&tl_point, &bl_point));
                Case::Tl
            }
            (false, true, false, false) => Case::Tr,
            (false, false, true, false) => Case::Bl,
            (false, false, false, true) => Case::Br,
            (true, false, true, false) => Case::TlBl,
            (false, true, false, true) => Case::TrBr,
            (true, true, false, false) => Case::TlTr,
            (false, false, true, true) => Case::BlBr,
            (true, false, false, true) => Case::TlBr,
            (false, true, true, false) => Case::TrBl,
            (true, true, true, false) => Case::TlTrBl,
            (true, true, false, true) => Case::TlTrBr,
            (true, false, true, true) => Case::TlBrBl,
            (false, true, true, true) => Case::TrBlBr,
        }
    }

    /// Reducible: https://www.youtube.com/watch?v=6oMZb3yP_H8&t=1036s
    fn compute_intersection(
        &self,
        pair1: (&WeightedPoint, &WeightedPoint),
        pair2: (&WeightedPoint, &WeightedPoint),
    ) -> (Point, Point) {
        let compute = |pair: (&WeightedPoint, &WeightedPoint)| -> Point {
            let mut intersect = Point::origin();
            if pair.0.x == pair.1.x {
                intersect.x = pair.0.x;
                intersect.y = {
                    pair.0.y
                        + (1. - pair.0.weight) / (pair.1.weight - pair.0.weight)
                            * (pair.1.y - pair.0.y)
                }
            } else if pair.0.y == pair.1.y {
                intersect.y = pair.0.y;
                intersect.x = {
                    pair.0.x
                        + (1. - pair.0.weight) / (pair.1.weight - pair.0.weight)
                            * (pair.1.x - pair.0.x)
                }
            }
            intersect
        };
        (compute(pair1), compute(pair2))
    }
}
