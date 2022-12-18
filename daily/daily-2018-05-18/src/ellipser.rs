use nannou::prelude::*;

/* STRUCT */
pub struct Ellipser{
    spacing: f32,
    size: f32
}

/* IMPLEMENTATION */
impl Ellipser{
    pub fn new (spacing: f32) -> Self {
        let size = 0.0;
        Ellipser { spacing, size }
    }

    pub fn update (&mut self, app: &App) {
        let _win = app.window_rect();
        let time = app.time;
        self.size = time.sin() * 50.0;
    }

    pub fn view (&self, win: &Rect, draw: &Draw) {
        let step = win.w()/self.spacing;
        for i in (1..(win.w() as i32)).step_by(step as usize) {
            let i_f = i as f32;

            draw.ellipse()
                .xy(win.xy())
                .wh(vec2(i_f, i_f * self.size))
                .no_fill()
                .stroke(WHITE)
                .stroke_weight(1.0);
        }
    }
}
