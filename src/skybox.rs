use crate::render_gl;
use std::{ffi::c_void};

pub struct Skybox {
    program: render_gl::Program,
    texture_id: gl::types::GLuint,
    skybox_vao: gl::types::GLuint,
    skybox_vbo: gl::types::GLuint,
    vertices: Vec<f32>
}

impl Skybox{
    pub fn new(program: render_gl::Program) -> Skybox{
        let skybox_vertices: Vec<f32> = vec![
            -1.0,  1.0, -1.0,
            -1.0, -1.0, -1.0,
             1.0, -1.0, -1.0,
             1.0, -1.0, -1.0,
             1.0,  1.0, -1.0,
            -1.0,  1.0, -1.0,
    
            -1.0, -1.0,  1.0,
            -1.0, -1.0, -1.0,
            -1.0,  1.0, -1.0,
            -1.0,  1.0, -1.0,
            -1.0,  1.0,  1.0,
            -1.0, -1.0,  1.0,
    
             1.0, -1.0, -1.0,
             1.0, -1.0,  1.0,
             1.0,  1.0,  1.0,
             1.0,  1.0,  1.0,
             1.0,  1.0, -1.0,
             1.0, -1.0, -1.0,
    
            -1.0, -1.0,  1.0,
            -1.0,  1.0,  1.0,
             1.0,  1.0,  1.0,
             1.0,  1.0,  1.0,
             1.0, -1.0,  1.0,
            -1.0, -1.0,  1.0,
    
            -1.0,  1.0, -1.0,
             1.0,  1.0, -1.0,
             1.0,  1.0,  1.0,
             1.0,  1.0,  1.0,
            -1.0,  1.0,  1.0,
            -1.0,  1.0, -1.0,
    
            -1.0, -1.0, -1.0,
            -1.0, -1.0,  1.0,
             1.0, -1.0, -1.0,
             1.0, -1.0, -1.0,
            -1.0, -1.0,  1.0,
             1.0, -1.0,  1.0
        ];

        let faces: Vec<String> = vec![ 
            "C:\\Users\\Rokas\\Desktop\\rust_minecraft\\minecraft_rust\\resources\\posx.png".to_string(),
            "C:\\Users\\Rokas\\Desktop\\rust_minecraft\\minecraft_rust\\resources\\negx.png".to_string(),
            "C:\\Users\\Rokas\\Desktop\\rust_minecraft\\minecraft_rust\\resources\\posy.png".to_string(),
            "C:\\Users\\Rokas\\Desktop\\rust_minecraft\\minecraft_rust\\resources\\negy.png".to_string(),
            "C:\\Users\\Rokas\\Desktop\\rust_minecraft\\minecraft_rust\\resources\\posz.png".to_string(),
            "C:\\Users\\Rokas\\Desktop\\rust_minecraft\\minecraft_rust\\resources\\negz.png".to_string()
        ];
        

        let texture_id = load_cubemap(&faces);

        let mut skybox_vao: gl::types::GLuint = 0;
        let mut skybox_vbo: gl::types::GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut skybox_vao);
            gl::BindVertexArray(skybox_vao);
        }

        //Vertices
        unsafe {
            gl::GenBuffers(1, &mut skybox_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, skybox_vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (skybox_vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, skybox_vertices.as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW);
            gl::VertexAttribPointer( 0, 3, gl::FLOAT, gl::FALSE, (3 * std::mem::size_of::<f32>()) as gl::types::GLint, std::ptr::null());
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        
        let skybox = Skybox{
            program: program,
            texture_id: texture_id,
            skybox_vao: skybox_vao,
            skybox_vbo: skybox_vbo,
            vertices: skybox_vertices,
        };
        return skybox;
    }

    

    pub fn draw(&self){
        unsafe {
            self.program.set_used();
            
            gl::DepthFunc(gl::LEQUAL);
            gl::BindVertexArray(self.skybox_vao);
            gl::EnableVertexAttribArray(0);
            
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.texture_id);
            gl::DrawArrays(gl::TRIANGLES, 0, 36 as i32);
            gl::BindVertexArray(0);
            gl::DepthFunc(gl::LESS);

        }
    }
}

fn load_cubemap(faces: &Vec<String>) -> gl::types::GLuint{
    let mut texture_id: gl::types::GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture_id);
    }

    for i in 0..faces.len(){
        let data = image::open(&std::path::Path::new(&faces[i])).unwrap().into_rgba8();
        let (width ,height) = data.dimensions();
    
        let img_ptr: *const c_void = data.as_ptr() as *const c_void;
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32, 
                0, 
                gl::RGBA8 as i32, 
                width as i32, 
                height as i32, 
                0, 
                gl::RGBA, 
                gl::UNSIGNED_BYTE, 
                img_ptr
            );
        }
    }

    unsafe {
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR  as i32);
    
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP,gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE  as i32);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP,gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE  as i32);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP,gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE  as i32);
    }
    
    return texture_id;
}