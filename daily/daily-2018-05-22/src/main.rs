use nannou::prelude::*;

mod flashback;
use crate::flashback::FlashBack;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(500, 500)
        .run();
}

struct Model {
    flashback: FlashBack,
    i_max: i32
}

fn model(app: &App) -> Model {
    let win = app.window_rect();
    let flashback = FlashBack::new(win.w(), win.h(), 1.0);
    let i_max = 10;
    Model { flashback, i_max }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win = app.window_rect();
    let thresh = win.w() / model.i_max as f32;
    model.flashback.update(win.wh(), thresh)
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    if app.elapsed_frames() < 1 {
        draw.background().color(BLACK);
    }

    for i in 1..=model.i_max {
        model.flashback.view(i, &draw);
    }

    draw.rect()
        .wh(win.wh())
        .color(srgba(0.0,0.0,0.0,0.1));
    draw.to_frame(app, &frame).unwrap();
}