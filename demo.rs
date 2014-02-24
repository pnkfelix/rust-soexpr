#[no_main]; // provided by the SDLMain.m we'll link in.

extern crate native;
extern crate sdl;

#[no_mangle]
pub extern "C" fn SDL_main(argc: int, argv: **u8) {
    native::start(argc, argv, proc() {
            sdl::init([sdl::InitVideo, sdl::InitTimer]);
            sdl::wm::set_caption("rust-sdl demo - video", "rust-sdl");
            let screen = sdl::video::set_video_mode
                (800, 600, 32, [sdl::video::HWSurface], [sdl::video::DoubleBuf])
                .ok().expect("Failed to set_video_mode");

            let purple = sdl::video::RGB(128, 0, 128);
            let black = sdl::video::RGB(0, 0, 0);
            let square = |x:u16,y:u16,w,color|
                { screen.fill_rect(Some(sdl::Rect {
                            x: x as i16, y: y as i16, w: w, h: w }), color)
                  || fail!("error on fill_rect attempt"); };

            'main : loop {
                'events : loop {
                    match sdl::event::poll_event() {
                        sdl::event::QuitEvent |
                        sdl::event::KeyEvent(sdl::event::EscapeKey, _, _, _) => break 'main,
                        sdl::event::NoEvent => break 'events,
                        sdl::event::MouseMotionEvent(ref state, x, y, _xrel, _yrel) if state.len() > 0 => square(x, y, 30, purple),

                        sdl::event::MouseMotionEvent(ref state, x, y, _xrel, _yrel) if state.len() == 0 => square(x, y, 3, black),
                        e => {
                            println!("e: {:?}", e);
                        }
                    }
                }
                screen.flip();
            }
            println!("Hello World");
        });
}
