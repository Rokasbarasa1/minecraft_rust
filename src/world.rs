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
use crate::render_gl;
use self::chunk::block;
use block::Block;
use chunk::Chunk;

pub struct World {
    loaded_textures: Vec<gl::types::GLuint>,
    chunk_grid: Vec<Vec<Chunk>>,
    visible_chunk_grid: Vec<Chunk>,
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

        let visible_chunk_grid: Vec<Chunk> = vec![];
        let mut chunk_grid: Vec<Vec<Chunk>> = vec![];
        generate_chunks(&mut chunk_grid, camera_position, square_chunk_width, &block_radius, chunks_layers_from_player);

        //VALIDATE VISIBILITY GO TO CHUNK TO VALIDATE

        check_visibility(&mut chunk_grid);

        let world = World{
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
        let mut keep = vec![];
        for i in 0..self.visible_chunk_grid.len() {
            if self.view_distance < distance_between_points(camera_position, Chunk::get_position(&self.visible_chunk_grid[i])){
                keep.push(false);
            }else{
                keep.push(true);
            }
        }
        let mut i = 0;
        self.visible_chunk_grid.retain(|_| (keep[i], i += 1).0);

        for i in 0..self.chunk_grid.len() {
            for k in 0..self.chunk_grid[i].len() {
                if !contains(&self.visible_chunk_grid, &self.chunk_grid[i][k]) {
                    if self.view_distance > distance_between_points(camera_position, Chunk::get_position(&self.chunk_grid[i][k])){
                        self.visible_chunk_grid.push(Chunk::copy(&self.chunk_grid[i][k]));
                    }
                }
            }
        }

        self.program.set_used();

        unsafe{
            gl::BindVertexArray(*vao);
        }

        for i in 0..self.visible_chunk_grid.len() {
            Chunk::render(&self.visible_chunk_grid[i], &self.loaded_textures, &self.program.id());
        } 
    }
}

