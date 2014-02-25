use std::num::{Zero, One};
use std::cmp::Ord;

use sdl;
use vid = sdl::video;
use evt = sdl::event;

static width:int  = 640;
static height:int = 480;
static video_bpp: int = 32;

#[deriving(Eq)]
enum RGBorA { R = 0, G = 1, B = 2, A = 3 }

struct Shape {
    surface: ~vid::Surface,
    width: int,
    height: int,
}

impl Shape {
    fn new(w: int, h: int, f: |int, int, RGBorA| -> u8) -> Shape {
        Shape { surface: new_shape(w, h, f),
                width: w,
                height: h }
    }
}

fn new_shape(w: int, h: int, f: |int, int, RGBorA| -> u8) -> ~vid::Surface {
    let shape = vid::Surface::new([vid::HWSurface], w, h, video_bpp,
                                  // Following mask values indicate
                                  // how each of {R,G,B,A} should be
                                  // extracted from the bits for a
                                  // pixel in the [u8] for the
                                  // surface.
                                  0x000000FF,
                                  0x0000FF00,
                                  0x00FF0000,
                                  0xFF000000);
    let shape = match shape {
        Ok(s) => s,
        Err(why)
            => fail!("Couldn't create {}x{} surface for shape: {}", w, h, why),
    };

    shape.with_lock(|bmp| {
            for i in range(0, w) {
                for j in range(0, h) {
                    for &c in [R,G,B,A].iter() {
                        let result = f(i, j, c);
                        let c = c as int;
                        bmp[j*w*4 + i*4 + c] = result;
                    }
                }
            }
        });

    shape
}

impl Shape {
    fn circle(radius: int, rgb: (u8,u8,u8)) -> Shape {
        let w = radius*2;
        let h = radius*2;
        Shape { surface: new_circle(w, h, radius, rgb),
                width: w,
                height: h }
    }
}

fn new_circle(w:int, h:int, radius:int, (r,g,b): (u8,u8,u8)) -> ~vid::Surface {
    let w_2 = w/2;
    let h_2 = h/2;
    new_shape(w, h, |i,j,c| {
            let dx = w_2 - i;
            let dy = h_2 - j;
            match (dx*dx + dy*dy < radius*radius, c) {
                (true,  A) => 0xFFu8,
                (true,  R) => r,
                (true,  G) => g,
                (true,  B) => b,
                _          => 0x00u8,
            }
        })
}

static video_flags : (&'static [vid::SurfaceFlag],
                      &'static [vid::VideoFlag])   = (&[vid::HWSurface],
                                                      &[vid::AnyFormat,
                                                        vid::DoubleBuf]);

trait BoundBox {
    fn width(&self) -> int;
    fn height(&self) -> int;
}

impl BoundBox for Shape {
    fn width(&self) -> int { self.width }
    fn height(&self) -> int { self.height }
}

struct Bouncing<T> {
    obj: T,
    x: int,
    y: int,
    dx: int,  // per tick
    dy: int,  // per tick
}

impl<T:BoundBox> BoundBox for Bouncing<T> {
    fn width(&self) -> int { self.obj.width() }
    fn height(&self) -> int { self.obj.height() }
}

trait Marking {
    fn erase_on(&self, screen: &vid::Surface);
    fn draw_on(&self, screen: &vid::Surface);
}

trait Turtle : Marking {
    fn tick(&mut self);
}

impl<V:Visible+BoundBox> Marking for Bouncing<V> {
    fn erase_on(&self, screen: &vid::Surface) {
        screen.fill_rect(Some(sdl::Rect { x: self.x as i16,
                                          y: self.y as i16,
                                          w: self.width() as u16,
                                          h: self.height() as u16, }),
                         vid::RGB(0,0,0));
    }

    fn draw_on(&self, screen: &vid::Surface) {
        screen.blit_at(self.obj.surface(), self.x as i16, self.y as i16);
    }
}

impl<T:Visible+BoundBox> Turtle for Bouncing<T> {
    fn tick(&mut self) {
        let x2 = self.x + self.dx;
        let y2 = self.y + self.dy;
        let outside_x = x2 < 0 || x2 + self.width() > width;
        let outside_y = y2 < 0 || y2 + self.height() > height;
        if outside_x { self.dx = -self.dx; } else { self.x  = x2; }
        if outside_y { self.dy = -self.dy; } else { self.y  = y2; }
    }
}

impl<T:BoundBox> Bouncing<T> {
    fn new(obj: T, (x,y): (int,int), (dx,dy): (int, int)) -> Bouncing<T> {
        Bouncing { obj: obj, x: x, y: y, dx: dx, dy: dy }
    }

    #[cfg(not_used_yet)]
    fn next(self) -> Bouncing<T> {
        Bouncing { x: self.x + self.dx, y: self.y + self.dy, .. self }
    }
}

struct Tortoise {
    ticks: int,
    head: (f32, f32),
    x: int,
    y: int,
    ddx: f32,
    ddy: f32,
}

impl Tortoise {
    fn new() -> Tortoise {
        Tortoise { ticks: 0, head: (0.0,20.0), x: 0, y: 0, ddx: 0.3, ddy: -0.2 }
    }

