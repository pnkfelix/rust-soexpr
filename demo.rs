#![feature(globs)]

#![allow(unused_imports)] // don't warn me about this right now

extern crate native;
extern crate rand;

extern crate sdl = "sdl2";

extern crate opengles;

extern crate cgmath;

extern crate gl;

extern crate time;

use std::cast;
use std::c_str;
use std::default::Default;
use std::mem;
use std::os;
use std::ptr;
use std::slice;
use std::slice::Vector;
use std::str;
use std::vec;
use std::num::{Zero,One,Float};

use gl::types::*;

use ang = cgmath::angle;
use cgmath::angle::{ToRad,Angle};
use mat = cgmath::matrix;
use cgmath::matrix::{ToMatrix4, Matrix};
use vec = cgmath::vector;
use cgmath::vector::{Vector,EuclideanVector};
use pt = cgmath::point;
use cgmath::partial_ord::PartOrdFloat;
use cgmath::approx::ApproxEq;

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

pub mod glsl {
    use gl::types::*;
    use gl;

    use mat = cgmath::matrix;

    use std::cast;
    use std::mem;
    use std::str;

    pub struct VertexShader {
        name: GLuint,
    }

    pub struct FragmentShader {
        name: GLuint,
    }

    pub struct Program {
        name: GLuint,
    }

    pub struct AttribLocation {
        pub name: GLint,
    }

    pub struct UniformLocation {
        pub name: GLint,
    }


    impl AttribLocation {
        pub fn enable_current_vertex_attrib_array(&self) {
            gl::EnableVertexAttribArray(self.name as GLuint);
        }
        pub unsafe fn vertex_attrib_pointer(&self, size: GLint, type_: GLenum,
                                     normalized: GLboolean, stride: GLsizei,
                                     pointer: *GLvoid) {
            gl::VertexAttribPointer(
                self.name as GLuint, size, type_, normalized, stride, pointer);
        }
    }

    pub trait UniformArg { fn set_at_location(&self, location: GLint); }

    impl UniformArg for GLfloat {
        fn set_at_location(&self, location: GLint) {
            let &v0 = self; gl::Uniform1f(location, v0);
        }
    }
    impl UniformArg for (GLfloat, GLfloat) {
        fn set_at_location(&self, location: GLint) {
            let &(v0, v1) = self; gl::Uniform2f(location, v0, v1);
        }
    }
    impl UniformArg for (GLfloat, GLfloat, GLfloat) {
        fn set_at_location(&self, location: GLint) {
            let &(v0, v1, v2) = self; gl::Uniform3f(location, v0, v1, v2);
        }
    }
    impl UniformArg for (GLfloat, GLfloat, GLfloat, GLfloat) {
        fn set_at_location(&self, location: GLint) {
            let &(v0, v1, v2, v3) = self; gl::Uniform4f(location, v0, v1, v2, v3);
        }
    }

    impl UniformArg for GLint {
        fn set_at_location(&self, location: GLint) {
            let &v0 = self; gl::Uniform1i(location, v0);
        }
    }
    impl UniformArg for (GLint, GLint) {
        fn set_at_location(&self, location: GLint) {
            let &(v0, v1) = self; gl::Uniform2i(location, v0, v1);
        }
    }
    impl UniformArg for (GLint, GLint, GLint) {
        fn set_at_location(&self, location: GLint) {
            let &(v0, v1, v2) = self; gl::Uniform3i(location, v0, v1, v2);
        }
    }
    impl UniformArg for (GLint, GLint, GLint, GLint) {
        fn set_at_location(&self, location: GLint) {
            let &(v0, v1, v2, v3) = self; gl::Uniform4i(location, v0, v1, v2, v3);
        }
    }

    impl UniformArg for GLuint {
        fn set_at_location(&self, location: GLint) {
            let &v0 = self; gl::Uniform1ui(location, v0);
        }
    }
    impl UniformArg for (GLuint, GLuint) {
        fn set_at_location(&self, location: GLint) {
            let &(v0, v1) = self; gl::Uniform2ui(location, v0, v1);
        }
    }
    impl UniformArg for (GLuint, GLuint, GLuint) {
        fn set_at_location(&self, location: GLint) {
            let &(v0, v1, v2) = self; gl::Uniform3ui(location, v0, v1, v2);
        }
    }
    impl UniformArg for (GLuint, GLuint, GLuint, GLuint) {
        fn set_at_location(&self, location: GLint) {
            let &(v0, v1, v2, v3) = self; gl::Uniform4ui(location, v0, v1, v2, v3);
        }
    }

