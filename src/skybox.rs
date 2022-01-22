// use crate::render_gl;
// use std::{ffi::c_void};
extern crate glium;
use std::io::Cursor;

pub struct Skybox {
    program: glium::Program,
    texture_id: glium::texture::SrgbTexture2d,
    skybox_vao: glium::VertexBuffer<Vertex>,
    skybox_indices: glium::index::NoIndices
}
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3]
}

impl Skybox{
    pub fn new(program: glium::Program, display: &glium::Display) -> Skybox{

        implement_vertex!(Vertex, position);

        let skybox_vertices: Vec<Vertex> = vec![
            Vertex { position: [-1.0,  1.0, -1.0]},
            Vertex { position: [-1.0, -1.0, -1.0]},
            Vertex { position: [ 1.0, -1.0, -1.0]},
            Vertex { position: [ 1.0, -1.0, -1.0]},
            Vertex { position: [ 1.0,  1.0, -1.0]},
            Vertex { position: [-1.0,  1.0, -1.0]},

            Vertex { position: [-1.0, -1.0,  1.0]},
            Vertex { position: [-1.0, -1.0, -1.0]},
            Vertex { position: [-1.0,  1.0, -1.0]},
            Vertex { position: [-1.0,  1.0, -1.0]},
            Vertex { position: [-1.0,  1.0,  1.0]},
            Vertex { position: [-1.0, -1.0,  1.0]},

            Vertex { position: [ 1.0, -1.0, -1.0]},
            Vertex { position: [ 1.0, -1.0,  1.0]},
            Vertex { position: [ 1.0,  1.0,  1.0]},
            Vertex { position: [ 1.0,  1.0,  1.0]},
            Vertex { position: [ 1.0,  1.0, -1.0]},
            Vertex { position: [ 1.0, -1.0, -1.0]},

            Vertex { position: [-1.0, -1.0,  1.0]},
            Vertex { position: [-1.0,  1.0,  1.0]},
            Vertex { position: [ 1.0,  1.0,  1.0]},
            Vertex { position: [ 1.0,  1.0,  1.0]},
            Vertex { position: [ 1.0, -1.0,  1.0]},
            Vertex { position: [-1.0, -1.0,  1.0]},

            Vertex { position: [-1.0,  1.0, -1.0]},
            Vertex { position: [ 1.0,  1.0, -1.0]},
            Vertex { position: [ 1.0,  1.0,  1.0]},
            Vertex { position: [ 1.0,  1.0,  1.0]},
            Vertex { position: [-1.0,  1.0,  1.0]},
            Vertex { position: [-1.0,  1.0, -1.0]},

            Vertex { position: [-1.0, -1.0, -1.0]},
            Vertex { position: [-1.0, -1.0,  1.0]},
            Vertex { position: [ 1.0, -1.0, -1.0]},
            Vertex { position: [ 1.0, -1.0, -1.0]},
            Vertex { position: [-1.0, -1.0,  1.0]},
            Vertex { position: [ 1.0, -1.0,  1.0]}
        ];

        

        let texture_id = load_cubemap(display);

        let vertex_buffer_skybox = glium::VertexBuffer::new(display, &skybox_vertices).unwrap();
        // let indices_skybox = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let indices_skybox = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList,
            &[
                // Front
                0u16, 2, 1, 0, 3, 2,
                // Right
                4, 6, 5, 4, 7, 6,
                // Back
                8, 10, 9, 8, 11, 10,
                // Left
                12, 14, 13, 12, 15, 14,
                // Bottom
                16, 18, 17, 16, 19, 18,
                // Top
                20, 22, 21, 20, 23, 22,
            ]
        ).unwrap();

        let image = image::load(Cursor::new(&include_bytes!("../resources/posy.png")),image::ImageFormat::Png).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();
        
        let image = image::load(Cursor::new(&include_bytes!("../resources/negy.png")),image::ImageFormat::Png).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();

        let image = image::load(Cursor::new(&include_bytes!("../resources/sides.png")),image::ImageFormat::Png).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();

        let cubemap = glium::texture::Cubemap::empty(display, 512).unwrap();

