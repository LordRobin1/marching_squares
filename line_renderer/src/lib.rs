use pixel_lib::*;

/// Draws a line between the given points using the bresenham algorithm.
/// @param buffer The buffer the line will be written to.
pub fn bresenham(buffer: &mut [u32], p1: &Point, p2: &Point, width: i32) {
    if (p2.y - p1.y).abs() < (p2.x - p1.x).abs() {
        // is not steep
        if p1.x <= p2.x {
            // goes to right
            // println!("not steep to right");
            plot_not_steep(buffer, p1, p2, width);
        } else {
            // goes to left
            // println!("not steep to left");
            plot_not_steep(buffer, p2, p1, width);
        }
    } else {
        // steep
        if p1.y <= p2.y {
            // goes to right
            // println!("steep to right");
            plot_steep(buffer, p1, p2, width);
        } else {
            // goes to left
            // println!("steep to left");
            plot_steep(buffer, p2, p1, width);
        }
    }
}

/// slopes between 0 and 1 / -1
fn plot_not_steep(buffer: &mut [u32], p1: &Point, p2: &Point, width: i32) {
    let dx = (p2.x - p1.x) as i32;
    let mut dy = (p2.y - p1.y) as i32;

    // allows slopes from 0 to -1
    let adjust = if dy >= 0 { 1 } else { -1 };
    dy *= adjust;

    let mut error = 2 * dy - dx;
    let mut y = p1.y as i32;

    for x in p1.x as i32..p2.x as i32 {
        // println!("{x}, {y}");
        let index = (x + y * width) as usize;
        if index < buffer.len() {
            buffer[(x + y * width) as usize] = 255u32 << 8;
        }
        if error > 0 {
            y += adjust;
            error += 2 * (dy - dx);
        } else {
            error += 2 * dy;
        }
    }
}

/// slopes between 1 and inf
fn plot_steep(buffer: &mut [u32], p1: &Point, p2: &Point, width: i32) {
    let mut dx = (p2.x - p1.x) as i32;
    let dy = (p2.y - p1.y) as i32;

    // allows slopes from 0 to -1
    let adjust = if dx >= 0 { 1 } else { -1 };
    dx *= adjust;

    let mut error = 2 * dx - dy;
    let mut x = p1.x as i32;

    for y in p1.y as i32..p2.y as i32 {
        // println!("{x}, {y}");
        let index = (x + y * width) as usize;
        if index < buffer.len() {
            buffer[index] = 255u32 << 8;
        }
        if error > 0 {
            x += adjust;
            error += 2 * (dx - dy);
        } else {
            error += 2 * dx;
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
