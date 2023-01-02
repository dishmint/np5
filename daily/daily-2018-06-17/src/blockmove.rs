use nannou::{prelude::*, color::{IntoLinSrgba, Shade}};
/* STRUCT */
pub struct BlockMove{
    window: Rect,
    position: Vec2,
    width: f32,
    color: Srgb,
    tempo: f32,
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

        BlockMove { window: win, position, width: w_start, color, tempo }
    }

    pub fn update (&mut self, app: &App) {
        let frac = (app.time.fract() * 10.0).round() / 10.0;
        let trigger = frac % (60.0/self.tempo);
        // let trigger = app.elapsed_frames() as f32 % (60.0/self.tempo);
        if  trigger == 0.0 {
            // println!("Modded: {trigger}");
            self.position.x += self.width;
            if self.position.x > (self.window.right() - self.width) {
                self.position.x = self.window.left() + self.width;
                self.position.y -= self.width;
                if self.position.y < (self.window.bottom() + self.width) {
                    self.position.y = self.window.top() - self.width;
                    }
                }
        }

        self.color =  get_color(self.window, self.position, self.width);
    }

    pub fn view (&self, draw: &Draw) {
        let sk: LinSrgba = self.color.into_lin_srgba();         
        draw.rect()
            .xy(self.position)
            .wh(vec2(self.width, self.width))
            .stroke_weight(1.0)
            .stroke_color(sk.lighten(0.075))
            .color(self.color);
    }

}
