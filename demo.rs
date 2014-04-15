#![feature(globs)]

#![allow(unused_imports)] // don't warn me about this right now

extern crate native;
extern crate rand;

extern crate sdl = "sdl2";

extern crate opengles;

extern crate gl;

extern crate time;

use std::cast;
use std::mem;
use std::os;
use std::ptr;
use std::str;
use std::vec;

use gl::types::*;

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
        dispatch(args[0], args[1], args.slice_from(2));
    } else {
        default().unwrap();
    }
}

fn dispatch(driver: &str, variant: &str, _args: &[~str]) {
    let _invoker = format!("{} {}", driver, variant);
    match variant {
/*
        "testsprite"
            => tests::testsprite::main(invoker, args),
        "soe"
            => tests::soe::main(invoker, args),
*/
        "hello"
            => hello().unwrap(),
        "gl"
            => match gl() {
                Ok(_) => {}
                Err(s) => { fail!("gl failed: {}", s); }
            },
        _otherwise
            => default().unwrap(),
    }
}

mod tests {
//    pub mod testsprite;
//    pub mod soe;
}

fn gl() -> Result<(), ~str> {

    let (width, height) = (800, 600);

    try!(sdl::init([sdl::InitVideo]));

    match vid::gl_load_library(None) {
        Ok(()) => {},
        Err(s) => {
            println!("gl_load_library() failed: {}", s);
            return Err(s)
        }
    }

    vid::gl_set_attribute(vid::GLContextMajorVersion, 3);
    vid::gl_set_attribute(vid::GLContextMinorVersion, 2);
    vid::gl_set_attribute(vid::GLContextProfileMask,
                          vid::ll::SDL_GL_CONTEXT_PROFILE_CORE as int);


    // THis is some code I put in when I was desparately trying to get
    // things to work.  I am stil lnot clear on how far it actually
    // got me; I'm surprised I left it in as long as I did, when it
    // seems to not actually matter at all in the end, compared to
    // simply not creating a Renderer (see below).a
    /*
    {
        // vid::ll::GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT
        let flags = vid::gl_get_attribute(vid::GLContextFlags);
        let flags = match flags {
            Ok(f) => f,
            Err(s) => return Err(s),
        };
        vid::gl_set_attribute(vid::GLContextFlags, 1 | flags);
    }
     */

    let win = try!(
        vid::Window::new("Hello World", 100, 100, width, height,
                         [vid::Shown]));

    let _ctxt = try!(win.gl_create_context());

    // UGH: if you create a window without the vid::OpenGL flag above,
    // then doing Renderer::from_window resets the PROFILE_MASK state
    // to 0 (and in general seems to cause a re-initialiation of
    // OpenGL in general).  The best way to avoid this, AFAICT, is to
    // just not ever create a Renderer from a window at all.  SDL2
    // itself could probably do a better job of helping the user catch
    // this sort of bug, but failing that, maybe rust-sdl2 could
    // provide some way to prevent this sort of bug.
    /*
    {
        println!("before rend::Renderer::from_window");
        let ren = try!(rend::Renderer::from_window(
            win, rend::DriverAuto, [rend::Accelerated, rend::TargetTexture]));
        println!("after rend::Renderer::from_window");
    }
     */

    // vid::gl_set_swap_interval(1) || fail!("oops");

    gl::load_with(vid::gl_get_proc_address);


// Vertex data
static VERTEX_DATA: [GLfloat, ..28] = [
    // X     Y    R    G    B  Tecoords
    -0.5,  0.5, 1.0, 0.0, 0.0, 0.0, 0.0, // Top-left
     0.5,  0.5, 0.0, 1.0, 0.0, 1.0, 0.0, // Top-right
     0.5, -0.5, 0.0, 0.0, 1.0, 1.0, 1.0, // Bottom-right
    -0.5, -0.5, 1.0, 1.0, 1.0, 0.0, 1.0  // Bottom-left
];

// Shader sources
static VS_SRC: &'static str =
   "#version 150 core
    in vec2 position;
    in vec3 color;
    in vec2 texcoord;

    out vec3 v2f_color;
    out vec2 v2f_texcoord;
    void main() {
       v2f_color = color;
       v2f_texcoord = texcoord;
       gl_Position = vec4(position, 0.0, 1.0);
    }";

static FS_SRC: &'static str =
   "#version 150 core
    in vec3 v2f_color;
    in vec2 v2f_texcoord;

    out vec4 out_color;

    uniform sampler2D tex;

    void main() {
       out_color = texture(tex, v2f_texcoord) * vec4(v2f_color, 1.0);
    }";


    // Create GLSL shaders
    let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let program = link_program(vs, fs);

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       cast::transmute(&VERTEX_DATA[0]),
                       gl::STATIC_DRAW);

        // Use shader program
        gl::UseProgram(program);
        "out_color".with_c_str(|ptr| gl::BindFragDataLocation(program, 0, ptr));

        // Specify the layout of the vertex data
        let pos_attr = "position".with_c_str(|ptr| gl::GetAttribLocation(program, ptr));
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(pos_attr as GLuint, 2, gl::FLOAT,
                                gl::FALSE as GLboolean,
                                7 * mem::size_of::<GLfloat>() as GLsizei,
                                ptr::null());

        let col_attr = "color".with_c_str(|ptr| gl::GetAttribLocation(program, ptr));
        gl::EnableVertexAttribArray(col_attr as GLuint);
        gl::VertexAttribPointer(col_attr as GLuint, 3, gl::FLOAT,
                                gl::FALSE as GLboolean,
                                7 * mem::size_of::<GLfloat>() as GLsizei,
                                cast::transmute(2 * mem::size_of::<GLfloat>() as GLsizeiptr));

        let tex_attr = "texcoord".with_c_str(|ptr| gl::GetAttribLocation(program, ptr));
        gl::EnableVertexAttribArray(tex_attr as GLuint);
        gl::VertexAttribPointer(tex_attr as GLuint, 2, gl::FLOAT,
                                gl::FALSE as GLboolean,
                                7 * mem::size_of::<GLfloat>() as GLsizei,
                                cast::transmute(5 * mem::size_of::<GLfloat>() as GLsizeiptr));
    }

    let uni_color = unsafe {
        "triangle_color".with_c_str(|ptr| gl::GetUniformLocation(program, ptr))
    };

    let mut tex = 0;
    unsafe { gl::GenTextures(1, &mut tex); }

    let image = try!(surf::Surface::from_bmp(&Path::new("sample.bmp")));
    let (width, height) = (image.get_width(), image.get_height());
    image.with_lock(|pixels| {
        unsafe {
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32,
                           width as i32, height as i32,
                           0,
                           // gl::RGBA,
                           gl::BGRA,
                           gl::UNSIGNED_BYTE,
                           pixels.as_ptr() as *GLvoid);
        }
    });

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);


    let elements : Vec<GLuint> = vec!(0, 1, 2, 2, 3, 0);
    let mut ebo = 0;
    unsafe {
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       (elements.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
                       elements.as_ptr() as *GLvoid,
                       gl::STATIC_DRAW);
    }

    let loop_start_time = time::precise_time_s();

    loop {
        match evt::poll_event() {
            evt::QuitEvent(_) | evt::KeyUpEvent(_, _, key::EscapeKey, _, _)
                => break,
            _ => {
                let time = time::precise_time_s();
                if (time - loop_start_time) > 3.0 {
                    break
                }
            }
        }

        // Use a uniform red
        // gl::Uniform3f(uni_color, 1.0, 0.0, 0.0);

        let time = time::precise_time_s();
        gl::Uniform3f(uni_color, ((time+4.0).sin() as f32 + 1.0)/2.0, 0.0, 0.0);

        gl::ClearColor(0.3, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        // Draw a rectangle from the 6 vertices
        // gl::DrawArrays(gl::TRIANGLES, 0, 6);
        unsafe { gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null()); }

        win.gl_swap_window();
    }

    // sdl::timer::delay(2000);

    return Ok(());

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader = gl::CreateShader(ty);
    unsafe {
        // Attempt to compile the shader
        src.with_c_str(|ptr| gl::ShaderSource(shader, 1, &ptr, ptr::null()));
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::from_elem(len as uint - 1, 0u8);     // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(shader, len, ptr::mut_null(), buf.as_mut_ptr() as *mut GLchar);
            fail!("{}", str::from_utf8_owned(buf.move_iter().collect()));
        }
    }
    shader
}

    fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        unsafe {
            // Get the link status
            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::from_elem(len as uint - 1, 0u8);     // subtract 1 to skip the trailing null character
                gl::GetProgramInfoLog(program, len, ptr::mut_null(), buf.as_mut_ptr() as *mut GLchar);
                fail!(str::from_utf8_owned(buf.move_iter().collect()));
            }
        }
        program
    }

}



