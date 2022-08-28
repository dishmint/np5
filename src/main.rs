use nannou::prelude::*;

mod morpher;
use crate::morpher::Morpher;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    morpher: Morpher
}

fn model(app: &App) -> Model {
    let win = app.window_rect();
    let wx = win.wh().x * 0.08;
    let morpher = Morpher::new(
        win
        , vec2(wx, wx)
        , 0.0
        , 5.0
        , 360
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