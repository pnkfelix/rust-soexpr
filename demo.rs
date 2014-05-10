#![feature(globs)]
#![feature(macro_rules)]
#![feature(phase)]

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
use std::fmt;
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
    match (variant, args.get(0).clone()) {
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
        ("glsl-cookbook", Some(s)) if "1".equiv(s) => glsl_cookbook_1(),
        ("glsl-cookbook", Some(s)) if "2".equiv(s) => glsl_cookbook_2(),
        ("glsl-cookbook", Some(s)) if "3".equiv(s) => glsl_cookbook_3(),
        ("gl-superbible", Some(s)) if "1".equiv(s) => gl_superbible_1(),
        ("gl-superbible", Some(s)) if "2".equiv(s) => gl_superbible_2(),
        ("gl-superbible", Some(s)) if "3".equiv(s) => gl_superbible_3(),
        ("gl-superbible", Some(s)) if "4".equiv(s) => gl_superbible_4(),
        _otherwise                      => fail!("Unrecognized variant: {}", variant),
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
    use std::fmt;
    use std::mem;
    use std::ptr;
    use std::str;

    pub struct VertexShader {
        name: GLuint,
    }

    pub struct FragmentShader {
        name: GLuint,
    }

    pub struct TesselationControlShader {
        name: GLuint,
    }

    pub struct TesselationEvaluationShader {
        name: GLuint,
    }

    impl Shader for VertexShader {
        fn new(name: GLuint) -> VertexShader { VertexShader { name: name } }
        fn name(&self) -> GLuint { self.name }
    }

    impl Shader for FragmentShader {
        fn new(name: GLuint) -> FragmentShader { FragmentShader { name: name } }
        fn name(&self) -> GLuint { self.name }
    }

    impl Shader for TesselationControlShader {
        fn new(name: GLuint) -> TesselationControlShader {
            TesselationControlShader { name: name } }
        fn name(&self) -> GLuint { self.name }
    }

    impl Shader for TesselationEvaluationShader {
        fn new(name: GLuint) -> TesselationEvaluationShader {
            TesselationEvaluationShader { name: name } }
        fn name(&self) -> GLuint { self.name }
    }

    pub struct ProgramBuilder {
        name: GLuint,
    }

    pub struct Program {
        name: GLuint,
    }

    pub struct AttribLocation<T/*:ToGLSLType*/> {
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

    pub trait VertexAttribPointerRTTI<T/*:ToGLSLType*/> {
        fn size(&self) -> GLint;
        fn gl_type(&self) -> GLenum;
        fn stride(&self) -> GLsizei;
        fn pointer(&self) -> *GLvoid;
    }

    /// Just use this to infer the type from the source attribute, under the assumption
    /// that the data source is (1.) tightly packed and (2.) contains no other data.
    pub struct Packed;
    impl<T:ToGLSLType2> VertexAttribPointerRTTI<T> for Packed {
        fn size(&self) -> GLint     { let d : T = Default::default(); d.count() }
        fn gl_type(&self) -> GLenum { let d : T = Default::default(); d.gl_type() }
        fn stride(&self) -> GLsizei { let d : T = Default::default(); d.stride() }
        fn pointer(&self) -> *GLvoid { ptr::null::<GLvoid>() }
    }

    impl<'a, T, ROW_TYPE, TUPLE:TupleReflect> VertexAttribPointerRTTI<T>
        for (&'a TUPLE, &'a ROW_TYPE) {
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

    impl<T> VertexAttribPointerRTTI<T> for (GLint, GLenum, GLsizei, *GLvoid) {
        fn size(&self) -> GLint { self.val0() }
        fn gl_type(&self) -> GLenum { self.val1() }
        fn stride(&self) -> GLsizei { self.val2() }
        fn pointer(&self) -> *GLvoid { self.val3() }
    }

    impl<T> AttribLocation<T> {
        pub fn enable_vertex_attrib_array(&self) {
            gl::EnableVertexAttribArray(self.name as GLuint);
        }
    }

    impl<T:ToGLSLType> AttribLocation<T> {
        pub unsafe fn vertex_attrib_pointer<R:VertexAttribPointerRTTI<T>>(
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

    impl ProgramBuilder {
        pub fn new_unattached() -> ProgramBuilder {
            let program = gl::CreateProgram();
            ProgramBuilder { name: program }
        }

        pub fn new(vs: &VertexShader, fs: &FragmentShader) -> ProgramBuilder {
            let mut pb = ProgramBuilder::new_unattached();
            pb.attach_shader(vs);
            pb.attach_shader(fs);
            pb
        }

        pub fn attach_shader<SHDR:Shader>(&mut self, s: &SHDR) {
            gl::AttachShader(self.name, s.name());
        }

        pub fn link(self) -> Result<Program, (ProgramBuilder, ~str)> {
            super::try_link_program(self.name)
                .map(|_| Program { name: self.name })
                .map_err(|m| (self,m))
        }

        pub fn bind_attrib_location<T:ToGLSLType>(&self, l: &AttribLocation<T>, g: &Global<T>) {
            g.name.with_c_str(|ptr| unsafe {
                gl::BindAttribLocation(self.name, l.name as GLuint, ptr)
            });
        }

        pub unsafe fn bind_frag_data_location<T>(&self, colorNumber: GLuint, g: &Global<T>) {
            g.name.with_c_str(|ptr| gl::BindFragDataLocation(self.name, colorNumber, ptr));
        }
    }

    impl Program {
        pub fn new(vs: &VertexShader, fs: &FragmentShader) -> Program {
            let program = ProgramBuilder::new(vs, fs);
            program.link().unwrap()
        }

        pub fn use_program(&self) {
            gl::UseProgram(self.name);
        }

        pub unsafe fn raw_attrib_location<'a>(&self, name: &'a str) -> GLint {
            name.with_c_str(|ptr| gl::GetAttribLocation(self.name, ptr))
        }

        pub unsafe fn attrib_location<T:ToGLSLType>(&self, g: &Global<T>) -> AttribLocation<T> {
            let name = self.raw_attrib_location(g.name);
            AttribLocation { name: name }
        }

        pub unsafe fn set_uniform
            <T:ToGLSLType,U:UniformArg+SettableTo<T>>(
                &self, g: &Global<T>, arg: U) {
            let loc = self.uniform_location(g);
            loc.uniform(arg);
        }

        pub unsafe fn raw_uniform_location(&self, name: &str) -> GLint  {
            name.with_c_str(|ptr| gl::GetUniformLocation(self.name, ptr))
        }

        pub unsafe fn uniform_location<T>(&self, g: &Global<T>) -> UniformLocation {
            let name = self.raw_uniform_location(g.name);
            UniformLocation { name: name }
        }

        // Returns the (size, type, name) of all active attributes
        pub fn active_attribs(&self) -> ~[(GLint, GLenum, ~str)] {
            let mut n_attribs : GLint = 0;
            unsafe { gl::GetProgramiv(self.name, gl::ACTIVE_ATTRIBUTES, &mut n_attribs); }
            let mut max_len : GLint = 0;
            unsafe { gl::GetProgramiv(self.name, gl::ACTIVE_ATTRIBUTE_MAX_LENGTH, &mut max_len); }
            let mut buf = Vec::from_elem(max_len as uint, 0u8);
            assert!(n_attribs >= 0);
            let mut attribs = Vec::with_capacity(n_attribs as uint);
            let n_attribs = n_attribs as GLuint;
            for i in range(0, n_attribs) {
                let mut written: GLint = 0;
                let mut size: GLint = 0;
                let mut typ: GLenum = 0;
                unsafe {
                    gl::GetActiveAttrib(self.name, i, max_len, &mut written, &mut size, &mut typ,
                                        cast::transmute(buf.as_mut_ptr()));
                }
                assert!(written >= 0);
                let written = written as uint;
                attribs.push((size, typ, str::from_utf8(buf.slice_to(written)).unwrap().to_owned()));
            }
            attribs.move_iter().collect()
        }

        // Returns the (size, type, name) of all active uniform variables
        pub fn active_uniforms(&self) -> ~[(GLint, GLenum, ~str)] {
            let mut n_uniforms : GLint = 0;
            unsafe { gl::GetProgramiv(self.name, gl::ACTIVE_UNIFORMS, &mut n_uniforms); }
            let mut max_len : GLint = 0;
            unsafe { gl::GetProgramiv(self.name, gl::ACTIVE_UNIFORM_MAX_LENGTH, &mut max_len); }
            let mut buf = Vec::from_elem(max_len as uint, 0u8);
            assert!(n_uniforms >= 0);
            let mut uniforms = Vec::with_capacity(n_uniforms as uint);
            let n_uniforms = n_uniforms as GLuint;
            for i in range(0, n_uniforms) {
                let mut written: GLint = 0;
                let mut size: GLint = 0;
                let mut typ: GLenum = 0;
                unsafe {
                    gl::GetActiveUniform(self.name, i, max_len, &mut written, &mut size, &mut typ,
                                         cast::transmute(buf.as_mut_ptr()));
                }
                assert!(written >= 0);
                let written = written as uint;
                uniforms.push((size, typ, str::from_utf8(buf.slice_to(written)).unwrap().to_owned()));
            }
            uniforms.move_iter().collect()
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

    pub struct TessellationControlShaderBuilder {
        header: ~str,
        lines: Vec<~str>
    }

    pub struct TessellationEvaluationShaderBuilder {
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

        pub fn out_global<T:ToGLSLType>(&mut self, qualifiers: &str, name: &str) -> Global<T> {
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

        pub fn in_global<T:ToGLSLType>(&mut self, qualifiers: &str, g: &Global<T>) -> Global<T> {
            self.global::<T>(format!("in {:s}", qualifiers), g.name)
        }
    }

    #[allow(non_camel_case_types)]
    pub enum GLSLType {
        sampler2d,
        vec2,
        vec3,
        vec4,
        mat4,
        float,
    }

    impl ::std::fmt::Show for GLSLType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f.buf, "{:s}", match self {
                &sampler2d => "sampler2d",
                &vec2 => "vec2",
                &vec3 => "vec3",
                &vec4 => "vec4",
                &mat4 => "mat4",
                &float => "float",
            })
        }
    }

    // hacking around lack of associated types by making it "easy" to make
    // dummy default values and then call (what should be) static methods
    // on them
    pub trait ToGLSLType : Default {
        fn type_<'a>(&'a self) -> &'a str;
    }

    pub trait ToGLSLType2 : ToGLSLType {
        // (analogous to VertexAttribPointerRTTI but a little simpler)
        fn count(&self) -> GLint;
        fn gl_type(&self) -> GLenum;
        fn stride(&self) -> GLsizei;
    }

    macro_rules! glsl_zstruct {
        ( $RustName:ident $string:expr $count:expr $glenum:expr $rep:ty)
            =>
        {
            pub struct $RustName;
            impl Default for $RustName { fn default() -> $RustName { $RustName } }
            impl ToGLSLType for $RustName {
                fn type_<'a>(&'a self) -> &'a str { stringify!($string) }
            }
            impl ToGLSLType2 for $RustName {
                fn count(&self) -> GLint { $count }
                fn gl_type(&self) -> GLenum { $glenum }
                fn stride(&self) -> GLsizei { mem::size_of::<$rep>() as GLsizei }
            }
        }
    }

    type GLfloatvec2 = (GLfloat, GLfloat);
    type GLfloatvec3 = (GLfloat, GLfloat, GLfloat);
    type GLfloatvec4 = (GLfloat, GLfloat, GLfloat, GLfloat);
    type GLfloatmat4 = ((GLfloat, GLfloat, GLfloat, GLfloat),
                        (GLfloat, GLfloat, GLfloat, GLfloat),
                        (GLfloat, GLfloat, GLfloat, GLfloat),
                        (GLfloat, GLfloat, GLfloat, GLfloat));

    glsl_zstruct!(Sampler2D sampler2D 1 gl::INT GLint)
    glsl_zstruct!(     Vec2 vec2      2 gl::FLOAT GLfloatvec2)
    glsl_zstruct!(     Vec3 vec3      3 gl::FLOAT GLfloatvec3)
    glsl_zstruct!(     Vec4 vec4      4 gl::FLOAT GLfloatvec4)
    glsl_zstruct!(     Mat4 mat4     16 gl::FLOAT GLfloatmat4)

    pub struct Global<T/*:ToGLSLType*/> {
        type_: ~str,
        name: ~str,
    }

    pub trait Shader {
        fn new(name: GLuint) -> Self;
        fn name(&self) -> GLuint;
    }

    pub trait ShaderBuilder<SHDR:Shader> {
        /// use via e.g. VertexShaderBuilder::new("#version 150 core")
        fn new<S:Str>(version_string: S) -> Self;

        fn new_150core() -> Self {
            ShaderBuilder::new("#version 150 core")
        }

        fn shader_type(&self) -> GLenum;

        fn compile(&mut self) -> SHDR {
            // println!("compiling VS: {:s}", self.lines.concat());
            let type_ = self.shader_type();
            let lines : Vec<_> =
                self.lines().iter().map(|s|s.as_slice()).collect();
            let name =
                super::compile_shader(lines.as_slice(), type_);
            Shader::new(name)
        }


        fn header<'a>(&'a self) -> &'a str;
        fn lines<'a>(&'a mut self) -> &'a mut Vec<~str>;

        fn clear(&mut self) {
            let hdr = self.header().to_owned();
            let lines = self.lines();
            lines.clear();
            lines.push(hdr);
        }
        fn push<S:Str>(&mut self, line: S) {
            let lines = self.lines();
            lines.push(line.into_owned());
            lines.push(~"\n");
        }

        fn global<T:ToGLSLType>(&mut self, qualifiers: &str, name: &str) -> Global<T> {
            let dummy_t : T = Default::default();
            let type_ = dummy_t.type_();
            self.push(format!("{:s} {:s} {:s};", qualifiers, type_, name));
            Global { type_: type_.into_owned(), name: name.into_owned() }
        }

        fn uniform_block(&mut self,
                         name: &str,
                         contents: &[(GLSLType, &str)],
                         instance_name: Option<&str>) {
            self.push(format!("uniform {:s} {}", name, "{"));
            for &(ref typ_, name) in contents.iter() {
                self.push(format!("  {} {:s};", typ_, name));
            }
            match instance_name {
                None    => self.push("};"),
                Some(s) => self.push(format!("{:s} {:s};", "}", s)),
            }
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

        fn def_main<C:ContentFiller>(&mut self, content: C) {
            self.def_fn("main", [], "void", content)
        }
    }


    impl ShaderBuilder<FragmentShader> for FragmentShaderBuilder {
        /// use via e.g. VertexShaderBuilder::new("#version 150 core")
        fn new<S:Str>(version_string: S) -> FragmentShaderBuilder {
            let hdr = version_string.into_owned() + "\n";
            FragmentShaderBuilder { header: hdr.clone(), lines: vec!(hdr) }
        }

        fn header<'a>(&'a self) -> &'a str { self.header.as_slice() }
        fn lines<'a>(&'a mut self) -> &'a mut Vec<~str> { &mut self.lines }
        fn shader_type(&self) -> GLenum { gl::FRAGMENT_SHADER }

        fn clear(&mut self) {
            self.lines.clear();
            self.lines.push(self.header.clone());
        }

        fn push<S:Str>(&mut self, line: S) {
            self.lines.push(line.into_owned());
            self.lines.push(~"\n");
        }
    }

    impl ShaderBuilder<VertexShader> for VertexShaderBuilder {
        fn new<S:Str>(version_string: S) -> VertexShaderBuilder {
            let hdr = version_string.into_owned() + "\n";
            VertexShaderBuilder { header: hdr.clone(), lines: vec!(hdr) }
        }

        fn header<'a>(&'a self) -> &'a str { self.header.as_slice() }
        fn lines<'a>(&'a mut self) -> &'a mut Vec<~str> { &mut self.lines }

        fn shader_type(&self) -> GLenum { gl::VERTEX_SHADER }
    }

    impl ShaderBuilder<TesselationControlShader>
        for TessellationControlShaderBuilder {
        fn new<S:Str>(version_string: S) -> TessellationControlShaderBuilder {
            let hdr = version_string.into_owned() + "\n";
            TessellationControlShaderBuilder {
                header: hdr.clone(), lines: vec!(hdr) }
        }
        fn header<'a>(&'a self) -> &'a str { self.header.as_slice() }
        fn lines<'a>(&'a mut self) -> &'a mut Vec<~str> { &mut self.lines }
        fn shader_type(&self) -> GLenum { gl::TESS_CONTROL_SHADER }
    }

    impl ShaderBuilder<TesselationEvaluationShader>
        for TessellationEvaluationShaderBuilder {
        fn new<S:Str>(version_string: S) -> TessellationEvaluationShaderBuilder {
            let hdr = version_string.into_owned() + "\n";
            TessellationEvaluationShaderBuilder {
                header: hdr.clone(), lines: vec!(hdr) }
        }
        fn header<'a>(&'a self) -> &'a str { self.header.as_slice() }
        fn lines<'a>(&'a mut self) -> &'a mut Vec<~str> { &mut self.lines }
        fn shader_type(&self) -> GLenum { gl::TESS_EVALUATION_SHADER }
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    fn new() -> Texture {
        Texture { singleton: Textures::new(1) }
    }

    #[allow(dead_code)]
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