#[cfg(solely_opengles)]
fn gl() -> Result<(), ~str> {
    // Attempting to adapt
    // http://stackoverflow.com/questions/21621054/
    //  why-wont-this-simple-opengl-es-2-0-sdl-2-program-
    //    let-me-change-my-point-sprite

    try!(sdl::init([sdl::InitVideo]));
    let (width, height) = (800, 600);

    match vid::gl_load_library(None) {
        Ok(()) => {},
        Err(s) => {
            println!("gl_load_library() failed: {}", s);
            return Err(s)
        }
    }

    let win = try!(
        vid::Window::new("Hello World", 100, 100, width, height, [vid::Shown]));

    let ctxt = try!(win.gl_create_context());
    let ren = try!(rend::Renderer::from_window(
        win, rend::DriverAuto, [rend::Accelerated, rend::TargetTexture]));

    let vertex =
        r"#version 100
          precision mediump float;
           void main()
            {
               gl_Position = vec4(0.0, 0.0, 0.0, 1.0);
               gl_PointSize = 128.0;
            }";
    let fragment =
        r"#version 100
        precision mediump float;
        void main()
        {
           gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
        }";

    unsafe {
        let programObject = gl::glCreateProgram();
        loadShader(programObject, gl::VERTEX_SHADER, vertex);
        loadShader(programObject, gl::FRAGMENT_SHADER, fragment);
        gl::glLinkProgram(programObject);
        gl::glUseProgram(programObject);
    }

    return Ok(());

    fn loadShader(program: gl::GLuint, type_: gl::GLenum, shaderSrc: &str) {
        unsafe {
            let shader = gl::glCreateShader(type_);
            shaderSrc.with_c_str(|src| {
                gl::glShaderSource(shader, 1, &src, std::ptr::null());
            });
            gl::glCompileShader(shader);
            gl::glAttachShader(program, shader);
        }
    }
}

