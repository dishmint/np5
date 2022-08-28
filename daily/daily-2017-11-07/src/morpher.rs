use nannou::prelude::*;
use nannou::rand::*;

pub struct Morpher {
    pub position: Point2,
    radius: Vec2,
    delta: f32,
    deltaspeed: f32,
    dimensions: Vec2,
    point_count: i32 
}

impl Morpher {
    pub fn new(win: Rect, radius: Vec2, delta: f32, deltaspeed: f32, point_count: i32) -> Self {
        let position = win.xy();
        let dimensions = win.wh();
        Morpher {position, radius, delta, deltaspeed, dimensions, point_count}
    }
    
    pub fn update(&mut self) {
        self.delta += self.deltaspeed;
        if self.delta > 254.0 || self.delta < 0.0 {
            self.deltaspeed *= -1.0;
        }
    }
    
    pub fn display(&self, draw: &Draw) {
        let points =  (0..self.point_count).map(|i| {
            let idx = i as f32;
            let fx = (idx/360.0) * TAU;
            let idc = fx.cos();
            let ids = fx.sin();
            let xb = self.dimensions.x / 100.;
            let rnd = thread_rng().gen_range((-1.*xb)..xb);

            let pt = vec2(
                (self.position.x + self.radius.x * idc) + rnd,
                self.position.y + self.radius.y * ids
            );
            if i % 15 == 0 {
                let rm = thread_rng().gen_range(-5.0..1.0);
                let pt = vec2(
                    (self.delta * self.radius.x) * idc,
                    (2.0 * self.radius.y) * ids + rm
                );
                pt2(pt.x,pt.y)
            } else {
                pt2(pt.x,pt.y)
            }
        });

        draw.polygon()
                .color(BLACK)
                .stroke(ORANGERED)
                // .join_round()
                .join_miter()
                .points(points);
    }
}