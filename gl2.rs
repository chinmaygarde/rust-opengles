/* automatically generated by rust-bindgen */

use core::libc::*;
use core::libc::types::common::c99::*;
use core::cast::{reinterpret_cast, transmute};
use core::ptr::to_unsafe_ptr;
use core::str::{as_c_str, from_bytes};
use core::sys::size_of;
use core::vec::from_elem;
use core::vec::raw::to_ptr;

// Linking
#[nolink]
#[link_args="-framework OpenGL"]
#[cfg(target_os = "macos")]
extern mod linkhack { }

#[nolink]
#[link_args="-lGL"]
#[cfg(target_os = "linux")]
extern mod linkhack { }

// Constants

/* BeginMode */
pub static POINTS:         c_uint = 0x0000 as c_uint;
pub static LINES:          c_uint = 0x0001 as c_uint;
pub static LINE_LOOP:      c_uint = 0x0002 as c_uint;
pub static LINE_STRIP:     c_uint = 0x0003 as c_uint;
pub static TRIANGLES:      c_uint = 0x0004 as c_uint;
pub static TRIANGLE_STRIP: c_uint = 0x0005 as c_uint;
pub static TRIANGLE_FAN:   c_uint = 0x0006 as c_uint;

pub static DEPTH_BUFFER_BIT:   c_uint = 0x00000100 as c_uint;
pub static STENCIL_BUFFER_BIT: c_uint = 0x00000400 as c_uint;
pub static COLOR_BUFFER_BIT:   c_uint = 0x00004000 as c_uint;

/* Errors. */
pub static NO_ERROR: c_uint = 0 as c_uint;
pub static INVALID_ENUM: c_uint = 0x0500 as c_uint;
pub static INVALID_VALUE: c_uint = 0x0501 as c_uint;
pub static INVALID_OPERATION: c_uint = 0x0502 as c_uint;
pub static STACK_OVERFLOW: c_uint = 0x0503 as c_uint;
pub static STACK_UNDERFLOW: c_uint = 0x0504 as c_uint;
pub static OUT_OF_MEMORY: c_uint = 0x0505 as c_uint;
pub static INVALID_FRAMEBUFFER_OPERATION: c_uint = 0x0506 as c_uint;

/* DataType */
pub static BYTE:           c_uint = 0x1400 as c_uint;
pub static UNSIGNED_BYTE:  c_uint = 0x1401 as c_uint;
pub static SHORT:          c_uint = 0x1402 as c_uint;
pub static UNSIGNED_SHORT: c_uint = 0x1403 as c_uint;
pub static INT:            c_uint = 0x1404 as c_uint;
pub static UNSIGNED_INT:   c_uint = 0x1405 as c_uint;
pub static FLOAT:          c_uint = 0x1406 as c_uint;
pub static FIXED:          c_uint = 0x140C as c_uint;

/* EnableCap */
pub static TEXTURE_2D:               c_uint = 0x0DE1 as c_uint;
pub static CULL_FACE:                c_uint = 0x0B44 as c_uint;
pub static BLEND:                    c_uint = 0x0BE2 as c_uint;
pub static DITHER:                   c_uint = 0x0BD0 as c_uint;
pub static STENCIL_TEST:             c_uint = 0x0B90 as c_uint;
pub static DEPTH_TEST:               c_uint = 0x0B71 as c_uint;
pub static SCISSOR_TEST:             c_uint = 0x0C11 as c_uint;
pub static POLYGON_OFFSET_FILL:      c_uint = 0x8037 as c_uint;
pub static SAMPLE_ALPHA_TO_COVERAGE: c_uint = 0x809E as c_uint;
pub static SAMPLE_COVERAGE:          c_uint = 0x80A0 as c_uint;

/* FrontFaceDirection */
pub static CW:  c_uint = 0x0900 as c_uint;
pub static CCW: c_uint = 0x0901 as c_uint;