#[cfg(exavolt_gist)]
fn gl() -> Result<(), ~str> {
    // Adapting from https://gist.github.com/exavolt/2360410

    try!(sdl::init([sdl::InitVideo]));
    let (width, height) = (800, 600);

    match vid::gl_load_library(None) {
        Ok(()) => {},
        Err(s) => {
            println!("gl_load_library() failed: {}", s);
            return Err(s)
        }
    }

    let win = try!(
        vid::Window::new("Hello World", 100, 100, width, height, [vid::Shown]));

    let ctxt = try!(win.gl_create_context());
    let ren = try!(rend::Renderer::from_window(
        win, rend::DriverAuto, [rend::Accelerated, rend::TargetTexture]));

    initGL();
    setViewport(width, height);
    render();

    sdl::timer::delay(2000);
    return Ok(());

    fn initGL() {
        // gl::ShadeModel(gl::SMOOTH); // WTF

        gl::load_with(vid::gl_get_proc_address);

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            // gl::glClearDepthf(1.0); // Unsupported on mac?  Grr.
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
            // gl2::glHint(gl2::PERSPECTIVE_CORRECTION_HINT, GL_NICEST); // WTF
        }
    }

    fn setViewport(width: int, height: int) {
        let height = if height == 0 { 1 } else { height };
        let ratio = width as GLfloat / height as GLfloat;
        unsafe {
            gl::Viewport(0,0, width as GLsizei, height as GLsizei);
            // gl::MatrixMode(gl::PROJECTION);
            // gl::LoadIdentity
            
        }
        
        unimplemented!()
    }

    fn render() {
        unimplemented!()
    }
}

