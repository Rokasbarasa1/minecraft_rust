extern crate image;
extern crate noise;
use crate::render_gl;
pub mod chunk;
pub mod block_model;
mod Block_model;
use self::{block_model::BlockModel, chunk::block};
use std::{ffi::c_void};
use block::Block;
use chunk::Chunk;
pub struct World {
    chunk_width: u32,
    loaded_textures: gl::types::GLuint,
    chunk_grid: Vec<Vec<Chunk>>,
    block_model: BlockModel,
    view_distance: f32,
    program: render_gl::Program,
    unbuilt_models: Vec<(usize, usize)>
}

impl World{
    pub fn new(camera_position: &glm::Vector3<f32>, square_chunk_width: &u32, program: &render_gl::Program,  chunks_layers_from_player: &u32, view_distance: &f32, world_gen_seed: &u32, max_height: &usize) -> World{
        
        let mut world = World{
            chunk_width: square_chunk_width.clone(),
            loaded_textures: 0,
            chunk_grid: vec![],
            block_model: block_model::BlockModel::init(),
            view_distance: view_distance.clone(),
            program: program.clone(),
            unbuilt_models: vec![]
        };

        //LOAD TEXTURES
        setup_texture(&mut world);
        
        //LOAD TERRAIN
        generate_chunks(&mut world.chunk_grid, camera_position, square_chunk_width, chunks_layers_from_player, world_gen_seed, max_height);

        //CHECK VISIBILITY 
        check_visibility(&mut world);

        //BUILD MESH 
        build_mesh(&mut world);

        return world;
    }

    pub fn draw(&mut self, camera_pos: &glm::Vector3<f32>){
        if self.unbuilt_models.len() != 0 {
            for i in 0..self.unbuilt_models.len(){
                build_mesh_single(self, self.unbuilt_models[i].0, self.unbuilt_models[i].1);
            }
            self.unbuilt_models.clear();
        }

        unsafe {
            gl::Enable(gl::CULL_FACE);
        }
        for i in 0..self.chunk_grid.len(){
            for k in 0..self.chunk_grid[i].len(){
                if self.view_distance > glm::distance(camera_pos.clone(), *Chunk::get_position(&self.chunk_grid[i][k])) {
                    self.program.set_used();
                    let chunk_model = Chunk::get_chunk_model(&self.chunk_grid[i][k]);
                    //if chunk_model.0 != 0 && chunk_model.1 != 0 && chunk_model.2 != 0 {
                        unsafe{
                            gl::BindVertexArray(chunk_model.0);
                            gl::EnableVertexAttribArray(0);
                            gl::EnableVertexAttribArray(1);
                            gl::EnableVertexAttribArray(2);
                            gl::EnableVertexAttribArray(3);
                            gl::BindTexture(gl::TEXTURE_2D, chunk_model.0);
                
                            let mut model = glm::ext::translate(&glm::mat4(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0),  glm::vec3(0.0, 0.0, 0.0));
                            model =  glm::ext::rotate(&model, glm::radians(0.0), glm::vec3(1.0, 0.3, 0.5));
                            let model_loc = gl::GetUniformLocation(self.program.id().clone(), "model".as_ptr() as *const std::os::raw::c_char);
                            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, &model[0][0]);
                            gl::DrawArrays(gl::TRIANGLES, 0, chunk_model.1 as i32);
                        }
                    //}
                }
            }
        }
        unsafe {
            gl::Disable(gl::CULL_FACE);
        }