/* GetPName */
pub static LINE_WIDTH:                    c_uint = 0x0B21 as c_uint;
pub static ALIASED_POINT_SIZE_RANGE:      c_uint = 0x846D as c_uint;
pub static ALIASED_LINE_WIDTH_RANGE:      c_uint = 0x846E as c_uint;
pub static CULL_FACE_MODE:                c_uint = 0x0B45 as c_uint;
pub static FRONT_FACE:                    c_uint = 0x0B46 as c_uint;
pub static DEPTH_RANGE:                   c_uint = 0x0B70 as c_uint;
pub static DEPTH_WRITEMASK:               c_uint = 0x0B72 as c_uint;
pub static DEPTH_CLEAR_VALUE:             c_uint = 0x0B73 as c_uint;
pub static DEPTH_FUNC:                    c_uint = 0x0B74 as c_uint;
pub static STENCIL_CLEAR_VALUE:           c_uint = 0x0B91 as c_uint;
pub static STENCIL_FUNC:                  c_uint = 0x0B92 as c_uint;
pub static STENCIL_FAIL:                  c_uint = 0x0B94 as c_uint;
pub static STENCIL_PASS_DEPTH_FAIL:       c_uint = 0x0B95 as c_uint;
pub static STENCIL_PASS_DEPTH_PASS:       c_uint = 0x0B96 as c_uint;
pub static STENCIL_REF:                   c_uint = 0x0B97 as c_uint;
pub static STENCIL_VALUE_MASK:            c_uint = 0x0B93 as c_uint;
pub static STENCIL_WRITEMASK:             c_uint = 0x0B98 as c_uint;
pub static STENCIL_BACK_FUNC:             c_uint = 0x8800 as c_uint;
pub static STENCIL_BACK_FAIL:             c_uint = 0x8801 as c_uint;
pub static STENCIL_BACK_PASS_DEPTH_FAIL:  c_uint = 0x8802 as c_uint;
pub static STENCIL_BACK_PASS_DEPTH_PASS:  c_uint = 0x8803 as c_uint;
pub static STENCIL_BACK_REF:              c_uint = 0x8CA3 as c_uint;
pub static STENCIL_BACK_VALUE_MASK:       c_uint = 0x8CA4 as c_uint;
pub static STENCIL_BACK_WRITEMASK:        c_uint = 0x8CA5 as c_uint;
pub static VIEWPORT:                      c_uint = 0x0BA2 as c_uint;
pub static SCISSOR_BOX:                   c_uint = 0x0C10 as c_uint;
/*      SCISSOR_TEST */
pub static COLOR_CLEAR_VALUE:             c_uint = 0x0C22 as c_uint;
pub static COLOR_WRITEMASK:               c_uint = 0x0C23 as c_uint;
pub static UNPACK_ALIGNMENT:              c_uint = 0x0CF5 as c_uint;
pub static PACK_ALIGNMENT:                c_uint = 0x0D05 as c_uint;
pub static MAX_TEXTURE_SIZE:              c_uint = 0x0D33 as c_uint;
pub static MAX_VIEWPORT_DIMS:             c_uint = 0x0D3A as c_uint;
pub static SUBPIXEL_BITS:                 c_uint = 0x0D50 as c_uint;
pub static RED_BITS:                      c_uint = 0x0D52 as c_uint;
pub static GREEN_BITS:                    c_uint = 0x0D53 as c_uint;
pub static BLUE_BITS:                     c_uint = 0x0D54 as c_uint;
pub static ALPHA_BITS:                    c_uint = 0x0D55 as c_uint;
pub static DEPTH_BITS:                    c_uint = 0x0D56 as c_uint;
pub static STENCIL_BITS:                  c_uint = 0x0D57 as c_uint;
pub static POLYGON_OFFSET_UNITS:          c_uint = 0x2A00 as c_uint;
/*      POLYGON_OFFSET_FILL */
pub static POLYGON_OFFSET_FACTOR:         c_uint = 0x8038 as c_uint;
pub static TEXTURE_BINDING_2D:            c_uint = 0x8069 as c_uint;
pub static SAMPLE_BUFFERS:                c_uint = 0x80A8 as c_uint;
pub static SAMPLES:                       c_uint = 0x80A9 as c_uint;
pub static SAMPLE_COVERAGE_VALUE:         c_uint = 0x80AA as c_uint;
pub static SAMPLE_COVERAGE_INVERT:        c_uint = 0x80AB as c_uint;

/* GetTarget */
pub static UNPACK_ROW_LENGTH: c_uint = 0x0CF2 as c_uint;

/* PixelFormat */
pub static DEPTH_COMPONENT: c_uint = 0x1902 as c_uint;
pub static ALPHA:           c_uint = 0x1906 as c_uint;
pub static RGB:             c_uint = 0x1907 as c_uint;
pub static RGBA:            c_uint = 0x1908 as c_uint;

pub static BGRA:            c_uint = 0x80e1 as c_uint;   // NB: Not OpenGL ES!
pub static RGBA8:           c_uint = 0x8058 as c_uint;   // NB: Not OpenGL ES!

/* Packed Pixels */
pub static UNSIGNED_INT_8_8_8_8_REV: c_uint = 0x8367 as c_uint; // NB: Not OpenGL ES!

/* Shaders */
pub static FRAGMENT_SHADER:                  c_uint = 0x8B30 as c_uint;
pub static VERTEX_SHADER:                    c_uint = 0x8B31 as c_uint;
pub static MAX_VERTEX_ATTRIBS:               c_uint = 0x8869 as c_uint;
pub static MAX_VERTEX_UNIFORM_VECTORS:       c_uint = 0x8DFB as c_uint;
pub static MAX_VARYING_VECTORS:              c_uint = 0x8DFC as c_uint;
pub static MAX_COMBINED_TEXTURE_IMAGE_UNITS: c_uint = 0x8B4D as c_uint;
pub static MAX_VERTEX_TEXTURE_IMAGE_UNITS:   c_uint = 0x8B4C as c_uint;
pub static MAX_TEXTURE_IMAGE_UNITS:          c_uint = 0x8872 as c_uint;
pub static MAX_FRAGMENT_UNIFORM_VECTORS:     c_uint = 0x8DFD as c_uint;
pub static SHADER_TYPE:                      c_uint = 0x8B4F as c_uint;
pub static DELETE_STATUS:                    c_uint = 0x8B80 as c_uint;
pub static LINK_STATUS:                      c_uint = 0x8B82 as c_uint;
pub static VALIDATE_STATUS:                  c_uint = 0x8B83 as c_uint;
pub static ATTACHED_SHADERS:                 c_uint = 0x8B85 as c_uint;
pub static ACTIVE_UNIFORMS:                  c_uint = 0x8B86 as c_uint;
pub static ACTIVE_UNIFORM_MAX_LENGTH:        c_uint = 0x8B87 as c_uint;
pub static ACTIVE_ATTRIBUTES:                c_uint = 0x8B89 as c_uint;
pub static ACTIVE_ATTRIBUTE_MAX_LENGTH:      c_uint = 0x8B8A as c_uint;
pub static SHADING_LANGUAGE_VERSION:         c_uint = 0x8B8C as c_uint;
pub static CURRENT_PROGRAM:                  c_uint = 0x8B8D as c_uint;

