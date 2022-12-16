use nannou::prelude::*;

mod rotor;
use crate::rotor::Rotor;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(500,500)
        .run();
}

struct Model {
    rotor: Rotor,
    alpha: f64
}

fn model(app: &App) -> Model {
    let win = app.window_rect();
    let rotor = Rotor::new(win.wh() * 0.5, 0.0, 0.01);
    let alpha = 0.01;
    Model {
        rotor,
        alpha
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.rotor.update();
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();

    if app.elapsed_frames() < 1 {
        draw.background().color(BLACK);
    }

    model.rotor.view(&draw);

    draw.rect()
        .wh(win.wh())
        .color(srgba(0.0,0.0,0.0,model.alpha));

    draw.to_frame(app, &frame).unwrap();
}