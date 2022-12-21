use nannou::prelude::*;

/* STRUCT */
pub struct FlashBack{
    w: f32,
    h: f32,
    step: f32,
}

/* IMPLEMENTATION */
impl FlashBack{
    pub fn new (w: f32, h: f32, step: f32) -> Self {
        FlashBack { w, h, step }
    }

    pub fn update (&mut self, wh: Vec2, threshold: f32) {
        self.w -= self.step;
        self.h -= self.step;

        if self.w < threshold {
            self.w = wh.x;
            self.h = wh.y;
        }
    }

    pub fn view (&self, ix: i32, draw: &Draw) {
        let i_f = ix as f32;
        draw.rect()
            .wh(vec2(self.w * i_f, self.h * (i_f * 0.5)))
            .no_fill()
            .stroke(srgb(1.0,0.0,1.0))
            .stroke_weight(1.0);
    }
}
