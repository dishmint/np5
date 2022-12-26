use nannou::prelude::*;

mod fzn;
mod hourglass;
use crate::hourglass::Hourglass;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(500,500)
        .run();
}

struct Model {
    hourglass: Hourglass
}

fn model(app: &App) -> Model {
    let win = app.window_rect();
    let size = win.w() * 0.01;
    let hourglass = Hourglass::new(size, 0.8);
    Model { hourglass }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.hourglass.update(app);
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    if app.elapsed_frames() < 1 {
        draw.background().color(BLACK);
    }
    
    model.hourglass.view(&draw);
    
    draw.rect()
        .wh(win.wh())
        .color(srgba(0.0,0.0,0.0,0.01));
    draw.to_frame(app, &frame).unwrap();
}