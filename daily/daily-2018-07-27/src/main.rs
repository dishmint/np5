use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(500,500)
        .run();
}
struct XLine {
    start: Point2,
    end: Point2
}

impl XLine {
    pub fn new (start: Point2, end: Point2) -> Self {
        XLine {start, end}
    }
}

struct Model {
    rate: f32,
    inner_box: Rect,
    line_1: XLine,
    line_2: XLine,
    line_3: XLine,
}

fn model(app: &App) -> Model {
    let rate = 1.0;

    let win = app.window_rect();
    let b_width = win.right() * 0.25;

    let inner_box = Rect::from_wh(vec2(b_width, b_width));

    let line_1 = XLine::new(vec2(0.0, win.top()), win.xy());
    let line_2 = XLine::new(vec2(0.0, inner_box.top()), vec2(0.0, inner_box.bottom()));
    let line_3 = XLine::new(win.xy(), vec2(0.0, win.bottom()));
    Model { rate, inner_box, line_1, line_2, line_3 }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win = app.window_rect();
    let time = app.time * model.rate;
    let sin_x = time.sin();
    let cos_x = time.cos();

    let sin_iw = sin_x * model.inner_box.right();

    let cos_w = cos_x * win.right();
    let cos_h = cos_x * win.bottom();

    model.line_1.start.x = cos_w;
    model.line_1.end     = vec2(cos_w, cos_h);

    model.line_2.start.x = sin_iw;
    model.line_2.end.x   = sin_iw;

    model.line_3.start = vec2(cos_w, cos_h );
    model.line_3.end.x = cos_w;
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    if app.elapsed_frames() < 1 {
        draw.background().color(BLACK);
    }
    /* STATIC SHAPES */
    draw.rect()
        .no_fill()
        .stroke_color(WHITE)
        .stroke_weight(1.0)
        .wh(model.inner_box.wh());
        
    draw.line()
        .color(WHITE)
        .start(win.top_left())
        .end(model.inner_box.top_left());
        
    draw.line()
        .color(WHITE)
        .start(win.top_left())
        .end(model.inner_box.top_right());
        
    draw.line()
        .color(WHITE)
        .start(model.inner_box.bottom_left())
        .end(win.bottom_right());
        
    draw.line()
        .color(WHITE)
        .start(model.inner_box.bottom_right())
        .end(win.bottom_right());

    /* FILLER */
    draw.line()
        .color(RED)
        .start(model.line_1.start)
        .end(model.line_1.end);

    draw.line()
        .color(BLUE)
        .start(model.line_2.start)
        .end(model.line_2.end);
    
    draw.line()
        .color(GREEN)
        .start(model.line_3.start)
        .end(model.line_3.end);

    draw.rect()
        .wh(win.wh())
        .color(srgba(0.0,0.0,0.0,10.0/255.0));
    draw.to_frame(app, &frame).unwrap();
}