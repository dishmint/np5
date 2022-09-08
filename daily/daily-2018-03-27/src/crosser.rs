use nannou::prelude::*;

pub struct Crosser {
    length: Vec2,
    weight: f32,
    color: Srgb<u8>,
    angle: f32,
    speed: f32
}

impl Crosser {
    pub fn new(length: Vec2, weight: f32, color: Srgb<u8>, speed: f32) -> Crosser {
        let angle = 0.0;
        Crosser {length, weight, color, angle, speed}
    }

    pub fn update(&mut self, time: f32){
        // self.angle = time.sin() * (TAU * 0.5);

        let tri = (time * self.speed)
            .sin()
            .asin() * (4.0 / TAU);

        self.angle = tri * (TAU * 0.5);
    }

    pub fn view(&self, draw: &Draw){
        draw.line()
            .weight(self.weight)
            .points(-self.length,self.length)
            .roll(self.angle)
            .color(self.color);

        draw.line()
            .weight(self.weight)
            .points(-self.length,self.length)
            .roll(-self.angle)
            .color(WHITE);
    }
}