    impl UniformArg for mat::Matrix4<f32> {
        fn set_at_location(&self, location: GLint) {
            unsafe {
                gl::UniformMatrix4fv(location, 1, gl::FALSE, cast::transmute(self));
            }
        }
    }

    impl<'a> UniformArg for &'a mat::Matrix4<f32> {
        fn set_at_location(&self, location: GLint) {
            unsafe {
                gl::UniformMatrix4fv(location, 1, gl::FALSE, cast::transmute(*self));
            }
        }
    }

    impl UniformLocation {
        pub fn uniform<U:UniformArg>(&self, arg: U) {
            arg.set_at_location(self.name);
        }
    }

    impl Program {
        pub fn link(vs: &VertexShader, fs: &FragmentShader) -> Program {
            let name = super::link_program(vs.name, fs.name);
            Program { name: name }
        }

        pub fn use_program(&self) {
            gl::UseProgram(self.name);
        }

        pub unsafe fn attrib_location(&self, g: &Global) -> AttribLocation {
            let name = g.name.with_c_str(|ptr| gl::GetAttribLocation(self.name, ptr));
            AttribLocation { name: name }
        }

        pub unsafe fn set_uniform<U:UniformArg>(&self, g: &Global, arg: U) {
            let loc = self.uniform_location(g);
            loc.uniform(arg);
        }

        pub unsafe fn uniform_location(&self, g: &Global) -> UniformLocation {
            let name = g.name.with_c_str(|ptr| gl::GetUniformLocation(self.name, ptr));
            UniformLocation { name: name }
        }

        pub unsafe fn bind_frag_data_location(&self, colorNumber: GLuint, g: &Global) {
            g.name.with_c_str(|ptr| gl::BindFragDataLocation(self.name, colorNumber, ptr));
        }

    }

    pub struct VertexShaderBuilder {
        header: ~str,
        lines: Vec<~str>
    }

    pub struct FragmentShaderBuilder {
        header: ~str,
        lines: Vec<~str>
    }

    impl VertexShaderBuilder {
        pub fn compile(&self) -> VertexShader {
            // println!("compiling VS: {:s}", self.lines.concat());
            let lines : Vec<_> =
                self.lines.iter().map(|s|s.as_slice()).collect();
            let name =
                super::compile_shader(lines.as_slice(), gl::VERTEX_SHADER);
            VertexShader { name: name }
        }

        pub fn out_global(&mut self, qualifiers: &str, type_: &str, name: &str) -> Global {
            self.global(format!("out {:s}", qualifiers), type_, name)
        }
    }

    impl FragmentShaderBuilder {
        pub fn compile(&self) -> FragmentShader {
            // println!("compiling FS: {:s}", self.lines.concat());
            let lines : Vec<_> =
                self.lines.iter().map(|s|s.as_slice()).collect();
            let name =
                super::compile_shader(lines.as_slice(), gl::FRAGMENT_SHADER);
            FragmentShader { name: name }
        }

        pub fn in_global(&mut self, qualifiers: &str, g: &Global) -> Global {
            self.global(format!("in {:s}", qualifiers), g.type_, g.name)
        }
    }

    pub struct Global {
        type_: ~str,
        name: ~str,
    }

    pub trait ShaderBuilder {
        /// use via e.g. VertexShaderBuilder::new("#version 150 core")
        fn new<S:Str>(version_string: S) -> Self;

        fn new_150core() -> Self {
            ShaderBuilder::new("#version 150 core")
        }

        fn clear(&mut self);
        fn push<S:Str>(&mut self, line: S);

        fn global(&mut self, qualifiers: &str, type_: &str, name: &str) -> Global {
            self.push(format!("{:s} {:s} {:s};", qualifiers, type_, name));
            Global { type_: type_.into_owned(), name: name.into_owned() }
        }

        fn then<S:Str>(&mut self, line: S) {
            self.push(line);
        }

        fn def_fn<C:ContentFiller>(
            &mut self, name: &str, args: &[&str], ret: &str, content: C) {
            let sig = args.connect(", ");
            self.push(format!("{:s} {:s}({:s}) {}", ret, name, sig,
                              "{"));
            content.fill(|line| { self.push(line); });
            self.push("}");
        }
    }


    impl ShaderBuilder for FragmentShaderBuilder {
        /// use via e.g. VertexShaderBuilder::new("#version 150 core")
        fn new<S:Str>(version_string: S) -> FragmentShaderBuilder {
            let hdr = version_string.into_owned() + "\n";
            FragmentShaderBuilder {
                header: hdr.clone(),
                lines: vec!(hdr),
            }
        }

