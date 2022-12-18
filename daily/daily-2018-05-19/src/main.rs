use nannou::prelude::*;

mod yyline;

use crate::yyline::YYLine;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    yyline: YYLine
}

fn model(_app: &App) -> Model {
    let yyline = YYLine::new(0.5);
    Model { yyline }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.yyline.update(app);
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    if app.elapsed_frames() < 1 {
        draw.background().color(BLACK);
    }

    model.yyline.view(app, &draw);

    draw.rect()
        .wh(win.wh())
        .color(srgba(0.0,0.0,0.0,0.01));
    draw.to_frame(app, &frame).unwrap();
}