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
    let morpher = Morpher::new(app.window_rect(), 50.0, 0.0, 5.0);
    Model { morpher }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.morpher.update();
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    model.morpher.display(&draw);
    draw.to_frame(app, &frame).unwrap();
}