        fn clear(&mut self) {
            self.lines.clear();
            self.lines.push(self.header.clone());
        }

        fn push<S:Str>(&mut self, line: S) {
            self.lines.push(line.into_owned());
            self.lines.push(~"\n");
        }
    }

    impl ShaderBuilder for VertexShaderBuilder {
        fn new<S:Str>(version_string: S) -> VertexShaderBuilder {
            let hdr = version_string.into_owned() + "\n";
            VertexShaderBuilder {
                header: hdr.clone(),
                lines: vec!(hdr),
            }
        }

        fn clear(&mut self) {
            self.lines.clear();
            self.lines.push(self.header.clone());
        }

        fn push<S:Str>(&mut self, line: S) {
            self.lines.push(line.into_owned());
            self.lines.push(~"\n");
        }
    }

    pub trait ContentFiller {
        /// Calls `line` on each line of content.
        fn fill(&self, line:|str::MaybeOwned|);
    }

    impl<'a> ContentFiller for &'a str {
        fn fill(&self, line:|str::MaybeOwned|) {
            line(str::Slice(*self))
        }
    }

    impl<'a, S: Str> ContentFiller for &'a [S] {
        fn fill(&self, line:|str::MaybeOwned|) {
            for s in self.iter() {
                line(str::Slice(s.as_slice()))
            }
        }
    }
}

struct VertexArrays {
    len: GLsizei,
    names: ~[GLuint],
}

impl VertexArrays {
    fn new(len: u32) -> VertexArrays {
        let len = len as GLsizei;
        assert!(len > 0);
        let mut names = slice::from_elem(len as uint, 0u32);
        unsafe { gl::GenVertexArrays(len, &mut names[0]) }
        VertexArrays { len: len, names: names }
    }

    fn bind(&mut self, idx: u32) {
        gl::BindVertexArray(self.names[idx]);
    }
}

impl Drop for VertexArrays {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(self.len, self.names.as_ptr());
        }
    }
}

struct VertexBuffers {
    len: GLsizei,
    names: ~[GLuint],
}

enum BufferDataUsage {
    StreamDraw = gl::STREAM_DRAW,
    StreamRead = gl::STREAM_READ,
    StreamCopy = gl::STREAM_COPY,
    StaticDraw = gl::STATIC_DRAW,
    StaticRead = gl::STATIC_READ,
    StaticCopy = gl::STATIC_COPY,
    DynamicDraw = gl::DYNAMIC_DRAW,
    DynamicRead = gl::DYNAMIC_READ,
    DynamicCopy = gl::DYNAMIC_COPY,
}

impl VertexBuffers {
    fn new(len: u32) -> VertexBuffers {
        let len = len as GLsizei;
        assert!(len > 0);
        let mut names = slice::from_elem(len as uint, 0u32);
        unsafe { gl::GenBuffers(len, &mut names[0]); }
        VertexBuffers { len: len, names: names }
    }

    fn bind_array(&mut self, idx: u32) {
        gl::BindBuffer(gl::ARRAY_BUFFER, self.names[idx])
    }

    fn bind_and_init_array<T>(&mut self,
                              idx: u32,
                              init: &[T],
                              usage: BufferDataUsage) {
        self.bind_array(idx);
        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER,
                           (init.len() * mem::size_of::<T>()) as GLsizeiptr,
                           cast::transmute(&init[0]),
                           usage as GLenum);
        }
    }
}

impl Drop for VertexBuffers {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(self.len, self.names.as_ptr()); }
    }
}


fn perspective<V:Primitive+Zero+One+Float+ApproxEq<V>+Mul<V,V>+PartOrdFloat<V>>(
    fovy: ang::Rad<V>, aspect: V, zNear: V, zFar: V) -> mat::Matrix4<V>
{
    let one : V = One::one();
    let two : V = one + one;

    assert!(aspect != Zero::zero());
    assert!(zFar != zNear);

    let rad = fovy;
    let tanHalfFovy = ang::rad(rad.div_s(two).s.tan());

    let mut result = mat::Matrix4::zero();
    *result.mut_cr(0,0) = one / (aspect * tanHalfFovy.s);
    *result.mut_cr(1,1) = one / (tanHalfFovy.s);
    *result.mut_cr(2,2) = - (zFar + zNear) / (zFar - zNear);
    *result.mut_cr(2,3) = - one;
    *result.mut_cr(3,2) = - (two * zFar * zNear) / (zFar - zNear);
    return result;
}

