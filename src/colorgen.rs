use cgmath::num_traits::ToPrimitive;
use colorgrad::Color;

pub struct ColorGenerator {
    colors: Vec<Color>,
    step: usize,
}

impl ColorGenerator {
    const MAX_COLORS: usize = 100;
    pub fn new() -> Self {
        let grad = colorgrad::sinebow();
        let colors = grad.colors(ColorGenerator::MAX_COLORS);
        Self { colors, step: 0 }
    }
    pub fn next_color(&mut self) -> cgmath::Vector3<f32> {
        if self.step == ColorGenerator::MAX_COLORS - 1 {
            self.step = 0;
        }
        let color = self.colors[self.step].to_rgba8();
        let r = color[0].to_f32().unwrap() / 255.0;
        let g = color[1].to_f32().unwrap() / 255.0;
        let b = color[2].to_f32().unwrap() / 255.0;
        self.step += 1;
        cgmath::vec3(r, g, b)
    }
}