fn hello() -> Result<(), ~str> {
    static SCREEN_WIDTH:int = 640;
    static SCREEN_HEIGHT:int = 480;
    try!(sdl::init([sdl::InitEverything]));
    let win = try!(
        vid::Window::new("Hello World", 100, 100, SCREEN_WIDTH, SCREEN_HEIGHT, [vid::Shown]));
    let ren = try!(rend::Renderer::from_window(
        win, rend::DriverAuto, [rend::Accelerated, rend::PresentVSync]));

    fn loadTexture(ren: &rend::Renderer, path: Path) -> Result<~rend::Texture, ~str> {
        let bmp = try!(surf::Surface::from_bmp(&path));
        ren.create_texture_from_surface(bmp)
    }

    let background = try!(loadTexture(ren, Path::new("background.bmp")));
    let image = try!(loadTexture(ren, Path::new("image.bmp")));


    fn renderTexture(tex: &rend::Texture, ren: &rend::Renderer, x: i32, y: i32) -> Result<(), ~str> {
        let q = try!(tex.query());
        let dst = rect::Rect::new(x, y, q.width, q.height);
        ren.copy(tex, None, Some(dst))
    }
    ren.clear();
    let ~rend::TextureQuery{ width: bW, height: bH, .. } =
        try!(background.query());
    try!(renderTexture(background, ren,  0, 0));
    try!(renderTexture(background, ren, bW, 0));
    try!(renderTexture(background, ren,  0, bH));
    try!(renderTexture(background, ren, bW, bH));

    let ~rend::TextureQuery{ width: iW, height: iH, .. } =
        try!(image.query());
    let x = SCREEN_WIDTH as i32 / 2 - iW / 2;
    let y = SCREEN_HEIGHT as i32 / 2 - iH / 2;
    try!(renderTexture(image, ren, x, y));

    // let tex = try!(loadTexture(ren, Path::new("hello.bmp")));
    // try!(ren.copy(tex, None, None))
    ren.present();
    sdl::timer::delay(2000);
    Ok(())
}

fn default() -> Result<(), ~str> {
    try!(sdl::init([sdl::InitVideo, sdl::InitTimer]));
    let (width, height) = (800, 600);
    let screen = vid::Window::new("rust-sdl demo - video",
        vid::PosUndefined, vid::PosUndefined, width, height, [])
        .ok().expect("Failed to create Window");
    let screen = rend::Renderer::from_window(screen, rend::DriverAuto,
                                             [rend::Accelerated]
                                             )
        .ok().expect("Failed to create Renderer from Window");
    println!("render_target_supported: {}", screen.render_target_supported());
    let _texture = screen.create_texture(pix::RGBA8888,
                                        rend::AccessTarget,
                                        width,
                                        height)
        .ok().expect("Failed to create Texture from Renderer");

    let purple = pix::RGB(128, 0, 128);
    let black = pix::RGB(0, 0, 0);
    if screen.render_target_supported() {
        // screen.set_render_target(Some(&*_texture));
    }
    screen.set_draw_color(black);
    screen.fill_rect(&Rect { x: 0, y: 0, w: width as i32, h: height as i32 });
    let square = |x:int, y:int, width, color| {
        let w = width;
        let r = Rect { x: x as i32, y: y as i32, w: w, h: w };
        (screen.set_draw_color(color) &&
         screen.fill_rect(&r))
            || fail!("error on fill_rect attempt");
        screen.present();
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
    }
    println!("Hello World");
    Ok(())
}
