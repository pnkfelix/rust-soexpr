#![feature(globs)]
#![feature(macro_rules)]

#![allow(unused_imports)] // don't warn me about this right now

extern crate native;
extern crate rand;
extern crate time;

extern crate sdl = "sdl2";
extern crate gl;

extern crate opengles;

extern crate cgmath;

extern crate gl;

extern crate time;

use std::cast;
use std::c_str;
use std::default::Default;
use std::libc;
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

use gl::types::{GLchar, GLint, GLuint, GLsizei, GLsizeiptr};
use gl::types::{GLfloat};

/*
use self::high_level::{VertexArrayObj, VertexBufferObj, ElementsBufferObj,
                       VertexShader, FragmentShader, ShaderProgram};
use self::high_level::{TextureUnit};
use self::high_level::{VertexAttribPointerArgsFrom, VertexAttribPointerArgs};
*/
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
        ("gl", _) => gl(),
/*
        ("open_gl", _) |
        ("open_gl_drawing", _)
            => open_gl_drawing(),
*/
/*
        ("open_gl_textures", Some(arg)) if arg.as_slice() == "colored"
            => open_gl_textures(ColoredKitten),
        ("open_gl_textures", Some(arg)) if arg.as_slice() == "mix"
            => open_gl_textures(KittenPuppy),
        ("open_gl_textures", _)
            => open_gl_textures(ColoredKitten),
*/
        ("hello", _)                    => hello(),
        _otherwise                      => default(),
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
    use std::default::Default;
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

    pub trait TupleReflect {
        fn size_and_gl_type(&self) -> (GLint, GLenum);
        unsafe fn offset<T>(&self, default_row: &T) -> *GLvoid {
            let offset = (self as *_ as uint) - cast::transmute(default_row);
            cast::transmute(offset)
        }
    }

    impl TupleReflect for GLfloat {
        fn size_and_gl_type(&self) -> (GLint, GLenum) { (1, gl::FLOAT) }
    }

    impl TupleReflect for (GLfloat, GLfloat) {
        fn size_and_gl_type(&self) -> (GLint, GLenum) { (2, gl::FLOAT) }
    }

    impl TupleReflect for (GLfloat, GLfloat, GLfloat) {
        fn size_and_gl_type(&self) -> (GLint, GLenum) { (3, gl::FLOAT) }
    }

    impl TupleReflect for (GLfloat, GLfloat, GLfloat, GLfloat) {
        fn size_and_gl_type(&self) -> (GLint, GLenum) { (4, gl::FLOAT) }
    }

    pub trait VertexAttribPointerRTTI {
        fn size(&self) -> GLint;
        fn gl_type(&self) -> GLenum;
        fn stride(&self) -> GLsizei;
        fn pointer(&self) -> *GLvoid;
    }

    impl<'a, ROW_TYPE, TUPLE:TupleReflect> VertexAttribPointerRTTI for (&'a TUPLE, &'a ROW_TYPE) {
        fn size(&self) -> GLint {
            let &(field, _) = self;
            field.size_and_gl_type().val0()
        }
        fn gl_type(&self) -> GLenum {
            let &(field, _) = self;
            field.size_and_gl_type().val1()
        }
        fn stride(&self) -> GLsizei {
            mem::size_of::<ROW_TYPE>() as GLsizei
        }
        fn pointer(&self) -> *GLvoid {
            let &(field, row_ptr) = self;

            // Check that the inputs are sane before doing the unsafe
            // computation.  This doesn't ensure that the result is
            // correct, but it should help catch bugs.
            assert!(row_ptr as *_ as uint <= field as *_ as uint);
            assert!(field as *_ as uint + mem::size_of::<TUPLE>()
                    <= row_ptr as *_ as uint + mem::size_of::<ROW_TYPE>());

            unsafe { field.offset(row_ptr) }
        }
    }

    impl VertexAttribPointerRTTI for (GLint, GLenum, GLsizei, *GLvoid) {
        fn size(&self) -> GLint { self.val0() }
        fn gl_type(&self) -> GLenum { self.val1() }
        fn stride(&self) -> GLsizei { self.val2() }
        fn pointer(&self) -> *GLvoid { self.val3() }
    }

    impl AttribLocation {
        pub fn enable_current_vertex_attrib_array(&self) {
            gl::EnableVertexAttribArray(self.name as GLuint);
        }
        pub unsafe fn vertex_attrib_pointer<R:VertexAttribPointerRTTI>(
            &self, normalized: GLboolean, args: R) {
            let size = args.size();
            let type_ = args.gl_type();
            let stride = args.stride();
            let pointer = args.pointer();
            gl::VertexAttribPointer(
                self.name as GLuint, size, type_, normalized, stride, pointer);
        }
    }

    pub trait UniformArg { fn set_at_location(&self, location: GLint); }

    pub trait SettableTo<T> { }

    impl<'a> SettableTo<Mat4> for &'a mat::Matrix4<f32> { }

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

        pub unsafe fn attrib_location<T>(&self, g: &Global<T>) -> AttribLocation {
            let name = g.name.with_c_str(|ptr| gl::GetAttribLocation(self.name, ptr));
            AttribLocation { name: name }
        }

        pub unsafe fn set_uniform
            <T:GLSLType,U:UniformArg+SettableTo<T>>(
                &self, g: &Global<T>, arg: U) {
            let loc = self.uniform_location(g);
            loc.uniform(arg);
        }

        pub unsafe fn uniform_location<T>(&self, g: &Global<T>) -> UniformLocation {
            let name = g.name.with_c_str(|ptr| gl::GetUniformLocation(self.name, ptr));
            UniformLocation { name: name }
        }

        pub unsafe fn bind_frag_data_location<T>(&self, colorNumber: GLuint, g: &Global<T>) {
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

        pub fn out_global<T:GLSLType>(&mut self, qualifiers: &str, name: &str) -> Global<T> {
            self.global::<T>(format!("out {:s}", qualifiers), name)
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

        pub fn in_global<T:GLSLType>(&mut self, qualifiers: &str, g: &Global<T>) -> Global<T> {
            self.global::<T>(format!("in {:s}", qualifiers), g.name)
        }
    }

    // hacking around lack of associated types by making it "easy" to make
    // dummy default values and then call (what should be) static methods
    // on them
    pub trait GLSLType : Default {
        fn type_<'a>(&'a self) -> &'a str;
    }

    macro_rules! glsl_zstruct {
        ( $RustName:ident $string:expr )
            =>
        {
            pub struct $RustName;
            impl Default for $RustName { fn default() -> $RustName { $RustName } }
            impl GLSLType for $RustName {
                fn type_<'a>(&'a self) -> &'a str { stringify!($string) }
            }
        }
    }

    glsl_zstruct!(Sampler2D sampler2D)
    glsl_zstruct!(Vec2 vec2)
    glsl_zstruct!(Vec3 vec3)
    glsl_zstruct!(Vec4 vec4)
    glsl_zstruct!(Mat4 mat4)

    pub struct Global<T/*:GLSLType*/> {
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

        fn global<T:GLSLType>(&mut self, qualifiers: &str, name: &str) -> Global<T> {
            let dummy_t : T = Default::default();
            let type_ = dummy_t.type_();
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
    names: ~[GLuint],
}

impl VertexArrays {
    fn new(len: u32) -> VertexArrays {
        let len = len as GLsizei;
        assert!(len > 0);
        let mut names = slice::from_elem(len as uint, 0u32);
        unsafe { gl::GenVertexArrays(len, &mut names[0]) }
        VertexArrays { names: names }
    }

    fn bind(&mut self, idx: u32) {
        gl::BindVertexArray(self.names[idx]);
    }
}

impl Drop for VertexArrays {
    fn drop(&mut self) {
        let len = self.names.len() as GLsizei;
        assert!(len >= 0);
        unsafe { gl::DeleteVertexArrays(len, self.names.as_ptr()); }
    }
}

struct VertexArray { singleton: VertexArrays }
impl VertexArray {
    fn new() -> VertexArray {
        VertexArray { singleton: VertexArrays::new(1) }
    }
    fn bind(&mut self) { self.singleton.bind(0) }
}

struct VertexBuffers {
    names: ~[GLuint],
    init_lens: ~[Option<uint>],
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
        let nones = slice::from_elem(len as uint, None);
        unsafe { gl::GenBuffers(len, &mut names[0]); }
        VertexBuffers { names: names, init_lens: nones }
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
        self.init_lens[idx] = Some(init.len());
    }
}

