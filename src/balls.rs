use winit::dpi::PhysicalSize;

enum Axis {
    Horizontal,
    Vertical,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
// ! don't touch
pub struct Balls {
    size: [f32; 2],
    _padding: u64,
    positions: [[f32; 4]; 4],  // 4 bytes padding for each vec
    velocities: [[f32; 4]; 4], // 8 bytes padding for each vec, to satisfy 16byte array stride
    colors: [[f32; 4]; 4],
}

impl Balls {
    pub fn new(size: PhysicalSize<u32>) -> Self {
        Self {
            size: [size.width as f32, size.height as f32],
            _padding: 0,
            positions: [
                [0.0, 0.0, 0.4, 0.0],
                [0.0, 0.5, 0.4, 0.0],
                [0.0, -0.5, 0.4, 0.0],
                [-0.5, 0.0, 0.4, 0.0],
            ],
            velocities: [
                [0.4, 0.2, 0.0, 0.0],
                [0.2, 0.4, 0.0, 0.0],
                [0.6, 0.1, 0.0, 0.0],
                [0.1, 0.5, 0.0, 0.0],
            ],
            colors: [
                [1.0, 0.0, 0.0, 1.0],
                [0.0, 1.0, 0.0, 1.0],
                [0.0, 0.0, 1.0, 1.0],
                [1.0, 0.0, 1.0, 1.0],
            ],
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size[0] = size.width as f32;
        self.size[1] = size.height as f32;
    }

    pub fn update(&mut self, delta_time: f32) {
        for (i, mut pos) in self.positions.into_iter().enumerate() {
            pos[0] += self.velocities[i][0] * delta_time;
            pos[1] += self.velocities[i][1] * delta_time;
            // println!("pos: {},{}", pos[0], pos[1]);
            pos[0] = pos[0].clamp(-1.0, 1.0);
            pos[1] = pos[1].clamp(-1.0, 1.0);

            if pos[0] >= 1.0 || pos[0] <= -1.0 {
                self.velocities[i] = flip(&self.velocities[i], Axis::Vertical);
            }
            if pos[1] >= 1.0 || pos[1] <= -1.0 {
                self.velocities[i] = flip(&self.velocities[i], Axis::Horizontal);
            }
            self.positions[i][0] = pos[0];
            self.positions[i][1] = pos[1];
            // println!("{}. pos: {:?}", i, self.positions[i]);
            // println!("{}. self.pos: {:?}", i, self.positions[i]);
        }
    }
}

fn flip(velocity: &[f32; 4], axis: Axis) -> [f32; 4] {
    match axis {
        Axis::Vertical => [-velocity[0], velocity[1], 0., 0.],
        Axis::Horizontal => [velocity[0], -velocity[1], 0., 0.],
    }
}
