use sdl;

pub fn main() {
    // let sprite;
    // let numsprites;
    // let sprite_rects;
    // let positions;
    // let velocities;
    // let sprites_visible;
    // let debug_flip;
    // let (sprite_w, sprite_h);

    // let screen;
    // let mem;
    // let (width, height) : (int, int);
    // let video_bpp;
    // let videoflags;
    // let background;
    // let (i, done) : (int, int);
    // let event;
    // let (then, now, frames) : (u32, u32, u32);

    sdl::init([sdl::InitVideo])
        || fail!("Couldn't initialize SDL: {}", sdl::get_error());

    fail!("testsprite unimplemented");
}