struct WindowOpts {
    width: int, height: int
}

/// A window with an attached (pre-initialized) GL context.
struct WinGL {
    win: ~vid::Window,
    ctxt: ~vid::GLContext,
}

impl WindowOpts {
    fn init(&self) -> Result<WinGL, ~str> {
        let (w, c) = try!(init_common(*self));
        Ok(WinGL{ win: w, ctxt: c })
    }
}

enum Redraw { Redraw, NothingChanged }

pub type EventHandler<'a> = 'a |e: evt::Event, seconds_elapsed: f64| -> Redraw;

impl WinGL {
    fn loop_timeout(&mut self,
                    seconds_max: f64,
                    process: EventHandler) -> Result<(), ~str> {
        sdl_loop_timeout(self.win, seconds_max, process)
    }
}

fn init_common(w: WindowOpts) -> Result<(~vid::Window, ~vid::GLContext), ~str> {

    let WindowOpts{ width, height } = w;

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

    let win = try!(
        vid::Window::new("Hello World", 100, 100, width, height,
                         [vid::Shown]));

    let ctxt = try!(win.gl_create_context());

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

    gl::load_with(vid::gl_get_proc_address);

    Ok((win, ctxt))
}

fn sdl_loop_timeout(win: &vid::Window,
                    seconds_max: f64,
                    process: EventHandler) -> Result<(), ~str> {
    let loop_start_time = time::precise_time_s();
    loop {
        let e = evt::poll_event();
        let elapsed;
        match e {
            evt::QuitEvent(_) | evt::KeyUpEvent(_, _, key::EscapeKey, _, _)
                => break,
            _ => {
                let time = time::precise_time_s();
                elapsed = time - loop_start_time;
                if elapsed > seconds_max {
                    break
                }
            }
        }
        match process(e, elapsed) {
            Redraw => win.gl_swap_window(),
            NothingChanged => {}
        }
    }
    Ok(())
}

