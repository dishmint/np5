use nannou::prelude::*;

/* STRUCT */
pub struct Flipper{
    theta: f32,
    direction: f32,
    size: f32,
    position: Vec2,
    color: Srgb
}

/* IMPLEMENTATION */
impl Flipper{
    pub fn new (theta: f32, direction: f32, size: f32, position: Vec2, color: Srgb ) -> Self {
        Flipper { theta, direction, size, position, color }
    }

    pub fn update (&mut self) {
        self.theta += self.direction;
        if self.theta > self.size || self.theta < 0.0 {
            self.direction *= -1.0;
        }
    }

    pub fn view (&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .wh(vec2(self.theta, self.size))
            .pitch(self.theta * 0.5)
            .yaw(self.theta)
            /* .roll(self.theta) */
            .stroke_weight(1.0)
            .stroke(self.color)
            .no_fill();
    }
}
