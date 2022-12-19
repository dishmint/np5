use nannou::prelude::*;

mod ellipser2;
use crate::ellipser2::Ellipser2;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(500,500)
        .run();
}

struct Model {
    ellipsers: Vec<Ellipser2>
}

fn model(_app: &App) -> Model {
    let mut ellipsers = Vec::new();
    for _ix in 0..10 {
        let e= Ellipser2::new(0.5);
        ellipsers.push(e);
    }

    Model { ellipsers }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for (i, ell) in model.ellipsers.iter_mut().enumerate() {
        ell.update(app, i);
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    if app.elapsed_frames() < 1 {
        draw.background().color(BLACK);
    }
    
    for (i, ell) in model.ellipsers.iter().enumerate() {
        ell.view(&draw, i);
    }

    draw.rect()
        .wh(win.wh())
        .color(srgba(0.0,0.0,0.0,0.01));
    draw.to_frame(app, &frame).unwrap();
}