/* StencilFunction */
pub static NEVER:    c_uint = 0x0200 as c_uint;
pub static LESS:     c_uint = 0x0201 as c_uint;
pub static EQUAL:    c_uint = 0x0202 as c_uint;
pub static LEQUAL:   c_uint = 0x0203 as c_uint;
pub static GREATER:  c_uint = 0x0204 as c_uint;
pub static NOTEQUAL: c_uint = 0x0205 as c_uint;
pub static GEQUAL:   c_uint = 0x0206 as c_uint;
pub static ALWAYS:   c_uint = 0x0207 as c_uint;

/* Shader Source */
pub static COMPILE_STATUS:       c_uint = 0x8B81 as c_uint;
pub static INFO_LOG_LENGTH:      c_uint = 0x8B84 as c_uint;
pub static SHADER_SOURCE_LENGTH: c_uint = 0x8B88 as c_uint;
pub static SHADER_COMPILER:      c_uint = 0x8DFA as c_uint;

/* Buffer Objects */
pub static ARRAY_BUFFER:                 c_uint = 0x8892 as c_uint;
pub static ELEMENT_ARRAY_BUFFER:         c_uint = 0x8893 as c_uint;
pub static ARRAY_BUFFER_BINDING:         c_uint = 0x8894 as c_uint;
pub static ELEMENT_ARRAY_BUFFER_BINDING: c_uint = 0x8895 as c_uint;

pub static STREAM_DRAW:  c_uint = 0x88E0 as c_uint;
pub static STATIC_DRAW:  c_uint = 0x88E4 as c_uint;
pub static DYNAMIC_DRAW: c_uint = 0x88E8 as c_uint;

/* CullFaceMode */
pub static FRONT: c_uint =           0x0404 as c_uint;
pub static BACK: c_uint =            0x0405 as c_uint;
pub static FRONT_AND_BACK: c_uint =  0x0408 as c_uint;

/* TextureMagFilter */
pub static NEAREST: c_uint = 0x2600 as c_uint;
pub static LINEAR:  c_uint = 0x2601 as c_uint;

/* TextureParameterName */
pub static TEXTURE_MAG_FILTER: c_uint = 0x2800 as c_uint;
pub static TEXTURE_MIN_FILTER: c_uint = 0x2801 as c_uint;
pub static TEXTURE_WRAP_S:     c_uint = 0x2802 as c_uint;
pub static TEXTURE_WRAP_T:     c_uint = 0x2803 as c_uint;

/* TextureWrapMode */
pub static REPEAT:          c_uint = 0x2901 as c_uint;
pub static CLAMP_TO_EDGE:   c_uint = 0x812F as c_uint;
pub static MIRRORED_REPEAT: c_uint = 0x8370 as c_uint;

pub static COLOR_ATTACHMENT0: c_uint = 0x8CE0 as c_uint;

pub static FRAMEBUFFER_COMPLETE: c_uint = 0x8CD5 as c_uint;

// Framebuffer Object
pub static FRAMEBUFFER:  c_uint = 0x8D40 as c_uint;
pub static RENDERBUFFER: c_uint = 0x8D41 as c_uint;

// Extensions
pub static TEXTURE_RECTANGLE_ARB: c_uint = 0x84F5 as c_uint;         // NB: Not OpenGL ES!

pub static UNPACK_CLIENT_STORAGE_APPLE: c_uint = 0x85B2 as c_uint;   // NB: Not OpenGL ES!
pub static TEXTURE_STORAGE_HINT_APPLE: c_uint = 0x85BC as c_uint;    // NB: Not OpenGL ES!
pub static STORAGE_CACHED_APPLE: c_uint = 0x85BE as c_uint;          // NB: Not OpenGL ES!
pub static STORAGE_SHARED_APPLE: c_uint = 0x85BF as c_uint;          // NB: Not OpenGL ES!


// Types

pub type GLvoid = c_void /* unknown kind Void */;

pub type GLchar = c_char;

pub type GLenum = c_uint;

pub type GLboolean = c_uchar;

pub type GLbitfield = c_uint;

pub type GLbyte = int8_t;

pub type GLshort = c_short;

pub type GLint = c_int;

pub type GLsizei = c_int;

pub type GLubyte = uint8_t;

pub type GLushort = c_ushort;

pub type GLuint = c_uint;

pub type GLfloat = f32;

pub type GLclampf = f32;

pub type GLfixed = int32_t;

pub type GLintptr = intptr_t;

pub type GLsizeiptr = ssize_t;


// Helper functions

