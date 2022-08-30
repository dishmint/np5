use nannou::prelude::*;
use std::cmp;

mod morpher;
use crate::morpher::Morpher;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(500,500)
        // .fullscreen()
        .run();
}

struct Model {
    morpher: Morpher
}

fn model(app: &App) -> Model {
    let win = app.window_rect();
    let mindim = cmp::min(win.w() as i32, win.h() as i32) as f32;
    let mxs = mindim * 0.125;
    let mx = vec2(mxs,mxs);
    let morpher = Morpher::new(
        win
        , mx
        , 0.0
        , 10.0
        , 360
        , 45 /* {15, 45, 75, 105, 135, 165, 195, 225, 255, 285, 315, 345} */
    );
    Model { morpher }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.morpher.update();
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    draw.background().color(BLACK);

    model.morpher.display(&draw);
    draw.to_frame(app, &frame).unwrap();
}