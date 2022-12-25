use nannou::prelude::*;

// https://github.com/processing/p5.js/blob/37484907defe7d424172b97adde966ffa8d4396b/src/core/shape/curves.js#L174
fn bezier_point(a: f32, b: f32, c: f32, d: f32, t: f32) -> f32 {
    let adjusted_t = 1.0 - t;
    pow(adjusted_t, 3) * a +
    3.0 * pow(adjusted_t, 2) * t * b +
    3.0 * adjusted_t * pow(t, 2) * c +
    pow(t, 3) * d
}

/* STRUCT */
pub struct Bez {
    pos_a: Vec2,
    pos_b: Vec2,
    size: f32,
    rate: f32,
    theta: f32,
    color: Rgb<u8>
}

/* IMPLEMENTATION */
impl Bez{
    pub fn new (size: f32, rate: f32, color: Rgb<u8>) -> Self {
        let pos_a = vec2(0.0,0.0);
        let pos_b = vec2(0.0,0.0);
        let theta = 0.5 * TAU;
        Bez { pos_a, pos_b, size, rate, theta, color }
    }

    pub fn update (&mut self, app: &App) {
        let win = app.window_rect();
        let wbnd = (win.wh() * 0.45) - self.size;
        let time = app.time * self.rate;
        
        let x_t = (time.cos() + 1.0) * 0.5;
        let y_t = (time.sin() + 1.0) * 0.5;
        
        let bez_xy_a = vec2(
            bezier_point(-wbnd.x, 0.0, wbnd.x, 0.0, x_t),
            bezier_point(-wbnd.y, 0.0, wbnd.y, 0.0, y_t)
        );
        
        let bez_xy_b = vec2(
            bezier_point(0.0, -wbnd.x, 0.0, wbnd.x , x_t),
            bez_xy_a.y
        );
        
        
        self.pos_a = bez_xy_a.rotate(self.theta) - vec2(0.0, win.h() * 0.125);
        self.pos_b = bez_xy_b.rotate(self.theta) - vec2(0.0, win.h() * 0.125);
    }

    fn show_bez(draw: &Draw, pos: Vec2, size: f32, color: Rgb<u8>){
        draw
            .ellipse()
            .xy(pos)
            .radius(size)
            .color(color);
    }

    pub fn view (&self, draw: &Draw) {
        Bez::show_bez(draw, self.pos_a, self.size, self.color);
        Bez::show_bez(draw, self.pos_b, self.size, self.color);
    }
}