impl Drop for VertexBuffers {
    fn drop(&mut self) {
        let len = self.names.len() as GLsizei;
        assert!(len >= 0);
        unsafe { gl::DeleteBuffers(len, self.names.as_ptr()); }
    }
}

struct VertexBuffer { singleton: VertexBuffers }
impl VertexBuffer {
    pub fn new() -> VertexBuffer {
        VertexBuffer { singleton: VertexBuffers::new(1) }
    }
    fn bind_array(&mut self) { self.singleton.bind_array(0) }
    fn bind_and_init_array<T>(&mut self, init: &[T], usage: BufferDataUsage) {
        self.singleton.bind_and_init_array(0, init, usage)
    }
}

trait ElementBufferData {
    fn type_(&self) -> GLenum;
}

impl ElementBufferData for u8  {
    fn type_(&self) -> GLenum { gl::UNSIGNED_BYTE  } }
impl ElementBufferData for u16 {
    fn type_(&self) -> GLenum { gl::UNSIGNED_SHORT } }
impl ElementBufferData for u32 {
    fn type_(&self) -> GLenum { gl::UNSIGNED_INT   } }

struct ElementBuffers {
    names: ~[GLuint],
    inits: ~[Option<(GLenum, GLsizei)>],
}

impl ElementBuffers {
    fn new(len: u32) -> ElementBuffers {
        let len = len as GLsizei;
        assert!(len > 0);
        let mut names = slice::from_elem(len as uint, 0u32);
        let nones = slice::from_elem(len as uint, None);
        unsafe { gl::GenBuffers(len, &mut names[0]); }
        ElementBuffers { names: names, inits: nones }
    }

