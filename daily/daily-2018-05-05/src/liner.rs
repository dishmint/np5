use nannou::prelude::*;

/* STRUCT */
pub struct Liner{
    xlock: f32,
    xbound: f32,
    ybound: f32,
    spacing: f32,
}

/* IMPLEMENTATION */
impl Liner{
    pub fn new (xlock: f32, spacing: f32, bound: Rect) -> Self {
        let xbound = bound.w() * 0.5;
        let ybound = bound.h() * 0.5;
        Liner { xlock, xbound, ybound, spacing }
    }

    pub fn update (&mut self) {
        self.xlock += self.spacing;
        if self.xlock > self.xbound || self.xlock < -self.xbound {
            self.spacing *= -1.0;
        }
    }

    pub fn view (&self, draw: &Draw) {
        draw.line()
            .start(vec2(self.xlock, self.ybound))
            .end(vec2(self.xlock, -self.ybound))
            .stroke_weight(1.0)
            .color(WHITE);
    }
}
