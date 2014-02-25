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
    #[cfg(not_used_yet)]
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
                                                      &[vid::AnyFormat]);

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

impl<T:BoundBox> Bouncing<T> {
    fn new(obj: T, (x,y): (int,int), (dx,dy): (int, int)) -> Bouncing<T> {
        Bouncing { obj: obj, x: x, y: y, dx: dx, dy: dy }
    }

    fn tick(&mut self) {
        let x2 = self.x + self.dx;
        let y2 = self.y + self.dy;
        let outside_x = x2 < 0 || x2 + self.width() > width;
        let outside_y = y2 < 0 || y2 + self.height() > height;
        if outside_x { self.dx = -self.dx; } else { self.x  = x2; }
        if outside_y { self.dy = -self.dy; } else { self.y  = y2; }
    }

    #[cfg(not_used_yet)]
    fn next(self) -> Bouncing<T> {
        Bouncing { x: self.x + self.dx, y: self.y + self.dy, .. self }
    }
}

impl<T:BoundBox> Bouncing<T> {
    fn erase_on(&self, screen: &vid::Surface) {
        screen.fill_rect(Some(sdl::Rect { x: self.x as i16,
                                          y: self.y as i16,
                                          w: self.width() as u16,
                                          h: self.height() as u16, }),
                         vid::RGB(0,0,0));
    }
}

impl Bouncing<Shape> {
    fn draw_on(&self, screen: &vid::Surface) {
        screen.blit_at(self.obj.surface, self.x as i16, self.y as i16);
    }
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

    let mut shape = Bouncing::new(shape, (300, 0), (1, 2));
    let mut shape2 = Bouncing::new(shape2, (0, 20), (-4, 3));

    let (mut x, mut y) = (0i16, 0i16);

    let mut frames = 0;
    let then = sdl::get_ticks();

    let mut shapes = ~[shape2, shape];

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
                shapes.push(Bouncing::new(circle(10, frames),
                                          (x as int, y as int),
                                          (frames % 3, -(frames % 4))));
            }
            evt::MouseButtonEvent(_, _, _x, _y) => {
                let shape0 = shapes.shift().unwrap();
                let shape1 = shapes.shift().unwrap();
                for s in shapes.iter() {
                    s.erase_on(screen);
                }
                shapes = ~[shape0, shape1];
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