fn gl_superbible_1() -> Result<(), ~str> {
    use VSB = self::glsl::VertexShaderBuilder;
    use FSB = self::glsl::FragmentShaderBuilder;
    use glsl::ShaderBuilder;

    let mut win = try!((WindowOpts{ width: 800, height: 800}).init());

    let mut vs : VSB = ShaderBuilder::new("#version 410 core"); // was 430
    vs.def_main("gl_Position = vec4(0.0, 0.0, 0.5, 1.0);");

    let mut fs : FSB = ShaderBuilder::new("#version 410 core"); // was 430
    fs.global::<glsl::Vec4>("out", "color");
    fs.def_main("color = vec4(0.0, 0.8, 1.0, 1.0);");

    let vs = vs.compile();
    let fs = fs.compile();
    let program = glsl::Program::new(&vs, &fs);

    let mut va = VertexArray::new();
    va.bind();

    win.loop_timeout(10.0, |_, currentTime| {
        let color: [GLfloat, ..4] = [(currentTime.sin() * 0.5 + 0.5) as GLfloat,
                                     (currentTime.cos() * 0.5 + 0.5) as GLfloat,
                                     0.0, 1.0];
        unsafe { gl::ClearBufferfv(gl::COLOR, 0, &color[0]); }
        program.use_program();
        gl::PointSize(10.0);
        gl::DrawArrays(gl::POINTS, 0, 1);
        Redraw
    })
}

