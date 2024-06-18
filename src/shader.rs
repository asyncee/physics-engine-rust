use std::{ffi::CString, fs, ptr};

use cgmath::Matrix;
use gl::types::{GLint, GLuint};

pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn from_files(vertex_path: &str, fragment_path: &str) -> Self {
        let vertex = fs::read_to_string(vertex_path).unwrap();
        let fragment = fs::read_to_string(fragment_path).unwrap();
        Shader::build(&vertex, &fragment)
    }

    pub fn build(vertex_source: &str, fragment_source: &str) -> Self {
        let vertex_c_string = CString::new(vertex_source).expect("c string for vertex shader");
        let frag_c_string = CString::new(fragment_source).expect("c string for fragment shader");

        unsafe {
            let s_vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(s_vertex, 1, &vertex_c_string.as_ptr(), ptr::null());
            gl::CompileShader(s_vertex);

            // error checking
            let mut success: GLint = 0;
            gl::GetShaderiv(s_vertex, gl::COMPILE_STATUS, &mut success);

            if success != 1 {
                let mut error_log_size: GLint = 0;
                gl::GetShaderiv(s_vertex, gl::INFO_LOG_LENGTH, &mut error_log_size);

                let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);

                gl::GetShaderInfoLog(
                    s_vertex,
                    error_log_size,
                    &mut error_log_size,
                    error_log.as_mut_ptr() as *mut _,
                );

                error_log.set_len(error_log_size as usize);
                let log = String::from_utf8(error_log).expect("vertex shader error");
                panic!("{log}");
            }

            let s_frag = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(s_frag, 1, &frag_c_string.as_ptr(), ptr::null());
            gl::CompileShader(s_frag);
            // error checking
            let mut success: GLint = 0;
            gl::GetShaderiv(s_frag, gl::COMPILE_STATUS, &mut success);

            if success != 1 {
                let mut error_log_size: GLint = 0;
                gl::GetShaderiv(s_frag, gl::INFO_LOG_LENGTH, &mut error_log_size);

                let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);

                gl::GetShaderInfoLog(
                    s_frag,
                    error_log_size,
                    &mut error_log_size,
                    error_log.as_mut_ptr() as *mut _,
                );

                error_log.set_len(error_log_size as usize);
                let log = String::from_utf8(error_log).expect("frag shader error");
                panic!("{log}");
            }

            let program = gl::CreateProgram();
            gl::AttachShader(program, s_vertex);
            gl::AttachShader(program, s_frag);
            gl::LinkProgram(program);

            let mut success: GLint = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

            if success != 1 {
                let mut error_log_size: GLint = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut error_log_size);
                let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
                gl::GetProgramInfoLog(
                    program,
                    error_log_size,
                    &mut error_log_size,
                    error_log.as_mut_ptr() as *mut _,
                );

                error_log.set_len(error_log_size as usize);
                let log = String::from_utf8(error_log).expect("program error");
                panic!("{log}");
            }

            gl::DeleteShader(s_vertex);
            gl::DeleteShader(s_frag);

            Shader { id: program }
        }
    }

    pub fn use_shader(&self) {
        unsafe { gl::UseProgram(self.id) }
    }

    pub fn set_matrix4(&self, matrix: cgmath::Matrix4<f32>, name: &str) {
        let cname = CString::new(name).expect("matrix name");
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.id, cname.as_ptr()),
                1,
                gl::FALSE,
                matrix.as_ptr(),
            );
        }
    }
}
