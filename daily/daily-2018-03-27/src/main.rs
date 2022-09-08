use nannou::prelude::*;

mod crosser;
use crate::crosser::Crosser;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(400,400)
        .run();
}

struct Model {
    crosser: Crosser
}

fn model(app: &App) -> Model {
    let win = app.window_rect();
    let len = vec2(win.w() * 0.25,0.0);

    let crosser = Crosser::new(len, 1.0, TOMATO, 0.60);
    Model {crosser}
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.time;
    model.crosser.update(time);
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    model.crosser.view(&draw);

    draw.rect()
        .wh(win.wh())
        .color(srgba(0.0,0.0,0.0,0.08));

    draw.to_frame(app, &frame).unwrap();
}