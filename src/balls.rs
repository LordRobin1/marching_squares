use winit::dpi::PhysicalSize;

enum Axis {
    Horizontal,
    Vertical,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Balls {
    size: [f32; 2],           // 8 bytes
    positions: [[f32; 3]; 4], // 48 bytes
    velocities: [[f32; 2]; 4],
    padding: [u64; 2], // 8 bytes
}
impl Balls {
    pub fn new(size: PhysicalSize<u32>) -> Self {
        Self {
            size: [size.width as f32, size.height as f32],
            positions: [
                [0.0, 0.0, 0.3],
                [0.0, 0.5, 0.3],
                [0.0, -0.5, 0.3],
                [-0.5, 0.0, 0.3],
            ],
            velocities: [[0.1, 0.2], [0.1, 0.2], [0.1, 0.2], [0.1, 0.2]],
            padding: [0, 0],
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
            println!("pos: {},{}", pos[0], pos[1]);
            if pos[0] >= 1.0 || pos[0] <= -1.0 {
                self.velocities[i] = flip(&self.velocities[i], Axis::Vertical);
            }
            if pos[1] >= 1.0 || pos[1] <= -1.0 {
                self.velocities[i] = flip(&self.velocities[i], Axis::Horizontal);
            }
            self.positions[i][0] = pos[0];
            self.positions[i][1] = pos[1];
            println!(
                "self.pos: {},{}",
                self.positions[i][0], self.positions[i][1]
            );
            println!("vel: {:?}", self.velocities);
        }
    }
}

fn flip(velocity: &[f32; 2], axis: Axis) -> [f32; 2] {
    match axis {
        Axis::Vertical => [-velocity[0], velocity[1]],
        Axis::Horizontal => [velocity[0], -velocity[1]],
    }
}