fn gl_superbible_2() -> Result<(), ~str> {
    use VSB = self::glsl::VertexShaderBuilder;
    use FSB = self::glsl::FragmentShaderBuilder;

    use glsl::ShaderBuilder;

    let mut win = try!((WindowOpts{ width: 800, height: 800}).init());

    let mut vs : VSB = ShaderBuilder::new("#version 410 core"); // was 430

    // gl_VertexID is an implicit input for index being processed by shader
    vs.def_main("const vec4 vertices[3] =
                           vec4[3](vec4( 0.25, -0.25, 0.5, 1.0),
                                   vec4(-0.25, -0.25, 0.5, 1.0),
                                   vec4( 0.25,  0.25, 0.5, 1.0));
                 gl_Position = vertices[gl_VertexID];");

    let mut fs : FSB = ShaderBuilder::new("#version 410 core"); // was 430
    fs.global::<glsl::Vec4>("out", "color");
    fs.def_main("color = vec4(0.0, 0.8, 1.0, 1.0);");

    let vs = vs.compile();
    let fs = fs.compile();
    let program = glsl::Program::new(&vs, &fs);

    // Even though this is unused, you still need to create and bind
    // it for the program to know to apply the vertex shadrs.
    let mut va = VertexArray::new();
    va.bind();

    win.loop_timeout(10.0, |_, currentTime| {
        let color: [GLfloat, ..4] = [(currentTime.sin() * 0.5 + 0.5) as GLfloat,
                                     (currentTime.cos() * 0.5 + 0.5) as GLfloat,
                                     0.0, 1.0];
        unsafe { gl::ClearBufferfv(gl::COLOR, 0, &color[0]); }
        program.use_program();
        gl::PointSize(10.0);
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
        Redraw
    })
}

fn gl_superbible_3() -> Result<(), ~str> {
    use VSB = self::glsl::VertexShaderBuilder;
    use FSB = self::glsl::FragmentShaderBuilder;

    use glsl::ShaderBuilder;

    let mut win = try!((WindowOpts{ width: 800, height: 800 }).init());

    let mut vs : VSB = ShaderBuilder::new("#version 400");
    vs.global::<glsl::Vec4>("layout (location = 0) in", "offset");
    vs.global::<glsl::Vec4>("layout (location = 1) in", "color");

    #[deriving(Default)]
    #[allow(non_camel_case_types)]
    struct VS_OUT { color: glsl::Vec4 }
    impl glsl::ToGLSLType for VS_OUT {
        fn type_<'a>(&'a self) -> &'a str { "VS_OUT { vec4 color; }" }
    }
    vs.global::<VS_OUT>("out", "vs_out");
    vs.def_main(
        "const vec4 vertices[3] = vec4[3](
            vec4( 0.25, -0.25, 0.5, 1.0),
            vec4(-0.25, -0.25, 0.5, 1.0),
            vec4( 0.25,  0.25, 0.5, 1.0));
         // Add `offset` to our hard coded vertex position
         gl_Position = vertices[gl_VertexID] + offset;
         // Output a fixed value for vs_color
         vs_out.color = color;
    ");

    let mut fs : FSB = ShaderBuilder::new("#version 410 core"); // was 430
    fs.global::<glsl::Vec4>("in", "vs_color");
    fs.global::<VS_OUT>("in", "fs_in");
    fs.global::<glsl::Vec4>("out", "color");
    fs.def_main("color = fs_in.color;"); // vec4(0.0, 0.8, 1.0, 1.0);");

    let vs = vs.compile();
    let fs = fs.compile();
    let program = glsl::Program::new(&vs, &fs);

    // Even though this is unused, you still need to create and bind
    // it for the program to know to apply the vertex shaders.
    let mut va = VertexArray::new();
    va.bind();

    win.loop_timeout(10.0, |_, time| {
        let bg_color : [GLfloat, ..4] = [time.sin() as GLfloat * 0.5 + 0.5,
                                      time.cos() as GLfloat * 0.5 + 0.5,
                                      0.0, 1.0];
        unsafe { gl::ClearBufferfv(gl::COLOR, 0, &bg_color[0]); }
        program.use_program();
        let offset_attrib = [time.sin() as f32 * 0.5,
                             time.cos() as f32 * 0.6,
                             0.0, 0.0];
        unsafe {
            gl::VertexAttrib4fv(0, offset_attrib.as_ptr());
        }
        let color_attrib = offset_attrib;
        unsafe {
            gl::VertexAttrib4fv(1, color_attrib.as_ptr());
        }
        gl::PointSize(10.0);
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
        Redraw
    })
}

