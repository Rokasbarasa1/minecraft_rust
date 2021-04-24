extern crate gl;
extern crate sdl2;
extern crate stb_image;
extern crate image;
extern crate serde_json;
extern crate glm;
extern crate nalgebra_glm;
extern crate imageproc;
pub mod chunk;

use std::{ffi::c_void};

use nalgebra_glm::pow;

use crate::render_gl;
trait IWorld{
    fn new(amount_textures: usize, camera_position: &glm::Vector3<f32>, square_chunk_width: &u32, block_radius: f32);
    //fn render(&self, camera_position: &glm::Vector3<f32>);
}

pub struct World {
    loaded_textures: Vec<gl::types::GLuint>,
    chunk_grid: Vec<Vec<chunk::Chunk>>,
    visible_chunk_grid: Vec<chunk::Chunk>,
    square_chunk_width: u32,
    chunks_layers_from_player: u32,
    block_radius: f32,
    view_distance: f32,
    program: render_gl::Program
}

impl World{
    pub fn new(amount_textures: &usize, camera_position: &glm::Vector3<f32>, square_chunk_width: &u32, block_radius: &f32, program: &render_gl::Program,  chunks_layers_from_player: &u32, VIEW_DISTANCE: &f32) -> World{
        //LOAD TEXTURES

        let mut loaded_textures: Vec<gl::types::GLuint> = vec![];
        let mut data = image::open(&std::path::Path::new("C:\\Users\\Rokas\\Desktop\\rust minecraft\\minecraft_rust\\TextureTemplate.png")).unwrap().into_rgba8();
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




        //LOAD TERRAIN MAP

        let mut image = image::ImageBuffer::<image::Rgb<u8>, Vec<u8> >::new(1000, 1000);
        *image.get_pixel_mut(5, 5) = image::Rgb([255,255,255]);        
        imageproc::noise::gaussian_noise(&image, -1.0, 100.0, 1115);
        image.save("output.png").unwrap();



        //LOAD TERRAIN
        let mut visible_chunk_grid: Vec<chunk::Chunk> = vec![];
        let mut chunk_grid: Vec<Vec<chunk::Chunk>> = vec![];
        generate_chunks(&mut chunk_grid, camera_position, square_chunk_width, &block_radius, chunks_layers_from_player);



        let mut world = World{
            loaded_textures: loaded_textures,
            chunk_grid: chunk_grid,
            square_chunk_width: square_chunk_width.clone(),
            visible_chunk_grid: visible_chunk_grid,
            chunks_layers_from_player: chunks_layers_from_player.clone(),
            block_radius: block_radius.clone(),
            view_distance: VIEW_DISTANCE.clone(),
            program: program.clone()
        };
        return world;
    }

    pub fn render(&mut self, camera_position: &glm::Vector3<f32>, vao: &gl::types::GLuint){
        // self.visible_chunk_grid.retain(|&chunk| {
        //     self.view_distance < distance_between_points(camera_position, chunk::Chunk::get_position(chunk))
        // });
        let mut keep = vec![];
        for i in 0..self.visible_chunk_grid.len() {
            let position_chunk = chunk::Chunk::get_position(&self.visible_chunk_grid[i]);
            //if (camera_position.x - position_chunk.x) > self.view_distance || (camera_position.z - position_chunk.z) > self.view_distance || (camera_position.y - position_chunk.y) > self.view_distance{
            if self.view_distance < distance_between_points(camera_position, chunk::Chunk::get_position(&self.visible_chunk_grid[i])){
                keep.push(false);
                //self.visible_chunk_grid.remove(i);
            }else{
                keep.push(true);
            }
        }
        let mut i = 0;
        self.visible_chunk_grid.retain(|_| (keep[i], i += 1).0);
        // self.visible_chunk_grid.retain(|&chunk| {
        //     self.view_distance < distance_between_points(camera_position, chunk::Chunk::get_position(chunk))
        // });
        //self.visible_chunk_grid.clear();
        
        for i in 0..self.chunk_grid.len() {
            for k in 0..self.chunk_grid[i].len() {
                if !contains(&self.visible_chunk_grid, &self.chunk_grid[i][k]) {
                    let position_chunk = chunk::Chunk::get_position(&self.chunk_grid[i][k]);
                    //if (camera_position.x - position_chunk.x) < self.view_distance || (camera_position.z - position_chunk.z) < self.view_distance || (camera_position.y - position_chunk.y) < self.view_distance{
                    if self.view_distance > distance_between_points(camera_position, chunk::Chunk::get_position(&self.chunk_grid[i][k])){
                        self.visible_chunk_grid.push(chunk::Chunk::copy(&self.chunk_grid[i][k]));
                    }
                }
            }
        }

        // let current_chunk_x = camera_position.x % self.square_chunk_width as f32 * self.block_radius + 0.5;
        // let current_chunk_z = camera_position.z % self.square_chunk_width as f32 * self.block_radius - 0.5;
        
        // println!("{}", current_chunk_x);  
        // println!("{} \n\n", current_chunk_z);
        // if current_chunk_x >= 1.0 {
        //     for i in 0..self.chunk_grid.len() {
        //         for k in 0..self.chunk_grid[i].len() {
        //             if i % self.chunks_layers_from_player as usize  == 0 {
        //                 self.chunk_grid[i] = vec![];
        //             }
        //         }
        //     }
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

        for i in 0..self.visible_chunk_grid.len() {
            chunk::Chunk::render(&self.visible_chunk_grid[i], &self.loaded_textures, &self.program.id());
        } 
        // for i in 0..self.chunk_grid.len() {
        //     for k in 0..self.chunk_grid[i].len() {
        //         chunk::Chunk::render(&self.chunk_grid[i][k], &self.loaded_textures, &self.program.id());
        //     }
        // }
    }
}

