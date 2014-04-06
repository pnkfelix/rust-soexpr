#![allow(unused_imports)] // don't warn me about this right now

extern crate native;
extern crate rand;
extern crate time;

extern crate sdl = "sdl2";
extern crate gl;

use std::cast;
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

use gl::types::{GLchar, GLint, GLuint, GLsizei, GLsizeiptr};

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

fn dispatch(driver: &str, variant: &str, args: &[~str]) -> Result<(), ~str> {
    let _invoker = ||format!("{} {}", driver, variant);
    match (variant, args.get(0)) {
/*
        "testsprite"
            => tests::testsprite::main(_invoker(), _args),
        "soe"
            => tests::soe::main(_invoker(), _args),
*/
        ("open_gl", _) |
        ("open_gl_drawing", _)
            => open_gl_drawing(),
        ("open_gl_textures", Some(arg)) if arg.as_slice() == "colored"
            => open_gl_textures(ColoredKitten),
        ("open_gl_textures", Some(arg)) if arg.as_slice() == "mix"
            => open_gl_textures(KittenPuppy),
        ("open_gl_textures", _)
            => open_gl_textures(ColoredKitten),
        ("hello", _)                    => hello(),
        _otherwise                      => default(),
    }
}

mod tests {
//    pub mod testsprite;
//    pub mod soe;
}

// Tried:
//   http://useful-linux-tips.blogspot.fr/2013/11/complete-minimal-sdl2-opengl-animation.html
// but it uses glFrustum, which apparently has been deprecated 

fn open_gl_init() -> Result<(~vid::Window,~vid::GLContext), ~str> {
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

    let context : ~vid::GLContext = try!(win.gl_create_context());

    Ok((win, context))
}

struct VertexArrayObj { vao: GLuint }
impl VertexArrayObj {
    fn new() -> VertexArrayObj {
        let mut vao : GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
        }
        VertexArrayObj { vao: vao }
    }
}

struct VertexBufferObj { vbo: GLuint }
impl VertexBufferObj {
    fn new() -> VertexBufferObj {
        let mut vbo : GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo); // Generate 1 buffer
        }
        VertexBufferObj { vbo: vbo }
    }

    fn bind_array(&self, vertices: &[f32]) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            let vertices_size = vertices.len() * mem::size_of::<f32>();
            gl::BufferData(gl::ARRAY_BUFFER,
                           vertices_size as GLsizeiptr,
                           vertices.as_ptr() as *libc::c_void,
                           gl::STATIC_DRAW);
        }
    }
}

struct ElementsBufferObj { ebo: GLuint }
impl ElementsBufferObj {
    fn new() -> ElementsBufferObj {
        let mut ebo : GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut ebo); }
        ElementsBufferObj { ebo: ebo }
    }

    fn bind_array(&self, elements: &[GLuint]) {
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        unsafe {
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                           (mem::size_of::<GLuint>() * elements.len()) as i64,
                           elements.as_ptr() as *libc::c_void,
                           gl::STATIC_DRAW);
        }
    }
}

struct VertexShader { vertexShader: GLuint }
impl VertexShader {
    fn gluint(&self) -> GLuint { self.vertexShader }
    fn new() -> VertexShader {
        VertexShader { vertexShader: gl::CreateShader(gl::VERTEX_SHADER) }
    }
    fn source(&self, vertexSource: &str) {
        unsafe {
            let vertexSource = vertexSource.to_c_str();
            vertexSource.with_ref(|p| {
                let tmp = ~[p];
                gl::ShaderSource(self.vertexShader, 1, tmp.as_ptr() as **GLchar, ptr::null());
                gl::CompileShader(self.vertexShader);
            });
            let mut status : GLint = 0;
            gl::GetShaderiv(self.vertexShader, gl::COMPILE_STATUS, &mut status);
            if status != gl::TRUE as GLint {
                let mut buffer = Vec::from_elem(512, 0 as libc::c_char);
                gl::GetShaderInfoLog(self.vertexShader, 512, ptr::mut_null(), buffer.as_mut_ptr());
                let buffer : Vec<char> = buffer.iter().map(|&c| c as u8 as char).collect();
                let end = buffer.iter().position(|&c|c == '\0').unwrap();
                fail!("vertexShader compilation failure {}", str::from_chars(buffer.slice_to(end)));
            }
        }
    }
}

