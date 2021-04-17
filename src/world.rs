extern crate gl;
extern crate sdl2;
extern crate stb_image;
extern crate image;
extern crate serde_json;
extern crate glm;
extern crate nalgebra_glm;
use std::{ffi::c_void};

use crate::render_gl;
trait IWorld{
    fn new(amount_textures: usize, camera_position: &glm::Vector3<f32>, square_chunk_width: &u32, block_radius: f32);
    //fn render(&self, camera_position: &glm::Vector3<f32>);
}

pub struct World {
    loaded_textures: Vec<gl::types::GLuint>,
    //map of values with noise to know how to draw map. Not gonna store all the map in a file, duhh
    current_chunks: Vec<Vec<glm::Vector3<f32>>>,
    square_chunk_width: u32,
    block_radius: f32,
    program: render_gl::Program
}

impl World{
    pub fn new(amount_textures: &usize, camera_position: &glm::Vector3<f32>, square_chunk_width: &u32, block_radius: &f32, program: &render_gl::Program,  chunks_layers_from_player: &u32) -> World{
        let mut loaded_textures: Vec<gl::types::GLuint> = vec![];
        let mut data = image::open(&std::path::Path::new("C:\\Users\\Rokas\\Desktop\\rust minecraft\\minecraft_rust\\TextureTemplate.png")).unwrap().into_rgb();
        let number: i32 = 1;
        for i in 0..*amount_textures {
            let mut texture: gl::types::GLuint = 0;
            unsafe{
                gl::GenTextures(number, &mut texture);
                gl::BindTexture(gl::TEXTURE_2D, texture);
            }
            setup_texture(texture, i, &mut data);
            loaded_textures.push(texture);
        }
        let mut current_chunks: Vec<Vec<glm::Vector3<f32>>> = vec![];
        generate_chunks(&mut current_chunks, camera_position, square_chunk_width, &block_radius, chunks_layers_from_player);
        let mut world = World{
            loaded_textures: loaded_textures,
            current_chunks: current_chunks,
            square_chunk_width: *square_chunk_width,
            block_radius: *block_radius,
            program: program.clone()
        };
        return world;
    }

    pub fn render(&self, camera_position: &glm::Vector3<f32>, vao: &gl::types::GLuint){
        let current_chunk_x = camera_position.x % 16.0 + 0.5;
        let current_chunk_z = camera_position.z % 16.0 + 0.5;
        
        // println!("{}", current_chunk_x);
        // println!("{} \n\n", current_chunk_z);
        // if current_chunk_x >= 1.0 {

        //    //Render chunks in X direction and remove in opposite end 
        // } else if current_chunk_x <= 0.0{
        //     //Render chunks in x direction and remove in opposite end 
        // }
    
        // if current_chunk_z >= 1.0 {
        //     //Render chunks in z direction and remove in opposite end 
        // } else if current_chunk_z <= 0.0{
        //     //Render chunks in z direction and remove in opposite end 
        // }
        self.program.set_used();
        unsafe{
            gl::BindVertexArray(*vao);
        }
        for i in 0..self.current_chunks.len() {
            for k in 0..self.current_chunks[i].len() {
                render_object(self.current_chunks[i][k], self.program.id().clone(), 4, &self.loaded_textures, k);
            }
        }
    }

    pub fn look_inside(world: &World) -> f32{
        return world.current_chunks[0][0].x;
    }
    // fn render(&self, camera_position: &glm::Vector3<f32>){

    //     generate_chunks(self, camera_position);
    //     //renderBlocks();
    // }

}

