use nannou::{prelude::*, color::{IntoLinSrgba, Shade}};
/* STRUCT */
pub struct BlockMove{
    window: Rect,
    position: Vec2,
    pub width: f32,
    color: Srgb,
    pub beat_time: f32,
    pub beat_length: f32,
    pub opt_outline: bool,
}

fn get_color(win: Rect, pos: Vec2, w_s: f32) -> Srgb {
    let r = 1.0;
    let g = map_range(pos.y, w_s, win.h() - w_s, 1.0/w_s, 1.0);
    let b = map_range(pos.x * 2.0, w_s * 2.0, (win.w() - w_s) * 2.0, 1.0/w_s, 1.0);
    srgb(r,g,b)
}

/* IMPLEMENTATION */
impl BlockMove{
    pub fn new (app: &App, size: f32, tempo: f32) -> Self {
        let win = app.window_rect();
        let w_bound = win.right() * 0.5;
        let w_start = (w_bound * size) * 0.5;
    
        let position = vec2(
            win.left() + w_start,
            win.top() - w_start
        );
    
        let color = get_color(win, position, w_start);

        let beat_time = 0.0;
        let beat_length = (60.0/tempo) * 1000.0; /* in millis */

        let opt_outline = false;
        BlockMove { window: win, position, width: w_start, color, beat_time, beat_length, opt_outline }
    }

    pub fn update (&mut self, app: &App) {
        /* Increase beat_time by time since previous update */
        self.beat_time += app.duration.since_prev_update.as_millis() as f32;

        /* If beat_time exceeds beat_length then consider this frame a downbeat and compute position */
        if self.beat_time >= self.beat_length {
            self.position.x += self.width;
            if self.position.x > (self.window.right() - self.width) {
                self.position.x = self.window.left() + self.width;
                self.position.y -= self.width;
                if self.position.y < (self.window.bottom() + self.width) {
                    self.position.y = self.window.top() - self.width;
                    }
                }
            self.beat_time = 0.0;
        }

        self.color =  get_color(self.window, self.position, self.width);
    }

    pub fn view (&self, draw: &Draw) {
        let sk: LinSrgba = self.color.into_lin_srgba();         
        let block = draw.rect()
            .xy(self.position)
            .wh(vec2(self.width, self.width))
            .color(self.color);
        if self.opt_outline {
            block
                .stroke_weight(1.0)
                .stroke_color(sk.lighten(0.075));
        }
    }

}
