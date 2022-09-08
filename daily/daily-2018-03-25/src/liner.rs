use nannou::prelude::*;

struct Mvec {
    up: Vec2,
    rt: Vec2,
    dn: Vec2,
    lt: Vec2,
    dir: i8
}
pub struct Liner {
    start: Vec2,
    end: Vec2,
    mvec: Mvec
}

impl Liner {
    pub fn new(win: Rect, inc: f32) -> Liner {
        let start = vec2(win.left() , 0.0);
        let end   = vec2(win.right(), 0.0);

        let mvec = Mvec {
            up: vec2(0.0, inc),
            rt: vec2(inc, 0.0),
            dn: vec2(0.0, -inc),
            lt: vec2(-inc, 0.0),
            dir: 0
        };

        Liner {start, end, mvec}
    }

    pub fn update(&mut self, app: &App) {
        let win = app.window_rect();

        // choose direction
        let sm = match self.mvec.dir {
            0 => (self.mvec.up, "up"),
            1 => (self.mvec.rt, "rt"),
            2 => (self.mvec.dn, "dn"),
            3 => (self.mvec.lt, "lt"),
            _ => (vec2(0.0,0.0), "--"),
        };

        let op = (self.mvec.dir + 2) % 4;
        let em = match op {
            0 => (self.mvec.up, "up"),
            1 => (self.mvec.rt, "rt"),
            2 => (self.mvec.dn, "dn"),
            3 => (self.mvec.lt, "lt"),
            _ => (vec2(0.0,0.0), "--"),
        };

        // println!("sm: {} | em: {}", sm.1, em.1);

        self.start += sm.0;
        self.end   += em.0;


        if self.start == win.top_left() {
            self.mvec.dir = 1;
        }

        if self.start == win.top_right() {
            self.mvec.dir = 2;
        }
        
        if self.start == win.bottom_right() {
            self.mvec.dir = 3;
        }

        if self.start == win.bottom_left() {
            self.mvec.dir = 0;
        }

    }

    pub fn view(&self, draw: &Draw) {
        draw.line()
            .start(self.start)
            .end(self.end)
            .color(TOMATO);
    }
}