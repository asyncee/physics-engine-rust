#[repr(C, packed)]
#[derive(PartialEq, Debug)]
pub struct Vertex {
    pub position: cgmath::Vector2<f32>,
    pub previous_position: cgmath::Vector2<f32>,
    pub acceleration: cgmath::Vector2<f32>,
    pub color: cgmath::Vector3<f32>,
}

impl Vertex {
    pub fn new(position: cgmath::Vector2<f32>, color: cgmath::Vector3<f32>) -> Self {
        Self {
            position,
            previous_position: position,
            acceleration: cgmath::vec2(0.0, 0.0),
            color: color,
        }
    }

    pub fn update_position(&mut self, dt: f32) {
        let velocity = self.position - self.previous_position;
        let position = self.position;
        self.previous_position = position;
        self.position = position + velocity + self.acceleration * (dt * dt);
        self.acceleration = cgmath::vec2(0.0, 0.0);
    }

    pub fn accelerate(&mut self, acceleration: cgmath::Vector2<f32>) {
        self.acceleration = self.acceleration + acceleration;
    }
}
