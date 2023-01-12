use nannou::prelude::*;
/* 
    — xyz
    — size
    — speed (affecting z only)
    — color (27,27,27) | (20,20,23)

*/

pub struct Creature {
       xyz: Point3,
       size: f32,
       speed: f32,
       color: Srgb<u8>
}

impl Creature {
    pub fn new(xyz: Point3, size: f32, speed: f32, color: Srgb<u8>) -> Creature {
        Creature {xyz, size, speed, color}
    }

    // pub fn x(&self) -> f32 {
    //     self.xyz.x
    // }

    // pub fn y(&self) -> f32 {
    //     self.xyz.y
    // }

    pub fn z(&self) -> f32 {
        self.xyz.z
    }
    
    pub fn update(&mut self, app: &App) {
        let time = app.time;
        let z = time * self.speed;
        self.xyz.z = z;
    }

    pub fn view(&self, draw: &Draw) {
        draw.ellipse()
            .xy(vec2(self.xyz.x, self.xyz.y))
            .radius(self.size*0.5)
            .color(self.color);
    }
}