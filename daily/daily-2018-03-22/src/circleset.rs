use nannou::prelude::*;

pub struct CircleSet {
    levels: i32,
    color: Rgb8,
    wh: Vec<Vec2>
}

impl CircleSet {
    pub fn new(levels: i32, color: Rgb8) -> CircleSet {
        let mut wh = Vec::new();
        
        for i in 0..levels {
            let val = levels as f32 * i as f32;
            wh.push(vec2(val, val));
        };
        CircleSet {levels, color, wh}
    }

    pub fn update(&mut self, escelator: f32, efx: f32) {
        // println!("UPDATE");
        self.wh.clear();
        // println!("CLEARED WH");
        for i in 0..self.levels {
            let wx = escelator * (i as f32 * efx);
            self.wh.push(vec2(wx, wx));
        }
    }

    pub fn view(&self, draw: &Draw){
        for i in 0..self.levels {
            draw.ellipse()
                .no_fill()
                .stroke_weight(0.75)
                .stroke_color(self.color)
                .wh(self.wh[i as usize]);
        }
    }
}