fn open_gl_drawing() -> Result<(), ~str> {
    let (win, _context) = try!(open_gl_init());

    // http://www.open.gl/context
    // and http://open.gl/drawing

    let vertices : &[f32] = &[-0.5,  0.5, 1.0, 0.0, 0.0, // tl Vertex 1 (X, Y, ..Red)
                               0.5,  0.5, 0.0, 1.0, 0.0, // tr Vertex 2 (X, Y, ..Green)
                               0.5, -0.5, 0.0, 0.0, 1.0, // br Vertex 3 (X, Y, ..Blue)
                              -0.5, -0.5, 1.0, 1.0, 1.0, // bl Vertex 4 (X, Y, .. White)
                               ];

    let _vao = VertexArrayObj::new();

    let vbo = VertexBufferObj::new();
    vbo.bind_array(vertices);

    let vertexSource = ~r#"
#version 150

in vec2 position;
in vec3 color;

out vec3 Color;

void main()
{
    Color = color;
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;
    let fragmentSource = ~r#"
#version 150

in vec3 Color;

out vec4 outColor;

void main()
{
    outColor = vec4(Color, 1.0);
}
"#;

    let vertexShader = VertexShader::new();
    vertexShader.source(vertexSource);

    let fragmentShader = gl::CreateShader(gl::FRAGMENT_SHADER);
    unsafe {
        let fragmentSource = fragmentSource.to_c_str();
        fragmentSource.with_ref(|p| {
            let tmp = ~[p];
            gl::ShaderSource(fragmentShader, 1, tmp.as_ptr() as **GLchar, ptr::null());
            gl::CompileShader(fragmentShader);
        });
        let mut status : GLint = 0;
        gl::GetShaderiv(fragmentShader, gl::COMPILE_STATUS, &mut status);
        if status != gl::TRUE as GLint {
            let mut buffer = Vec::from_elem(512, 0 as libc::c_char);
            gl::GetShaderInfoLog(fragmentShader, 512, ptr::mut_null(), buffer.as_mut_ptr());
            let buffer : Vec<char> = buffer.iter().map(|&c| c as u8 as char).collect();
            let end = buffer.iter().position(|&c|c == '\0').unwrap();
            fail!("fragmentShader compilation failure {}", str::from_chars(buffer.slice_to(end)));
        }
    }

    let shaderProgram = gl::CreateProgram();
    unsafe {
        gl::AttachShader(shaderProgram, vertexShader.gluint());
        gl::AttachShader(shaderProgram, fragmentShader);
        let name = "outColor".to_c_str();
        name.with_ref(|n| gl::BindFragDataLocation(shaderProgram, 0, n));
        gl::LinkProgram(shaderProgram);
        gl::UseProgram(shaderProgram);
    }

    let name = "position".to_c_str();
    let posAttrib = name.with_ref(|n| unsafe { gl::GetAttribLocation(shaderProgram, n) });
    gl::EnableVertexAttribArray(posAttrib as GLuint);
    unsafe {
        gl::VertexAttribPointer(
            posAttrib as GLuint, 2, gl::FLOAT, gl::FALSE,
            5*mem::size_of::<f32>() as GLsizei, ptr::null());
    }
    let name = "color".to_c_str();
    let colAttrib = name.with_ref(|n| unsafe { gl::GetAttribLocation(shaderProgram, n) });
    gl::EnableVertexAttribArray(colAttrib as GLuint);
    unsafe {
        gl::VertexAttribPointer(colAttrib as GLuint,
                                3, gl::FLOAT, gl::FALSE,
                                5*mem::size_of::<f32>() as GLsizei,
                                cast::transmute::<uint, *libc::c_void>(2*mem::size_of::<f32>()));
    }
    let uniColor = {
        let name = "triangleColor".to_c_str();
        name.with_ref(|n| unsafe { gl::GetUniformLocation(shaderProgram, n) })
    };


    let elements : ~[GLuint] = ~[0, 1, 2, 2, 3, 0];
    let ebo = ElementsBufferObj::new();
    ebo.bind_array(elements);

    loop {
        let windowEvent = evt::poll_event();
        match windowEvent {
            QuitEvent(_) |
            KeyUpEvent(_, _, key::EscapeKey, _, _) => break,
            _ => {}
        }

        let time_ = time::precise_time_s() as f32;
        gl::Uniform3f(uniColor, ((time_*4.0).sin() + 1.0)/2.0, 0.0, 0.0);

        // Clear the screen to black
        gl::ClearColor(0.0f32, 0.0f32, 0.0f32, 1.0f32);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        // Draw a rectangle from the two triangles from 4 distinct vertices.
        unsafe { gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null()); }

        win.gl_swap_window();
    }

    Ok(())
}