        for i in 0..self.chunk_grid.len(){
            for k in 0..self.chunk_grid[i].len(){
                if self.view_distance > glm::distance(camera_pos.clone(), *Chunk::get_position(&self.chunk_grid[i][k])) {
                    self.program.set_used();
                    let transparent_chunk_model = Chunk::get_transparent_chunk_model(&self.chunk_grid[i][k]);
                    //if chunk_model.0 != 0 && chunk_model.1 != 0 && chunk_model.2 != 0 {
                        unsafe{
                            gl::BindVertexArray(transparent_chunk_model.0);
                            gl::EnableVertexAttribArray(0);
                            gl::EnableVertexAttribArray(1);
                            gl::EnableVertexAttribArray(2);
                            gl::EnableVertexAttribArray(3);
                            gl::BindTexture(gl::TEXTURE_2D, transparent_chunk_model.0);
                
                            let mut model = glm::ext::translate(&glm::mat4(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0),  glm::vec3(0.0, 0.0, 0.0));
                            model =  glm::ext::rotate(&model, glm::radians(0.0), glm::vec3(1.0, 0.3, 0.5));
                            let model_loc = gl::GetUniformLocation(self.program.id().clone(), "model".as_ptr() as *const std::os::raw::c_char);
                            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, &model[0][0]);
                            gl::DrawArrays(gl::TRIANGLES, 0, transparent_chunk_model.1 as i32);
                        }
                    //}
                }
            }
        }
    }

    pub fn destroy_block(&mut self, camera_front: &glm::Vector3<f32>, camera_pos: &glm::Vector3<f32>){
        let (position, mut end, direction) = (camera_pos.clone(), camera_pos.clone(), camera_front.clone());

        while glm::distance(position.clone(), end.clone()) < 6.0 {
            let block_index = get_block(self, &end);
            if block_index.0 != 9999 && block_index.1 != 9999 && block_index.2 != 9999 && block_index.3 != 9999 && block_index.4 != 9999 {
                let mut block = &mut Chunk::get_blocks_vector_mutable(&mut self.chunk_grid[block_index.0][block_index.1])[block_index.2][block_index.3][block_index.4];
                block::Block::set_block_id(&mut block, 240); //240 is air
                block::Block::set_invisiblie(&mut block);
                println!("block removed");
                check_blocks_around_block(self, block_index.0, block_index.1, block_index.2, block_index.3, block_index.4);
                break;

            }
            ray_step(&mut end, &direction, 0.1)
        }
    }

    pub fn place_block(&mut self, camera_front: &glm::Vector3<f32>, camera_pos: &glm::Vector3<f32>, selected_block: usize){
        let (position, mut end, direction) = (camera_pos.clone(), camera_pos.clone(), camera_front.clone());
        let mut last_air_block_index: (usize, usize, usize, usize, usize) = (0,0,0,0,0);
        while glm::distance(position.clone(), end.clone()) < 6.0 {
            let block_index = get_block_or_air(self, &end);
            
            if block_index.0 != 9999 && block_index.1 != 9999 && block_index.2 != 9999 && block_index.3 != 9999 && block_index.4 != 9999 {
                let mut block = &mut Chunk::get_blocks_vector_mutable(&mut self.chunk_grid[block_index.0][block_index.1])[block_index.2][block_index.3][block_index.4];
                
                if Block::is_air(& block){
                    last_air_block_index = block_index.clone();
                }else{
                    
                    let mut block = &mut Chunk::get_blocks_vector_mutable(&mut self.chunk_grid[last_air_block_index.0][last_air_block_index.1])[last_air_block_index.2][last_air_block_index.3][last_air_block_index.4];
                    Block::set_visible(&mut block);
                    block::Block::set_block_id(&mut block, selected_block);
                    check_blocks_around_block(self, last_air_block_index.0, last_air_block_index.1, last_air_block_index.2, last_air_block_index.3, last_air_block_index.4);
                    break;
                }
            }
            ray_step(&mut end, &direction, 0.1)
        }
    }
}

fn get_block(world: & mut World, end: &glm::Vector3<f32>) -> (usize, usize, usize, usize, usize){
    let mut index_i: usize = 9999;
    let mut index_k: usize = 9999;
    let mut index_j: usize = 9999; 
    let mut index_l: usize = 9999; 
    let mut index_m: usize = 9999;
    let mut min:f32 = 9999.0;
    for i in 0..world.chunk_grid.len(){
        for k in 0..world.chunk_grid[i].len(){
            let position = Chunk::get_position(&world.chunk_grid[i][k]).clone();
            let distance = glm::distance(glm::vec2(position.x, position.z), glm::vec2(end.x.clone(), end.z.clone()));
            if distance < min && distance < world.chunk_width as f32 / 2.0  {
                min = distance;
                index_i = i;
                index_k = k;
            }
        }
    }
    if min < world.chunk_width as f32 / 2.0{
        min = 9999.0;
        for j in 0..Chunk::get_blocks_vector(&world.chunk_grid[index_i][index_k]).len() {
            for l in 0..Chunk::get_blocks_vector(&world.chunk_grid[index_i][index_k])[j].len() {
                for m in 0..Chunk::get_blocks_vector(&world.chunk_grid[index_i][index_k])[j][l].len() {
                    if block::Block::is_visible(&Chunk::get_blocks_vector(&world.chunk_grid[index_i][index_k])[j][l][m]){
                        let distance = glm::distance(block::Block::get_position(&Chunk::get_blocks_vector(&world.chunk_grid[index_i][index_k])[j][l][m]).clone(), end.clone());
                        if distance < (f32::powf(0.5, 2.0) + f32::powf(0.5, 2.0)).sqrt()  && min > distance{
                            index_j = j; 
                            index_l = l; 
                            index_m = m; 
                            min = distance;
                        }
                    }
                }
            }
        }
        return (index_i,index_k,index_j,index_l,index_m);
    }else{
        return (index_i,index_k,index_j,index_l,index_m);
    }
}

