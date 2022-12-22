use nannou::prelude::*;

/* STRUCT */
pub struct Collipser{
    pos: Vec2,
    size: Vec2,
    speed: Vec2
}

/* IMPLEMENTATION */
impl Collipser{
    pub fn new (speed: Vec2) -> Self {
        let pos = vec2(0.0, 0.0);
        let size = vec2(0.0, 0.0);
        Collipser { pos, size, speed }
    }

    pub fn update (&mut self, app: &App) {
        let win = app.window_rect();
        let time = app.duration.since_start.as_secs_f32() * self.speed;

        self.size.x = time.x.cos() * win.w();
        self.size.y = time.y.sin() * win.h();
    }

    pub fn view (&self, draw: &Draw, index: f32) {
        draw.ellipse()
            .xy(self.pos)
            .wh(self.size * /* 1.0/ */index)
            .stroke_weight(1.0)
            .stroke(srgb(0.0,0.93,0.06))
            .no_fill();
    }
}
