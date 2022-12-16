use nannou::prelude::*;

pub struct Rotor{
    position: Vec2,
    dimensions: Vec2,
    theta: f32,
    direction: f32,
}

impl Rotor {
    pub fn new(dimensions: Vec2, theta: f32, direction: f32) -> Self{
        let position = vec2(0.0,0.0);
        Rotor{
            position,
            dimensions,
            theta,
            direction
        }
    }

    pub fn update(&mut self){
        self.theta += self.direction;
        // if dbg!(abs(self.theta)) > TAU {
        if abs(self.theta) > TAU {
            self.direction *= -1.0;
        }
    }

    pub fn view(&self, draw: &Draw){
        draw.rect()
            .stroke_weight(1.0)
            .stroke(WHITE)
            .no_fill()
            .xy(self.position)
            .wh(self.dimensions)
            .roll(self.theta as f32);
    }
}