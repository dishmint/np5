use nannou::prelude::*;
use crate::fzn::{bezier_point, show_bez, norm_range};

/* STRUCT */
struct HourglassPoints{
    p1: Vec2, 
    p2: Vec2,
}
pub struct Hourglass{
    size: f32,
    rate: f32,
    pos: HourglassPoints,
    color: Srgb
}

/* IMPLEMENTATION */
impl Hourglass{
    pub fn new (size: f32, rate: f32) -> Self {
        let color = srgb(0.0,0.0,0.0);
        Hourglass { size, rate, pos: HourglassPoints{p1: vec2(0.0, 0.0), p2: vec2(0.0, 0.0)}, color }
    }

    pub fn update (&mut self, app: &App) {
        /* window props */
        let winb = (app.window_rect().wh() * 0.5) - self.size;
        
        /* time based params */
        let time = app.time * self.rate;
        let x_t = 1.0 - norm_range(time.cos());
        let y_t = 1.0 - norm_range(time.sin());

        /* Bezier */
        let btx = bezier_point(0.0, -winb.x, 0.0, winb.x * 0.375, x_t);
        let bty = bezier_point(-winb.y, -winb.y * 0.5, 0.0, winb.y, y_t);

        self.pos.p1 = vec2(btx, bty);
        self.pos.p2 = vec2(-btx, bty);

        let r = x_t;
        let g = y_t;
        let b = x_t * y_t;
        self.color = srgb(r,g,b);
    }

    pub fn view (&self, draw: &Draw) {
        show_bez(draw, self.pos.p1, self.size, RED);
        show_bez(draw, self.pos.p2, self.size, RED);

        draw.line()
            .start(self.pos.p1)
            .end(self.pos.p2)
            .stroke_weight(1.0)
            .color(self.color);
    }
}
