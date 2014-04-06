#![allow(unused_imports)] // don't warn me about this right now

extern crate native;
extern crate rand;

extern crate sdl = "sdl2";
extern crate gl;

use std::libc;
use std::mem;
use std::os;
use std::ptr;
use std::str;

use evt = sdl::event;
use vid = sdl::video;
use pix = sdl::pixels;
use key = sdl::keycode;
use rend = sdl::render;
use surf = sdl::surface;
use rect = sdl::rect;
use sdl::event::{QuitEvent, NoEvent};
use sdl::event::KeyUpEvent;
use sdl::rect::{Rect};
use sdl::event::MouseMotionEvent;

#[start]
pub fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

#[main]
fn main() {
    let args = os::args();
    if args.len() >= 2 {
        match dispatch(args[0], args[1], args.slice_from(2)) {
            Ok(_) => {},
            Err(s) => fail!("dispatch failed: {}", s),
        }
    } else {
        match default() {
            Ok(_) => {},
            Err(s) => fail!("default failed: {}", s),
        }
    }
}

fn dispatch(driver: &str, variant: &str, _args: &[~str]) -> Result<(), ~str> {
    let _invoker = ||format!("{} {}", driver, variant);
    match variant {
/*
        "testsprite"
            => tests::testsprite::main(_invoker(), _args),
        "soe"
            => tests::soe::main(_invoker(), _args),
*/
        "open_gl"  => open_gl(),
        "hello"    => hello(),
        _otherwise => default(),
    }
}

mod tests {
//    pub mod testsprite;
//    pub mod soe;
}

// Tried:
//   http://useful-linux-tips.blogspot.fr/2013/11/complete-minimal-sdl2-opengl-animation.html
// but it uses glFrustum, which apparently has been deprecated 

fn open_gl() -> Result<(), ~str> {
    // http://www.open.gl/context
    static SCREEN_WIDTH:i32 = 800;
    static SCREEN_HEIGHT:i32 = 600;

    try!(sdl::init([sdl::InitEverything]));
    try!(vid::gl_set_attribute(vid::GLContextProfileMask,
                               vid::ll::SDL_GL_CONTEXT_PROFILE_CORE as int));
    try!(vid::gl_set_attribute(vid::GLContextMajorVersion, 3));
    try!(vid::gl_set_attribute(vid::GLContextMinorVersion, 2));

    let (width, height) = (SCREEN_WIDTH, SCREEN_HEIGHT);
    let win = try!({
        let (x,y) = (vid::Positioned(100), vid::Positioned(100));
        vid::Window::new("Hello World", x, y, width, height,
                         [ vid::OpenGL, vid::Resizable, vid::Shown])
    });
    // This line needs to come after we create the window
    gl::load_with(vid::gl_get_proc_address);
    let _mainGLContext : ~vid::GLContext = try!(win.gl_create_context());

    let vertices : &[f32] = &[ 0.0,  0.5, // Vertex 1 (X, Y)
                               0.5, -0.5, // Vertex 2 (X, Y)
                              -0.5, -0.5, // Vertex 3 (X, Y)
                               ];

    let mut vao : gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
    }


    let mut vbo : gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo); // Generate 1 buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let vertices_size = vertices.len() * mem::size_of::<f32>();
        gl::BufferData(gl::ARRAY_BUFFER,
                       vertices_size as gl::types::GLsizeiptr,
                       vertices.as_ptr() as *libc::c_void,
                       gl::STATIC_DRAW);
    }
    let vertexSource = ~r#"
#version 150

in vec2 position;