fn gl_superbible_4() -> Result<(), ~str> {
    use VSB = self::glsl::VertexShaderBuilder;
    use FSB = self::glsl::FragmentShaderBuilder;
    use TCSB = self::glsl::TessellationControlShaderBuilder;
    use TESB = self::glsl::TessellationEvaluationShaderBuilder;

    use glsl::ShaderBuilder;

    let mut win = try!((WindowOpts{ width: 800, height: 800 }).init());

    let mut vs : VSB = ShaderBuilder::new("#version 400");
    vs.global::<glsl::Vec4>("layout (location = 0) in", "offset");
    vs.global::<glsl::Vec4>("layout (location = 1) in", "color");

    #[deriving(Default)]
    #[allow(non_camel_case_types)]
    struct VS_OUT { color: glsl::Vec4 }
    impl glsl::ToGLSLType for VS_OUT {
        fn type_<'a>(&'a self) -> &'a str { "VS_OUT { vec4 color; }" }
    }
    vs.def_main(
        "const vec4 vertices[3] = vec4[3](
            vec4( 0.25, -0.25, 0.5, 1.0),
            vec4(-0.25, -0.25, 0.5, 1.0),
            vec4( 0.25,  0.25, 0.5, 1.0));
         // Add `offset` to our hard coded vertex position
         gl_Position = vertices[gl_VertexID] + offset;
    ");

    let mut tcs : TCSB = ShaderBuilder::new("#version 410 core"); // was 430 core
    // tcs.global::<VS_OUT>("out", "vs_out");
    tcs.push("layout (vertices = 3) out;");
    tcs.def_main(
        "if (gl_InvocationID == 0) {
            gl_TessLevelInner[0] = 5.0;
            gl_TessLevelOuter[0] = 5.0;
            gl_TessLevelOuter[1] = 5.0;
            gl_TessLevelOuter[2] = 5.0;
        }
        gl_out[gl_InvocationID].gl_Position =
            gl_in[gl_InvocationID].gl_Position;");

    let mut tes : TESB = ShaderBuilder::new("#version 410 core"); // was 430 core
    tes.push("layout (triangles, equal_spacing, cw) in;");
    tes.def_main(
        "gl_Position = (gl_TessCoord.x * gl_in[0].gl_Position +
                        gl_TessCoord.y * gl_in[1].gl_Position +
                        gl_TessCoord.z * gl_in[2].gl_Position);");


    let mut fs : FSB = ShaderBuilder::new("#version 410 core"); // was 430
    fs.global::<glsl::Vec4>("out", "color");
    fs.def_main("color = vec4(0.0, 0.8, 1.0, 1.0);");

    let vs = vs.compile();
    let fs = fs.compile();
    let tcs = tcs.compile();
    let tes = tes.compile();
    let mut pbldr = glsl::ProgramBuilder::new(&vs, &fs);
    pbldr.attach_shader(&tcs);
    pbldr.attach_shader(&tes);
    let program = pbldr.link();
    let program = match program {
        Ok(p) => p,
        Err((_pb, s)) => fail!("link failed {}", s),
    };

    // Even though this is unused, you still need to create and bind
    // it for the program to know to apply the vertex shaders.
    let mut va = VertexArray::new();
    va.bind();

    gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

    win.loop_timeout(10.0, |_, time| {
        let bg_color : [GLfloat, ..4] = [time.sin() as GLfloat * 0.5 + 0.5,
                                      time.cos() as GLfloat * 0.5 + 0.5,
                                      0.0, 1.0];
        unsafe { gl::ClearBufferfv(gl::COLOR, 0, &bg_color[0]); }
        program.use_program();
        let offset_attrib = [time.sin() as f32 * 0.5,
                             time.cos() as f32 * 0.6,
                             0.0, 0.0];
        unsafe {
            gl::VertexAttrib4fv(0, offset_attrib.as_ptr());
        }
        let color_attrib = offset_attrib;
        unsafe {
            gl::VertexAttrib4fv(1, color_attrib.as_ptr());
        }
        // gl::PointSize(10.0);
        gl::DrawArrays(gl::PATCHES, 0, 3);
        Redraw
    })
}

fn glsl_cookbook_3() -> Result<(), ~str> {
    use glsl::ShaderBuilder;
    use glsl::TupleReflect;

    let mut win = try!((WindowOpts{ width: 800, height: 800 }).init());

    // "Compiling a shader"
    let mut vs : glsl::VertexShaderBuilder = ShaderBuilder::new("#version 400");
    vs.global::<glsl::Vec3>("layout (location = 0) in", "VertexPosition");
    vs.global::<glsl::Vec3>("layout (location = 1) in", "VertexColor");
    vs.global::<glsl::Vec2>("layout (location = 2) in", "VertexTexCoord");

    vs.global::<glsl::Vec3>("out", "Color");
    vs.global::<glsl::Vec2>("out", "TexCoord");

    vs.def_main("TexCoord = VertexTexCoord;\
               \nColor = VertexColor;\
               \ngl_Position = vec4( VertexPosition, 1.0 );");

    let vs = vs.compile();

    // "Linking a shader program"
    let mut fs : glsl::FragmentShaderBuilder = ShaderBuilder::new("#version 400");
    fs.global::<glsl::Vec2>("in", "TexCoord");
    fs.global::<glsl::Vec3>("in", "Color");

    // When only one fragment output variable, it is always assigned
    // to data location 0; thus use of `layout (location = ...)` is
    // redundant in that case.  But, more robust to say it explicitly.
    fs.global::<glsl::Vec4>("layout (location = 0) out", "FragColor");

    fs.uniform_block("BlobSettings", [(glsl::vec4, "InnerColor"),
                                      (glsl::vec4, "OuterColor"),
                                      (glsl::float, "RadiusInner"),
                                      (glsl::float, "RadiusOuter")],
                     Some("Blob"));

    fs.def_main("float dx = TexCoord.x - 0.5;\
               \nfloat dy = TexCoord.y - 0.5;\
               \nfloat dist = sqrt(dx * dx + dy * dy);\
               \n//FragColor = mix( Blob.InnerColor, Blob.OuterColor,\
               \n//                  smoothstep( Blob.RadiusInner, Blob.RadiusOuter, dist));\
               \nFragColor = mix( Blob.InnerColor, vec4(Color,1.0),\
               \n                  smoothstep( Blob.RadiusInner, Blob.RadiusOuter, dist));\
               \n//FragColor = vec4(Color, 1.0);\
               \n");

    let fs = fs.compile();

    let program = glsl::ProgramBuilder::new(&vs, &fs);

    let vpos_loc : glsl::AttribLocation<glsl::Vec3> = glsl::AttribLocation {
        name: 0 // implied by `layout (location = 0)`
    };

    let vcol_loc : glsl::AttribLocation<glsl::Vec3> = glsl::AttribLocation {
        name: 1 // implied by `layout (location = 1)`
    };

    let vtex_loc : glsl::AttribLocation<glsl::Vec2> = glsl::AttribLocation {
        name: 2 // implied by `layout (location = 2)`
    };

    let positionData : Vec<f32> = vec!(-0.8, -0.8, 0.0, // lower-left
                                        0.8, -0.8, 0.0, // upper-left
                                       -0.8,  0.8, 0.0, // lower-right
                                        0.8,  0.8, 0.0, // upper-right
                                        0.8, -0.8, 0.0, // (upper-left)
                                       -0.8,  0.8, 0.0); // (lower-right)

    let colorData : Vec<f32> = vec!(1.0, 0.0, 0.0,
                                    0.0, 0.5, 0.0,
                                    0.0, 0.0, 0.5,
                                    0.0, 0.0, 0.0,
                                    0.0, 0.5, 0.0,
                                    0.0, 0.0, 0.5);

    let textureData : Vec<f32> = vec!(1.0, 1.0,
                                      1.0, 0.0,
                                      0.0, 1.0,
                                      1.0, 1.0,
                                      1.0, 0.0,
                                      0.0, 1.0);

    let mut vbos = VertexBuffers::new(3);
    vbos.bind_and_init_array(0, positionData.slice_from(0), StaticDraw);
    vbos.bind_and_init_array(1, colorData.slice_from(0), StaticDraw);
    vbos.bind_and_init_array(2, textureData.slice_from(0), StaticDraw);

    let mut vba = VertexArray::new();
    vba.bind();
    vpos_loc.enable_vertex_attrib_array();
    vcol_loc.enable_vertex_attrib_array();
    vtex_loc.enable_vertex_attrib_array();

    vbos.bind_array(0);
    unsafe {
        vpos_loc.vertex_attrib_pointer(gl::FALSE, glsl::Packed);
    }

    vbos.bind_array(1);
    unsafe {
        vcol_loc.vertex_attrib_pointer(gl::FALSE, glsl::Packed);
    }

    vbos.bind_array(2);
    unsafe {
        vtex_loc.vertex_attrib_pointer(gl::FALSE, glsl::Packed);
    }

    let program = program.link();
    let program = match program {
        Ok(p) => p,
        Err((_builder, m)) => {
            // In principle one could attempt to recover by modifying
            // the builder in some respect.  In principle... but we
            // just fail for now.
            fail!(m)
        }
    };

    // "Using uniform blocks and uniform block objects"
    let blockIndex = unsafe { "BlobSettings".with_c_str(|p|gl::GetUniformBlockIndex(program.name, p)) };
    let mut blockSize : GLint = 0;
    unsafe {
        gl::GetActiveUniformBlockiv(program.name, blockIndex, gl::UNIFORM_BLOCK_DATA_SIZE, &mut blockSize);
    }
    println!("blockIndex: {} blockSize: {}", blockIndex, blockSize);
    assert!(blockSize > 0);
    let mut blockBuffer = Vec::from_elem(blockSize as uint, 0 as GLubyte);
    let names = ["BlobSettings.InnerColor", "BlobSettings.OuterColor", "BlobSettings.RadiusInner", "BlobSettings.RadiusOuter"];
    let mut indices = [0 as GLuint, ..4];
    {
        let strs : Vec<c_str::CString> = names.iter().map(|s|s.to_c_str()).collect();
        unsafe { gl::GetUniformIndices(program.name, indices.len() as GLint, strs.iter().map(|cs| {
            cs.as_bytes().as_ptr() as *GLchar}).collect::<Vec<_>>().as_ptr(), indices.as_mut_ptr()); }
    }
    let mut offset = [0 as GLint, ..4];
    unsafe { gl::GetActiveUniformsiv(program.name, offset.len() as GLint, indices.as_ptr(),
                                     gl::UNIFORM_OFFSET, offset.as_mut_ptr()); }

    let outerColor : [GLfloat, ..4] = [0.0, ..4];
    let innerColor : [GLfloat, ..4] = [1.0, 1.0, 0.75, 1.0];
    let innerRadius : GLfloat = 0.25;
    let outerRadius : GLfloat = 0.45;

    unsafe {
        let blockBuffer = blockBuffer.as_mut_ptr() as uint;
        println!("blockIndex: {} blockSize: {} blockBuffer: {:?} offsets: {:?}", blockIndex, blockSize, blockBuffer, offset);
        mem::move_val_init::<[GLfloat, ..4]>(cast::transmute(blockBuffer + offset[0] as uint), innerColor);
        mem::move_val_init::<[GLfloat, ..4]>(cast::transmute(blockBuffer + offset[1] as uint), outerColor);
        mem::move_val_init::<GLfloat>(cast::transmute(blockBuffer + offset[2] as uint), innerRadius);
        mem::move_val_init::<GLfloat>(cast::transmute(blockBuffer + offset[3] as uint), outerRadius);
    }

    let mut uboHandle : GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut uboHandle);
        gl::BindBuffer( gl::UNIFORM_BUFFER, uboHandle );
        gl::BufferData( gl::UNIFORM_BUFFER, blockSize as GLsizeiptr, blockBuffer.as_mut_ptr() as *GLvoid, gl::DYNAMIC_DRAW );
    }

    gl::BindBufferBase( gl::UNIFORM_BUFFER, blockIndex, uboHandle );

    // "Getting a list of active vertex input attributes and indices"
    let attribs = program.active_attribs();
    println!("Active Attributes");
    println!("Index | Name");
    println!("------------------------------------------------");
    for &(ref _count, ref _type, ref name) in attribs.iter() {
        let loc = unsafe { program.raw_attrib_location(name.as_slice()) };
        println!("{:-5d} | {}", loc, name);
    }

    // "Getting a list of active uniform variables"
    println!("Active Uniform Variables");
    let uniforms = program.active_uniforms();
    println!("Location | Name");
    println!("------------------------------------------------");
    for &(ref _count, ref _type, ref name) in uniforms.iter() {
        let loc = unsafe { program.raw_uniform_location(name.as_slice()) };
        println!("{:-8d} | {}\n", loc, name);
    }

    program.use_program();

    win.loop_timeout(10.0, |_, _time| {
        vba.bind();

        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::DrawArrays(gl::TRIANGLES, 0, 6);

        Redraw
    })
}