    fn bind_elements(&mut self, idx: u32) {
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.names[idx])
    }

    fn bind_and_init_elements<T:ElementBufferData>(&mut self,
                                                   idx: u32,
                                                   init: &[T],
                                                   usage: BufferDataUsage) {
        self.bind_elements(idx);

        // require positive len in part to catch bugs but mostly
        // because extracting generic type-associated data currently
        // requires a concrete value in Rust.  (UFCS will fix.)
        assert!(init.len() > 0);

        unsafe {
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                           (init.len() * mem::size_of::<T>()) as GLsizeiptr,
                           cast::transmute(&init[0]),
                           usage as GLenum);
        }

        let len = init.len() as GLsizei;
        // This check on the other hand is and will always be necessary.
        assert!(len >= 0);

        self.inits[idx] = Some((init[0].type_(), len));
    }

    fn draw_elements(&self, idx: u32) {
        match self.inits[idx] {
            None => fail!("called draw_elements on uninitialized idx"),
            Some((type_, len)) => 
                unsafe { gl::DrawElements(gl::TRIANGLES,
                                          len,
                                          type_,
                                          // ptr::null()
                                          self.names[idx] as *GLvoid
                                          ); }
        }
    }
}

impl Drop for ElementBuffers {
    fn drop(&mut self) {
        let len = self.names.len() as GLsizei;
        assert!(len >= 0);
        unsafe { gl::DeleteBuffers(len, self.names.as_ptr()); }
    }
}

struct ElementBuffer { singleton: ElementBuffers }
impl ElementBuffer {
    fn new() -> ElementBuffer {
        ElementBuffer { singleton: ElementBuffers::new(1) }
    }
    fn bind_elements(&mut self) { self.singleton.bind_elements(0) }
    fn bind_and_init_elements<T:ElementBufferData>(&mut self,
                                                   init: &[T],
                                                   usage: BufferDataUsage) {
        self.singleton.bind_and_init_elements(0, init, usage)
    }
    fn draw_elements(&self) { self.singleton.draw_elements(0) }
}

struct TextureUnit {
    idx: GLuint
}

impl glsl::UniformArg for TextureUnit {
    fn set_at_location(&self, location: GLint) {
        // even though the various TEXTURE_UNIT's are typed as GLenum
        // (== c_uint) and conceptually are somewhat like GLuint, the
        // only glUniform* call that works with them is glUniform1i.
        //
        // It may be worth considering changing gl-rs to type TEXTURE*
        // and MAX_COMBINED_TEXTURE_IMAGE_UNITS as GLint instead.
        gl::Uniform1i(location, self.idx as GLint);
    }
}

impl glsl::SettableTo<glsl::Sampler2D> for TextureUnit { }

impl TextureUnit {
    fn new(idx: GLuint) -> TextureUnit {
        assert!(idx < gl::MAX_COMBINED_TEXTURE_IMAGE_UNITS);
        TextureUnit{ idx: idx }
    }

    fn active(&self) {
        gl::ActiveTexture(gl::TEXTURE0 + self.idx);
    }
}

struct Textures {
    len: GLsizei,
    names: ~[GLuint],
}

enum TextureTarget {
    Texture1D = gl::TEXTURE_1D,
    Texture2D = gl::TEXTURE_2D,
    Texture3D = gl::TEXTURE_3D
}

impl Textures {
    fn new(len: uint) -> Textures {
        let len = len as GLsizei;
        assert!(len > 0);
        let mut names = slice::from_elem(len as uint, 0u32);
        unsafe { gl::GenTextures(len, &mut names[0]) }
        Textures { len: len, names: names }
    }

    fn bind(&self, idx: uint, dim: TextureTarget) {
        gl::BindTexture(dim as GLenum, self.names[idx]);
    }

    fn bind_and_set_image_2d(&self, idx: uint, image: &surf::Surface) {
        self.bind(idx, Texture2D);
        let (width, height) = (image.get_width(), image.get_height());
        let format = image.get_pixel_format();
        let format = sdl_format_to_gl_format_type(format);
        image.with_lock(|pixels| {
            unsafe {
                gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as GLint,
                               width as GLsizei, height as GLsizei,
                               0,
                               // gl::RGBA,
                               format.val0(),
                               format.val1(),
                               pixels.as_ptr() as *GLvoid);
            }
        });

    }
}

struct Texture {
    singleton: Textures
}

impl Texture {
    fn new() -> Texture {
        Texture { singleton: Textures::new(1) }
    }

