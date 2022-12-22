use nannou::prelude::*;

mod collipser;
use crate::collipser::Collipser;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(500,500)
        .run();
}

struct Model {
    collipser: Collipser,
    c_max: i32
}

fn model(_app: &App) -> Model {
    let x_speed = 0.25;
    let _y_speed = x_speed * 0.5;
    let y_speed = x_speed;
    let collipser = Collipser::new(vec2(x_speed, y_speed));
    let c_max = 10;
    Model { collipser, c_max }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.collipser.update(app);
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    if app.elapsed_frames() < 1 {
        draw.background().color(BLACK);
    }
    for i in 1..=model.c_max {
        model.collipser.view(&draw, i as f32)
    }
    draw.rect()
        .wh(win.wh())
        .color(srgba(0.01,0.01,0.01,0.025));
    draw.to_frame(app, &frame).unwrap();
}