fn generate_chunks(chunk_grid: &mut Vec<Vec<chunk::Chunk>>, camera_position: &glm::Vector3<f32>, square_chunk_width: &u32, block_radius: &f32, render_out_from_player: &u32){
    let half_chunk_width = (*square_chunk_width as f32 / 2.0).floor();
    let mut x_pos = camera_position.x + half_chunk_width + (*render_out_from_player as f32 * *square_chunk_width as f32);
    let mut z_pos = camera_position.z + half_chunk_width + (*render_out_from_player as f32 * *square_chunk_width as f32);
    let x_pos_temp = z_pos;

    let mut width_adjust= 0;
    if square_chunk_width % 2 == 1 {
        width_adjust = 1;
    }
    for i in 0..(*render_out_from_player * 2 - width_adjust) as usize {  //Z line Go from positive to negative
        let mut collumn: Vec<chunk::Chunk> = vec![];
        chunk_grid.push(collumn);
        for k in 0..(*render_out_from_player * 2 - width_adjust) as usize {  //Z line Go from positive to negative
            chunk_grid[i].push(chunk::Chunk::init(i.clone() as i32, k.clone() as i32, glm::vec3(x_pos.clone(), -4.0, z_pos.clone()), square_chunk_width, block_radius));
            x_pos -= *square_chunk_width as f32;
        }
        x_pos = x_pos_temp;
        z_pos -= *square_chunk_width as f32 ;
    }
}

fn setup_texture(texture:  gl::types::GLuint, increment: usize, data: & mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPLACE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT  as i32);

        gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MIN_FILTER, gl::NEAREST  as i32);
        gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MAG_FILTER, gl::NEAREST  as i32);

        let cropped_images =  image::imageops::crop( data, 16*(increment as u32), 0, 16, 16).to_image();
        let (width ,height) = cropped_images.dimensions();
        let img_data = cropped_images.into_raw();
        
        let img_ptr: *const c_void = img_data.as_ptr() as *const c_void;
        gl::TexImage2D(
            gl::TEXTURE_2D, 
            0, 
            gl::RGBA8 as i32, 
            width as i32, 
            height as i32, 
            0, 
            gl::RGBA, 
            gl::UNSIGNED_BYTE, 
            img_ptr
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);
    }   
}

fn distance_between_points(point1: &glm::Vector3<f32>, point2: &glm::Vector3<f32>) -> f32{
    let measurement = (f32::powi(point1.x.clone() - point2.x.clone(), 2) + f32::powi(point1.y.clone() - point2.y.clone(), 2) + f32::powi(point1.z.clone() - point2.z.clone(), 2)).sqrt();
    //println!("Distance {}", measurement);
    return measurement;
    //COULD ALSO USE JUST X AND Z TO MAKE THE RENDERED SPACE A CUBE
}

fn contains(list: &Vec<chunk::Chunk>,chunk: &chunk::Chunk) -> bool{
    for i in 0..list.len() {
        if chunk::Chunk::compare(chunk, &list[i]) {
            return true;
        }
    }
    return false;
}