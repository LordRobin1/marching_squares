use crate::balls::*;
use pixel_lib::*;

#[macro_export]
/// # Creates a closure which takes a `Point` and is then used to compute the implicit function
/// Pass a closure to this macro which takes a `Point` as the 1st argument and everything else that it needs after that
///
/// ---
/// Example:
/// ```
/// let func = |p: Point, constant: f32| -> f32 {
///     p.x * constant
/// };
/// let implicit_func = implicit_fn!(func, 3.14);
macro_rules! implicit_fn {
    ($func: expr, $($arg: expr),*) => {
        |p: Point| -> f32 {
            $func(p, $($arg),*)
        }
    };
}

/// Squares for the marching squares algorithm
pub struct Square<'a> {
    origin: Point,
    dimension: f32,
    weights: Vec<f32>,
    implicit_fn: &'a dyn Fn(Point) -> f32,
}

struct WeightedPoint {
    x: f32,
    y: f32,
    weight: f32,
}
impl WeightedPoint {
    pub fn new(x: f32, y: f32, weight: f32) -> Self {
        Self { x, y, weight }
    }
}

impl<'a> Square<'a> {
    pub fn new(
        origin: Point,
        dimension: f32,
        weights: Vec<f32>,
        implicit_fn: &'a dyn Fn(Point) -> f32,
    ) -> Self {
        Self {
            origin,
            dimension,
            weights,
            implicit_fn,
        }
    }

    pub fn march(&mut self, buffer: &mut [u32], balls: &[Ball], width: u32, height: u32) {
        let impl_fun = self.implicit_fn;
        let tl = impl_fun(self.origin);
        let tr = impl_fun(self.origin.add(&Point {
            x: self.dimension,
            y: 0.,
        }));
        let bl = impl_fun(self.origin.add(&Point {
            x: 0.,
            y: self.dimension,
        }));
        let br = impl_fun(self.origin.add(&Point {
            x: self.dimension,
            y: self.dimension,
        }));

        self.weights = vec![tl, tr, bl, br];

        self.shade(width, height, buffer);
    }

    /// Calculates the contours intersection points with the square
    /// and calls `rasterize_line` if it intersects
    fn shade(&self, width: u32, height: u32, buffer: &mut [u32]) {
        let hori_bound = self.origin.x + self.dimension;
        let vert_bound = self.origin.y + self.dimension;
        let tl = WeightedPoint::new(self.origin.x, self.origin.y, self.weights[0]);
        let tr = WeightedPoint::new(hori_bound, self.origin.y, self.weights[1]);
        let bl = WeightedPoint::new(self.origin.x, vert_bound, self.weights[2]);
        let br = WeightedPoint::new(hori_bound, vert_bound, self.weights[3]);

        // some of the cases can be combined
        match (
            self.weights[0] > 1.,
            self.weights[1] > 1.,
            self.weights[2] > 1.,
            self.weights[3] > 1.,
        ) {
            (true, true, true, true) => (),
            (false, false, false, false) => (),
            (true, false, false, false) => {
                // X O
                // O O
                let (p1, p2) = self.compute_intersection((&tl, &tr), (&tl, &bl));
                self.rasterize_line(&p1, &p2, buffer, width, height);
            }
            (false, true, false, false) => {
                // O X
                // O O
                let (p1, p2) = self.compute_intersection((&tr, &tl), (&tr, &br));
                self.rasterize_line(&p1, &p2, buffer, width, height);
            }
            (false, false, true, false) => {
                // O O
                // X O
                let (p1, p2) = self.compute_intersection((&bl, &tl), (&bl, &br));
                self.rasterize_line(&p1, &p2, buffer, width, height);
            }
            (false, false, false, true) => {
                // O O
                // O X
                let (p1, p2) = self.compute_intersection((&br, &bl), (&br, &tr));
                self.rasterize_line(&p1, &p2, buffer, width, height);
            }
            (true, false, true, false) => {
                // X O    O X
                // X O    O X
                let (p1, p2) = self.compute_intersection((&tl, &tr), (&bl, &br));
                self.rasterize_line(&p1, &p2, buffer, width, height);
            }
            (false, true, false, true) => {
                // O X
                // O X
                let (p1, p2) = self.compute_intersection((&tl, &tr), (&bl, &br));
                self.rasterize_line(&p1, &p2, buffer, width, height);
            }
            (true, true, false, false) => {
                // X X    O O
                // O O    X X
                let (p1, p2) = self.compute_intersection((&tl, &bl), (&tr, &br));
                self.rasterize_line(&p1, &p2, buffer, width, height);
            }
            (false, false, true, true) => {
                // O O
                // X X
                let (p1, p2) = self.compute_intersection((&tl, &bl), (&tr, &br));
                self.rasterize_line(&p1, &p2, buffer, width, height);
            }
            (true, false, false, true) => {
                // X O
                // O X
                let (p1, p2) = self.compute_intersection((&tl, &tr), (&tl, &bl));
                self.rasterize_line(&p1, &p2, buffer, width, height);
                let (p3, p4) = self.compute_intersection((&br, &bl), (&bl, &tl));
                self.rasterize_line(&p3, &p4, buffer, width, height);
            }
            (false, true, true, false) => {
                // O X
                // X O
                let (p1, p2) = self.compute_intersection((&tl, &tr), (&tl, &bl));
                self.rasterize_line(&p1, &p2, buffer, width, height);
                let (p3, p4) = self.compute_intersection((&br, &bl), (&bl, &tl));
                self.rasterize_line(&p3, &p4, buffer, width, height);
            }
            (true, true, true, false) => {
                // X X
                // X O
                let (p1, p2) = self.compute_intersection((&bl, &br), (&tr, &br));
                self.rasterize_line(&p1, &p2, buffer, width, height);
            }
            (true, true, false, true) => {
                // X X
                // O X
                let (p1, p2) = self.compute_intersection((&bl, &br), (&tl, &bl));
                self.rasterize_line(&p1, &p2, buffer, width, height);
            }
            (true, false, true, true) => {
                // X O
                // X X
                let (p1, p2) = self.compute_intersection((&tl, &tr), (&br, &tr));
                self.rasterize_line(&p1, &p2, buffer, width, height);
            }
            (false, true, true, true) => {
                // O X
                // X X
                let (p1, p2) = self.compute_intersection((&tl, &tr), (&tl, &bl));
                self.rasterize_line(&p1, &p2, buffer, width, height);
            }
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

    fn rasterize_line(&self, p1: &Point, p2: &Point, buffer: &mut [u32], width: u32, height: u32) {
        let y_range = if self.origin.y + self.dimension > height as f32 {
            self.origin.y as u32..height
        } else {
            self.origin.y as u32..(self.origin.y + self.dimension) as u32
        };

        for y in y_range {
            let x_range = if self.origin.x + self.dimension > width as f32 {
                self.origin.x as u32..(width)
            } else {
                self.origin.x as u32..(self.origin.x + self.dimension) as u32
            };
            for x in x_range {
                let i = (x + y * width) as usize;
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
