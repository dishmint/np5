use nannou::prelude::*;

/* STRUCT */
pub struct Triangle{
    center: Point2,
    size: f32,
    color: Srgb,
    speed : f32
}

/* IMPLEMENTATION */
impl Triangle{
    pub fn new (center: Point2, size: f32, color: Srgb, speed: f32) -> Self {
        Triangle { center, size, color, speed }
    }

    pub fn update (&mut self, app: &App) {
        let win = app.window_rect();
        let time = app.time * self.speed;

        self.center.x = time.sin() * ((win.w() * 0.5) - self.size);
        self.center.y += time.sin();
    }

    pub fn view (&self, draw: &Draw) {
        draw.tri()
            .points(
                vec2(self.center.x - self.size, self.center.y),
                vec2(self.center.x, self.center.y - self.size),
                vec2(self.center.x + self.size, self.center.y),
            )
            .stroke_weight(1.0)
            .stroke(self.color)
            .no_fill()
            .roll(PI);
            // .roll(abs(self.center.y) * 0.1);
            // .roll(self.center.y * 0.1);
            // .pitch(self.center.y * 0.1);
            // .yaw(self.center.y * 0.1);
    }
}
