use nannou::prelude::*;

mod ellipser;
use crate::ellipser::Ellipser;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(500,500)
        .run();
}

struct Model {
    ellipser: Ellipser
}

fn model(_app: &App) -> Model {
    let ellipser = Ellipser::new(10.0);
    Model { ellipser }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.ellipser.update(app);
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    if app.elapsed_frames() < 1 {
        draw.background().color(BLACK);
    }

    model.ellipser.view(&win,&draw);

    draw.rect()
        .wh(win.wh())
        .color(srgba(0.0,0.0,0.0,0.05));
    draw.to_frame(app, &frame).unwrap();
}