use nannou::prelude::*;

mod blockmove;
use crate::blockmove::BlockMove;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(500,500)
        .run();
}

struct Model {
    blockmove: BlockMove,
}

fn model(app: &App) -> Model {
    let size = 0.5;
    let mut blockmove = BlockMove::new(app, size, 250.0);
    blockmove.opt_outline = false;

    Model { blockmove }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.blockmove.update(app);
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    if app.elapsed_frames() < 1 {
        draw.background().color(BLACK);
    }

    model.blockmove.view(&draw);

    // let alpha = model.blockmove.beat_time / model.blockmove.beat_length;
    // let alpha = 1.0 / (win.w() / model.blockmove.width);

    draw.rect()
        .wh(win.wh())
        /* .color(srgba(0.0,0.0,0.0,0.05)); */
        /* .color(srgba(0.0,0.0,0.0,alpha)); */
        /* .color(srgba(0.0,0.0,0.0,0.0)); */
        .color(srgba(0.0,0.0,0.0,0.01));
        /* .color(srgba(0.0,0.0,0.0,alpha)); */
    draw.to_frame(app, &frame).unwrap();
}