    fn bind(&self, dim: TextureTarget) { self.singleton.bind(0, dim) }
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
    use glsl::TupleReflect;

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
        xy: (GLfloat, GLfloat),
        rgb: (GLfloat, GLfloat, GLfloat),
        texcoord: (GLfloat, GLfloat)
    }

// Vertex data
type VertexDataType = [GLfloat, ..56];
static VERTEX_DATA: VertexDataType = [
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

    type RowsType = [VertexDataRow, ..8];
    assert_eq!(mem::size_of::<RowsType>(), mem::size_of::<VertexDataType>());
    let rows : &RowsType = unsafe { cast::transmute(&VERTEX_DATA) };

    // Shader sources
    let mut vs : glsl::VertexShaderBuilder = ShaderBuilder::new_150core();
    let position_g = vs.global::<glsl::Vec2>("in", "position");
    let color_g    = vs.global::<glsl::Vec3>("in", "color");
    let texcoord_g = vs.global::<glsl::Vec2>("in", "texcoord");

    let v2f_color    = vs.out_global::<glsl::Vec3>("", "v2f_color");
    let v2f_texcoord = vs.out_global::<glsl::Vec2>("", "v2f_texcoord");

    let model_g = vs.global::<glsl::Mat4>("uniform", "model");
    let view_g  = vs.global::<glsl::Mat4>("uniform", "view");
    let proj_g  = vs.global::<glsl::Mat4>("uniform", "proj");

    vs.def_fn("main", [], "void", "
        v2f_color = color;
        v2f_texcoord = texcoord;
        gl_Position = model * vec4(position, 0.0, 1.0);"
              );

    let mut fs : glsl::FragmentShaderBuilder = ShaderBuilder::new_150core();
    fs.in_global("", &v2f_color);
    fs.in_global("", &v2f_texcoord);
    let out_color_g  = fs.global::<glsl::Vec4>("out", "out_color");
    let tex_kitten_g = fs.global::<glsl::Sampler2D>("uniform", "texKitten");
    let tex_puppy_g  = fs.global::<glsl::Sampler2D>("uniform", "texPuppy");
    fs.def_fn("main", [], "void", "
        vec4 colKitten = texture(texKitten, v2f_texcoord);
        vec4 colPuppy  = texture(texPuppy, v2f_texcoord);
        out_color = mix(colKitten, colPuppy.rgba, 0.5) * vec4(v2f_color, 1.0);
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
        vao = VertexArray::new();
        vao.bind();

        // Create a Vertex Buffer Object and copy the vertex data to it
        vbo = VertexBuffer::new();
        vbo.bind_and_init_array(rows.slice_from(0), StaticDraw);

        // Use shader program
        program1.use_program();
        program1.bind_frag_data_location(0, &out_color_g);

        // Specify the layout of the vertex data
        let default : VertexDataRow = Default::default();

        let pos_attr = program1.attrib_location(&position_g);
        pos_attr.enable_current_vertex_attrib_array();
        pos_attr.vertex_attrib_pointer(gl::FALSE as GLboolean, (&default.xy, &default));

        let col_attr = program1.attrib_location(&color_g);
        col_attr.enable_current_vertex_attrib_array();
        col_attr.vertex_attrib_pointer(gl::FALSE as GLboolean, (&default.rgb, &default));

        let tex_attr = program1.attrib_location(&texcoord_g);
        tex_attr.enable_current_vertex_attrib_array();
        tex_attr.vertex_attrib_pointer(gl::FALSE as GLboolean, (&default.texcoord, &default));
    }

    // let uni_color = unsafe {
    //     "triangle_color".with_c_str(|ptr| gl::GetUniformLocation(program1.name, ptr))
    // };

    let textures = Textures::new(5);

    let texture_unit0 = TextureUnit::new(0);
    let texture_unit1 = TextureUnit::new(1);
    let texture_unit2 = TextureUnit::new(2);
    let texture_unit3 = TextureUnit::new(3);
    texture_unit0.active();
    let image = try!(surf::Surface::from_bmp(&Path::new("paris.bmp")));
    textures.bind_and_set_image_2d(1, image);
    set_misc_tex_params();

    texture_unit1.active();
    let image = try!(surf::Surface::from_bmp(&Path::new("bertin.bmp")));
    textures.bind_and_set_image_2d(2, image);
    set_misc_tex_params();

    texture_unit2.active();
    let image = try!(surf::Surface::from_bmp(&Path::new("kitten.bmp")));
    textures.bind_and_set_image_2d(3, image);
    set_misc_tex_params();

    texture_unit3.active();
    let image = try!(surf::Surface::from_bmp(&Path::new("puppy.bmp")));
    textures.bind_and_set_image_2d(4, image);
    set_misc_tex_params();

    fn set_misc_tex_params() {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
    }

    unsafe {
        program1.set_uniform(&tex_kitten_g, texture_unit0);
    }

    unsafe {
        program1.set_uniform(&tex_puppy_g, texture_unit1);
    }


    let elements : Vec<GLuint> = vec!(4, 5, 6, 6, 7, 4,
                                      0, 1, 2, 2, 3, 0);
    let mut ebo = ElementBuffer::new();
    ebo.bind_and_init_elements(elements.slice_from(0), StaticDraw);

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
        ebo.draw_elements();

        win.gl_swap_window();
    }

    // sdl::timer::delay(2000);

    return Ok(());
}

