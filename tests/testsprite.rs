use std::rand::random;
use std::vec::Items;
use std::vec;

use sdl;
use vid = sdl::video;
use evt = sdl::event;
use self::video_flags::LocalClone;

use self::video_flags::AutoRemove;
use self::video_flags::AutoContains;
use self::video_flags::AutoPush;
use self::video_flags::Toggle;

static NUM_SPRITES: int = 100;
static MAX_SPEED: int = 1;

type ArgsIter<'a> = Items<'a, ~str>;

type VideoModeFlags = (~[vid::SurfaceFlag], ~[vid::VideoFlag]);


struct VideoMode {
    flags: VideoModeFlags,
    width: int,
    height: int,
    bpp: int,
}

fn load_sprite(file: Path) -> ~vid::Surface {
    let temp;

    let sprite = match vid::Surface::from_bmp(&file) {
        Ok(s) => s,
        Err(why) => fail!("Couldn't load {}: {}", file.display(), why),
    };

    unsafe {
        if (*(*sprite.raw).format).palette.is_not_null() {
            sprite.set_color_key([vid::SrcColorKey, vid::RLEAccel],
                                 vid::Color::from_mapped(*((*sprite.raw).pixels as *u32),
                                                         (*sprite.raw).format));
        }
    }

    temp = sprite.display_format();
    let temp = match temp {
        Ok(t) => t,
        Err(why) => fail!("Couldn't convert background {}", why),
    };

    // We're ready to roll. :)
    sprite
}

struct Vel { x: i16, y: i16 }

struct Context<'a> {
    sprite: &'a vid::Surface,
    positions: ~[sdl::Rect],
    velocities: ~[Vel],
    t: int, // formerly static var in MoveSprites
    sprites_visible: bool,
    numsprites: int,
    screen_w: u16,
    screen_h: u16,
    sprite_w: u16,
    sprite_h: u16,
    sprite_rects: ~[sdl::Rect],
    debug_flip: bool,
}

fn flag_has(s: &vid::Surface, flag: uint) -> bool {
    unsafe {
        ((*s.raw).flags as uint & flag) == flag
    }
}

fn move_sprites(c: &mut Context, screen: &vid::Surface, background: vid::Color) {
    // SDL_Rect area, *position, *velocity;

    // Erase all the sprites if necessary
    if ( c.sprites_visible ) {
        screen.fill_rect(None, background);
    }

    // Move the sprite, bounce at the wall, and draw
    for i in range(0, c.numsprites) {
        let position = &mut c.positions[i];
        let velocity = &mut c.velocities[i];
        position.x += velocity.x;
        if ( (position.x < 0) || (position.x as u16 >= (c.screen_w - c.sprite_w)) ) {
            velocity.x = -velocity.x;
            position.x += velocity.x;
        }
        position.y += velocity.y;
        if ( (position.y < 0) || (position.y as u16 >= (c.screen_h - c.sprite_w)) ) {
            velocity.y = -velocity.y;
            position.y += velocity.y;
        }

        /* Blit the sprite onto the screen */
        let area = position;
        screen.blit_rect(c.sprite, None, Some(*area));
        c.sprite_rects.push(*area);
    }

    if (c.debug_flip) {
        if flag_has(screen, vid::DoubleBuf as uint) {
            let color = vid::RGB(255, 0, 0);
            let r = sdl::Rect {
                x: ((((c.t as f32).sin() * 2f32 * 3.1459) + 1.0) / 2.0 * ((c.screen_w-20) as f32)) as i16 ,
                y: 0,
                w: 20,
                h: c.screen_h,
            };
            screen.fill_rect(Some(r), color);
            c.t += 2;
        }
    }

    /* Update the screen! */
    if flag_has(screen, vid::DoubleBuf as uint) {
        screen.flip();
    } else {
        screen.update_rects(c.sprite_rects);
    }
    c.sprites_visible = true;
}

