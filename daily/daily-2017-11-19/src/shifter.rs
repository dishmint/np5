use nannou::prelude::*;

pub struct Shifter {
    pub dir: i8,
    pub delta: f32,
    pub speed: f32,
    pub color: Rgb8,
    pub points: Vec<Point2>,
    pub alpha: f32
}

impl Shifter {
    pub fn new(speed: f32, delta: f32) -> Shifter {
        let dir = 1;
        let color = rgb(255,255,255);
        let alpha = 1.0;
        let mut points = Vec::new();
        
        points.push(pt2(0.0,0.0));
        points.push(pt2(0.0,0.0));

        Shifter {dir, speed, delta, color, points, alpha}
    }

    pub fn update(&mut self, win: Rect) {
        self.delta += self.speed * (self.dir as f32);

        for i in 0..2 {
            self.points[i].x = -self.delta;
            self.points[i].y = i as f32 * self.delta;
        };

        let pt1 = self.points[0];

        let wd = win.w() / 2.0;

        if pt1.x > wd || pt1.x < -wd {
            self.dir *= -1;
            let r = (random::<f32>() * 255.0) as u8;
            let g = (random::<f32>() * 255.0) as u8;
            let b = (random::<f32>() * 255.0) as u8;
            self.color = rgb(r,g,b);
        }

        if pt1.x >= 0.0 {
            self.alpha = 0.0;
        } else {
            self.alpha = map_range(pt1.x, 0.0, -wd, 0.09, 0.5);
        }

    }

    pub fn view(&self, draw: &Draw){
        draw.polyline()
            .color(self.color)
            .points(self.points.clone());
    }
}