fn get_block_or_air(world: & mut World, end: &glm::Vector3<f32>) -> (usize, usize, usize, usize, usize){
    let mut index_i: usize = 9999;
    let mut index_k: usize = 9999;
    let mut index_j: usize = 9999; 
    let mut index_l: usize = 9999; 
    let mut index_m: usize = 9999;
    let mut min:f32 = 9999.0;
    for i in 0..world.chunk_grid.len(){
        for k in 0..world.chunk_grid[i].len(){
            let position = Chunk::get_position(&world.chunk_grid[i][k]).clone();
            let distance = glm::distance(glm::vec2(position.x, position.z), glm::vec2(end.x.clone(), end.z.clone()));
            if distance < min && distance < world.chunk_width as f32 / 2.0  {
                min = distance;
                index_i = i;
                index_k = k;
            }
        }
    }
    if min < world.chunk_width as f32 / 2.0{
        min = 9999.0;
        for j in 0..Chunk::get_blocks_vector(&world.chunk_grid[index_i][index_k]).len() {
            for l in 0..Chunk::get_blocks_vector(&world.chunk_grid[index_i][index_k])[j].len() {
                for m in 0..Chunk::get_blocks_vector(&world.chunk_grid[index_i][index_k])[j][l].len() {
                    let distance = glm::distance(block::Block::get_position(&Chunk::get_blocks_vector(&world.chunk_grid[index_i][index_k])[j][l][m]).clone(), end.clone());
                    if distance < (f32::powf(0.5, 2.0) + f32::powf(0.5, 2.0)).sqrt()  && min > distance{
                        index_j = j; 
                        index_l = l; 
                        index_m = m; 
                        min = distance;
                    }
                }
            }
        }
        return (index_i,index_k,index_j,index_l,index_m);
    }else{
        return (index_i,index_k,index_j,index_l,index_m);
    }
}

fn ray_step(end: &mut glm::Vector3<f32>, direction: &glm::Vector3<f32>, scale: f32){
    let new_end = glm::vec3(end.x, end.y, end.z) + glm::vec3(scale * direction.x, scale * direction.y, scale * direction.z);
    end.x = new_end.x;
    end.y = new_end.y;
    end.z = new_end.z;

}

fn generate_chunks(chunk_grid: &mut Vec<Vec<Chunk>>, camera_position: &glm::Vector3<f32>, square_chunk_width: &u32, render_out_from_player: &u32, WORLD_GEN_SEED: &u32, max_height: &usize){
    let half_chunk_width = (*square_chunk_width as f32 / 2.0).floor();
    let mut x_pos = camera_position.x + (half_chunk_width + (*render_out_from_player as f32 * *square_chunk_width as f32));
    let mut z_pos = camera_position.z + (half_chunk_width + (*render_out_from_player as f32 * *square_chunk_width as f32));
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
        for k in 0..chunk_widht.clone() as usize {  //X line Go from positive to negative
            chunk_grid[i].push(chunk::Chunk::init(i.clone() as i32, k.clone() as i32, glm::vec3(x_pos.clone(), -10.0, z_pos.clone()), square_chunk_width, WORLD_GEN_SEED, max_height));
            x_pos -= *square_chunk_width as f32;
        }
        x_pos = x_pos_temp;
        z_pos -= *square_chunk_width as f32 ;
    }
}

