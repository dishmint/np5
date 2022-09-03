use nannou::prelude::*;
extern crate random_choice;
use self::random_choice::random_choice;

use nannou::rand::*;

pub struct Grid {
    rows: i32,
    cols: i32,
    points: Vec<Point2>,
    matrix: Vec<bool>,
    fill: Vec<f64>
}

impl Grid {
    pub fn new(win: Rect, rows: i32, cols: i32) -> Grid {
        let w = win.w();
        let h = win.h();
        let ws = w / cols as f32;
        let hs = h / rows as f32;
        let mut points = Vec::new();
        let mut matrix = Vec::new();
        let mut fill = Vec::new();

        let samples = vec![true,false];
        let weights: Vec<f64> = vec![0.50, 0.50];

        for x in ((ws as i32)..(w as i32)).step_by(ws as usize) {
            for y in ((hs as i32)..(h as i32)).step_by(hs as usize) {
                let x = x as f32 - (w / 2.0);
                let y = y as f32 - (h / 2.0);
                points.push(pt2(x,y));
                
                let choice = random_choice().random_choice_f64(&samples, &weights, 1)[0].clone();
                matrix.push(choice);

                let rnd = thread_rng().gen_range(-1.0..1.0);
                let fc = rnd - (choice as usize as f64);
                fill.push(fc);
            }
        }
 
        Grid {rows, cols, points, matrix, fill}
    }

    pub fn update(&mut self, win: Rect) {
        self.points.clear();
        self.matrix.clear();
        self.fill.clear();

        let w = win.w();
        let h = win.h();
        let ws = w / self.cols as f32;
        let hs = h / self.rows as f32;

        let samples = vec![true,false];
        let weights: Vec<f64> = vec![0.50, 0.50];

        for x in ((ws as i32)..(w as i32)).step_by(ws as usize) {
            for y in ((hs as i32)..(h as i32)).step_by(hs as usize) {
                let x = x as f32 - (w / 2.0);
                let y = y as f32 - (h / 2.0);
                self.points.push(pt2(x,y));
                
                let choice = random_choice().random_choice_f64(&samples, &weights, 1)[0].clone();
                self.matrix.push(choice);

                let rnd = thread_rng().gen_range(-1.0..1.0);
                let fc = rnd - (choice as usize as f64);
                self.fill.push(fc);
            }
        }

    }

    pub fn view(&self, win: Rect, draw: &Draw){
        let ms = win.w().min(win.h());
        let sw = 0.5;
        let radius = vec2(ms / self.cols as f32, ms / self.rows as f32) - sw;
        for (i, cell) in self.matrix.iter().enumerate() {
            let point =  self.points[i];
            let fill = abs(self.fill[i]) as f32;
            
            let cshape = draw.ellipse()
                .wh(radius)
                .x_y(point.x, point.y)
                .gray(fill)
                .stroke_weight(0.5);
            if cell.clone() {
                cshape
                    .stroke(WHITE);
            } else {
                cshape
                    .stroke(RED);
            }
        }
    }
}