pub fn destroy<T>(_x: T) {
    // Just let the object drop.
}


// Exposed Rust API using Rust naming conventions

pub fn attach_shader(program: GLuint, shader: GLuint) {
    unsafe {
        ll::glAttachShader(program, shader);
    }
}

pub fn bind_buffer(target: GLenum, buffer: GLuint) {
    unsafe {
        ll::glBindBuffer(target, buffer);
    }
}

pub fn bind_framebuffer(target: GLenum, framebuffer: GLuint) {
    unsafe {
        ll::glBindFramebuffer(target, framebuffer);
    }
}

pub fn bind_texture(target: GLenum, texture: GLuint) {
    unsafe {
        ll::glBindTexture(target, texture);
    }
}

// FIXME: There should be some type-safe wrapper for this...
pub fn buffer_data<T>(target: GLenum, data: &[T], usage: GLenum) {
    unsafe {
        ll::glBufferData(target,
                         (data.len() * size_of::<T>()) as GLsizeiptr,
                         to_ptr(data) as *GLvoid,
                         usage);
    }
}

// FIXME: As above
// Note: offset is the element offset index, not byte offset
pub fn buffer_sub_data<T>(target: GLenum, element_offset_index: uint, data: &[T]) {
    unsafe {
        let size = size_of::<T>();
        ll::glBufferSubData(target,
                            (element_offset_index * size) as GLintptr,
                            (data.len() * size) as GLsizeiptr,
                            to_ptr(data) as *GLvoid);
    }
}

pub fn check_framebuffer_status(target: GLenum) -> GLenum {
    unsafe {
        ll::glCheckFramebufferStatus(target)
    }
}

pub fn clear(mask: GLbitfield) {
    unsafe {
        ll::glClear(mask);
    }
}

pub fn clear_color(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
    unsafe {
        ll::glClearColor(red, green, blue, alpha);
    }
}

pub fn compile_shader(shader: GLuint) {
    unsafe {
        ll::glCompileShader(shader);
    }
}

pub fn create_program() -> GLuint {
    unsafe {
        return ll::glCreateProgram();
    }
}

pub fn create_shader(shader_type: GLenum) -> GLuint {
    unsafe {
        return ll::glCreateShader(shader_type);
    }
}

pub fn cull_face(mode: GLenum) {
    unsafe {
        ll::glCullFace(mode);
    }
}

pub fn delete_buffers(buffers: &[GLuint]) {
    unsafe {
        ll::glDeleteBuffers(buffers.len() as GLsizei, to_ptr(buffers));
    }
}

pub fn delete_frame_buffers(frame_buffers: &[GLuint]) {
    unsafe {
        ll::glDeleteFramebuffers(frame_buffers.len() as GLsizei, to_ptr(frame_buffers));
    }
}

pub fn delete_program(program: GLuint) {
    unsafe {
        ll::glDeleteProgram(program);
    }
}

pub fn delete_render_buffers(render_buffers: &[GLuint]) {
    unsafe {
        ll::glDeleteRenderbuffers(render_buffers.len() as GLsizei, to_ptr(render_buffers));
    }
}

pub fn delete_shader(shader: GLuint) {
    unsafe {
        ll::glDeleteShader(shader);
    }
}

pub fn delete_textures(textures: &[GLuint]) {
    unsafe {
        return ll::glDeleteTextures(textures.len() as GLsizei, to_ptr(textures));
    }
}

pub fn depth_func(func: GLenum) {
    unsafe {
        ll::glDepthFunc(func);
    }
}

pub fn depth_mask(flag: bool) {
    unsafe {
        ll::glDepthMask(flag as GLboolean);
    }
}

pub fn detach_shader(program: GLuint, shader: GLuint) {
    unsafe {
        ll::glDetachShader(program, shader);
    }
}

pub fn draw_arrays(mode: GLenum, first: GLint, count: GLsizei) {
    unsafe {
        return ll::glDrawArrays(mode, first, count);
    }
}

pub fn draw_elements(mode: GLenum, element_type: GLenum, indices: &[u8]) {
    unsafe {
        return ll::glDrawElements(mode,
                                  indices.len() as GLsizei,
                                  element_type,
                                  cast::transmute(&indices[0]));
    }
}

pub fn enable(cap: GLenum) {
    unsafe {
        ll::glEnable(cap);
    }
}

pub fn enable_vertex_attrib_array(index: GLuint) {
    unsafe {
        ll::glEnableVertexAttribArray(index);
    }
}

pub fn disable_vertex_attrib_array(index: GLuint) {
    unsafe {
        ll::glDisableVertexAttribArray(index);
    }
}

pub fn finish() {
    unsafe {
        return ll::glFinish();
    }
}

pub fn flush() {
    unsafe {
        return ll::glFlush();
    }
}

pub fn framebuffer_texture_2d(target: GLenum,
                              attachment: GLenum,
                              textarget: GLenum,
                              texture: GLuint,
                              level: GLint) {
    unsafe {
        ll::glFramebufferTexture2D(target, attachment, textarget, texture, level);
    }
}

pub fn front_face(mode: GLenum) {
    unsafe {
        ll::glFrontFace(mode);
    }
}

