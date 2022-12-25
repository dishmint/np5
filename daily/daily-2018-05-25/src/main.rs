use nannou::prelude::*;

mod bez;
use crate::bez::Bez;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(500,500)
        .run();
}

struct Model {
    bez: Bez
}

fn model(app: &App) -> Model {
    let w = app.window_rect().w();
    let size = w * 0.01;
    let color = RED;
    let bez = Bez::new(size, 0.5, color);
    Model { bez }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.bez.update(&app);
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    if app.elapsed_frames() < 1 {
        draw.background().color(BLACK);
    }
    model.bez.view(&draw);
    draw.rect()
        .wh(win.wh())
        .color(srgba(0.0,0.0,0.0,0.01));
    draw.to_frame(app, &frame).unwrap();
}