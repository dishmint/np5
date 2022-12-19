use nannou::prelude::*;
// use rand::prelude::*;

mod triangle;
use crate::triangle::Triangle;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(500,500)
        .run();
}

struct Model {
    triangles: Vec<Triangle>
}

fn model(app: &App) -> Model {
    let win = app.window_rect();
    let mut triangles = Vec::new();
    let max_triangles = 50;

    for i in 1..=max_triangles {
        let center = vec2(0.0, (i as f32 / max_triangles as f32) - (win.h() * 0.25));
        // let center = vec2(0.0, 0.0);
        let r = random_range(0.0, 1.0);
        let b = random_range(0.0, 1.0);
        let t = Triangle::new(center, i as f32, srgb(r, 0.0, b), 0.75);
        triangles.push(t)
    }

    Model { triangles }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for tri in model.triangles.iter_mut() {
        tri.update(app);
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    if app.elapsed_frames() < 1 {
        draw.background().color(BLACK);
    }
    
    for tri in model.triangles.iter() {
        tri.view(&draw);
    }

    draw.rect()
        .wh(win.wh())
        .color(srgba(0.0,0.0,0.0,0.05));
    draw.to_frame(app, &frame).unwrap();
}