pub fn sdl_format_to_gl_format_type(f: pix::PixelFormat) -> (GLenum, GLenum) {
    use self::pix::ll::*;
    let flag = unsafe { (*f.raw).format };
    match flag {
        SDL_PIXELFORMAT_BGRA8888 => (gl::BGRA, gl::UNSIGNED_BYTE),

        // FIXME: these cannot both be right.
        SDL_PIXELFORMAT_RGBA8888 => (gl::BGRA, gl::UNSIGNED_INT_8_8_8_8_REV),
        SDL_PIXELFORMAT_ARGB8888 => (gl::BGRA, gl::UNSIGNED_INT_8_8_8_8_REV),

        SDL_PIXELFORMAT_ABGR8888 |
        SDL_PIXELFORMAT_UNKNOWN |
        SDL_PIXELFORMAT_INDEX1LSB |
        SDL_PIXELFORMAT_INDEX1MSB |
        SDL_PIXELFORMAT_INDEX4LSB |
        SDL_PIXELFORMAT_INDEX4MSB |
        SDL_PIXELFORMAT_INDEX8 |
        SDL_PIXELFORMAT_RGB332 |
        SDL_PIXELFORMAT_RGB444 |
        SDL_PIXELFORMAT_RGB555 |
        SDL_PIXELFORMAT_BGR555 |
        SDL_PIXELFORMAT_ARGB4444 |
        SDL_PIXELFORMAT_RGBA4444 |
        SDL_PIXELFORMAT_ABGR4444 |
        SDL_PIXELFORMAT_BGRA4444 |
        SDL_PIXELFORMAT_ARGB1555 |
        SDL_PIXELFORMAT_RGBA5551 |
        SDL_PIXELFORMAT_ABGR1555 |
        SDL_PIXELFORMAT_BGRA5551 |
        SDL_PIXELFORMAT_RGB565 |
        SDL_PIXELFORMAT_BGR565 |
        SDL_PIXELFORMAT_RGB24 |
        SDL_PIXELFORMAT_BGR24 |
        SDL_PIXELFORMAT_RGB888 |
        SDL_PIXELFORMAT_RGBX8888 |
        SDL_PIXELFORMAT_BGR888 |
        SDL_PIXELFORMAT_BGRX8888 |
        SDL_PIXELFORMAT_ARGB2101010 |
        SDL_PIXELFORMAT_YV12 |
        SDL_PIXELFORMAT_IYUV |
        SDL_PIXELFORMAT_YUY2 |
        SDL_PIXELFORMAT_UYVY |
        SDL_PIXELFORMAT_YVYU
            => fail!("unhandled SDL_PixelFormatFlag 0x{:x}", flag),
        _
            => fail!("unknown SDL_PixelFormatFlag 0x{:x}", flag),
    }
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




/*
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

// Tried:
//   http://useful-linux-tips.blogspot.fr/2013/11/complete-minimal-sdl2-opengl-animation.html
// but it uses glFrustum, which apparently has been deprecated 

fn open_gl_init() -> Result<(~vid::Window,~vid::GLContext), ~str> {
    static SCREEN_WIDTH:i32 = 800;
    static SCREEN_HEIGHT:i32 = 600;

    try!(sdl::init([sdl::InitEverything]));
    vid::gl_set_attribute(vid::GLContextProfileMask,
                          vid::ll::SDL_GL_CONTEXT_PROFILE_CORE as int) || fail!();
    vid::gl_set_attribute(vid::GLContextMajorVersion, 3) || fail!();
    vid::gl_set_attribute(vid::GLContextMinorVersion, 2) || fail!();

    let (width, height) = (SCREEN_WIDTH, SCREEN_HEIGHT);
    let win = ({
        let (x,y) = (vid::Positioned(100), vid::Positioned(100));
        vid::Window::new("Hello World", x, y, width, height,
                         [ vid::OpenGL, vid::Resizable, vid::Shown])
    }) || fail!();
    // This line needs to come after we create the window
    gl::load_with(vid::gl_get_proc_address);

    let context : ~vid::GLContext = try!(win.gl_create_context());

    Ok((win, context))
}

pub mod high_level {
    use std::cast;
    use std::libc;
    use std::mem;
    use std::os;
    use std::ptr;
    use std::str;

    use surf = sdl::surface;
    use pix = sdl::pixels;

    use gl;
    use gl::types::{GLchar, GLint, GLuint, GLsizei, GLsizeiptr};
    use gl::types::{GLenum, GLboolean, GLvoid, GLfloat};

    pub struct VertexArrayObj { vao: GLuint }
    impl VertexArrayObj {
        pub fn new() -> VertexArrayObj {
            let mut vao : GLuint = 0;
            unsafe {
                gl::GenVertexArrays(1, &mut vao);
                gl::BindVertexArray(vao);
            }
            VertexArrayObj { vao: vao }
        }
    }

    pub struct VertexBufferObj { vbo: GLuint }
    impl VertexBufferObj {
        pub fn new() -> VertexBufferObj {
            let mut vbo : GLuint = 0;
            unsafe {
                gl::GenBuffers(1, &mut vbo); // Generate 1 buffer
            }
            VertexBufferObj { vbo: vbo }
        }

        pub fn bind_array(&self, vertices: &[f32]) {
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

    pub struct ElementsBufferObj { ebo: GLuint }
    impl ElementsBufferObj {
        pub fn new() -> ElementsBufferObj {
            let mut ebo : GLuint = 0;
            unsafe { gl::GenBuffers(1, &mut ebo); }
            ElementsBufferObj { ebo: ebo }
        }

        pub fn bind_array(&self, elements: &[GLuint]) {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            unsafe {
                gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                               (mem::size_of::<GLuint>() * elements.len()) as i64,
                               elements.as_ptr() as *libc::c_void,
                               gl::STATIC_DRAW);
            }
        }
    }

    pub struct VertexShader { vertexShader: GLuint }
    impl VertexShader {
        pub fn new() -> VertexShader {
            VertexShader { vertexShader: gl::CreateShader(gl::VERTEX_SHADER) }
        }
        pub fn source(&self, vertexSource: &str) {
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

    pub struct FragmentShader { fragmentShader: GLuint }
    impl FragmentShader {
        pub fn new() -> FragmentShader {
            let fragmentShader = gl::CreateShader(gl::FRAGMENT_SHADER);
            FragmentShader { fragmentShader: fragmentShader }
        }
        pub fn source(&self, fragmentSource: &str) {
            unsafe {
                let fragmentSource = fragmentSource.to_c_str();
                fragmentSource.with_ref(|p| {
                    let tmp = ~[p];
                    gl::ShaderSource(self.fragmentShader, 1, tmp.as_ptr() as **GLchar, ptr::null());
                    gl::CompileShader(self.fragmentShader);
                });
                let mut status : GLint = 0;
                gl::GetShaderiv(self.fragmentShader, gl::COMPILE_STATUS, &mut status);
                if status != gl::TRUE as GLint {
                    let mut buffer = Vec::from_elem(512, 0 as libc::c_char);
                    gl::GetShaderInfoLog(self.fragmentShader, 512, ptr::mut_null(), buffer.as_mut_ptr());
                    let buffer : Vec<char> = buffer.iter().map(|&c| c as u8 as char).collect();
                    let end = buffer.iter().position(|&c|c == '\0').unwrap();
                    fail!("fragmentShader compilation failure {}", str::from_chars(buffer.slice_to(end)));
                }
            }
        }
    }

    pub struct ShaderProgram { shaderProgram: GLuint }
    impl ShaderProgram {
        pub fn new() -> ShaderProgram {
            let shaderProgram = gl::CreateProgram();
            ShaderProgram { shaderProgram: shaderProgram }
        }
        pub fn attach_shaders(&self, vs: &VertexShader, fs: &FragmentShader) {
            gl::AttachShader(self.shaderProgram, vs.vertexShader);
            gl::AttachShader(self.shaderProgram, fs.fragmentShader);
        }
        pub unsafe fn bind_frag_data_location(&self, color: u32, name: &str) {
            let name = name.to_c_str();
            name.with_ref(|n| gl::BindFragDataLocation(self.shaderProgram, color, n));
        }
        pub fn link_and_use(&self) {
            gl::LinkProgram(self.shaderProgram);
            gl::UseProgram(self.shaderProgram);
        }
        pub unsafe fn get_attrib_location(&self, name: &str) -> AttribLocation {
            let name = name.to_c_str();
            let posAttrib = name.with_ref(|n| gl::GetAttribLocation(self.shaderProgram, n));
            AttribLocation{ attrib: posAttrib }
        }
        pub unsafe fn get_uniform_location(&self, name: &str) -> UniformLocation {
            let name = name.to_c_str();
            let attrib = name.with_ref(|n| gl::GetUniformLocation(self.shaderProgram, n));
            UniformLocation { attrib: attrib }
        }
    }

    #[allow(non_camel_case_types)]
    pub trait GL_TYPE {
        fn tag(&self) -> GLenum; // this should be static (associated constant) but oh well
        fn count_in_bytes(&self, count: uint) -> uint { // same here, static method
            (count*mem::size_of::<Self>())
        }
    }

    impl GL_TYPE for GLfloat { fn tag(&self) -> GLenum { gl::FLOAT } }

    /// This assumes the input is coming from an array of T, and thus
    /// all the offset, len, and stride can all be correctly expressed
    /// in units of T.
    pub struct VertexAttribPointerArgsFrom<T/*:GL_TYPE*/> {
        pub t: T,
        pub size: GLint,
        pub normalized: bool,
        pub stride: uint,
        pub offset: uint,
    }

    pub struct VertexAttribPointerArgs {
        pub size: GLint,
        pub type_: GLenum,
        pub normalized: GLboolean,
        pub stride: GLsizei,
        pub pointer: *GLvoid,
    }

    pub trait ToVertexAttribPointerArgs {
        fn recompose(self) -> VertexAttribPointerArgs;
    }
    impl ToVertexAttribPointerArgs for VertexAttribPointerArgs {
        fn recompose(self) -> VertexAttribPointerArgs { self }
    }

    impl<T:GL_TYPE> ToVertexAttribPointerArgs for VertexAttribPointerArgsFrom<T> {
        fn recompose(self) -> VertexAttribPointerArgs {
            VertexAttribPointerArgs {
                size: self.size,
                type_: self.t.tag(),
                normalized: if self.normalized { gl::TRUE } else { gl::FALSE },
                stride: self.t.count_in_bytes(self.stride) as GLsizei,
                pointer: unsafe {
                    cast::transmute::<uint, *gl::types::GLvoid>
                        (self.t.count_in_bytes(self.offset)) },
            }
        }
    }

    impl ToVertexAttribPointerArgs for (GLint, GLenum, GLboolean, GLsizei, *GLvoid) {
        fn recompose(self) -> VertexAttribPointerArgs {
            let (size, type_, normalized, stride, pointer) = self;
            VertexAttribPointerArgs {
                size: size, type_: type_, normalized: normalized, stride: stride, pointer: pointer
            }
        }
    }
    impl ToVertexAttribPointerArgs for (GLint, GLenum, GLboolean, GLsizei) {
        fn recompose(self) -> VertexAttribPointerArgs {
            let (size, type_, normalized, stride) = self;
            VertexAttribPointerArgs {
                size: size, type_: type_, normalized: normalized, stride: stride,
                pointer: ptr::null()
            }
        }
    }

    pub struct AttribLocation { attrib: GLint }
    impl AttribLocation {
        pub fn enable_vertex_attrib_array(&self) {
            gl::EnableVertexAttribArray(self.attrib as GLuint);
        }
        pub unsafe fn vertex_attrib_pointer<A:ToVertexAttribPointerArgs>(&self, args: A) {
            let args = args.recompose();
            gl::VertexAttribPointer(
                self.attrib as GLuint, args.size, args.type_, args.normalized,
                args.stride, args.pointer);
        }
    }
    pub struct UniformLocation { attrib: GLint }
    impl UniformLocation {
        pub fn uniform1i(&self, v0: GLint) {
            gl::Uniform1i(self.attrib, v0);
        }
        pub fn uniform3f(&self, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
            gl::Uniform3f(self.attrib, v0, v1, v2);
        }
    }

    pub fn tex_image_2d(image: &surf::Surface, level: GLint, border: GLint) {
        let format = unsafe { (*image.get_pixel_format().raw).format };
        let (format, xfer_type) = match format {
            pix::ll::SDL_PIXELFORMAT_RGB444   => (gl::RGB4, gl::UNSIGNED_INT),
            pix::ll::SDL_PIXELFORMAT_RGB555   => (gl::RGB5, gl::UNSIGNED_INT),
            pix::ll::SDL_PIXELFORMAT_RGBA8888 => (gl::RGBA, gl::UNSIGNED_INT),
            pix::ll::SDL_PIXELFORMAT_ARGB8888 => (gl::BGRA, gl::UNSIGNED_INT_8_8_8_8_REV),
            _ => fail!("unhandled pixel_format in image: {:x}", format),
        };
        image.with_lock(|pixels| {
            unsafe {
                gl::TexImage2D(gl::TEXTURE_2D,                // target
                               level,
                               gl::RGB as GLint,              // internal format
                               image.get_width() as GLsizei,
                               image.get_height() as GLsizei,
                               border,
                               format,
                               xfer_type,
                               pixels.as_ptr() as *libc::c_void);
            }
        });
    }

    pub struct TextureUnit { glenum: GLenum }
    impl TextureUnit {
        pub fn from_byte(b: u8) -> TextureUnit {
            TextureUnit { glenum: gl::TEXTURE0 + (b as GLenum) }
        }
        pub fn active(&self) {
            gl::ActiveTexture(self.glenum);
        }

        pub fn with_active<A>(&self, f: || -> A) -> A {
            let mut result : GLint = 0;
            unsafe { gl::GetIntegerv(gl::ACTIVE_TEXTURE, &mut result); }
            let prev = result;
            // debug!("switching from {:x} to {:x}", prev, self.glenum);
            self.active();
            let res = f();
            // debug!("switching back to {:x} from {:x} ", prev, self.glenum);
            gl::ActiveTexture(prev as GLuint);
            res
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

    let fragmentShader = FragmentShader::new();
    fragmentShader.source(fragmentSource);

    let shaderProgram = ShaderProgram::new();
    shaderProgram.attach_shaders(&vertexShader, &fragmentShader);
    unsafe { shaderProgram.bind_frag_data_location(0, "outColor"); }
    shaderProgram.link_and_use();

    let posAttrib = unsafe { shaderProgram.get_attrib_location("position") };
    posAttrib.enable_vertex_attrib_array();
    unsafe {
        posAttrib.vertex_attrib_pointer(
            VertexAttribPointerArgsFrom {
                t: 0.0 as GLfloat, size: 2, normalized: false, stride: 5, offset: 0 });
    }
    let colAttrib = unsafe { shaderProgram.get_attrib_location("color") };
    colAttrib.enable_vertex_attrib_array();
    unsafe {
        colAttrib.vertex_attrib_pointer(
            VertexAttribPointerArgsFrom {
                t: 0.0 as GLfloat, size: 3, normalized: false, stride: 5, offset: 2 })
    }

    let uniColor = unsafe { shaderProgram.get_uniform_location("triangleColor") };

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
        uniColor.uniform3f(((time_*4.0).sin() + 1.0)/2.0, 0.0, 0.0);

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
    use Vapa = high_level::VertexAttribPointerArgs;
    use VapaF = high_level::VertexAttribPointerArgsFrom;

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
    let fragmentShader = FragmentShader::new();
    fragmentShader.source(fragmentSource);

    // Link the vertex and fragment shader into a shader program
    let shaderProgram = ShaderProgram::new();
    unsafe {
        shaderProgram.attach_shaders(&vertexShader, &fragmentShader);
        shaderProgram.bind_frag_data_location(0, "outColor");
        shaderProgram.link_and_use();
    }

    // Specify the layout of the vertex data
    let posAttrib = unsafe { shaderProgram.get_attrib_location("position") };
    posAttrib.enable_vertex_attrib_array();
    unsafe {
        posAttrib.vertex_attrib_pointer(VapaF {
            t: 0.0 as GLfloat, size: 2, normalized: false, stride: 7, offset: 0 });
    }

    let colAttrib = unsafe { shaderProgram.get_attrib_location("color") };
    colAttrib.enable_vertex_attrib_array();
    unsafe {
        colAttrib.vertex_attrib_pointer(VapaF {
            t: 0.0 as GLfloat, size: 3, normalized: false, stride: 7, offset: 2 });
    }

    let texAttrib = unsafe { shaderProgram.get_attrib_location("texcoord") };
    texAttrib.enable_vertex_attrib_array();
    unsafe {
        texAttrib.vertex_attrib_pointer(VapaF {
            t: 0.0 as GLfloat, size: 2, normalized: false, stride: 7, offset: 5 });
    }

    // Load textures
    let mut textures : ~[GLuint] = ~[ 0, 0 ];
    unsafe { gl::GenTextures(2, textures.as_mut_ptr()); }

    let tunit0 = TextureUnit::new(0);
    try!(tunit0.with_active(|| {
        gl::BindTexture(gl::TEXTURE_2D, textures[0]);
        {
            let file = Path::new("sample.bmp");
            let image = try!(surf::Surface::from_bmp(&file));
            high_level::tex_image_2d(image, 0, 0);
        }

        let uniCol = unsafe { shaderProgram.get_uniform_location("texKitten") };
        uniCol.uniform1i(0);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        Ok(())
    }));

    let tunit1 = TextureUnit::new(1);
    try!(tunit1.with_active(|| {
        gl::BindTexture(gl::TEXTURE_2D, textures[1]);
        {
            let file = Path::new("sample2.bmp");
            let image = try!(surf::Surface::from_bmp(&file));
            high_level::tex_image_2d(image, 0, 0);
        }

        let uniCol = unsafe { shaderProgram.get_uniform_location("texPuppy") };
        uniCol.uniform1i(1);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        Ok(())
    }));

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
*/

fn hello() -> Result<(), ~str> {
    static SCREEN_WIDTH:i32 = 640;
    static SCREEN_HEIGHT:i32 = 480;
    // http://www.open.gl/context/
    try!(sdl::init([sdl::InitEverything]));

    let (width, height) = (SCREEN_WIDTH, SCREEN_HEIGHT);
    let win = try!({
        let (x,y) = (vid::Positioned(100), vid::Positioned(100));
        vid::Window::new("Hello World", x, y, width as int, height as int,
                         [ vid::OpenGL, vid::Resizable, vid::Shown])
    });

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