fn gl() -> Result<(), ~str> {
    use glsl::ShaderBuilder;

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

    #[deriving(Default)]
    struct VertexDataRow {
        x: GLfloat,
        y: GLfloat,
        r: GLfloat,
        g: GLfloat,
        b: GLfloat,
        texcoord: (GLfloat, GLfloat)
    }

// Vertex data
type VERTEX_DATA_TYPE = [GLfloat, ..56];
static VERTEX_DATA: VERTEX_DATA_TYPE = [
    // X     Y    R    G    B  Texcoords
    -1.0,  0.5, 0.5, 0.5, 0.5, 0.0, 0.0, // Top-left
     0.0,  0.5, 1.0, 0.0, 0.0, 1.0, 0.0, // Top-right
     0.0, -0.5, 0.0, 1.0, 0.0, 1.0, 1.0, // Bottom-right
    -1.0, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, // Bottom-left

     0.5,  0.5, 1.0, 1.0, 1.0, 0.0, 1.0, // Top-left
     1.5,  0.5, 1.0, 1.0, 1.0, 1.0, 1.0, // Top-right
     1.5, -0.5, 1.0, 1.0, 1.0, 1.0, 0.0, // Bottom-right
     0.5, -0.5, 1.0, 1.0, 1.0, 0.0, 0.0, // Bottom-left
];

    type rows_type = [VertexDataRow, ..8];
    assert_eq!(mem::size_of::<rows_type>(), mem::size_of::<VERTEX_DATA_TYPE>());
    let rows : &rows_type = unsafe { cast::transmute(&VERTEX_DATA) };

    // Shader sources
    let mut vs : glsl::VertexShaderBuilder = ShaderBuilder::new_150core();
    let position_g = vs.global("in", "vec2", "position");
    let color_g    = vs.global("in", "vec3", "color");
    let texcoord_g = vs.global("in", "vec2", "texcoord");

    let v2f_color    = vs.out_global("", "vec3", "v2f_color");
    let v2f_texcoord = vs.out_global("", "vec2", "v2f_texcoord");

    let model_g = vs.global("uniform", "mat4", "model");
    let view_g  = vs.global("uniform", "mat4", "view");
    let proj_g  = vs.global("uniform", "mat4", "proj");

    vs.def_fn("main", [], "void", "
        v2f_color = color;
        v2f_texcoord = texcoord;
        gl_Position = model * vec4(position, 0.0, 1.0);"
              );

    let mut fs : glsl::FragmentShaderBuilder = ShaderBuilder::new_150core();
    fs.in_global("", &v2f_color);
    fs.in_global("", &v2f_texcoord);
    let out_color_g  = fs.global("out", "vec4", "out_color");
    let tex_kitten_g = fs.global("uniform", "sampler2D", "texKitten");
    let tex_puppy_g  = fs.global("uniform", "sampler2D", "texPuppy");
    fs.def_fn("main", [], "void", "
        vec4 colKitten = texture(texKitten, v2f_texcoord);
        vec4 colPuppy  = texture(texPuppy, v2f_texcoord);
        out_color = mix(colKitten, colPuppy, 0.5) * vec4(v2f_color, 1.0);
        // out_color = colKitten * vec4(v2f_color, 1.0);
        // out_color = mix(colKitten, colPuppy, 0.5);"
              );

    // Create GLSL shaders
    let vs = vs.compile();
    let fs = fs.compile();
    let program1 = glsl::Program::link(&vs, &fs);

    let mut vao;
    let mut vbo;

    unsafe {
        // Create Vertex Array Object
        vao = VertexArrays::new(1);
        vao.bind(0);

        // Create a Vertex Buffer Object and copy the vertex data to it
        vbo = VertexBuffers::new(1);
        vbo.bind_and_init_array(0, rows.slice_from(0), StaticDraw);

        // Use shader program
        program1.use_program();
        program1.bind_frag_data_location(0, &out_color_g);

        // Specify the layout of the vertex data
        let default : VertexDataRow = Default::default();
        let row_size = mem::size_of::<VertexDataRow>() as GLsizei;
        let pos_offset = (&default.x as *_ as uint) - cast::transmute(&default);
        let pos_attr = program1.attrib_location(&position_g);
        pos_attr.enable_current_vertex_attrib_array();
        pos_attr.vertex_attrib_pointer(
            2, gl::FLOAT, gl::FALSE as GLboolean, row_size, cast::transmute(pos_offset));

        let col_offset = (&default.r as *_ as uint) - cast::transmute(&default);
        let col_attr = program1.attrib_location(&color_g);
        col_attr.enable_current_vertex_attrib_array();
        col_attr.vertex_attrib_pointer(
            3, gl::FLOAT, gl::FALSE as GLboolean, row_size, cast::transmute(col_offset));

        let tex_offset = (&default.texcoord as *_ as uint) - cast::transmute(&default);
        let tex_attr = program1.attrib_location(&texcoord_g);
        tex_attr.enable_current_vertex_attrib_array();
        tex_attr.vertex_attrib_pointer(
            2, gl::FLOAT, gl::FALSE as GLboolean, row_size, cast::transmute(tex_offset));
    }

    // let uni_color = unsafe {
    //     "triangle_color".with_c_str(|ptr| gl::GetUniformLocation(program1.name, ptr))
    // };

    let mut textures = vec!(0, 0);
    unsafe { gl::GenTextures(2, textures.as_mut_ptr()); }

    gl::ActiveTexture(gl::TEXTURE0);
    gl::BindTexture(gl::TEXTURE_2D, *textures.get(0));
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
    unsafe {
        program1.set_uniform(&tex_kitten_g, 0i32);
    }


    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);


    gl::ActiveTexture(gl::TEXTURE1);
    gl::BindTexture(gl::TEXTURE_2D, *textures.get(1));
    let image = try!(surf::Surface::from_bmp(&Path::new("sample2.bmp")));
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

    unsafe {
        program1.set_uniform(&tex_puppy_g, 1i32);
    }

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);


    let elements : Vec<GLuint> = vec!(4, 5, 6, 6, 7, 4,
                                      0, 1, 2, 2, 3, 0);
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
                if (time - loop_start_time) > 5.0 {
                    break
                }
            }
        }

        // Use a uniform red
        // gl::Uniform3f(uni_color, 1.0, 0.0, 0.0);

        let time = time::precise_time_s();
        // gl::Uniform3f(uni_color, ((time+4.0).sin() as f32 + 1.0)/2.0, 0.0, 0.0);

        gl::ClearColor(0.3, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        let trans = mat::Matrix4::<f32>::identity();

        let rot = &mat::Matrix3::from_angle_z(ang::deg(time as f32 * 180.0f32)
                                              .to_rad())
            .to_matrix4();
        // (Apparently `*` on Matrix4 does not multiply the same way
        // that `mul_m` does.  It seems like it delegates to `mul_v`,
        // which does a dot-product against a vector, using each
        // column of the rhs as the vector to use in the dot-product.
        // At least, that's my current understanding.  It is a
        // somewhat strange design choice here.)
        //
        // Anyway, using `mul_m` fixes the issue for me.
        let trans = trans.mul_m(rot);
        // let result = trans.mul_v(&Vector4::new(1.0, 0.0, 0.0, 1.0));
        // println!("{:f}, {:f}, {:f}", result.x, result.y, result.z);
        unsafe {
            program1.set_uniform(&model_g, &trans);
        }

        let view = mat::Matrix4::<f32>::identity();
        unsafe {
            program1.set_uniform(&view_g, &view);
        }

        let proj = perspective(ang::deg(45.0f32).to_rad(),
                               800.0 / 600.0,
                               1.0,
                               10.0);
        unsafe {
            program1.set_uniform(&proj_g, &proj);
        }

        // Draw a rectangle from the 6 vertices
        // gl::DrawArrays(gl::TRIANGLES, 0, 6);
        unsafe { gl::DrawElements(gl::TRIANGLES, 12, gl::UNSIGNED_INT, ptr::null()); }

        win.gl_swap_window();
    }

    // sdl::timer::delay(2000);

    return Ok(());
}

pub fn compile_shader(src: &[&str], ty: GLenum) -> GLuint {
    let shader = gl::CreateShader(ty);
    unsafe {
        // Attempt to compile the shader
        let strs : Vec<c_str::CString> = src.iter().map(|s|s.to_c_str()).collect();
        gl::ShaderSource(shader,
                         {
                             let src_len = src.len() as GLsizei;
                             assert!(src_len > 0);
                             src_len
                         },
                         strs.iter().map(|cs| {
                             cs.as_bytes_no_nul().as_ptr() as *GLchar
                         }).collect::<Vec<_>>().as_ptr(),
                         strs.iter().map(|cs| {
                             let l = cs.len() as GLint;
                             assert!(l >= 0);
                             l
                         }).collect::<Vec<_>>().as_ptr());
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
