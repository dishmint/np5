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
    let _win = app.window_rect();
    let shifter = Shifter::new(1.0, 1.0);
    Model {shifter}
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win = app.window_rect();
    model.shifter.update(win);
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let dims = app.window_rect().w_h();
    let center = app.window_rect().x_y();

     // draw.background().color(BLACK);

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    model.shifter.view(&draw);

    draw.rect()
        .xy(vec2(center.0, center.1))
        .w_h(dims.0, dims.1)
        .color(srgba(0.0,0.0,0.0, model.shifter.alpha));

    // draw.text("Shifter").xy(vec2(center.0, center.1)).color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}