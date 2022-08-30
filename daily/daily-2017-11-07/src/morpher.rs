use nannou::prelude::*;
use nannou::rand::*;
use std::cmp;
pub struct Morpher {
    pub position: Point2,
    radius: Vec2,
    delta: f32,
    deltaspeed: f32,
    dimensions: Vec2,
    half_dim: f32,
    point_count: i32,
    step_size: i32 
}

impl Morpher {
    pub fn new(win: Rect, radius: Vec2, delta: f32, deltaspeed: f32, point_count: i32, step_size: i32) -> Self {
        let position = win.xy();
        let dimensions = win.wh();
        let half_dim = (cmp::min(dimensions.x as i32, dimensions.y as i32) as f32) / 2.0;
        Morpher {position, radius, delta, deltaspeed, dimensions, point_count, step_size, half_dim}
    }
    
    pub fn update(&mut self) {
        self.delta += self.deltaspeed;
        if self.delta > (self.half_dim * 2.0.sqrt()) || self.delta < 0.5 {
            self.deltaspeed *= -1.0;
        }
    }
    
    pub fn display(&self, draw: &Draw) {
        let pc = self.point_count as f32;
        let _circ = (pc/360.) * TAU;
        let xb = self.dimensions.x / 100.;
        let points =  (0..self.point_count).map(|i| {
            let idx = i as f32;
            let fx = (idx/pc) * TAU;
            let idc = fx.cos();
            let ids = fx.sin();
        
            let rnd = thread_rng().gen_range((-1.*xb)..xb);

            let pt = vec2(
                self.position.x + self.radius.x * idc + rnd,
                self.position.y + self.radius.y * ids
            );

            if i % self.step_size == 0 {
                let pt =  if i % 2 == 1 {
                    vec2(
                        ((self.position.x + self.radius.x) + self.delta) * idc,
                        ((self.position.y + self.radius.y) + self.delta) * ids
                    )
                } else {
                    vec2(
                        (self.position.x + (self./* half_dim */radius.x * 2.0.sqrt())) * idc,
                        (self.position.y + (self./* half_dim */radius.y * 2.0.sqrt())) * ids
                    )
                };

                /*                 
                let pt =  vec2(
                    (self.position.x + (self.half_dim * 2.0.sqrt())) * idc,
                    (self.position.y + (self.half_dim * 2.0.sqrt())) * ids
                );
                */
                
                 // let pt = vec2(
                //     ((self.position.x + self.radius.x) + self.delta) * idc,
                //     ((self.position.y + self.radius.y) + self.delta) * ids
                // );
                pt2(pt.x,pt.y)
            } else {
                pt2(pt.x,pt.y)
            }
        });

        draw.polygon()
                .no_fill()
                .stroke(ORANGERED)
                // .stroke(rgba(255.0,69.0,0.0,150.0))
                .join_miter()
                .points(points)
                /* .rotate(circ/2.) */;
    }
}