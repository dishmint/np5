use nannou::prelude::*;

mod grid;

use crate::grid::Grid;
fn main() {
    nannou::app(model)
        .update(update)
        .size(400,400)
        .simple_window(view)
        .run();
}

struct Model {
    grid: Grid
}

fn model(app: &App) -> Model {
    let win = app.window_rect();
    let grid = Grid::new(win, 10,10);
    Model {grid}
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win = app.window_rect();
    model.grid.update(win);
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    draw.background().color(BLACK);

    model.grid.view(win, &draw);
    draw.to_frame(app, &frame).unwrap();
}