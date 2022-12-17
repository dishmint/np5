use nannou::prelude::*;

mod liner;
use crate::liner::Liner;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(500,500)
        .run();
}

struct Model {
    liner: Liner
}

fn model(app: &App) -> Model {
    let win = app.window_rect();
    let liner = Liner::new(0.0,10.0, win);
    Model { liner }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.liner.update();
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    if app.elapsed_frames() < 1 {
        draw.background().color(BLACK);
    }

    model.liner.view(&draw);

    draw.rect()
        .wh(win.wh())
        .color(srgba(0.0,0.0,0.0,0.05));
    draw.to_frame(app, &frame).unwrap();
}