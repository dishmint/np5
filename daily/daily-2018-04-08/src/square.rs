use nannou::prelude::*;

pub struct Square {
    wh: Vec2,
    xy: Vec2,
    change: f32,
    pub speed: f32,
    cdist: f32,
    stroke: Srgb<u8>
}

impl Square {
    pub fn new(s: f32, speed: f32, cdist: f32, stroke:Srgb<u8>) -> Square {
        let wh = vec2(s,s);
        let xy = vec2(0.0,0.0);
        let change = 0.0;

        Square {wh, xy, change, speed, cdist, stroke}
    }

    pub fn update(&mut self, time: f32) {
        self.change = time * self.speed;
        let new_xy = vec2(
            self.cdist * self.change.cos(),
            self.cdist * self.change.sin()
        );
        
        self.xy = new_xy;
    }

    pub fn view(&self, draw: &Draw) {
        draw.rect()
            .xy(self.xy)
            .wh(self.wh)
            .stroke_weight(1.0)
            .stroke(self.stroke)
            .no_fill()
            .roll(-self.change);

        /* 

        // Fade the square

        let _clr: Rgb<f32> = rgb(self.stroke.red as f32, self.stroke.green as f32, self.stroke.blue as f32);

        draw.rect()
            .xy(self.xy)
            .wh(self.wh * 0.98)
            .color(srgba(1.0,1.0,1.0,0.04))
            // .color(srgba(clr.red,clr.green,clr.blue,0.025))
            .roll(-self.change);
        */

    }
}