void main()
{
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;
    let fragmentSource = ~r#"
#version 150

out vec4 outColor;

void main()
{
    outColor = vec4(1.0, 1.0, 1.0, 1.0);
}
"#;

    let vertexShader = gl::CreateShader(gl::VERTEX_SHADER);
    unsafe {
        let vertexSource = vertexSource.to_c_str();
        vertexSource.with_ref(|p| {
            let tmp = ~[p];
            gl::ShaderSource(vertexShader, 1, tmp.as_ptr() as **gl::types::GLchar, ptr::null());
            gl::CompileShader(vertexShader);
        });
        let mut status : gl::types::GLint = 0;
        gl::GetShaderiv(vertexShader, gl::COMPILE_STATUS, &mut status);
        if status != gl::TRUE as gl::types::GLint {
            let mut buffer = Vec::from_elem(512, 0 as libc::c_char);
            gl::GetShaderInfoLog(vertexShader, 512, ptr::mut_null(), buffer.as_mut_ptr());
            let buffer : Vec<char> = buffer.iter().map(|&c| c as u8 as char).collect();
            let end = buffer.iter().position(|&c|c == '\0').unwrap();
            fail!("vertexShader compilation failure {}", str::from_chars(buffer.slice_to(end)));
        }
    }

    let fragmentShader = gl::CreateShader(gl::FRAGMENT_SHADER);
    unsafe {
        let fragmentSource = fragmentSource.to_c_str();
        fragmentSource.with_ref(|p| {
            let tmp = ~[p];
            gl::ShaderSource(fragmentShader, 1, tmp.as_ptr() as **gl::types::GLchar, ptr::null());
            gl::CompileShader(fragmentShader);
        });
        let mut status : gl::types::GLint = 0;
        gl::GetShaderiv(fragmentShader, gl::COMPILE_STATUS, &mut status);
        if status != gl::TRUE as gl::types::GLint {
            let mut buffer = Vec::from_elem(512, 0 as libc::c_char);
            gl::GetShaderInfoLog(vertexShader, 512, ptr::mut_null(), buffer.as_mut_ptr());
            let buffer : Vec<char> = buffer.iter().map(|&c| c as u8 as char).collect();
            let end = buffer.iter().position(|&c|c == '\0').unwrap();
            fail!("fragmentShader compilation failure {}", str::from_chars(buffer.slice_to(end)));
        }
    }

    let shaderProgram = gl::CreateProgram();
    unsafe {
        gl::AttachShader(shaderProgram, vertexShader);
        gl::AttachShader(shaderProgram, fragmentShader);
        let name = "outColor".to_c_str();
        name.with_ref(|n| gl::BindFragDataLocation(shaderProgram, 0, n));
        gl::LinkProgram(shaderProgram);
        gl::UseProgram(shaderProgram);
    }

    let name = "position".to_c_str();
    let posAttrib = name.with_ref(|n| unsafe { gl::GetAttribLocation(shaderProgram, n) });
    unsafe {
        gl::VertexAttribPointer(posAttrib as gl::types::GLuint, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::EnableVertexAttribArray(posAttrib as gl::types::GLuint);
    }

    loop {
        let windowEvent = evt::poll_event();
        match windowEvent {
            QuitEvent(_) |
            KeyUpEvent(_, _, key::EscapeKey, _, _) => break,
            _ => {}
        }

        // Clear the screen to black
        gl::ClearColor(0.0f32, 0.0f32, 0.0f32, 1.0f32);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        // Draw a triangle from the 3 vertices
        gl::DrawArrays(gl::TRIANGLES, 0, 3);

        win.gl_swap_window();
    }

    Ok(())
}

fn hello() -> Result<(), ~str> {
    static SCREEN_WIDTH:i32 = 640;
    static SCREEN_HEIGHT:i32 = 480;
    // http://www.open.gl/context/
    try!(sdl::init([sdl::InitEverything]));

    let (width, height) = (SCREEN_WIDTH, SCREEN_HEIGHT);
    let win = try!({
        let (x,y) = (vid::Positioned(100), vid::Positioned(100));
        vid::Window::new("Hello World", x, y, width, height,
                         [ vid::OpenGL, vid::Resizable, vid::Shown])
    });

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
    // try!(ren.copy(tex, None, None))
    try!(renderTexture(tex, ren, 0, 0));
    ren.present();
    sdl::timer::delay(2000);
    Ok(())
}

fn default() -> Result<(), ~str> {
    try!(sdl::init([sdl::InitVideo, sdl::InitTimer]));
    let screen = vid::Window::new("rust-sdl demo - video",
        vid::PosUndefined, vid::PosUndefined, 800, 600, [])
        .ok().expect("Failed to create Window");
    let screen = rend::Renderer::from_window(screen, rend::DriverAuto,
                                             [rend::Software] // [rend::Accelerated]
                                             )
        .ok().expect("Failed to create Renderer from Window");
    let purple = pix::RGB(128, 0, 128);
    let black = pix::RGB(0, 0, 0);
    let square = |x:int, y:int, width, color| {
        let w = width;
        let r = Rect { x: x as i32, y: y as i32, w: w, h: w };
        (screen.set_draw_color(color) &&
         screen.fill_rect(&r))
            || fail!("error on fill_rect attempt");
    };

    'main : loop {
        'events : loop {
            match evt::poll_event() {
                QuitEvent(_) | KeyUpEvent(_, _, key::EscapeKey, _, _)
                    => break 'main,
                NoEvent
                    => break 'events,
                MouseMotionEvent(_timestamp, ref _window, _which,
                                 ref state, x, y, _xrel, _yrel) if state.len() > 0
                    => square(x, y, 30, purple),
                MouseMotionEvent(_timestamp, ref _window, _which,
                                 ref state, x, y, _xrel, _yrel) if state.len() == 0
                    => square(x, y, 3, black),
                evt::FingerMotionEvent(..) |
                evt::FingerDownEvent(..) |
                evt::FingerUpEvent(..) => {}
                e
                    => println!("e: {:?}", e),
            }
        }
        screen.present();
    }
    println!("Hello World");
    Ok(())
}