fn generate_chunks(current_chunks: &mut Vec<Vec<glm::Vector3<f32>>>, camera_position: &glm::Vector3<f32>, square_chunk_width: &u32, block_radius: &f32, render_out_from_player: &u32){
    let half_chunk_width = (*square_chunk_width as f32 / 2.0).floor();
    //println!("Z {}\n", render_out_from_player);
    let mut x_pos = camera_position.x - half_chunk_width - (f32::floor(*render_out_from_player as f32/2.0) * *square_chunk_width as f32);
    let mut z_pos = camera_position.z + half_chunk_width + (f32::floor(*render_out_from_player as f32/2.0) * *square_chunk_width as f32);

    z_pos += 7.0;
    let mut adjusted_x_pos = 0.0;
    let mut adjusted_z_pos = 0.0;
    for i in 0..(u32::pow(*render_out_from_player, 2) as usize) {
        let new_chunk: Vec<glm::Vector3<f32>> = vec![];
        current_chunks.push(new_chunk);

        for k in 0..u32::pow(*square_chunk_width, 2){
            current_chunks[i].push(glm::vec3( block_radius * x_pos,  block_radius * -3.0,  block_radius * z_pos));
            x_pos += 1.0;
            adjusted_x_pos += 1.0;
            if adjusted_x_pos % *square_chunk_width as f32 == 0.0 {
                x_pos -= *square_chunk_width as f32;
                adjusted_x_pos -= *square_chunk_width as f32;
                z_pos -= 1.0;
                adjusted_z_pos -= 1.0;
            }
        }
        if (i+1) % *render_out_from_player as usize == 0 {
            x_pos -= (*render_out_from_player as f32 - 1.0) * *square_chunk_width as f32;
            adjusted_x_pos -= (*render_out_from_player as f32 - 1.0) * *square_chunk_width as f32;
            z_pos -= 0.0;
            adjusted_z_pos -= 0.0;
        }else{
            x_pos += *square_chunk_width as f32;
            adjusted_x_pos += *square_chunk_width as f32;
            z_pos += *square_chunk_width as f32;
            adjusted_z_pos += *square_chunk_width as f32;
        }
    }
    
    // if (current_chunks[0].is_empty() ){ // check if chunks undefined, if so render them

        
    //     //Generate all 9 chunks
    // }
    // else {
    //     let current_chunk_x = camera_position.x % 16.0 + 0.5;
    //     let current_chunk_z = camera_position.z % 16.0 + 0.5;
    //     if current_chunk_x >= 1.0 {
    //         //Render chunks in X direction and remove in opposite end 
    //     } else if current_chunk_x <= 0.0{
    //         //Render chunks in x direction and remove in opposite end 
    //     }

    //     if current_chunk_z >= 1.0 {
    //         //Render chunks in z direction and remove in opposite end 
    //     } else if current_chunk_z <= 0.0{
    //         //Render chunks in z direction and remove in opposite end 
    //     }

    // }
}

fn setup_texture(texture:  gl::types::GLuint, increment: usize, data: & mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) {
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPLACE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT  as i32);

        gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MIN_FILTER, gl::LINEAR  as i32);
        gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MAG_FILTER, gl::LINEAR  as i32);

        let cropped_images =  image::imageops::crop( data, 16*(increment as u32), 0, 16, 16).to_image();
        let (width ,height) = cropped_images.dimensions();
        let img_data = cropped_images.into_raw();
        
        let img_ptr: *const c_void = img_data.as_ptr() as *const c_void;
        
        gl::TexImage2D(
            gl::TEXTURE_2D, 
            0, 
            gl::RGB as i32, 
            width as i32, 
            height as i32, 
            0, 
            gl::RGB, 
            gl::UNSIGNED_BYTE, 
            img_ptr
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);
    }   
}

pub fn render_object(cube_position: glm::Vector3<f32>, program: gl::types::GLuint, amount_textures: usize, loaded_textures: &[u32], i: usize){
    unsafe{
        if(i==0){
            gl::BindTexture(gl::TEXTURE_2D, loaded_textures[0]);
        } else if(i==255){
            gl::BindTexture(gl::TEXTURE_2D, loaded_textures[1]);
        }
        else{
            gl::BindTexture(gl::TEXTURE_2D, loaded_textures[2]);
        }
        
        let mut model = glm::ext::translate(&glm::mat4(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0),  cube_position);
        model =  glm::ext::rotate(&model, glm::radians(0.0), glm::vec3(1.0, 0.3, 0.5));
        let model_loc = gl::GetUniformLocation(program, "model".as_ptr() as *const std::os::raw::c_char);
        gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, &model[0][0]);
        gl::DrawArrays(
            gl::TRIANGLES,
            0,
            36,
        );
    }
}