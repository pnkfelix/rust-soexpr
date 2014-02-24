#[no_main]; // provided by the SDLMain.m we'll link in.

extern crate native;
extern crate sdl;

use std::os;

use evt = sdl::event;
use vid = sdl::video;
use sdl::event::{QuitEvent, KeyEvent, NoEvent};
use Motion = sdl::event::MouseMotionEvent;

#[no_mangle]
pub extern "C" fn SDL_main(argc: int, argv: **u8) {
    native::start(argc, argv, proc() {
            let args = os::args();
            if args.len() >= 2 && args[1] == ~"testsprite" {
                tests::testsprite::main(format!("{} {}", args[0], args[1]),
                                        args.slice_from(2));
            } else {
                default();
            }
        });
}

mod tests {
    pub mod testsprite;
}

fn default() {
    sdl::init([sdl::InitVideo, sdl::InitTimer]);
    sdl::wm::set_caption("rust-sdl demo - video", "rust-sdl");
    let screen = vid::set_video_mode
        (800, 600, 32, [vid::HWSurface], [vid::DoubleBuf])
        .ok().expect("Failed to set_video_mode");

    let purple = vid::RGB(128, 0, 128);
    let black = vid::RGB(0, 0, 0);
    let square = |x:u16, y:u16, width, color| {
        let w = width;
        let r = sdl::Rect { x: x as i16, y: y as i16, w: w, h: w };
        screen.fill_rect(Some(r), color)
            || fail!("error on fill_rect attempt");
    };

    'main : loop {
        'events : loop {
            match evt::poll_event() {
                QuitEvent | KeyEvent(evt::EscapeKey, _, _, _)
                    => break 'main,
                NoEvent
                    => break 'events,
                Motion(ref state, x, y, _xrel, _yrel) if state.len() > 0
                    => square(x, y, 30, purple),
                Motion(ref state, x, y, _xrel, _yrel) if state.len() == 0
                    => square(x, y, 3, black),
                e
                    => println!("e: {:?}", e),
            }
        }
        screen.flip();
    }
    println!("Hello World");
}
