use nannou::prelude::*;

pub struct Shifter {
    pub dir: i8,
    pub delta: f32,
    pub speed: f32,
    pub color: Rgb8,
    pub points: Vec<Point2>
}

impl Shifter {
    pub fn new(win: Rect, speed: f32, delta: f32) -> Shifter {
        let dir = 1;
        let color = rgb(255,255,255);
        let mut points = Vec::new();
        
        for i in 0..2 {
            let x = win.w() / 2.0;
            let y = (win.h() / 2.0) * i as f32;
            points.push(pt2(x,y))
        };
        Shifter {dir, speed, delta, color, points}
    }

    pub fn update(&mut self, win: Rect) {
        self.delta += self.speed * self.dir as f32;

    for point in self.points.iter_mut() {
            for i in 0..2 {
                point.x = (win.w() / 2.0) - self.delta;
                point.y = (win.h() / 2.0) - (i as f32 * self.delta);
            };
        };

        let _pt1 = self.points[0];
        let pt2 = self.points[1];

        if pt2.x > (win.w() / 2.0) || pt2.x < 0.0 {
            self.dir *= -1;
            let r = (random::<f32>() * 255.0) as u8;
            let g = (random::<f32>() * 255.0) as u8;
            let b = (random::<f32>() * 255.0) as u8;
            self.color = rgb(r,g,b);
        }
    }

    pub fn view(&self, draw: &Draw){
        draw.polyline()
            .color(self.color)
            .points(self.points.clone());
    }
}