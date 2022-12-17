use nannou::prelude::*;

mod flipper;
use crate::flipper::Flipper;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(500,500)
        .run();
}

struct Model {
    flipper_left: Flipper,
    flipper_center: Flipper,
    flipper_right: Flipper,
}

fn model(app: &App) -> Model {
    let win = app.window_rect();
    let offset = vec2(0.33, 0.0) * win.wh();

    let flipper_left =
        Flipper::new(
            0.0, 
            0.01, 
            win.w() * 0.33,
            win.xy() - offset,
            srgb(1.0, 1.0, 1.0)
        );

    let flipper_center =
        Flipper::new(
            0.0,
            0.01,
            win.w() * 0.33,
            win.xy(),
            srgb(1.0, 0.39, 0.28)
        );
    
    let flipper_right =
        Flipper::new(
            0.0,
            0.01,
            win.w() * 0.33,
            win.xy() + offset,
            srgb(1.0, 1.0, 1.0)
        );
    
    Model { flipper_left, flipper_center, flipper_right }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.flipper_left.update();
    model.flipper_center.update();
    model.flipper_right.update();
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    if app.elapsed_frames() < 1 {
        draw.background().color(BLACK);
    }
    
    model.flipper_left.view(&draw);
    model.flipper_center.view(&draw);
    model.flipper_right.view(&draw);
    
    draw.rect()
        .wh(win.wh())
        .color(srgba(0.0,0.0,0.0,0.01));
    draw.to_frame(app, &frame).unwrap();
}