fn setup_texture(world: &mut World) {

    let data = image::open(&std::path::Path::new("C:\\Users\\Rokas\\Desktop\\rust_minecraft\\minecraft_rust\\TextureTemplate.png")).unwrap().into_rgba8();
    let mut texture: gl::types::GLuint = 0;
    
    unsafe {
        gl::GenTextures(0, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPLACE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT  as i32);

        gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MIN_FILTER, gl::NEAREST  as i32);
        gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MAG_FILTER, gl::NEAREST  as i32);
        //gl::TexEnvf(gl::TEXTURE, gl::TEXTURE_ENV_MODE, gl::MODULATE);
        let (width ,height) = data.dimensions();
        
        let img_ptr: *const c_void = data.as_ptr() as *const c_void;
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
    world.loaded_textures = texture;
}

fn check_visibility(world: &mut World){
    for i in 0..world.chunk_grid.len() {
        for k in 0..world.chunk_grid[i].len() {
            for j in 0..Chunk::get_blocks_vector(&mut world.chunk_grid[i][k]).len() {
                for l in 0..Chunk::get_blocks_vector(&mut world.chunk_grid[i][k])[j].len() {
                    for m in 0..Chunk::get_blocks_vector(&mut world.chunk_grid[i][k])[j][l].len() {
                        check_block_sides(&mut world.chunk_grid , i.clone(), k.clone(), j.clone(), l.clone(), m.clone(), world.chunk_width as usize);
                    }
                }
            }
        }
    }
}

fn check_block_sides(chunk_grid: &mut Vec<Vec<Chunk>>, i: usize, k: usize, j: usize, l: usize, m: usize, chunk_width: usize){
    let block_id = Block::get_id(&Chunk::get_blocks_vector(&mut chunk_grid[i][k])[j][l][m]);
    
    if Block::is_air(&Chunk::get_blocks_vector(&chunk_grid[i][k])[j][l][m]) {
        Block::set_invisiblie(&mut Chunk::get_blocks_vector_mutable(&mut chunk_grid[i][k])[j][l][m])
    }else {
        let z_chunk_flag: u32; 
        if i == 0 { z_chunk_flag = 0 }else if i == chunk_grid.len()-1 { z_chunk_flag = 2 } else { z_chunk_flag = 1 }; //Z axis
        let x_chunk_flag: u32; 
        if k == 0 { x_chunk_flag = 0 }else if k == chunk_grid.len()-1 { x_chunk_flag = 2 } else { x_chunk_flag = 1 }; //X axis

        let z_block_flag: u32; 
        if j == 0 { z_block_flag = 0 }else if j == chunk_width-1 { z_block_flag = 2 } else { z_block_flag = 1 }; //Z axis
        let x_block_flag: u32; 
        if l == 0 { x_block_flag = 0 }else if l == chunk_width-1 { x_block_flag = 2 } else { x_block_flag = 1 }; //X axis
        let y_block_flag: u32; 
        if m == 0 { y_block_flag = 0 }else if m == Chunk::get_blocks_vector(&chunk_grid[i][k])[j][l].len()-1 { y_block_flag = 2 } else { y_block_flag = 1 }; //Y axis

        // I = Z chunk, K = X chunk, J = Z block, L = X block, M = Y block
        let mut sides: Vec<bool> = vec![];

        // //Z block go +
        if z_block_flag == 2{
            if z_chunk_flag == 2 || chunk_grid.len()-1 == 0 && z_chunk_flag == 0{
                sides.push(true);
            }else{
                if Block::is_air(&Chunk::get_blocks_vector(&chunk_grid[i+1][k])[0][l][m]) || block_id != 3 && Block::is_water(&Chunk::get_blocks_vector(&chunk_grid[i+1][k])[0][l][m]){ 
                    sides.push(true); 
                }else {
                    sides.push(false); 
                }
            }
        }else{
            if Block::is_air(&Chunk::get_blocks_vector(&chunk_grid[i][k])[j+1][l][m]) || block_id != 3 && Block::is_water(&Chunk::get_blocks_vector(&chunk_grid[i][k])[j+1][l][m])  { 
                sides.push(true); 
            }else {
                sides.push(false); 
            }
        }

        // Z block go -
        if z_block_flag == 0{
            if z_chunk_flag == 0{
                sides.push(true);
            }else{
                if Block::is_air(&Chunk::get_blocks_vector(&chunk_grid[i-1][k])[chunk_width-1][l][m]) || block_id != 3 && Block::is_water(&Chunk::get_blocks_vector(&chunk_grid[i-1][k])[chunk_width-1][l][m]) { 
                    sides.push(true); 
                }else {
                    sides.push(false); 
                }
            }
        }else{
            if Block::is_air(&Chunk::get_blocks_vector(&chunk_grid[i][k])[j-1][l][m]) || block_id != 3 && Block::is_water(&Chunk::get_blocks_vector(&chunk_grid[i][k])[j-1][l][m]) { 
                sides.push(true); 
            }else {
                sides.push(false); 
            }
        }


        // X block go +
        if x_block_flag == 2{
            if x_chunk_flag == 2 || chunk_grid.len()-1 == 0 && x_chunk_flag == 0{
                sides.push(true);
            }else{
                if Block::is_air(&Chunk::get_blocks_vector(&chunk_grid[i][k+1])[j][0][m]) || block_id != 3 && Block::is_water(&Chunk::get_blocks_vector(&chunk_grid[i][k+1])[j][0][m])  { 
                    sides.push(true); 
                }else {
                    sides.push(false); 
                }
            }
        }else{
            if Block::is_air(&Chunk::get_blocks_vector(&chunk_grid[i][k])[j][l+1][m]) || block_id != 3 && Block::is_water(&Chunk::get_blocks_vector(&chunk_grid[i][k])[j][l+1][m])  { 
                sides.push(true); 
            }else {
                sides.push(false); 
            }
        }

        // X block go -
        if x_block_flag == 0{
            if x_chunk_flag == 0{
                sides.push(true);
            }else{
                if Block::is_air(&Chunk::get_blocks_vector(&chunk_grid[i][k-1])[j][chunk_width-1][m]) || block_id != 3 && Block::is_water(&Chunk::get_blocks_vector(&chunk_grid[i][k-1])[j][chunk_width-1][m])  { 
                    sides.push(true); 
                }else {
                    sides.push(false); 
                }
            }
        }else{
            if Block::is_air(&Chunk::get_blocks_vector(&chunk_grid[i][k])[j][l-1][m]) || block_id != 3 && Block::is_water(&Chunk::get_blocks_vector(&chunk_grid[i][k])[j][l-1][m])  { 
                sides.push(true); 
            }else {
                sides.push(false); 
            }
        }



        // Y block go -
        if y_block_flag == 0{
            sides.push(true);
        }else{
            if Block::is_air(&Chunk::get_blocks_vector(&chunk_grid[i][k])[j][l][m-1]) || block_id != 3 && Block::is_water(&Chunk::get_blocks_vector(&chunk_grid[i][k])[j][l][m-1])  { 
                sides.push(true); 
            }else {
                sides.push(false); 
            }
        }

        // Y block go +
        if y_block_flag == 2 || 0 == Chunk::get_blocks_vector(&chunk_grid[i][k])[j][l].len()-1{
            sides.push(true);
        }else{
            if Block::is_air(&Chunk::get_blocks_vector(&chunk_grid[i][k])[j][l][m+1]) || block_id != 3 && Block::is_water(&Chunk::get_blocks_vector(&chunk_grid[i][k])[j][l][m+1])  { 
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
                break;
            }
        }
        
        if visible == 1{
            Block::set_visibility_vector(&mut Chunk::get_blocks_vector_mutable(&mut chunk_grid[i][k])[j][l][m], sides);            
        }else{
            Block::set_invisiblie(&mut Chunk::get_blocks_vector_mutable(&mut chunk_grid[i][k])[j][l][m])
        }
    } 
}

