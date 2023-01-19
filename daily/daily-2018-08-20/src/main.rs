use nannou::prelude::*;
use rand::prelude::*;

mod liner;
use crate::liner::Liner;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    liners: Vec<Liner>,
}

fn model(app: &App) -> Model {
    let win = app.window_rect();
    let mut liners: Vec<Liner> = Vec::new();

    let max = 300;
    for i in 0..max {
        let color = srgb(thread_rng().gen_range(0.0..=1.0), 0.0, 0.0);
        liners.push(Liner::new(win, i, max, color));
    }

    Model { liners }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for liner in model.liners.iter_mut() {
        liner.update(app);
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    if app.elapsed_frames() < 1 {
        draw.background().color(BLACK);
    }
    
    for liner in model.liners.iter() {
        liner.view(&draw);
    }

    draw.rect()
        .wh(win.wh())
        .color(srgba(0.0,0.0,0.0,0.04));
    draw.to_frame(app, &frame).unwrap();
}