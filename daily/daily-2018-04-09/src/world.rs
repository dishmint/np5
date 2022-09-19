use nannou::prelude::*;
use nannou::noise::{NoiseFn, Perlin};
use rand::{thread_rng as trn, Rng};
use crate::wall::Wall;
/*

    TODO: state Vec should be of a custom type instead of Rect. It's becoming a pain just to modify the w/h.
    
    */

pub struct World {
    state: Vec<Wall>,
    color: Srgb<u8>
}

impl World {
    pub fn new(n: i32, color: Srgb<u8>, rect: Rect) -> World{
        let mut rng = trn();
        let mut state  = Vec::new();
        let perlin = Perlin::new();
        for _i in 0..n-1{
            let x = rng.gen_range(rect.left()..rect.right());
            let y = rng.gen_range(rect.left()..rect.right());
            
            let nx = x / rect.w();
            let ny = y / rect.h();

            let p = perlin.get([nx as f64, ny as f64]);
            let w = (rect.w() as f64 * p) as f32;
            let h = (rect.h() as f64 * p) as f32;

            let r = Wall::new(x,y,w,h, WHITE);

            state.push(r);
        }
        World{state, color}
    }

    pub fn update(&mut self, _app: &App, zval: f32){
        for wall in self.state.iter_mut() {
            let new_w = wall.w() + zval;
            let new_h = wall.h() + zval;
            wall.set_wh(vec2(new_w,new_h));
        }
    }

    pub fn view(&self, draw: &Draw){
        /* 
        What I actually want is a horizontal line 1/3rd down from the top to be the road horizon.
        Then I want buildings to go by onn the left and right sides.
        I want to specify a path through the z dimension, and display buildings on that path.
        */
        for cond in self.state.iter() {
            draw.rect()
                .w_h(cond.w() as f32, cond.h() as f32)
                .x_y(cond.x() as f32, cond.y() as f32)
                .stroke(self.color)
                .stroke_weight(1.0)
                .no_fill();
        }   
    }
}