    fn shape(&self) -> Shape {
        let (h_dx, h_dy) = self.head;
        let (h_dx, h_dy) = (h_dx as int, h_dy as int);
        let head_radius = 15;
        let body_radius = 20;
        let w = head_radius*2 + body_radius*2 + h_dx.abs();
        let h = head_radius*2 + body_radius*2 + h_dy.abs();
        let w_2 = w/2;
        let h_2 = h/2;
        let (h_ctr_x, h_ctr_y) = (w_2 + h_dx, h_2 + h_dy);
        Shape::new(w, h, |i, j, c| { 
                let h_dx = h_ctr_x - i;
                let h_dy = h_ctr_y - j;
                let b_dx = w_2 - i;
                let b_dy = h_2 - j;
                if (h_dx*h_dx + h_dy*h_dy < head_radius*head_radius ||
                    b_dx*b_dx + b_dy*b_dy < body_radius*body_radius) {
                    255
                } else {
                    0
                }
            })
    }
}

impl Tortoise {
    fn x(&self) -> int {
        self.x
    }

    fn y(&self) -> int {
        self.y
    }
}

impl BoundBox for Tortoise {
    fn width(&self) -> int { self.shape().width }
    fn height(&self) -> int { self.shape().height }
}

impl Marking for Tortoise {
    fn erase_on(&self, screen: &vid::Surface) {
        screen.fill_rect(Some(sdl::Rect { x: self.x() as i16,
                                          y: self.y() as i16,
                                          w: self.width() as u16,
                                          h: self.height() as u16, }),
                         vid::RGB(0,0,0));
    }
    fn draw_on(&self, screen: &vid::Surface) {
        screen.blit_at(self.shape().surface, self.x() as i16, self.y() as i16);
    }
}

impl Turtle for Tortoise {
    fn tick(&mut self) {
        self.ticks += 1;
        let (mut dx, mut dy) = self.head;
        let (x2, y2) = (self.x + dx as int, self.y + dy as int);

        fn reset_unit<N:Num+Ord>(x:N) -> N {
            let n: N = One::one();
            if x > Zero::zero() { -n } else { n } 
        }

        if x2 < 0 || (x2 + self.width() > width) {
            dx = reset_unit(dx);
            self.ddx = -self.ddx;
        } else {
            self.x = x2;
            dx += self.ddx;
        }
        if y2 < 0 || (y2 + self.height() > height) {
            dy = reset_unit(dy);
            self.ddy = -self.ddy;
        } else {
            self.y = y2;
            dy += self.ddy;
        }
        self.head = (dx, dy);
    }
}

trait Visible {
    fn surface<'a>(&'a self) -> &'a vid::Surface;

    fn draw_at(&self, screen: &vid::Surface, x: i16, y: i16) {
        screen.blit_at(self.surface(), x, y);
    }
}

impl Visible for Shape {
    fn surface<'a>(&'a self) -> &'a vid::Surface { &*self.surface }
}

pub fn main(invoker: &str, args: &[~str]) {
    println!("running {} args: {}", invoker, args);

    sdl::init([sdl::InitVideo])
        || fail!("Couldn't initialize SDL: {}", sdl::get_error());

    let screen = vid::set_video_mode(width, height, video_bpp,
                                     video_flags.ref0().as_slice(),
                                     video_flags.ref1().as_slice());

    let screen = match screen {
        Ok(s) => s,
        Err(why)
            => fail!("Couldn't set {}x{} video mode: {}", width, height, why),
    };

    let circle = |radius: int, color_basis:int| {
        let num = color_basis;
        Shape::circle(radius, (((num << 4) | (num >> 4)) as u8,
                               ((num << 2) | (num >> 2)) as u8,
                               num as u8))
    };

    let shape  = circle(30, 0x1234);
    let shape2 = circle(30, 0xF0DC);

    let shape = Bouncing::new(shape, (300, 0), (1, 2));
    let shape2 = Bouncing::new(shape2, (0, 20), (-4, 3));

    let (mut x, mut y) = (0i16, 0i16);

    let mut frames = 0;
    let then = sdl::get_ticks();

    let mut shapes = ~[~shape2 as ~Turtle, ~shape as ~Turtle];

    let turtle = Tortoise::new();
    shapes.push(~turtle as ~Turtle);

    loop {
        frames += 1;
        if frames > 1000 { break; }
        x = x % width as i16;
        y = y % height as i16;

        // screen.clear();

        for s in shapes.iter() {
            s.erase_on(screen);
        }

        for s in shapes.mut_iter() {
            s.tick();
        }

        for s in shapes.iter() {
            s.draw_on(screen);
        }

        screen.flip();
        match evt::poll_event() {
            evt::KeyEvent(evt::EscapeKey, _, _, _) | evt::QuitEvent => break,
            evt::NoEvent => {}
            evt::MouseMotionEvent(_, x, y, _x_rel, _y_rel) => {
                shapes.push(~Bouncing::new(circle(10, frames),
                                          (x as int, y as int),
                                          (frames % 3, -(frames % 4)))
                            as ~Turtle);
            }
            evt::MouseButtonEvent(_, _, _x, _y) => {
                let shape0 = shapes.shift().unwrap();
                let shape1 = shapes.shift().unwrap();
                let shape2 = shapes.shift().unwrap();
                for s in shapes.iter() {
                    s.erase_on(screen);
                }
                shapes = ~[shape0, shape1, shape2];
            }
            _ => {}
        }
    }

    // Print out some timing information
    let now = sdl::get_ticks();
    if now > then {
        println!("{:2.2f} frames per second\n",
                 (frames as f64 * 1000.0)/(now-then) as f64);
    }
}
