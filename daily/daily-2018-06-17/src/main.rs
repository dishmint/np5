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
    let blockmove = BlockMove::new(app, size, 170.0);

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

    draw.rect()
        .wh(win.wh())
        .color(srgba(0.0,0.0,0.0,0.04));
    draw.to_frame(app, &frame).unwrap();
}