fn check_blocks_around_block(world: &mut World, i: usize, k: usize, j: usize, l: usize, m: usize){
    check_block_sides(&mut world.chunk_grid, i, k, j, l, m, world.chunk_width as usize);
    //Check up 
    if Chunk::get_blocks_vector_mutable(&mut world.chunk_grid[i][k])[j][l].len() != 1 && m != Chunk::get_blocks_vector_mutable(&mut world.chunk_grid[i][k])[j][l].len() -1  && m < Chunk::get_blocks_vector_mutable(&mut world.chunk_grid[i][k])[j][l].len() + 1 {
        check_block_sides(&mut world.chunk_grid, i, k, j, l, m+1, world.chunk_width as usize);
    }
    //Check down
    if Chunk::get_blocks_vector_mutable(&mut world.chunk_grid[i][k])[j][l].len() != 1 && m != 0 {
        check_block_sides(&mut world.chunk_grid, i, k, j, l, m-1, world.chunk_width as usize);
    }


    //Check left
    if j != 0  {
        check_block_sides(&mut world.chunk_grid, i, k, j-1, l, m, world.chunk_width as usize);
    }
    else if i != 0 {
        check_block_sides(&mut world.chunk_grid, i-1, k, world.chunk_width as usize-1, l, m, world.chunk_width as usize);
        world.unbuilt_models.push((i-1,k));
    }
    //Check right
    if j != world.chunk_width as usize -1 {
        check_block_sides(&mut world.chunk_grid, i, k, j+1, l, m, world.chunk_width as usize);
    }else if i != world.chunk_grid.len()-1 {
         check_block_sides(&mut world.chunk_grid, i+1, k, 0, l, m, world.chunk_width as usize);
         world.unbuilt_models.push((i+1,k));
    } 


    //Check front 
    if l != 0  {
        check_block_sides(&mut world.chunk_grid, i, k, j, l-1, m, world.chunk_width as usize);
    }else if k != 0{
        check_block_sides(&mut world.chunk_grid, i, k-1, j, world.chunk_width as usize-1, m, world.chunk_width as usize);
        world.unbuilt_models.push((i,k-1));
    }
    //Check back
    if l != world.chunk_width as usize -1{
        check_block_sides(&mut world.chunk_grid, i, k, j, l+1, m, world.chunk_width as usize);
    }else if k != world.chunk_grid[i].len()-1 {
        check_block_sides(&mut world.chunk_grid, i, k+1, j, 0, m, world.chunk_width as usize);
        world.unbuilt_models.push((i,k+1));
    }

    world.unbuilt_models.push((i,k));
    
}

