#[no_main]; // provided by the SDLMain.m we'll link in.

extern crate native;
extern crate sdl;

#[no_mangle]
pub extern "C" fn SDL_main(argc: int, argv: **u8) {
    native::start(argc, argv, proc() {
            sdl::init([sdl::InitVideo]);
            sdl::wm::set_caption("rust-sdl demo - video", "rust-sdl");
            let screen = sdl::video::set_video_mode(800, 600, 32, [sdl::video::HWSurface], [sdl::video::DoubleBuf]).ok();
            'main : loop {
                'events : loop {
                    match sdl::event::poll_event() {
                        sdl::event::QuitEvent => break 'main,
                        sdl::event::NoEvent => break 'events,
                        e => {
                            println!("e: {:?}", e);
                        }
                    }
                }
            }
            println!("Hello World");
        });
}