        let  framebuffer1 = glium::framebuffer::SimpleFrameBuffer::new(&display,
                    cubemap.main_level().image(glium::texture::CubeLayer::PositiveX)).unwrap();
        let  framebuffer2 = glium::framebuffer::SimpleFrameBuffer::new(&display,
                    cubemap.main_level().image(glium::texture::CubeLayer::NegativeX)).unwrap();
        let  framebuffer3 = glium::framebuffer::SimpleFrameBuffer::new(&display,
                    cubemap.main_level().image(glium::texture::CubeLayer::PositiveY)).unwrap();
        let  framebuffer4 = glium::framebuffer::SimpleFrameBuffer::new(&display,
                    cubemap.main_level().image(glium::texture::CubeLayer::NegativeY)).unwrap();
        let  framebuffer5 = glium::framebuffer::SimpleFrameBuffer::new(&display,
                    cubemap.main_level().image(glium::texture::CubeLayer::PositiveZ)).unwrap();
        let  framebuffer6 = glium::framebuffer::SimpleFrameBuffer::new(&display,
                    cubemap.main_level().image(glium::texture::CubeLayer::NegativeZ)).unwrap();
        
        let skybox = Skybox{
            program: program,
            texture_id: texture_id,
            skybox_vao: vertex_buffer_skybox,
            skybox_indices: indices_skybox
        };
        return skybox;
    }

    

    pub fn draw(&self, target: &glium::Frame){
        let t:f32 = 1.0;
        let model = [
        	[ t.cos(), 0.0 , t.sin(), 0.0],
        	[ 0.0 , 1.0, 0.0, 0.0],
        	[-t.sin(), 0.0 , t.cos(), 0.0],
        	[ 0.0,      0.0 ,           0.0, 1.0f32],
        ];
        

        // let mut view = glm::ext::look_at(camera_pos, camera_pos + player.camera_front, player.camera_up);
        // view[3][0] = 0.0;
        // view[3][1] = 0.0;
        // view[3][2] = 0.0;
        // view[3][3] = 0.0;

        // view[0][3] = 0.0;
        // view[1][3] = 0.0;
        // view[2][3] = 0.0;

        // let view_loc = gl::GetUniformLocation(skybox_shader.id(), "view".as_ptr() as *const std::os::raw::c_char);

        // gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);

        // let projection = glm::ext::perspective(glm::radians(player.fov), (WINDOW_WIDTH as f32)/(WINDOW_HEIGHT as f32), 0.1, 5000.0);
        
        // let projection_loc = gl::GetUniformLocation(skybox_shader.id(), "projection".as_ptr() as *const std::os::raw::c_char);
        // gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, &projection[0][0]);

        target.draw(&vertex_buffer_skybox, &indices_skybox, &program, &uniform! { matrix: matrix },
                    &Default::default()).unwrap();
        // unsafe {
        //     self.program.set_used();
            
        //     gl::DepthFunc(gl::LEQUAL);
        //     gl::BindVertexArray(self.skybox_vao);
        //     gl::EnableVertexAttribArray(0);
            
        //     gl::ActiveTexture(gl::TEXTURE0);
        //     gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.texture_id);
        //     gl::DrawArrays(gl::TRIANGLES, 0, 36 as i32);
        //     gl::BindVertexArray(0);
        //     gl::DepthFunc(gl::LESS);

        // }
    }
}

fn load_cubemap(display: &glium::Display) -> glium::texture::SrgbTexture2d{

    
    // let mut texture_id: gl::types::GLuint = 0;
    // unsafe {
    //     gl::GenTextures(1, &mut texture_id);
    //     gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture_id);
    // }

    // for i in 0..faces.len(){
    //     let data = image::open(&std::path::Path::new(&faces[i])).unwrap().into_rgba8();
    //     let (width ,height) = data.dimensions();
    
    //     let img_ptr: *const c_void = data.as_ptr() as *const c_void;
    //     unsafe {
    //         gl::TexImage2D(
    //             gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32, 
    //             0, 
    //             gl::RGBA8 as i32, 
    //             width as i32, 
    //             height as i32, 
    //             0, 
    //             gl::RGBA, 
    //             gl::UNSIGNED_BYTE, 
    //             img_ptr
    //         );
    //     }
    // }

    // unsafe {
    //     gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    //     gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR  as i32);
    
    //     gl::TexParameteri(gl::TEXTURE_CUBE_MAP,gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE  as i32);
    //     gl::TexParameteri(gl::TEXTURE_CUBE_MAP,gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE  as i32);
    //     gl::TexParameteri(gl::TEXTURE_CUBE_MAP,gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE  as i32);
    // }
    
    return texture;
}