pub fn gen_buffers(n: GLsizei) -> ~[GLuint] {
    unsafe {
        let result = from_elem(n as uint, 0 as GLuint);
        ll::glGenBuffers(n, to_ptr(result));
        return result;
    }
}

pub fn gen_framebuffers(n: GLsizei) -> ~[GLuint] {
    unsafe {
        let result = from_elem(n as uint, 0);
        ll::glGenFramebuffers(n, to_ptr(result));
        return result;
    }
}

pub fn gen_textures(n: GLsizei) -> ~[GLuint] {
    unsafe {
        let result = from_elem(n as uint, 0 as GLuint);
        ll::glGenTextures(n, to_ptr(result));
        return result;
    }
}

pub fn get_attrib_location(program: GLuint, name: ~str) -> c_int {
    unsafe {
        return as_c_str(name, |name_bytes|
            ll::glGetAttribLocation(program, name_bytes as *GLchar));
    }
}

pub fn get_error() -> GLenum {
    unsafe {
        return ll::glGetError();
    }
}

pub fn get_program_iv(program: GLuint, pname: GLenum) -> GLint {
    unsafe {
        let result: GLint = 0 as GLint;
        ll::glGetProgramiv(program, pname, to_unsafe_ptr(&result));
        return result;
    }
}

pub fn get_shader_info_log(shader: GLuint) -> ~str {
    unsafe {
        let result = from_elem(1024u, 0u8);
        let result_len: GLsizei = 0 as GLsizei;
        ll::glGetShaderInfoLog(shader,
                               1024 as GLsizei,
                               to_unsafe_ptr(&result_len),
                               to_ptr(result) as *GLchar);
        return from_bytes(result);
    }
}

pub fn get_shader_iv(shader: GLuint, pname: GLenum) -> GLint {
    unsafe {
        let result: GLint = 0 as GLint;
        ll::glGetShaderiv(shader, pname, to_unsafe_ptr(&result));
        return result;
    }
}

pub fn get_uniform_location(program: GLuint, name: ~str) -> c_int {
    unsafe {
        do as_c_str(name) |name_bytes| {
            ll::glGetUniformLocation(program, name_bytes as *GLchar)
        }
    }
}

pub fn link_program(program: GLuint) {
    unsafe {
        return ll::glLinkProgram(program);
    }
}

pub fn pixel_store_i(pname: GLenum, param: GLint) {
    unsafe {
        ll::glPixelStorei(pname, param);
    }
}

pub fn shader_source(shader: GLuint, strings: &[~[u8]]) {
    unsafe {
        let pointers = strings.map(|string| to_ptr(*string));
        let lengths = strings.map(|string| string.len() as GLint);
        ll::glShaderSource(shader, pointers.len() as GLsizei,
                           to_ptr(pointers) as **GLchar, to_ptr(lengths));
        destroy(lengths);
        destroy(pointers);
    }
}

// FIXME: Does not verify buffer size -- unsafe!
pub fn tex_image_2d(target: GLenum,
                    level: GLint,
                    internal_format: GLint,
                    width: GLsizei,
                    height: GLsizei,
                    border: GLint,
                    format: GLenum,
                    ty: GLenum,
                    opt_data: Option<&[u8]>) {
    match opt_data {
        Some(data) => {
            unsafe {
                let pdata = transmute(to_ptr(data));
                ll::glTexImage2D(target, level, internal_format, width, height, border, format, ty,
                                 pdata);
            }
        }
        None => {
            unsafe {
                ll::glTexImage2D(target, level, internal_format, width, height, border, format, ty,
                                 ptr::null());
            }
        }
    }
}

pub fn tex_parameter_i(target: GLenum, pname: GLenum, param: GLint) {
    unsafe {
        ll::glTexParameteri(target, pname, param);
    }
}

pub fn uniform_1f(location: GLint, x: GLfloat) {
    unsafe {
        ll::glUniform1f(location, x);
    }
}

pub fn uniform_1i(location: GLint, x: GLint) {
    unsafe {
        ll::glUniform1i(location, x);
    }
}

pub fn uniform_2f(location: GLint, x: GLfloat, y: GLfloat) {
    unsafe {
        ll::glUniform2f(location, x, y);
    }
}

pub fn uniform_3f(location: GLint, x: GLfloat, y: GLfloat, z: GLfloat) {
    unsafe {
        ll::glUniform3f(location, x, y, z);
    }
}

pub fn uniform_4f(location: GLint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
    unsafe {
        ll::glUniform4f(location, x, y, z, w);
    }
}

pub fn uniform_matrix_4fv(location: GLint, transpose: bool, value: &[f32]) {
    unsafe {
        ll::glUniformMatrix4fv(location,
                               1 as GLsizei,
                               transpose as GLboolean,
                               cast::transmute(&value[0]));
    }
}

pub fn use_program(program: GLuint) {
    unsafe {
        ll::glUseProgram(program);
    }
}

pub fn validate_program(program: GLuint) {
    unsafe {
        ll::glValidateProgram(program);
    }
}

pub fn vertex_attrib_pointer_f32(index: GLuint,
                                 size: GLint,
                                 normalized: bool,
                                 stride: GLsizei,
                                 offset: GLuint) {
    unsafe {
        ll::glVertexAttribPointer(index,
                                  size,
                                  FLOAT,
                                  normalized as GLboolean,
                                  stride,
                                  reinterpret_cast(&(offset as uint)));
    }
}

