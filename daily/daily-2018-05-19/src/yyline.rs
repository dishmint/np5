use nannou::prelude::*;

/* STRUCT */
pub struct YYLine {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
    speed: f32,
}

/* IMPLEMENTATION */
impl YYLine{
    pub fn new (speed:f32) -> Self {
        let x1 = 0.0;
        let x2 = 0.0;

        let y1 = 0.0;
        let y2 = 0.0;
        YYLine { x1, x2, y1, y2, speed }
    }

    pub fn update (&mut self, app: &App) {
        let win = app.window_rect();
        let time = app.time * self.speed;
        
        self.x1 = time.sin() * win.w();
        self.x2 = self.x1 * -1.0;
        
        self.y1 = time.sin() * win.h();
        self.y2 = self.y1 * -1.0;
    }

    pub fn view (&self, app: &App, draw: &Draw) {
        let win = app.window_rect();

        draw.line()
            .start(vec2(self.x1, -win.h()))
            .end(vec2(self.x2, win.h()))
            .stroke_weight(1.0)
            .color(WHITE);

        draw.line()
            .start(vec2(-win.w(), self.y1))
            .end(vec2(win.w(), self.y2))
            .stroke_weight(1.0)
            .color(RED);
    }
}
