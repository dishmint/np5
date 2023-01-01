use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(500, 500)
        .run();
}

struct Direction {
    a: Vec2, 
    b: Vec2,
}

struct Model {
    direction: Direction,
    start: Vec2,
    end: Vec2,
    scale: f32,
    theta: f32,
    speed: f32,
}

fn model(app: &App) -> Model {
    let win = app.window_rect();

    let center = win.xy();
    
    let dir_a = vec2(1.0,1.0);
    let dir_b = vec2(1.0,1.0);

    let direction = Direction {a: dir_a, b: dir_b};

    let scale = 1.0;

    let theta = 0.0;

    let speed = 1.0;

    Model { direction, start: center, end: center, scale, theta, speed }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.time;
    model.theta = time * model.speed;
    // let theta = time.sin();

    let win = app.window_rect();
    
    let width  = win.w() * 0.5;
    let height = win.h() * 0.5;
    
    model.start.x = clamp(model.start.x, -width, width);
    model.end.y   = clamp(model.end.y,   -height, height);
    
    model.start.x += model.direction.a.x;
    model.end.y   += model.direction.b.y;
    
    if model.start.x > width {
        model.start.y += model.direction.a.y;
        if model.start.y > height {
            model.start = vec2(-width,-height)
        }
    }

    if model.end.y > height {
        model.end.x += model.direction.b.x;
        if model.end.x > width {
            model.end = vec2(-width,-height)
        }
    }


}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    if app.elapsed_frames() < 1 {
        draw.background().color(BLACK);
    }
    /* LINE 0 */
    draw.line()
        .start(model.start)
        .end(model.end)
        .stroke_weight(1.0)
        .color(WHITE)
        /* .roll(model.theta) */;
    /* LINE 1 - Top Left */
    draw.line()
        .start(model.start * -model.scale)
        .end(model.end)
        .stroke_weight(1.0)
        .color(RED)
        /* .roll(model.theta) */;
    /* LINE 2 - Top Right */
    draw.line()
        .start(model.start * model.scale)
        .end(model.end * model.scale)
        .stroke_weight(1.0)
        .color(RED)
        /* .roll(model.theta) */;
    /* LINE 3 - Bottom Left */
    draw.line()
        .start(model.start * -model.scale)
        .end(model.end * -model.scale)
        .stroke_weight(1.0)
        .color(WHITE)
        /* .roll(model.theta) */;
    /* LINE 4 - Bottom Right */
    draw.line()
        .start(model.start * model.scale)
        .end(model.end * -model.scale)
        .stroke_weight(1.0)
        .color(WHITE)
        /* .roll(model.theta) */;

    draw.rect()
        .wh(win.wh())
        // .color(srgba(0.0,0.0,0.0,0.0));
        .color(srgba(0.0,0.0,0.0,0.01));
        // .color(srgba(0.0,0.0,0.0,0.25));
    draw.to_frame(app, &frame).unwrap();
}