fn fastest_flags(mode: VideoMode) -> VideoModeFlags {
    let mut flags = mode.flags.clone_();

    // Hardware acceleration is only used in fullscreen mode
    flags.push(vid::Fullscreen);

    // Check for various video capabilities
    let info = vid::get_video_info();
    if info.flags.contains(&vid::BlitHWColorkey) &&
        info.flags.contains(&vid::BlitFill) {
        // We use accelerated colorkeying and color filling
        flags.push(vid::HWSurface);
    }

    // If we have enough video memory, and will use accelerated
    // blits directly to it, then use page flipping.
    if flags.contains(vid::HWSurface) {

        let video_mem = unsafe {
            let ll_info = vid::ll::SDL_GetVideoInfo();
            (*ll_info).video_mem
        };
        // Direct hardware blitting without double-buffering
        // causes really bad flickering.
        if (video_mem as int) * 1024 > (mode.height * mode.width * mode.bpp / 8) {
            flags.push(vid::DoubleBuf);
        } else {
            flags.remove(&vid::HWSurface);
        }
    }

    flags
}

pub fn main(invoker: &str, args: &[~str]) {
    // let sprite;
    let mut numsprites;
    // let sprite_rects;
    // let positions;
    // let velocities;
    let sprites_visible;
    let mut debug_flip;
    // let (sprite_w, sprite_h);

    let screen;
    // let mem;
    let (mut width, mut height) : (int, int);
    let mut video_bpp;
    let mut videoflags;
    let background;
    // let (i, done) : (int, int);
    // let event;
    let (then, now, mut frames) : (uint, uint, u32);

    sdl::init([sdl::InitVideo])
        || fail!("Couldn't initialize SDL: {}", sdl::get_error());

    numsprites = NUM_SPRITES;
    videoflags = (~[vid::SWSurface], ~[vid::AnyFormat]);
    width = 640;
    height = 480;
    video_bpp = 8;
    debug_flip = false;
    let mut i;
    let mut ai = args.iter();
    let next_int = |mut ai: ArgsIter| ai.next().and_then(|x|FromStr::from_str(x.as_slice()));
    fn is_int(s: &str) -> bool {
        let x : Option<int> = FromStr::from_str(s);
        x.is_some()
    }

    // Argument processing
    loop {
        i = ai.next();
        match i {
            None => break,
            Some(s) => {
                match s.as_slice() {
                    "-width"  => width = next_int(ai).unwrap(),
                    "-height" => height = next_int(ai).unwrap(),
                    "-bpp"  => { video_bpp = next_int(ai).unwrap();
                                 videoflags = (~[], ~[vid::AnyFormat]); }
                    "-fast"   => videoflags = fastest_flags(VideoMode {
                            flags: videoflags,
                            width: width,
                            height: height,
                            bpp: video_bpp
                        }),
                    "-hw"     => videoflags.toggle(vid::HWSurface),
                    "-flip"   => videoflags.toggle(vid::DoubleBuf),
                    "-debugflip" => debug_flip = !debug_flip,
                    "-fullscreen" => videoflags.toggle(vid::Fullscreen),
                    x if is_int(x) => numsprites = FromStr::from_str(x).unwrap(),
                    other => {
                        println!("Usage: {} [-bpp N] [-hw] [-flip] [-fast] [-fullscreen] [numsprites]", invoker);
                        fail!("Incorrect argument: {}", other);
                    }
                }
            }
        }
    }

    screen = vid::set_video_mode(width, height, video_bpp,
                                 videoflags.ref0().as_slice(),
                                 videoflags.ref1().as_slice());
    let screen = match screen {
        Ok(s) => s,
        Err(why)
            => fail!("Couldn't set {}x{} video mode: {}", width, height, why),
    };

    // Load the sprite
    let sprite = FromStr::from_str("icon.bmp").map(load_sprite)
        .expect("failed to load sprite icon.bmp");

    let mut positions : ~[sdl::Rect] = vec::with_capacity(numsprites as uint);
    let mut velocities : ~[Vel] = vec::with_capacity(numsprites as uint);
    // (rust runtime will do vec OOM checking)

    fn random_velocity() -> Vel {
        let (mut x, mut y) : (i16, i16);
        loop {
            x = ((random::<int>() % (MAX_SPEED*2 + 1)) - MAX_SPEED) as i16;
            y = ((random::<int>() % (MAX_SPEED*2 + 1)) - MAX_SPEED) as i16;
            if x != 0 && y != 0 { break; }
        }
        Vel{ x: x, y: y }
    }

    let screen_w = unsafe { (*screen.raw).w } as u16;
    let screen_h = unsafe { (*screen.raw).h } as u16;
    let sprite_w = unsafe { (*sprite.raw).w } as u16;
    let sprite_h = unsafe { (*sprite.raw).h } as u16;

    let screen_format = unsafe { (*screen.raw).format };

    let random_x = || { random::<i16>() % (screen_w as u16 - sprite_w) as i16 };
    let random_y = || { random::<i16>() % (screen_h as u16 - sprite_h) as i16 };

    for i in range(0, numsprites) {
        positions.push(sdl::Rect { x: random_x(),
                                   y: random_y(),
                                   w: sprite_w as u16,
                                   h: sprite_h as u16,
            });
        velocities.push(random_velocity());
    }
    background = vid::RGB(0,0,0); // .to_mapped(screen_format);

    // Print out information about our surfaces
    unsafe {
        println!("Screen is at {} bits per pixel", (*screen_format).BitsPerPixel);
        if flag_has(screen, vid::HWSurface as uint) {
            println!("Screen is in video memory");
        } else {
            println!("Screen is in system memory");
        }
        if flag_has(screen, vid::DoubleBuf as uint) {
            println!("Screen has double-buffering enabled");
        }
        if flag_has(sprite, vid::HWSurface as uint) {
            println!("Sprite is in video memory");
        } else {
            println!("Sprite is in system memory");
        }
        /* Run a sample blit to trigger blit acceleration */
        { let dst = sdl::Rect { x: 0, y: 0, w: sprite_w, h: sprite_h };
          screen.blit_rect(sprite, None, Some(dst)) || fail!("blit_rect failure");
          screen.fill_rect(Some(dst), background);
        }

        let SDL_HWACCEL = 0x00000100;
        if flag_has(sprite, SDL_HWACCEL) {
            println!("Sprite blit uses hardware acceleration");
        }
        if flag_has(sprite, vid::RLEAccel as uint) {
            println!("Sprite blit uses RLE acceleration");
        }

        // Loop, blitting sprites and waiting for a keystroke
        frames = 0;
        then = sdl::get_ticks();
        sprites_visible = 0;

        let mut context = Context {
            sprite: sprite,
            positions: positions,
            velocities: velocities,
            t: 0,
            sprites_visible: sprites_visible != 0,
            numsprites: numsprites,
            screen_w: screen_w,
            screen_h: screen_h,
            sprite_w: sprite_w,
            sprite_h: sprite_h,
            sprite_rects: ~[],
            debug_flip: debug_flip,
        };
        loop {
            // Check for events
            frames += 1;
            match evt::poll_event() {
                evt::MouseButtonEvent(_m, true, _x, _y)
                    => sdl::mouse::warp_mouse(context.screen_w/2 as u16,
                                              context.screen_h/2 as u16),
                evt::KeyEvent(_/*key*/, true, _/*mods*/, _/*code*/)
                    | evt::QuitEvent
                    // Any keypress quits the app...
                    => break,
                _ => {}
            }
            move_sprites(&mut context, screen, background);
        }

        // Rust should take care of freeing the owned memory (e.g. sprite)

        // Print out some timing information
        now = sdl::get_ticks();
        if ( now > then ) {
            println!("{:2.2f} frames per second\n",
                     (frames as f64 * 1000.0)/(now-then) as f64);
        }
    }
}



