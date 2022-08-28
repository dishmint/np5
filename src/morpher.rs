use nannou::prelude::*;

pub struct Morpher {
    pub position: Point2,
    radius: f32,
    delta: f32,
    deltaspeed: f32
}

impl Morpher {
    pub fn new(win: Rect, radius: f32, delta: f32, deltaspeed: f32) -> Self {
        let position = win.xy();
        Morpher {position, radius, delta, deltaspeed}
    }
    
    pub fn update(&mut self) {
        self.delta += self.deltaspeed;
        if self.delta > 254.0 || self.delta < 0.0 {
            self.deltaspeed *= -1.0;
        }
    }
    
    pub fn display(&self, draw: &Draw) {
        for value in 0..360 {
            let i = value as f32;
            let x = self.position.x + self.radius * i.cos();
            let y = self.position.y + self.radius * i.sin();
            draw.ellipse()
                .color(RED)
                .x_y(x,y)
                .w_h(2.0,2.0);
        }
    }
}