fn glsl_cookbook_2() -> Result<(), ~str> {
    use glsl::ShaderBuilder;
    use glsl::TupleReflect;

    let mut win = try!((WindowOpts{ width: 800, height: 800 }).init());

    // "Compiling a shader"
    let mut vs : glsl::VertexShaderBuilder = ShaderBuilder::new("#version 400");
    vs.global::<glsl::Vec3>("layout (location = 0) in", "VertexPosition");
    vs.global::<glsl::Vec3>("layout (location = 1) in", "VertexColor");
    vs.global::<glsl::Vec2>("layout (location = 2) in", "VertexTexCoord");

    vs.global::<glsl::Vec3>("out", "Color");
    vs.global::<glsl::Vec2>("out", "TexCoord");

    vs.def_main("TexCoord = VertexTexCoord;\
               \nColor = VertexColor;\
               \ngl_Position = vec4( VertexPosition, 1.0 );");

    let vs = vs.compile();

    // "Linking a shader program"
    let mut fs : glsl::FragmentShaderBuilder = ShaderBuilder::new("#version 400");
    fs.global::<glsl::Vec2>("in", "TexCoord");
    fs.global::<glsl::Vec3>("in", "Color");

    // When only one fragment output variable, it is always assigned
    // to data location 0; thus use of `layout (location = ...)` is
    // redundant in that case.  But, more robust to say it explicitly.
    fs.global::<glsl::Vec4>("layout (location = 0) out", "FragColor");

    fs.uniform_block("BlobSettings", [(glsl::vec4, "InnerColor"),
                                      (glsl::vec4, "OuterColor"),
                                      (glsl::float, "RadiusInner"),
                                      (glsl::float, "RadiusOuter")],
                     Some("Blob"));

    fs.def_main("float dx = TexCoord.x - 0.5;\
               \nfloat dy = TexCoord.y - 0.5;\
               \nfloat dist = sqrt(dx * dx + dy * dy);\
               \n//FragColor = mix( Blob.InnerColor, Blob.OuterColor,\
               \n//                  smoothstep( Blob.RadiusInner, Blob.RadiusOuter, dist));\
               \nFragColor = mix( Blob.InnerColor, vec4(Color,1.0),\
               \n                  smoothstep( Blob.RadiusInner, Blob.RadiusOuter, dist));\
               \n//FragColor = vec4(Color, 1.0);\
               \n");

    let fs = fs.compile();

    let program = glsl::ProgramBuilder::new(&vs, &fs);

    let vpos_loc : glsl::AttribLocation<glsl::Vec3> = glsl::AttribLocation {
        name: 0 // implied by `layout (location = 0)`
    };

    let vcol_loc : glsl::AttribLocation<glsl::Vec3> = glsl::AttribLocation {
        name: 1 // implied by `layout (location = 1)`
    };

    let vtex_loc : glsl::AttribLocation<glsl::Vec2> = glsl::AttribLocation {
        name: 2 // implied by `layout (location = 2)`
    };

    let positionData : Vec<f32> = vec!(-0.8, -0.8, 0.0, // lower-left
                                        0.8, -0.8, 0.0, // upper-left
                                       -0.8,  0.8, 0.0, // lower-right
                                        0.8,  0.8, 0.0, // upper-right
                                        0.8, -0.8, 0.0, // (upper-left)
                                       -0.8,  0.8, 0.0); // (lower-right)

    let colorData : Vec<f32> = vec!(1.0, 0.0, 0.0,
                                    0.0, 0.5, 0.0,
                                    0.0, 0.0, 0.5,
                                    0.0, 0.0, 0.0,
                                    0.0, 0.5, 0.0,
                                    0.0, 0.0, 0.5);

    let textureData : Vec<f32> = vec!(1.0, 1.0,
                                      1.0, 0.0,
                                      0.0, 1.0,
                                      1.0, 1.0,
                                      1.0, 0.0,
                                      0.0, 1.0);

    let mut vbos = VertexBuffers::new(3);
    vbos.bind_and_init_array(0, positionData.slice_from(0), StaticDraw);
    vbos.bind_and_init_array(1, colorData.slice_from(0), StaticDraw);
    vbos.bind_and_init_array(2, textureData.slice_from(0), StaticDraw);

    let mut vba = VertexArray::new();
    vba.bind();
    vpos_loc.enable_vertex_attrib_array();
    vcol_loc.enable_vertex_attrib_array();
    vtex_loc.enable_vertex_attrib_array();

    vbos.bind_array(0);
    unsafe {
        vpos_loc.vertex_attrib_pointer(gl::FALSE, glsl::Packed);
    }

    vbos.bind_array(1);
    unsafe {
        vcol_loc.vertex_attrib_pointer(gl::FALSE, glsl::Packed);
    }

    vbos.bind_array(2);
    unsafe {
        vtex_loc.vertex_attrib_pointer(gl::FALSE, glsl::Packed);
    }

    let program = program.link();
    let program = match program {
        Ok(p) => p,
        Err((_builder, m)) => {
            // In principle one could attempt to recover by modifying
            // the builder in some respect.  In principle... but we
            // just fail for now.
            fail!(m)
        }
    };

    // "Using uniform blocks and uniform block objects"
    let blockIndex = unsafe { "BlobSettings".with_c_str(|p|gl::GetUniformBlockIndex(program.name, p)) };
    let mut blockSize : GLint = 0;
    unsafe {
        gl::GetActiveUniformBlockiv(program.name, blockIndex, gl::UNIFORM_BLOCK_DATA_SIZE, &mut blockSize);
    }
    println!("blockIndex: {} blockSize: {}", blockIndex, blockSize);
    assert!(blockSize > 0);
    let mut blockBuffer = Vec::from_elem(blockSize as uint, 0 as GLubyte);
    let names = ["BlobSettings.InnerColor", "BlobSettings.OuterColor", "BlobSettings.RadiusInner", "BlobSettings.RadiusOuter"];
    let mut indices = [0 as GLuint, ..4];
    {
        let strs : Vec<c_str::CString> = names.iter().map(|s|s.to_c_str()).collect();
        unsafe { gl::GetUniformIndices(program.name, indices.len() as GLint, strs.iter().map(|cs| {
            cs.as_bytes().as_ptr() as *GLchar}).collect::<Vec<_>>().as_ptr(), indices.as_mut_ptr()); }
    }
    let mut offset = [0 as GLint, ..4];
    unsafe { gl::GetActiveUniformsiv(program.name, offset.len() as GLint, indices.as_ptr(),
                                     gl::UNIFORM_OFFSET, offset.as_mut_ptr()); }

    let outerColor : [GLfloat, ..4] = [0.0, ..4];
    let innerColor : [GLfloat, ..4] = [1.0, 1.0, 0.75, 1.0];
    let innerRadius : GLfloat = 0.25;
    let outerRadius : GLfloat = 0.45;

    unsafe {
        let blockBuffer = blockBuffer.as_mut_ptr() as uint;
        println!("blockIndex: {} blockSize: {} blockBuffer: {:?} offsets: {:?}", blockIndex, blockSize, blockBuffer, offset);
        mem::move_val_init::<[GLfloat, ..4]>(cast::transmute(blockBuffer + offset[0] as uint), innerColor);
        mem::move_val_init::<[GLfloat, ..4]>(cast::transmute(blockBuffer + offset[1] as uint), outerColor);
        mem::move_val_init::<GLfloat>(cast::transmute(blockBuffer + offset[2] as uint), innerRadius);
        mem::move_val_init::<GLfloat>(cast::transmute(blockBuffer + offset[3] as uint), outerRadius);
    }

    let mut uboHandle : GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut uboHandle);
        gl::BindBuffer( gl::UNIFORM_BUFFER, uboHandle );
        gl::BufferData( gl::UNIFORM_BUFFER, blockSize as GLsizeiptr, blockBuffer.as_mut_ptr() as *GLvoid, gl::DYNAMIC_DRAW );
    }

    gl::BindBufferBase( gl::UNIFORM_BUFFER, blockIndex, uboHandle );

    // "Getting a list of active vertex input attributes and indices"
    let attribs = program.active_attribs();
    println!("Active Attributes");
    println!("Index | Name");
    println!("------------------------------------------------");
    for &(ref _count, ref _type, ref name) in attribs.iter() {
        let loc = unsafe { program.raw_attrib_location(name.as_slice()) };
        println!("{:-5d} | {}", loc, name);
    }

    // "Getting a list of active uniform variables"
    println!("Active Uniform Variables");
    let uniforms = program.active_uniforms();
    println!("Location | Name");
    println!("------------------------------------------------");
    for &(ref _count, ref _type, ref name) in uniforms.iter() {
        let loc = unsafe { program.raw_uniform_location(name.as_slice()) };
        println!("{:-8d} | {}\n", loc, name);
    }

    program.use_program();

    win.loop_timeout(10.0, |_, _time| {
        vba.bind();

        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::DrawArrays(gl::TRIANGLES, 0, 6);
        Redraw
    })
}