fn build_mesh(world: &mut World){
    for i in 0..world.chunk_grid.len() {
        for k in 0..world.chunk_grid[i].len() {
            Chunk::build_mesh(&mut world.chunk_grid[i][k], &world.block_model);
            Chunk::populate_mesh(&mut world.chunk_grid[i][k]);
            let raw_model: (gl::types::GLuint, usize) = get_raw_model(world, i.clone(), k.clone()); 
            let texture_model: (gl::types::GLuint, usize, gl::types::GLuint) = (raw_model.0, raw_model.1, world.loaded_textures);
            Chunk::set_chunk_model(&mut world.chunk_grid[i][k], texture_model);

            let raw_transparent_model: (gl::types::GLuint, usize) = get_raw_model_transparent(world, i.clone(), k.clone()); 
            let texture_transparent_model: (gl::types::GLuint, usize, gl::types::GLuint) = (raw_transparent_model.0, raw_transparent_model.1, world.loaded_textures);
            Chunk::set_transparent_chunk_model(&mut world.chunk_grid[i][k], texture_transparent_model);
        }
    }
}

fn build_mesh_single(world: &mut World, i: usize, k: usize){


    Chunk::build_mesh(&mut world.chunk_grid[i][k], &world.block_model);
    Chunk::populate_mesh(&mut world.chunk_grid[i][k]);

    let raw_model: (gl::types::GLuint, usize) = get_raw_model(world, i.clone(), k.clone()); 
    let texture_model: (gl::types::GLuint, usize, gl::types::GLuint) = (raw_model.0, raw_model.1, world.loaded_textures);

    Chunk::set_chunk_model(&mut world.chunk_grid[i][k], texture_model);

    let raw_transparent_model: (gl::types::GLuint, usize) = get_raw_model_transparent(world, i.clone(), k.clone()); 
    let texture_transparent_model: (gl::types::GLuint, usize, gl::types::GLuint) = (raw_transparent_model.0, raw_transparent_model.1, world.loaded_textures);
    Chunk::set_transparent_chunk_model(&mut world.chunk_grid[i][k], texture_transparent_model);
}

