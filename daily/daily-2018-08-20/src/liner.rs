use nannou::prelude::*;
use rand::prelude::*;

/* STRUCT */
pub struct Liner{
    size: Vec2,
    pos: Vec2,
    color: Srgb,
}

/* IMPLEMENTATION */
impl Liner{
    pub fn new (win: Rect, index: usize, max: usize, color: Srgb) -> Self {
        let x_bound = thread_rng().gen_range(win.left()..=win.right());
        let y_bound = thread_rng().gen_range(win.bottom()..=win.top());
        let size = vec2(x_bound, y_bound);

        let pos_x = thread_rng().gen_range(win.left()..=win.right());
        let pos = vec2((pos_x / max as f32) * index as f32, 0.0);

        Liner { size, pos, color }
    }

    pub fn update (&mut self, app: &App) {
        let win = app.window_rect();
        let time = app.time;
        
        let x_val = self.size.x + self.pos.x;
        let y_val = self.size.y + self.pos.y;

        let x_fac = if x_val < win.left() || x_val > win.right() { time.sin() } else { time.cos() };
        let y_fac = if y_val < win.bottom() || y_val > win.top() { time.sin() } else { time.cos() };

        self.size.x += x_fac;
        self.size.y += y_fac;
    }

    pub fn view (&self, draw: &Draw) {
        draw.line()
            .stroke_weight(1.0)
            .color(self.color)
            .start(self.pos)
            .end(self.pos + self.size);
    }
}