fn generate_chunks(chunk_grid: &mut Vec<Vec<Chunk>>, camera_position: &glm::Vector3<f32>, square_chunk_width: &u32, block_radius: &f32, render_out_from_player: &u32){
    let half_chunk_width = (*square_chunk_width as f32 / 2.0).floor();
    let mut x_pos = camera_position.x + half_chunk_width + (*render_out_from_player as f32 * *square_chunk_width as f32);
    let mut z_pos = camera_position.z + half_chunk_width + (*render_out_from_player as f32 * *square_chunk_width as f32);
    let x_pos_temp = z_pos;

    let mut width_adjust= 0;
    if square_chunk_width % 2 == 1 {
        width_adjust = 1;
    }

    let chunk_widht;
    if *render_out_from_player == 1 {
        chunk_widht = 1;
    }else{
        chunk_widht = *render_out_from_player * 2 - width_adjust
    };
    for i in 0..chunk_widht.clone() as usize {  //Z line Go from positive to negative
        let collumn: Vec<chunk::Chunk> = vec![];
        chunk_grid.push(collumn);
        for k in 0..chunk_widht.clone() as usize {  //Z line Go from positive to negative
            chunk_grid[i].push(chunk::Chunk::init(i.clone() as i32, k.clone() as i32, glm::vec3(x_pos.clone(), -4.0, z_pos.clone()), square_chunk_width, block_radius));
            x_pos -= *square_chunk_width as f32;
        }
        x_pos = x_pos_temp;
        z_pos -= *square_chunk_width as f32 ;
    }
    println!(" ")
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

fn contains(list: &Vec<Chunk>,chunk: &Chunk) -> bool{
    for i in 0..list.len() {
        if chunk::Chunk::compare(chunk, &list[i]) {
            return true;
        }
    }
    return false;
}

fn check_visibility(chunk_grid: &mut Vec<Vec<Chunk>>){
    let jSize = Chunk::get_blocks_vector(&mut chunk_grid[0][0]).len();

    let mut index: u128 = 0;
    for i in 0..chunk_grid.len() {
        for k in 0..chunk_grid[i].len() {
            for j in 0..Chunk::get_blocks_vector(&mut chunk_grid[i][k]).len() {
                for l in 0..Chunk::get_blocks_vector(&mut chunk_grid[i][k])[j].len() {
                    for m in 0..Chunk::get_blocks_vector(&mut chunk_grid[i][k])[j][l].len() {
                        // println!("Iteration: {}", index);
                        // if index ==  241{
                        //     println!("Were here boys");
                        // }
                        check_block_sides(chunk_grid , i.clone(), k.clone(), j.clone(), l.clone(), m.clone(), jSize.clone());
                        //index += 1;
                    }
                }
            }
        }
    }
}

fn check_block_sides(chunk_grid: &mut Vec<Vec<Chunk>>, i: usize, k: usize, j: usize, l: usize, m: usize, chunk_width: usize){
    if Block::is_air(&Chunk::get_blocks_vector(&chunk_grid[i][k])[j][l][m]) {
        Block::set_invisiblie(&mut Chunk::get_blocks_vector_mutable(&mut chunk_grid[i][k])[j][l][m])
    }
    else {
        let zChunkFlag: u32; 
        if i == 0 { zChunkFlag = 0 }else if i == chunk_grid.len()-1 { zChunkFlag = 2 } else { zChunkFlag = 1 }; //Z axis
        let xChunkFlag: u32; 
        if k == 0 { xChunkFlag = 0 }else if k == chunk_grid.len()-1 { xChunkFlag = 2 } else { xChunkFlag = 1 }; //X axis

        let zBlockFlag: u32; 
        if j == 0 { zBlockFlag = 0 }else if j == chunk_width-1 { zBlockFlag = 2 } else { zBlockFlag = 1 }; //Z axis
        let xBlockFlag: u32; 
        if l == 0 { xBlockFlag = 0 }else if l == chunk_width-1 { xBlockFlag = 2 } else { xBlockFlag = 1 }; //X axis
        let yBlockFlag: u32; 
        if m == 0 { yBlockFlag = 0 }else if m == Chunk::get_blocks_vector(&chunk_grid[i][k])[j][l].len()-1 { yBlockFlag = 2 } else { yBlockFlag = 1 }; //Y axis

        // I = Z chunk, K = X chunk, J = Z block, L = X block, M = Y block
        let mut sides: Vec<bool> = vec![];

        // //Z block go +
        if zBlockFlag == 2{
            if zChunkFlag == 2 || chunk_grid.len()-1 == 0 && zChunkFlag == 0{
                sides.push(true); // No block there bcs edge of chunks
            }else{
                if Block::is_air(&Chunk::get_blocks_vector(&chunk_grid[i+1][k])[0][l][m]) { 
                    sides.push(true); 
                }else {
                    sides.push(false); 
                }
            }
        }else{
            if Block::is_air(&Chunk::get_blocks_vector(&chunk_grid[i][k])[j+1][l][m]) { 
                sides.push(true); 
            }else {
                sides.push(false); 
            }
        }

        // Z block go -
        if zBlockFlag == 0{
            if zChunkFlag == 0{
                sides.push(true); // No block there bcs edge of chunks
            }else{
                if Block::is_air(&Chunk::get_blocks_vector(&chunk_grid[i-1][k])[chunk_width-1][l][m]) { 
                    sides.push(true); 
                }else {
                    sides.push(false); 
                }
            }
        }else{
            if Block::is_air(&Chunk::get_blocks_vector(&chunk_grid[i][k])[j-1][l][m]) { 
                sides.push(true); 
            }else {
                sides.push(false); 
            }
        }


        // X block go +
        //Check if end of chunk if 2 it is end of chunk
        if xBlockFlag == 2{
            //Check if end of chunks was == 2 before
            if xChunkFlag == 2 || chunk_grid.len()-1 == 0 && xChunkFlag == 0{
                sides.push(true); // No block there bcs edge of chunks
            }else{
                if Block::is_air(&Chunk::get_blocks_vector(&chunk_grid[i][k+1])[j][0][m]) { 
                    sides.push(true); 
                }else {
                    sides.push(false); 
                }
            }
        }else{
            if Block::is_air(&Chunk::get_blocks_vector(&chunk_grid[i][k])[j][l+1][m]) { 
                sides.push(true); 
            }else {
                sides.push(false); 
            }
        }

        // X block go -
        if xBlockFlag == 0{
            if xChunkFlag == 0{
                sides.push(true); // No block there bcs edge of chunks
            }else{
                if Block::is_air(&Chunk::get_blocks_vector(&chunk_grid[i][k-1])[j][chunk_width-1][m]) { 
                    sides.push(true); 
                }else {
                    sides.push(false); 
                }
            }
        }else{
            if Block::is_air(&Chunk::get_blocks_vector(&chunk_grid[i][k])[j][l-1][m]) { 
                sides.push(true); 
            }else {
                sides.push(false); 
            }
        }



        //sides.push(true);
        // Y block go -
        if yBlockFlag == 0{
            sides.push(true);
        }else{
            if Block::is_air(&Chunk::get_blocks_vector(&mut chunk_grid[i][k])[j][l][m-1]) { 
                sides.push(true); 
            }else {
                sides.push(false); 
            }
        }

        //sides.push(true);
        // Y block go +
        if yBlockFlag == 2 || 0 == Chunk::get_blocks_vector(&chunk_grid[i][k])[j][l].len()-1{
            sides.push(true); // y axis has no more chunks bro
        }else{
            if Block::is_air(&Chunk::get_blocks_vector(&chunk_grid[i][k])[j][l][m+1]) { 
                sides.push(true); 
            }else {
                sides.push(false); 
            }
        }



        //Assigning invisible or visisble
        let mut visible: u8 = 0;
        for i in 0..sides.len(){
            if sides[i] == true {
                visible += 1;
            }
            // if i == 5 && sides[i] == true {
            //     println!("Were here again boys");
            // }
        }
        
        if visible > 0{
            Block::set_visibility_vector(&mut Chunk::get_blocks_vector_mutable(&mut chunk_grid[i][k])[j][l][m], sides);            
        }else{
            Block::set_invisiblie(&mut Chunk::get_blocks_vector_mutable(&mut chunk_grid[i][k])[j][l][m])
        }
    } 
}