// Modules below are not ported from testsprite.c; they are locally
// grown to ease working with abstractions like VideoModeFlags.

mod video_flags {

    use std::tuple::Tuple2;

    use vid = sdl::video;
    use self::vid::{SurfaceFlag, VideoFlag};

    pub trait LocalClone {
        fn clone_(&self) -> Self;
    }

    // work around lack of clone()
    impl LocalClone for super::VideoModeFlags {
        fn clone_(&self) -> super::VideoModeFlags {
            let (l,r) = self.sides();
            (l.iter().map(|&x|x).collect(), r.iter().map(|&x|x).collect())
        }
    }

    pub trait Toggle<T> {
        fn toggle(&mut self, t: T);
    }

    impl<T:Eq> Toggle<T> for ~[T] {
        fn toggle(&mut self, t: T) {
            match self.position_elem(&t) {
                None => self.push(t),
                Some(i) => { self.remove(i); }
            }
        }
    }

    pub trait Sided<L,R> {
        fn sides<'a>(&'a self) -> (&'a L, &'a R);

        fn sides_mut<'a>(&'a mut self) -> (&'a mut L, &'a mut R);

        fn left_mut<'a>(&'a mut self) -> &'a mut L {
            let (l, _) = self.sides_mut();
            l
        }
        fn right_mut<'a>(&'a mut self) -> &'a mut R {
            let (_, r) = self.sides_mut();
            r
        }

        fn left<'a>(&'a self) -> &'a L {
            let (l, _) = self.sides();
            l
        }
        fn right<'a>(&'a self) -> &'a R {
            let (_, r) = self.sides();
            r
        }
    }

    impl<L,R> Sided<L,R> for (L, R) {
        fn sides<'a>(&'a self) -> (&'a L, &'a R) {
            let &(ref l, ref r) = self;
            (l, r)
        }

        fn sides_mut<'a>(&'a mut self) -> (&'a mut L, &'a mut R) {
            let &(ref mut l, ref mut r) = self;
            (l, r)
        }
    }

    pub trait AutoPush<T> {
        fn push(&mut self, T);
    }

    pub trait AutoRemove<T> {
        fn remove(&mut self, &T);
    }

    pub trait AutoContains<T> {
        fn contains(&self, T) -> bool;
    }

    impl<L,R,T:Or<L,R>> AutoPush<T> for (~[L], ~[R]) {
        fn push(&mut self, t: T) {
            let (ls, rs) = self.sides_mut();
            t.move(|l| ls.push(l), |r| rs.push(r))
        }
    }

    impl<L:Eq,R:Eq,T:Or<L,R>> AutoContains<T> for (~[L], ~[R]) {
        fn contains(&self, t: T) -> bool {
            let (ls, rs) = self.sides();
            t.move(|l| ls.contains(&l), |r| rs.contains(&r))
        }
    }

    impl<L:Eq,R:Eq,T:Or<L,R>> AutoRemove<T> for (~[L], ~[R]) {
        fn remove(&mut self, t: &T) {
            fn rem<T:Eq>(v: &mut ~[T], t: &T) {
                match v.position_elem(t) {
                    None => {},
                    Some(i) => { v.remove(i); }
                }
            }
            let (ls, rs) = self.sides_mut();
            t.borrow(|l| rem(ls, l), |r| rem(rs, r))
        }
    }

    impl<L:Eq,R:Eq,T:Or<L,R>> Toggle<T> for (~[L], ~[R]) {
        fn toggle(&mut self, t: T) {
            let (ls, rs) = self.sides_mut();
            t.move(|l| ls.toggle(l), |r| rs.toggle(r))
        }
    }

    pub trait Or<L,R> {
        fn move<A>(self, |L| -> A, |R| -> A) -> A;
        fn borrow<A>(&self, |&L| -> A, |&R| -> A) -> A;
    }

    impl Or<SurfaceFlag, VideoFlag> for SurfaceFlag {
        fn move<A>(self, l:|SurfaceFlag| -> A, _:|VideoFlag| -> A) -> A {
            l(self)
        }
        fn borrow<A>(&self, l:|&SurfaceFlag| -> A, _:|&VideoFlag| -> A) -> A {
            l(self)
        }
    }

    impl Or<SurfaceFlag, VideoFlag> for VideoFlag {
        fn move<A>(self, _:|SurfaceFlag| -> A, r:|VideoFlag| -> A) -> A {
            r(self)
        }
        fn borrow<A>(&self, _:|&SurfaceFlag| -> A, r:|&VideoFlag| -> A) -> A {
            r(self)
        }
    }
}
