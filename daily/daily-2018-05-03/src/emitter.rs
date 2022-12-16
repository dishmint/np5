use nannou::prelude::*;

/* STRUCT */
pub struct Emitter{
    position: Vec2,
    dimensions: Vec2,
    direction: Vec2,
}

/* IMPLEMENTATION */
impl Emitter{
    pub fn new (app: &App) -> Self {
        let win = app.window_rect();
        
        /* FIELDS */
        let position = vec2(0.0,0.0);
        let dimensions = win.wh() * 0.25;
        let direction = vec2(1.0,1.0);

        Emitter { position, dimensions, direction }
    }

    pub fn update(&mut self, app: &App) {
        let win = app.window_rect();
        self.dimensions.x += self.direction.x;
        self.dimensions.y += self.direction.y;

        if self.dimensions.x > win.w() || self.dimensions.x < 0.0 {
            self.direction.x *= -1.0;
        }

        if self.dimensions.y > win.w() || self.dimensions.y < 0.0 {
            self.direction.y *= -1.0;
        }
    }

    pub fn view (&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .wh(self.dimensions)
            .stroke_weight(1.0)
            .stroke(WHITE)
            .no_fill();
    }
}