fn glsl_cookbook_1() -> Result<(), ~str> {
    use glsl::ShaderBuilder;
    use glsl::TupleReflect;

    let mut win = try!((WindowOpts{ width: 800, height: 600 }).init());

    // "Compiling a shader"
    let mut vs : glsl::VertexShaderBuilder = ShaderBuilder::new("#version 400");
    vs.global::<glsl::Vec3>("layout (location = 0) in", "VertexPosition");
    vs.global::<glsl::Vec3>("layout (location = 1) in", "VertexColor");

    vs.global::<glsl::Vec3>("out", "LightIntensity");
    vs.global::<glsl::Vec3>("out", "Color");

    let rot_g = vs.global::<glsl::Mat4>("uniform", "RotationMatrix");

    vs.def_main("Color = VertexColor;
                 gl_Position = RotationMatrix * vec4( VertexPosition, 1.0 );");

    let vs = vs.compile();

    // "Linking a shader program"
    let mut fs : glsl::FragmentShaderBuilder = ShaderBuilder::new("#version 400");
    fs.global::<glsl::Vec3>("in", "Color");

    // When only one fragment output variable, it is always assigned
    // to data location 0; thus use of `layout (location = ...)` is
    // redundant in that case.  But, more robust to say it explicitly.
    fs.global::<glsl::Vec4>("layout (location = 0) out", "FragColor");

    fs.def_main("FragColor = vec4(Color, 1.0);");

    let fs = fs.compile();

    let program = glsl::ProgramBuilder::new(&vs, &fs);

    // "Sending data to a shader using per-vertex attributes and vertex buffer objects"

    let vpos_loc : glsl::AttribLocation<glsl::Vec3> = glsl::AttribLocation {
        name: 0 // implied by `layout (location = 0)`
    };

    let vcol_loc : glsl::AttribLocation<glsl::Vec3> = glsl::AttribLocation {
        name: 1 // implied by `layout (location = 1)`
    };

    let positionData : Vec<f32> = vec!(-0.8, -0.8, 0.0,
                                        0.8, -0.8, 0.0,
                                        0.0,  0.8, 0.0);
    let colorData : Vec<f32> = vec!(1.0, 0.0, 0.0,
                                    0.0, 1.0, 0.0,
                                    0.0, 0.0, 1.0);

    let mut vbos = VertexBuffers::new(2);
    vbos.bind_and_init_array(0, positionData.slice_from(0), StaticDraw);
    vbos.bind_and_init_array(1, colorData.slice_from(0), StaticDraw);

    let mut vba = VertexArray::new();
    vba.bind();
    vpos_loc.enable_vertex_attrib_array();
    vcol_loc.enable_vertex_attrib_array();

    vbos.bind_array(0);
    unsafe {
        vpos_loc.vertex_attrib_pointer(gl::FALSE, glsl::Packed);
    }

    vbos.bind_array(1);
    unsafe {
        vcol_loc.vertex_attrib_pointer(gl::FALSE, glsl::Packed);
    }

    let program = program.link().unwrap();

    // "Getting a list of active vertex input attributes and indices"
    let attribs = program.active_attribs();
    println!("Active Attributes");
    println!("Index | Name");
    println!("------------------------------------------------");
    for &(ref _count, ref _type, ref name) in attribs.iter() {
        let loc = unsafe { program.raw_attrib_location(name.as_slice()) };
        println!("{:-5d} | {}", loc, name);
    }

    // "Getting a list of active uniform variables"
    println!("Active Uniform Variables");
    let uniforms = program.active_uniforms();
    println!("Location | Name");
    println!("------------------------------------------------");
    for &(ref _count, ref _type, ref name) in uniforms.iter() {
        let loc = unsafe { program.raw_uniform_location(name.as_slice()) };
        println!("{:-8d} | {}\n", loc, name);
    }

    program.use_program();

    win.loop_timeout(5.0, |_, time| {
        vba.bind();

        gl::Clear(gl::COLOR_BUFFER_BIT);

        let rot = &mat::Matrix3::from_angle_z(ang::deg(time as f32 * 180.0f32)
                                              .to_rad())
            .to_matrix4();

        unsafe {
            program.set_uniform(&rot_g, rot);
        }

        gl::DrawArrays(gl::TRIANGLES, 0, 3);
        Redraw
    })
}

fn gl() -> Result<(), ~str> {
    use glsl::ShaderBuilder;
    use glsl::TupleReflect;

    let mut win = try!((WindowOpts{ width: 800, height: 600 }).init());

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

    vs.def_main("
        v2f_color = color;
        v2f_texcoord = texcoord;
        gl_Position = model * vec4(position, 0.0, 1.0);");

    let mut fs : glsl::FragmentShaderBuilder = ShaderBuilder::new_150core();
    fs.in_global("", &v2f_color);
    fs.in_global("", &v2f_texcoord);
    fs.global::<glsl::Vec4>("out", "out_color");
    let tex_kitten_g = fs.global::<glsl::Sampler2D>("uniform", "texKitten");
    let tex_puppy_g  = fs.global::<glsl::Sampler2D>("uniform", "texPuppy");
    fs.def_main("
        vec4 colKitten = texture(texKitten, v2f_texcoord);
        vec4 colPuppy  = texture(texPuppy, v2f_texcoord);
        out_color = mix(colKitten, colPuppy.rgba, 0.5) * vec4(v2f_color, 1.0);
        // out_color = colKitten * vec4(v2f_color, 1.0);
        // out_color = mix(colKitten, colPuppy, 0.5);"
              );

    // Create GLSL shaders
    let vs = vs.compile();
    let fs = fs.compile();
    let program1 = glsl::Program::new(&vs, &fs);

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

        // Specify the layout of the vertex data
        let default : VertexDataRow = Default::default();

        let pos_attr = program1.attrib_location(&position_g);
        pos_attr.enable_vertex_attrib_array();
        pos_attr.vertex_attrib_pointer(gl::FALSE as GLboolean, (&default.xy, &default));

        let col_attr = program1.attrib_location(&color_g);
        col_attr.enable_vertex_attrib_array();
        col_attr.vertex_attrib_pointer(gl::FALSE as GLboolean, (&default.rgb, &default));

        let tex_attr = program1.attrib_location(&texcoord_g);
        tex_attr.enable_vertex_attrib_array();
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

    win.loop_timeout(5.0, |_, time| {
        // Use a uniform red
        // gl::Uniform3f(uni_color, 1.0, 0.0, 0.0);

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

        Redraw
    })
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

            println!("source for non-compiling shader");
            for &s in src.iter() { println!("{:s}", s); }

            fail!("{}", str::from_utf8_owned(buf.move_iter().collect()));
        }
    }
    shader
}

fn try_link_program(program: GLuint) -> Result<(), ~str> {
    // let program = gl::CreateProgram();
    // gl::AttachShader(program, vs);
    // gl::AttachShader(program, fs);
    gl::LinkProgram(program);
    unsafe {
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::from_elem(len as uint - 1, 0u8);     // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(program, len, ptr::mut_null(), buf.as_mut_ptr() as *mut GLchar);
            let msg = str::from_utf8_owned(buf.move_iter().collect());
            match msg {
                Some(msg) => return Err(msg),
                None => return Err(~"link failure; \
                                     graphics driver provided malformed log"),
            }
        } else {
            return Ok(())
        }
    }
}

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
