use nannou::prelude::*;

mod shifter;
use crate::shifter::Shifter;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(400,400)
        .run();
}

struct Model {
    shifter: Shifter,
}

fn model(app: &App) -> Model {
    let win = app.window_rect();
    let shifter = Shifter::new(win, 1.0, 1.0);
    Model {shifter}
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win = app.window_rect();
    model.shifter.update(win);
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let dims = app.window_rect().w_h();

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    draw.rect().w_h(dims.0, dims.1).color(srgba(0.0,0.0,0.0,0.1));
    
    model.shifter.view(&draw);

    draw.to_frame(app, &frame).unwrap();
}