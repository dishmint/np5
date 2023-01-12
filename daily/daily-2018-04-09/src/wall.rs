use nannou::prelude::*;

pub struct Wall {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: Srgb<u8>
}

impl Wall {
    pub fn new(x: f32,y: f32,w: f32,h: f32,color: Srgb<u8>) -> Wall {
        Wall {x, y, w, h, color}
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn w(&self) -> f32 {
        self.w
    }

    pub fn h(&self) -> f32 {
        self.h
    }
    
    pub fn set_x(&mut self, newx: f32) -> &mut Self {
        self.x = newx;
        self
    }

    pub fn set_y(&mut self, newy: f32) -> &mut Self {
        self.y = newy;
        self
    }

    pub fn set_xy(&mut self, new: Vec2) -> &mut Self {
        self.x = new.x;
        self.y = new.y;
        self
    }
    
    pub fn set_w(&mut self, neww: f32) -> &mut Self {
        self.w = neww;
        self
    }

    pub fn set_h(&mut self, newh: f32) -> &mut Self {
        self.h = newh;
        self
    }

    pub fn set_wh(&mut self, new: Vec2) -> &mut Self {
        self.w = new.x;
        self.h = new.y;
        self
    }
}