use winit::dpi::PhysicalSize;

enum Axis {
    Horizontal,
    Vertical,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
// need to insert padding for each vector probably, because vec3 has 16 byte in memory
pub struct Balls {
    size: [f32; 2],            // 8 bytes
    positions: [[f32; 4]; 4],  // 48 bytes
    velocities: [[f32; 3]; 4], // 48 bytes
    padding: [u64; 7],         // 56 bytes
}

impl Balls {
    pub fn new(size: PhysicalSize<u32>) -> Self {
        Self {
            size: [size.width as f32, size.height as f32],
            positions: [
                [0.0, 0.0, 0.3, 0.0],
                [0.0, 0.5, 0.3, 0.0],
                [0.0, -0.5, 0.3, 0.0],
                [-0.5, 0.0, 0.3, 0.0],
            ],
            velocities: [
                [0.1, 0.2, 0.0],
                [0.1, 0.2, 0.0],
                [0.1, 0.2, 0.0],
                [0.1, 0.2, 0.0],
            ],
            padding: [0, 0, 0, 0, 0, 0, 0],
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size[0] = size.width as f32;
        self.size[1] = size.height as f32;
    }

    pub fn update(&mut self, _delta_time: f32) {
        for (i, mut pos) in self.positions.into_iter().enumerate() {
            pos[0] += self.velocities[i][0];
            pos[1] += self.velocities[i][1];
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
            println!("{}. pos: {:?}", i, self.positions[i]);
            println!("{}. self.pos: {:?}", i, self.positions[i]);
        }
    }
}

fn flip(velocity: &[f32; 3], axis: Axis) -> [f32; 3] {
    match axis {
        Axis::Vertical => [-velocity[0], velocity[1], 0.0],
        Axis::Horizontal => [velocity[0], -velocity[1], 0.0],
    }
}