pub fn viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    unsafe {
        ll::glViewport(x, y, width, height);
    }
}

// Apple extensions
#[cfg(target_os="macos")]
pub mod apple {
    use super::{GLenum, GLsizei};
    use cast::transmute;
    use vec::raw::to_ptr;

    pub unsafe fn texture_range(target: GLenum, buffer: &[u8]) {
        super::glTextureRangeAPPLE(target, buffer.len() as GLsizei, transmute(to_ptr(buffer)));
    }
}

#[nolink]
extern mod ll {

// Lower-level API

pub fn glActiveTexture(++texture: GLenum);

pub fn glAttachShader(++program: GLuint, ++shader: GLuint);

pub fn glBindAttribLocation(++program: GLuint, ++index: GLuint, ++name: *GLchar);

pub fn glBindBuffer(++target: GLenum, ++buffer: GLuint);

pub fn glBindFramebuffer(++target: GLenum, ++framebuffer: GLuint);

pub fn glBindRenderbuffer(++target: GLenum, ++renderbuffer: GLuint);

pub fn glBindTexture(++target: GLenum, ++texture: GLuint);

pub fn glBlendColor(++red: GLclampf, ++green: GLclampf, ++blue: GLclampf, ++alpha: GLclampf);

pub fn glBlendEquation(++mode: GLenum);

pub fn glBlendEquationSeparate(++modeRGB: GLenum, ++modeAlpha: GLenum);

pub fn glBlendFunc(++sfactor: GLenum, ++dfactor: GLenum);

pub fn glBlendFuncSeparate(++srcRGB: GLenum, ++dstRGB: GLenum, ++srcAlpha: GLenum, ++dstAlpha: GLenum);

pub fn glBufferData(++target: GLenum, ++size: GLsizeiptr, ++data: *GLvoid, ++usage: GLenum);

pub fn glBufferSubData(++target: GLenum, ++offset: GLintptr, ++size: GLsizeiptr, ++data: *GLvoid);

pub fn glCheckFramebufferStatus(++target: GLenum) -> GLenum;

pub fn glClear(++mask: GLbitfield);

pub fn glClearColor(++red: GLclampf, ++green: GLclampf, ++blue: GLclampf, ++alpha: GLclampf);

// Unsupported on Mac:
//fn glClearDepthf(++depth: GLclampf);

pub fn glClearStencil(++s: GLint);

pub fn glColorMask(++red: GLboolean, ++green: GLboolean, ++blue: GLboolean, ++alpha: GLboolean);

pub fn glCompileShader(++shader: GLuint);

pub fn glCompressedTexImage2D(++target: GLenum, ++level: GLint, ++internalformat: GLenum, ++width: GLsizei, ++height: GLsizei, ++border: GLint, ++imageSize: GLsizei, ++data: *GLvoid);

pub fn glCompressedTexSubImage2D(++target: GLenum, ++level: GLint, ++xoffset: GLint, ++yoffset: GLint, ++width: GLsizei, ++height: GLsizei, ++format: GLenum, ++imageSize: GLsizei, ++data: *GLvoid);

pub fn glCopyTexImage2D(++target: GLenum, ++level: GLint, ++internalformat: GLenum, ++x: GLint, ++y: GLint, ++width: GLsizei, ++height: GLsizei, ++border: GLint);

pub fn glCopyTexSubImage2D(++target: GLenum, ++level: GLint, ++xoffset: GLint, ++yoffset: GLint, ++x: GLint, ++y: GLint, ++width: GLsizei, ++height: GLsizei);

pub fn glCreateProgram() -> GLuint;

pub fn glCreateShader(++_type: GLenum) -> GLuint;

pub fn glCullFace(++mode: GLenum);

pub fn glDeleteBuffers(++n: GLsizei, ++buffers: *GLuint);

pub fn glDeleteFramebuffers(++n: GLsizei, ++framebuffers: *GLuint);

pub fn glDeleteProgram(++program: GLuint);

pub fn glDeleteRenderbuffers(++n: GLsizei, ++renderbuffers: *GLuint);

pub fn glDeleteShader(++shader: GLuint);

pub fn glDeleteTextures(++n: GLsizei, ++textures: *GLuint);

pub fn glDepthFunc(++func: GLenum);

pub fn glDepthMask(++flag: GLboolean);

// Unsupported on Mac:
//fn glDepthRangef(++zNear: GLclampf, ++zFar: GLclampf);

pub fn glDetachShader(++program: GLuint, ++shader: GLuint);

pub fn glDisable(++cap: GLenum);

pub fn glDisableVertexAttribArray(++index: GLuint);

pub fn glDrawArrays(++mode: GLenum, ++first: GLint, ++count: GLsizei);

pub fn glDrawElements(++mode: GLenum, ++count: GLsizei, ++_type: GLenum, ++indices: *GLvoid);

pub fn glEnable(++cap: GLenum);

pub fn glEnableVertexAttribArray(++index: GLuint);

pub fn glFinish();

pub fn glFlush();

pub fn glFramebufferRenderbuffer(++target: GLenum, ++attachment: GLenum, ++renderbuffertarget: GLenum, ++renderbuffer: GLuint);

pub fn glFramebufferTexture2D(++target: GLenum, ++attachment: GLenum, ++textarget: GLenum, ++texture: GLuint, ++level: GLint);

pub fn glFrontFace(++mode: GLenum);

pub fn glGenBuffers(++n: GLsizei, ++buffers: *GLuint);

pub fn glGenerateMipmap(++target: GLenum);

pub fn glGenFramebuffers(++n: GLsizei, ++framebuffers: *GLuint);

pub fn glGenRenderbuffers(++n: GLsizei, ++renderbuffers: *GLuint);

pub fn glGenTextures(++n: GLsizei, ++textures: *GLuint);

pub fn glGetActiveAttrib(++program: GLuint, ++index: GLuint, ++bufsize: GLsizei, ++length: *GLsizei, ++size: *GLint, ++_type: *GLenum, ++name: *GLchar);

pub fn glGetActiveUniform(++program: GLuint, ++index: GLuint, ++bufsize: GLsizei, ++length: *GLsizei, ++size: *GLint, ++_type: *GLenum, ++name: *GLchar);

pub fn glGetAttachedShaders(++program: GLuint, ++maxcount: GLsizei, ++count: *GLsizei, ++shaders: *GLuint);

pub fn glGetAttribLocation(++program: GLuint, ++name: *GLchar) -> c_int;

pub fn glGetBooleanv(++pname: GLenum, ++params: *GLboolean);

pub fn glGetBufferParameteriv(++target: GLenum, ++pname: GLenum, ++params: *GLint);

pub fn glGetError() -> GLenum;

pub fn glGetFloatv(++pname: GLenum, ++params: *GLfloat);

pub fn glGetFramebufferAttachmentParameteriv(++target: GLenum, ++attachment: GLenum, ++pname: GLenum, ++params: *GLint);

pub fn glGetIntegerv(++pname: GLenum, ++params: *GLint);

pub fn glGetProgramiv(++program: GLuint, ++pname: GLenum, ++params: *GLint);

pub fn glGetProgramInfoLog(++program: GLuint, ++bufsize: GLsizei, ++length: *GLsizei, ++infolog: *GLchar);

pub fn glGetRenderbufferParameteriv(++target: GLenum, ++pname: GLenum, ++params: *GLint);

pub fn glGetShaderiv(++shader: GLuint, ++pname: GLenum, ++params: *GLint);

pub fn glGetShaderInfoLog(++shader: GLuint, ++bufsize: GLsizei, ++length: *GLsizei, ++infolog: *GLchar);

// Unsupported on Mac:
//fn glGetShaderPrecisionFormat(++shadertype: GLenum, ++precisiontype: GLenum, ++range: *GLint, ++precision: *GLint);

pub fn glGetShaderSource(++shader: GLuint, ++bufsize: GLsizei, ++length: *GLsizei, ++source: *GLchar);

pub fn glGetString(++name: GLenum) -> *GLubyte;

pub fn glGetTexParameterfv(++target: GLenum, ++pname: GLenum, ++params: *GLfloat);

pub fn glGetTexParameteriv(++target: GLenum, ++pname: GLenum, ++params: *GLint);

pub fn glGetUniformfv(++program: GLuint, ++location: GLint, ++params: *GLfloat);

pub fn glGetUniformiv(++program: GLuint, ++location: GLint, ++params: *GLint);

pub fn glGetUniformLocation(++program: GLuint, ++name: *GLchar) -> c_int;

pub fn glGetVertexAttribfv(++index: GLuint, ++pname: GLenum, ++params: *GLfloat);

pub fn glGetVertexAttribiv(++index: GLuint, ++pname: GLenum, ++params: *GLint);

pub fn glGetVertexAttribPointerv(++index: GLuint, ++pname: GLenum, ++pointer: **GLvoid);

pub fn glHint(++target: GLenum, ++mode: GLenum);

pub fn glIsBuffer(++buffer: GLuint) -> GLboolean;

pub fn glIsEnabled(++cap: GLenum) -> GLboolean;

pub fn glIsFramebuffer(++framebuffer: GLuint) -> GLboolean;

pub fn glIsProgram(++program: GLuint) -> GLboolean;

pub fn glIsRenderbuffer(++renderbuffer: GLuint) -> GLboolean;

pub fn glIsShader(++shader: GLuint) -> GLboolean;

pub fn glIsTexture(++texture: GLuint) -> GLboolean;

pub fn glLineWidth(++width: GLfloat);

pub fn glLinkProgram(++program: GLuint);

pub fn glPixelStorei(++pname: GLenum, ++param: GLint);

pub fn glPolygonOffset(++factor: GLfloat, ++units: GLfloat);

pub fn glReadPixels(++x: GLint, ++y: GLint, ++width: GLsizei, ++height: GLsizei, ++format: GLenum, ++_type: GLenum, ++pixels: *GLvoid);

// Unsupported on Mac:
// fn glReleaseShaderCompiler();

pub fn glRenderbufferStorage(++target: GLenum, ++internalformat: GLenum, ++width: GLsizei, ++height: GLsizei);

pub fn glSampleCoverage(++value: GLclampf, ++invert: GLboolean);

pub fn glScissor(++x: GLint, ++y: GLint, ++width: GLsizei, ++height: GLsizei);

// Unsupported on Mac:
//fn glShaderBinary(++n: GLsizei, ++shaders: *GLuint, ++binaryformat: GLenum, ++binary: *GLvoid, ++length: GLsizei);

pub fn glShaderSource(++shader: GLuint, ++count: GLsizei, ++string: **GLchar, ++length: *GLint);

pub fn glStencilFunc(++func: GLenum, ++reference: GLint, ++mask: GLuint);

pub fn glStencilFuncSeparate(++face: GLenum, ++func: GLenum, ++reference: GLint, ++mask: GLuint);

pub fn glStencilMask(++mask: GLuint);

pub fn glStencilMaskSeparate(++face: GLenum, ++mask: GLuint);

pub fn glStencilOp(++_fail: GLenum, ++zfail: GLenum, ++zpass: GLenum);

pub fn glStencilOpSeparate(++face: GLenum, ++_fail: GLenum, ++zfail: GLenum, ++zpass: GLenum);

pub fn glTexImage2D(++target: GLenum, ++level: GLint, ++internalformat: GLint, ++width: GLsizei, ++height: GLsizei, ++border: GLint, ++format: GLenum, ++_type: GLenum, ++pixels: *GLvoid);

pub fn glTexParameterf(++target: GLenum, ++pname: GLenum, ++param: GLfloat);

pub fn glTexParameterfv(++target: GLenum, ++pname: GLenum, ++params: *GLfloat);

pub fn glTexParameteri(++target: GLenum, ++pname: GLenum, ++param: GLint);

pub fn glTexParameteriv(++target: GLenum, ++pname: GLenum, ++params: *GLint);

pub fn glTexSubImage2D(++target: GLenum, ++level: GLint, ++xoffset: GLint, ++yoffset: GLint, ++width: GLsizei, ++height: GLsizei, ++format: GLenum, ++_type: GLenum, ++pixels: *GLvoid);

pub fn glUniform1f(++location: GLint, ++x: GLfloat);

pub fn glUniform1fv(++location: GLint, ++count: GLsizei, ++v: *GLfloat);

pub fn glUniform1i(++location: GLint, ++x: GLint);

pub fn glUniform1iv(++location: GLint, ++count: GLsizei, ++v: *GLint);

pub fn glUniform2f(++location: GLint, ++x: GLfloat, ++y: GLfloat);

pub fn glUniform2fv(++location: GLint, ++count: GLsizei, ++v: *GLfloat);

pub fn glUniform2i(++location: GLint, ++x: GLint, ++y: GLint);

pub fn glUniform2iv(++location: GLint, ++count: GLsizei, ++v: *GLint);

pub fn glUniform3f(++location: GLint, ++x: GLfloat, ++y: GLfloat, ++z: GLfloat);

pub fn glUniform3fv(++location: GLint, ++count: GLsizei, ++v: *GLfloat);

pub fn glUniform3i(++location: GLint, ++x: GLint, ++y: GLint, ++z: GLint);

pub fn glUniform3iv(++location: GLint, ++count: GLsizei, ++v: *GLint);

pub fn glUniform4f(++location: GLint, ++x: GLfloat, ++y: GLfloat, ++z: GLfloat, ++w: GLfloat);

pub fn glUniform4fv(++location: GLint, ++count: GLsizei, ++v: *GLfloat);

pub fn glUniform4i(++location: GLint, ++x: GLint, ++y: GLint, ++z: GLint, ++w: GLint);

pub fn glUniform4iv(++location: GLint, ++count: GLsizei, ++v: *GLint);

pub fn glUniformMatrix2fv(++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLfloat);

pub fn glUniformMatrix3fv(++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLfloat);

pub fn glUniformMatrix4fv(++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLfloat);

pub fn glUseProgram(++program: GLuint);

pub fn glValidateProgram(++program: GLuint);

pub fn glVertexAttrib1f(++indx: GLuint, ++x: GLfloat);

pub fn glVertexAttrib1fv(++indx: GLuint, ++values: *GLfloat);

pub fn glVertexAttrib2f(++indx: GLuint, ++x: GLfloat, ++y: GLfloat);

pub fn glVertexAttrib2fv(++indx: GLuint, ++values: *GLfloat);

pub fn glVertexAttrib3f(++indx: GLuint, ++x: GLfloat, ++y: GLfloat, ++z: GLfloat);

pub fn glVertexAttrib3fv(++indx: GLuint, ++values: *GLfloat);

pub fn glVertexAttrib4f(++indx: GLuint, ++x: GLfloat, ++y: GLfloat, ++z: GLfloat, ++w: GLfloat);

pub fn glVertexAttrib4fv(++indx: GLuint, ++values: *GLfloat);

pub fn glVertexAttribPointer(++indx: GLuint, ++size: GLint, ++_type: GLenum, ++normalized: GLboolean, ++stride: GLsizei, ++ptr: *GLvoid);

pub fn glViewport(++x: GLint, ++y: GLint, ++width: GLsizei, ++height: GLsizei);


}

// Apple extensions
#[cfg(target_os="macos")]
extern {

pub fn glTextureRangeAPPLE(++target: GLenum, ++length: GLsizei, ++pointer: *GLvoid);

}

