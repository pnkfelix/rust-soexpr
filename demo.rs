#![allow(unused_imports)] // don't warn me about this right now

extern crate native;
extern crate rand;

extern crate sdl = "sdl2";

use std::os;

use evt = sdl::event;
use vid = sdl::video;
use pix = sdl::pixels;
use key = sdl::keycode;
use rend = sdl::render;
use surf = sdl::surface;
use rect = sdl::rect;
use sdl::event::{QuitEvent, NoEvent};
use KeyEvent = sdl::event::KeyUpEvent;
use sdl::rect::{Rect};
use Motion = sdl::event::MouseMotionEvent;

#[start]
pub fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

#[main]
fn main() {
    let args = os::args();
    if args.len() >= 2 {
        dispatch(args[0], args[1], args.slice_from(2));
    } else {
        default();
    }
}

fn dispatch(driver: &str, variant: &str, args: &[~str]) {
    let invoker = format!("{} {}", driver, variant);
    match variant {
/*
        "testsprite"
            => tests::testsprite::main(invoker, args),
        "soe"
            => tests::soe::main(invoker, args),
*/
        "hello"
            => hello().unwrap(),
        _otherwise
            => default(),
    }
}

mod tests {
//    pub mod testsprite;
//    pub mod soe;
}

fn hello() -> Result<(), ~str> {
    static SCREEN_WIDTH:int = 640;
    static SCREEN_HEIGHT:int = 480;
    try!(sdl::init([sdl::InitEverything]));
    let win = try!(
        vid::Window::new("Hello World", 100, 100, SCREEN_WIDTH, SCREEN_HEIGHT, [vid::Shown]));
    let ren = try!(rend::Renderer::from_window(
        win, rend::DriverAuto, [rend::Accelerated, rend::PresentVSync]));
    let file = Path::new("hello.bmp");
    let bmp = try!(surf::Surface::from_bmp(&file));
    let tex = try!(ren.create_texture_from_surface(bmp));
    fn renderTexture(tex: &rend::Texture, ren: &rend::Renderer, x: i32, y: i32) -> Result<(), ~str> {
        let q = try!(tex.query());
        let dst = rect::Rect::new(x, y, q.width, q.height);
        ren.copy(tex, None, Some(dst))
    }
    ren.clear();
    ren.copy(tex, None, None);
    ren.present();
    sdl::timer::delay(2000);
    Ok(())
}

#[cfg(not(with_default))]
fn default() { unimplemented!() }
#[cfg(with_default)]
fn default() {
    sdl::init([sdl::InitVideo, sdl::InitTimer]);
    let screen = vid::Window::new("rust-sdl demo - video",
        vid::PosUndefined, vid::PosUndefined, 800, 600, [])
        .ok().expect("Failed to set_video_mode");

    let purple = pix::RGB(128, 0, 128);
    let black = pix::RGB(0, 0, 0);
    let square = |x:u16, y:u16, width, color| {
        let w = width;
        let r = Rect { x: x as i32, y: y as i32, w: w, h: w };
        screen.fill_rect(Some(r), color)
            || fail!("error on fill_rect attempt");
    };

    'main : loop {
        'events : loop {
            match evt::poll_event() {
                QuitEvent | KeyEvent(key::EscapeKey, _, _, _)
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
