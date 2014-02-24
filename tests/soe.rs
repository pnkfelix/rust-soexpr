use sdl;
use vid = sdl::video;
use evt = sdl::event;

static width:int = 640;
static height:int = 480;
static video_bpp: int = 32;

#[deriving(Eq)]
enum RGBorA { R = 0, G = 1, B = 2, A = 3 }

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

    let shape  = new_circle(100, 100, 50, (0xF0u8, 0x20u8, 0x30u8));
    let shape2 = new_circle(100, 100, 50, (0x10u8, 0xA0u8, 0xB0u8));

    // shape.set_alpha([vid::SrcAlpha], 128u8);
    // shape2.set_alpha([vid::SrcAlpha], 128u8);

    let (mut x, mut y) = (0i16, 0i16);
    loop {
        x = x % width as i16;
        y = y % height as i16;
        screen.clear();
        screen.blit_at(shape2, x + 50, y - 50);
        screen.blit_at(shape, x, y);
        screen.flip();
        match evt::poll_event() {
            evt::KeyEvent(evt::EscapeKey, _, _, _) | evt::QuitEvent => break,
            evt::NoEvent => {}
            _ => {}
        }
        x += 1;
        y += 1;
    }
}
