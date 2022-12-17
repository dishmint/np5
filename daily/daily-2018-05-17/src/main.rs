use nannou::prelude::*;

mod cube;
use crate::cube::Cube;

/* TODO: 3D Cuboid not implemented. Moving on for now. */
fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(_app: &App, _model: &Model, frame: Frame){
    frame.clear(PURPLE);
}