enum GlTexturesVariant {
    ColoredKitten,
    KittenPuppy,
}

fn open_gl_textures(variant: GlTexturesVariant) -> Result<(), ~str> {
    let (win, _context) = try!(open_gl_init());

    let vertexSource = ~r#"
#version 150 core

in vec2 position;
in vec3 color;
in vec2 texcoord;

out vec3 Color;
out vec2 Texcoord;

void main() {
    Color = color;
    Texcoord = texcoord;
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;
    let fragmentSource = format!(
        "{}{}{}",
        r#"
#version 150 core

in vec3 Color;
in vec2 Texcoord;

out vec4 outColor;

uniform sampler2D texKitten;
uniform sampler2D texPuppy;

void main()
{
    vec4 colKitten = texture(texKitten, Texcoord);
    vec4 colPuppy = texture(texPuppy, Texcoord);
    "#,
        match variant {
            ColoredKitten => "outColor = colKitten * vec4(Color, 1.0);",
            KittenPuppy   => "outColor = mix(colKitten, colPuppy, 0.5);"
        },
        r#"
}
"#);

    let _vao = VertexArrayObj::new();
    let vbo = VertexBufferObj::new();

    let vertices : &[f32] = &[
        // Position       Color     Texcoords
         -0.5,  0.5, 1.0, 0.0, 0.0, 0.0, 0.0, // Top-left
          0.5,  0.5, 0.0, 1.0, 0.0, 1.0, 0.0, // Top-right
          0.5, -0.5, 0.0, 0.0, 1.0, 1.0, 1.0, // Bot-right
         -0.5, -0.5, 1.0, 1.0, 1.0, 0.0, 1.0, // Bot-left
    ];

    vbo.bind_array(vertices);

    let elements : ~[GLuint] = ~[0, 1, 2,
                                 2, 3, 0];
    let ebo = ElementsBufferObj::new();
    ebo.bind_array(elements);

    // Create and compile the vertex shader
    let vertexShader = VertexShader::new();
    vertexShader.source(vertexSource);

    // Create and compile the fragment shader
    let fragmentShader = gl::CreateShader(gl::FRAGMENT_SHADER);
    unsafe {
        let fragmentSource = fragmentSource.to_c_str();
        fragmentSource.with_ref(|p| {
            let tmp = ~[p];
            gl::ShaderSource(fragmentShader, 1, tmp.as_ptr() as **GLchar, ptr::null());
            gl::CompileShader(fragmentShader);
        });
        let mut status : GLint = 0;
        gl::GetShaderiv(fragmentShader, gl::COMPILE_STATUS, &mut status);
        if status != gl::TRUE as GLint {
            let mut buffer = Vec::from_elem(512, 0 as libc::c_char);
            gl::GetShaderInfoLog(fragmentShader, 512, ptr::mut_null(), buffer.as_mut_ptr());
            let buffer : Vec<char> = buffer.iter().map(|&c| c as u8 as char).collect();
            let end = buffer.iter().position(|&c|c == '\0').unwrap();
            fail!("fragmentShader compilation failure {}", str::from_chars(buffer.slice_to(end)));
        }
    }
    // Link the vertex and fragment shader into a shader program
    let shaderProgram = gl::CreateProgram();
    unsafe {
        gl::AttachShader(shaderProgram, vertexShader.gluint());
        gl::AttachShader(shaderProgram, fragmentShader);
        let name = "outColor".to_c_str();
        name.with_ref(|n| gl::BindFragDataLocation(shaderProgram, 0, n));
        gl::LinkProgram(shaderProgram);
        gl::UseProgram(shaderProgram);
    }

    // Specify the layout of the vertex data
    let name = "position".to_c_str();
    let posAttrib = name.with_ref(|n| unsafe { gl::GetAttribLocation(shaderProgram, n) });
    gl::EnableVertexAttribArray(posAttrib as GLuint);
    unsafe {
        gl::VertexAttribPointer(posAttrib as GLuint,
                                2, gl::FLOAT, gl::FALSE,
                                7*mem::size_of::<f32>() as GLsizei,
                                ptr::null());
    }

    let name = "color".to_c_str();
    let colAttrib = name.with_ref(|n| unsafe { gl::GetAttribLocation(shaderProgram, n) });
    gl::EnableVertexAttribArray(colAttrib as GLuint);
    unsafe {
        gl::VertexAttribPointer(colAttrib as GLuint,
                                3, gl::FLOAT, gl::FALSE,
                                7*mem::size_of::<f32>() as GLsizei,
                                cast::transmute::<uint, *libc::c_void>(2*mem::size_of::<f32>()));
    }

    let name = "texcoord".to_c_str();
    let texAttrib = name.with_ref(|n| unsafe { gl::GetAttribLocation(shaderProgram, n) });
    gl::EnableVertexAttribArray(texAttrib as GLuint);
    unsafe {
        gl::VertexAttribPointer(texAttrib as GLuint,
                                2, gl::FLOAT, gl::FALSE,
                                7*mem::size_of::<f32>() as GLsizei,
                                cast::transmute::<uint, *libc::c_void>(5*mem::size_of::<f32>()));
    }

    // Load textures
    let mut textures : ~[GLuint] = ~[ 0, 0 ];
    unsafe { gl::GenTextures(2, textures.as_mut_ptr()); }

    gl::ActiveTexture(gl::TEXTURE0);
    gl::BindTexture(gl::TEXTURE_2D, textures[0]);
    {
        let file = Path::new("sample.bmp");
        let image = try!(surf::Surface::from_bmp(&file));
        image.with_lock(|pixels| {
            unsafe {
                gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as GLint,
                               image.get_width() as GLsizei, image.get_height() as GLsizei,
                               0, gl::RGBA, gl::UNSIGNED_BYTE, pixels.as_ptr() as *libc::c_void);
            }
        });
    }
    let name = "texKitten".to_c_str();
    name.with_ref(|n| gl::Uniform1i(unsafe { gl::GetUniformLocation(shaderProgram, n) }, 0));

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

    gl::ActiveTexture(gl::TEXTURE1);
    gl::BindTexture(gl::TEXTURE_2D, textures[1]);
    {
        let file = Path::new("sample2.bmp");
        let image = try!(surf::Surface::from_bmp(&file));
        image.with_lock(|pixels| {
            unsafe {
                gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as GLint,
                               image.get_width() as GLsizei, image.get_height() as GLsizei,
                               0, gl::RGBA, gl::UNSIGNED_BYTE, pixels.as_ptr() as *libc::c_void);
            }
        });
    }
    let name = "texPuppy".to_c_str();
    name.with_ref(|n|gl::Uniform1i(unsafe { gl::GetUniformLocation(shaderProgram, n) }, 1));

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

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

        // Draw a rectangle from the two triangles using 6 indices
        unsafe { gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null()); }

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