fn get_raw_model(world: &mut World, i: usize, k: usize) -> (gl::types::GLuint, usize){
    let vao_id = create_vao();
    
    //Vertices
    let mut vbo_id_vert: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo_id_vert);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id_vert);
        gl::BufferData(gl::ARRAY_BUFFER, (Chunk::get_vertices(&world.chunk_grid[i][k]).len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, Chunk::get_vertices(&world.chunk_grid[i][k]).as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW);
        gl::VertexAttribPointer( 0, 3, gl::FLOAT, gl::FALSE, (3 * std::mem::size_of::<f32>()) as gl::types::GLint, std::ptr::null());
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    //Brightness
    let mut vbo_id_bright: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo_id_bright);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id_bright);
        gl::BufferData(gl::ARRAY_BUFFER, (Chunk::get_brightnesses(&world.chunk_grid[i][k]).len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, Chunk::get_brightnesses(&world.chunk_grid[i][k]).as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW);
        gl::VertexAttribPointer( 1, 1, gl::FLOAT, gl::FALSE, (1 * std::mem::size_of::<f32>()) as gl::types::GLint, std::ptr::null());
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    //UV's
    let mut vbo_id_tex: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo_id_tex);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id_tex);
        gl::BufferData(gl::ARRAY_BUFFER, (Chunk::get_uv(&world.chunk_grid[i][k]).len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, Chunk::get_uv(&world.chunk_grid[i][k]).as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW);
        gl::VertexAttribPointer( 2, 2, gl::FLOAT, gl::FALSE, (2 * std::mem::size_of::<f32>()) as gl::types::GLint, std::ptr::null());
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    //Opacity
    let mut vbo_id_opacity: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo_id_opacity);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id_opacity);
        gl::BufferData(gl::ARRAY_BUFFER, (Chunk::get_opacity(&world.chunk_grid[i][k]).len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, Chunk::get_opacity(&world.chunk_grid[i][k]).as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW);
        gl::VertexAttribPointer( 3, 1, gl::FLOAT, gl::FALSE, (1 * std::mem::size_of::<f32>()) as gl::types::GLint, std::ptr::null());
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    Chunk::set_vao_vbo(&mut world.chunk_grid[i][k], vao_id, vbo_id_vert, vbo_id_tex, vbo_id_bright, vbo_id_opacity);
    return (vao_id, Chunk::get_vertices(&world.chunk_grid[0][0]).len());
}

fn get_raw_model_transparent(world: &mut World, i: usize, k: usize) -> (gl::types::GLuint, usize){
    let vao_id = create_vao();
    
    //Vertices
    let mut vbo_id_vert: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo_id_vert);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id_vert);
        gl::BufferData(gl::ARRAY_BUFFER, (Chunk::get_transparent_vertices(&world.chunk_grid[i][k]).len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, Chunk::get_transparent_vertices(&world.chunk_grid[i][k]).as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW);
        gl::VertexAttribPointer( 0, 3, gl::FLOAT, gl::FALSE, (3 * std::mem::size_of::<f32>()) as gl::types::GLint, std::ptr::null());
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    //Brightness
    let mut vbo_id_bright: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo_id_bright);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id_bright);
        gl::BufferData(gl::ARRAY_BUFFER, (Chunk::get_transparent_brightnesses(&world.chunk_grid[i][k]).len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, Chunk::get_transparent_brightnesses(&world.chunk_grid[i][k]).as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW);
        gl::VertexAttribPointer( 1, 1, gl::FLOAT, gl::FALSE, (1 * std::mem::size_of::<f32>()) as gl::types::GLint, std::ptr::null());
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    //UV's
    let mut vbo_id_tex: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo_id_tex);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id_tex);
        gl::BufferData(gl::ARRAY_BUFFER, (Chunk::get_transparent_uv(&world.chunk_grid[i][k]).len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, Chunk::get_transparent_uv(&world.chunk_grid[i][k]).as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW);
        gl::VertexAttribPointer( 2, 2, gl::FLOAT, gl::FALSE, (2 * std::mem::size_of::<f32>()) as gl::types::GLint, std::ptr::null());
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    //Opacity
    let mut vbo_id_opacity: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo_id_opacity);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id_opacity);
        gl::BufferData(gl::ARRAY_BUFFER, (Chunk::get_transparent_opacity(&world.chunk_grid[i][k]).len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, Chunk::get_transparent_opacity(&world.chunk_grid[i][k]).as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW);
        gl::VertexAttribPointer( 3, 1, gl::FLOAT, gl::FALSE, (1 * std::mem::size_of::<f32>()) as gl::types::GLint, std::ptr::null());
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    Chunk::set_transparent_vao_vbo(&mut world.chunk_grid[i][k], vao_id, vbo_id_vert, vbo_id_tex, vbo_id_bright, vbo_id_opacity);
    return (vao_id, Chunk::get_transparent_vertices(&world.chunk_grid[0][0]).len());
}

fn create_vao() -> gl::types::GLuint{
    let mut vao_id: gl::types::GLuint = 0;
    unsafe{
        gl::GenVertexArrays(1, &mut vao_id);
        gl::BindVertexArray(vao_id);
    }
    return vao_id;
}