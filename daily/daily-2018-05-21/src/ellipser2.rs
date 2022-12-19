use nannou::prelude::*;
use nannou::ease::*;
/* STRUCT */
pub struct Ellipser2{
    size: f32,
    ptxy: Point2,
    pt2xy: Point2
}

/* IMPLEMENTATION */
impl Ellipser2{
    pub fn new (size: f32) -> Self {
        let ptxy = vec2(0.0,0.0);
        let pt2xy = vec2(0.0,0.0);
        Ellipser2 { size, ptxy, pt2xy }
    }

    pub fn update (&mut self, app: &App, index: usize) {
        let i_f = 1.0/index as f32;
        let win = app.window_rect();
        let win_bound = win.wh() - self.size;
        let time = app.time;
        let t_cos = time.cos();
        let t_sin = time.sin();
        
        let e_cos = t_cos * i_f;
        let e_sin = t_sin * i_f;
        
        /* 
        NOTE: 
            in my p5 sketch I'm mappping ptx like this:
            `map(cos(frameCount * 0.01) * i, -1, 1, ...)`
            when `cos` is multipled by `i` it is no longer in the range between -1 and 1. This is probably why the nannou app looks different.

            The other problem I just noticed is that `self.ptxy.y` should be using t_sin not e_sin.
            
            // self.ptxy.x = e_cos * (win.w() - self.size);
            // self.ptxy.y = e_sin * (win.h() - self.size);

        */
        
        // self.ptxy.x = e_cos * (win.w() - self.size);
        // self.ptxy.y = t_sin * (win.h() - self.size);

        self.ptxy.x = map_range(e_cos, -1.0, 1.0, -win_bound.x, win_bound.x);
        self.ptxy.y = map_range(t_sin, -1.0, 1.0, -win_bound.y, win_bound.y);

        // self.ptxy.x = map(
        //     e_cos,
        //     -1.0,
        //     1.0,
        //     -win_bound.x,
        //     win_bound.x,
        //     |imin, imax, omin, omax| {
        //         println!("imin: {:?}\nimax: {:?}\nomin: {:?}\nomax: {:?}\n")
        //     }
        // );
        // self.ptxy.y = map(
        //     t_sin,
        //     -1.0,
        //     1.0,
        //     -win_bound.x,
        //     win_bound.x,
        //     |imin, imax, omin, omax| {
        //         println!("imin: {:?}\nimax: {:?}\nomin: {:?}\nomax: {:?}\n")
        //     }
        // );

        
        self.pt2xy.x = self.ptxy.x * 1.5;
        self.pt2xy.y = self.ptxy.y * 1.5 - e_sin;
    }

    pub fn view (&self, draw: &Draw, index: usize) {
        let i_f = 1.0 - (1.0 / (index as f32 + 1.0)) ;
        let stroke_color = srgb(1.0 / (i_f + 0.1), i_f / 0.04, i_f);

        // dbg!(stroke_color);

        draw.ellipse()
            .xy(self.ptxy)
            .wh(vec2(self.size, self.size))
            .stroke_weight(1.0)
            .no_fill()
            .stroke(stroke_color);
        
        draw.ellipse()
            .xy(self.pt2xy)
            .wh(vec2(self.size, self.size))
            .stroke_weight(1.0)
            .no_fill()
            .stroke(stroke_color);
    }
}
