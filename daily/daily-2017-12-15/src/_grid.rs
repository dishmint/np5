use nannou::prelude::*;
extern crate random_choice;
use self::random_choice::random_choice;

pub struct Grid {
    rows: i32,
    cols: i32,
    matrix: Vec<Vec<bool>>,
    points: Vec<Vec<Point2>>
}

impl Grid {
    pub fn new(win: Rect, rows: i32, cols: i32) -> Grid {
        let mut matrix = Vec::new();
        let samples = vec![true,false];
        let weights: Vec<f64> = vec![0.50, 0.50];

        for _i in 0..rows {
            let mut row = Vec::new();
            for _j in 0..cols {
                let choice = random_choice().random_choice_f64(&samples, &weights, 1)[0].clone();
                row.push(choice);
            }
            matrix.push(row);
        };
        
        
        let mut points = Vec::new();

        let wc = win.w() / cols as f32;
        let hr = win.h() / rows as f32;

        for i in 0..rows {
            let mut row = Vec::new();
            for j in 0..cols {
                let x = map_range(j as f32, 0.0, (cols - 1) as f32,wc, win.w() - wc);
                let y = map_range(i as f32, 0.0, (rows - 1) as f32,hr, win.h() - hr);
                let point = pt2(x,y);
                row.push(point);
            }
            points.push(row);
        };
        
       
        Grid {rows, cols, matrix, points}
    }

    // pub fn update(&mut self){

    // }
    pub fn view(&self, draw: &Draw){
        
    }
}