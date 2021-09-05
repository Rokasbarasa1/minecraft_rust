extern crate image;
extern crate noise;
extern crate stopwatch;
use crate::render_gl;
pub mod chunk;
pub mod block_model;
use self::{block_model::BlockModel, chunk::block};
use std::{ffi::c_void, u32};
use block::Block;
use chunk::Chunk;
use stopwatch::Stopwatch;
use std::collections::HashMap;

pub struct World {
    pub chunk_width: usize,
    pub loaded_textures: gl::types::GLuint,
    pub chunk_grid: Vec<Vec<Chunk>>,
    pub block_model: BlockModel,
    pub view_distance: f32,
    pub program: render_gl::Program,
    //Index of i, index of k, check visibility, rebuild the block, rebuilt bool
    pub unbuilt_models: Vec<(usize, usize, bool, bool, bool)>,
    pub set_blocks: HashMap<String, u8>,
    pub change_block: Vec<(usize, usize, usize, usize, usize, u8)>
}

impl World{
    pub fn new(camera_position: &glm::Vector3<f32>, program: &render_gl::Program, square_chunk_width: &usize, chunks_layers_from_player: &usize, view_distance: &f32, world_gen_seed: &u32, mid_height: &u8, underground_height: &u8, sky_height: &u8, noise_resolution: &f32) -> World{
        let mut world = World{
            chunk_width: square_chunk_width.clone(),
            loaded_textures: 0,
            chunk_grid: vec![],
            block_model: block_model::BlockModel::init(),
            view_distance: ((chunks_layers_from_player - 1) / 2 * square_chunk_width + square_chunk_width) as f32, //+  (square_chunk_width.clone() as f32 * chunks_layers_from_player.clone() as f32) - *chunks_layers_from_player as f32,
            program: program.clone(),
            unbuilt_models: vec![],
            set_blocks: HashMap::new(),
            change_block: vec![]
        };

        //LOAD TEXTURES
        setup_texture(&mut world);
        
        //LOAD CHUNKS AROUND PLAYER
        generate_chunks(&mut world.chunk_grid, &mut world.change_block, camera_position, &square_chunk_width, &chunks_layers_from_player, world_gen_seed, mid_height, &mut world.set_blocks, underground_height, sky_height, noise_resolution);

        //CHECK CHUNK VISIBILITY 
        check_visibility(&mut world);

        //BUILD CHUNK MESH 
        build_mesh(&mut world);

        return world;
    }

    pub fn draw(&mut self, camera_pos: &glm::Vector3<f32>){


        if self.unbuilt_models.len() != 0 {
            //This refresh breaks because the old chunks are replaced by new

            if self.unbuilt_models.len() > 1 && self.unbuilt_models[0].3 && !self.unbuilt_models[0].4{
                self.chunk_grid[self.unbuilt_models[0].0][self.unbuilt_models[0].1].regenerate(&mut self.change_block, self.unbuilt_models[0].0, self.unbuilt_models[0].1, &mut self.set_blocks);
                self.unbuilt_models[0].4 = true;
            }

            if self.unbuilt_models[0].2{
                check_chunk_visibility(self, self.unbuilt_models[0].0, self.unbuilt_models[0].1);
            }

            build_mesh_single(self, self.unbuilt_models[0].0, self.unbuilt_models[0].1);
            self.unbuilt_models.remove(0);
        }

        
        // Set any blocks. Mostly leaves
        if self.change_block.len() != 0 && self.unbuilt_models.len() == 0{

            for i in 0..self.change_block.len(){                
                self.chunk_grid[self.change_block[i].0][self.change_block[i].1].blocks[self.change_block[i].2][self.change_block[i].3][self.change_block[i].4].id = self.change_block[i].5;

                check_blocks_around_block(self, self.change_block[i].0, self.change_block[i].1, self.change_block[i].2, self.change_block[i].3, self.change_block[i].4);
            }

            self.change_block.clear();
            for k in 0..self.unbuilt_models.len(){
                build_mesh_single(self, self.unbuilt_models[k].0, self.unbuilt_models[k].1);
            }
            self.unbuilt_models.clear()
        }

        self.program.set_used();
        unsafe {
            gl::Enable(gl::CULL_FACE);
        }

        let mut change_direction: usize = 0;

        for i in 0..self.chunk_grid.len(){
            for k in 0..self.chunk_grid[i].len(){
                self.program.set_used();
                let chunk_model = self.chunk_grid[i][k].chunk_model;
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
                if change_direction == 0 && self.unbuilt_models.len() == 0 && !distance(self.view_distance, &camera_pos, &self.chunk_grid[i][k].position){
                    change_direction = get_direction(&camera_pos, &self.chunk_grid[i][k].position);
                }
            }
        }
        unsafe {
            gl::Disable(gl::CULL_FACE);
        }

        //Seperate render call for partialy transparent objects
        for i in 0..self.chunk_grid.len(){
            for k in 0..self.chunk_grid[i].len(){
                self.program.set_used();
                let transparent_chunk_model = self.chunk_grid[i][k].transparent_chunk_model;
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
            }
        }

        let length: usize = self.chunk_grid.len();
        if change_direction != 0 {
            for i in 0..length{
                if change_direction == 2 || change_direction == 4{
                    match change_direction {
                        2 => {
                            // -X
                            if i != self.chunk_grid.len()-1{
                                if i == 0{
                                    self.chunk_grid.swap(length-1, 0);
                                    
                                    for k in 0..length{
                                        let mut new_position = self.chunk_grid[length-1][k].position.clone();
                                        let mut grid = self.chunk_grid[length-1][k].get_grid().clone();
                                        new_position.z += self.chunk_width as f32;
                                        grid.0 -= 1;
                                        
                                        self.chunk_grid[0][k].grid_x = grid.0;
                                        self.chunk_grid[0][k].grid_z = grid.1;
                                        self.chunk_grid[0][k].position = new_position;
                                        
                                        self.unbuilt_models.push((0,k,true, true, false));
                                        self.unbuilt_models.push((1,k,true, false, false));
                                        self.unbuilt_models.push((length-1,k,true, false, false));
                                    }
                                }else{
                                    self.chunk_grid.swap(length - i, length - i - 1);
                                }
                            }
                        },
                        4 =>  { //+X
                            if i != self.chunk_grid.len()-1{
                                if i == 0{
                                    
                                    self.chunk_grid.swap(0, length-1);

                                    for k in 0..length{
                                        let mut new_position = self.chunk_grid[0][k].position.clone();
                                        let mut grid = self.chunk_grid[0][k].get_grid().clone();
                                        new_position.z -= self.chunk_width as f32;
                                        grid.0 += 1;

                                        self.chunk_grid[length-1][k].grid_x = grid.0;
                                        self.chunk_grid[length-1][k].grid_z = grid.1;
                                        self.chunk_grid[length-1][k].position = new_position;
                                        
                                        self.unbuilt_models.push((length-1,k,true, true, false));
                                        self.unbuilt_models.push((length-2,k,true, false, false));
                                        self.unbuilt_models.push((0,k,true, false, false));
                                    }
                                    
                                }else{
                                    self.chunk_grid.swap(i, i-1);
                                }
                            }
                        },
                        _ => println!("KKA")
                    }
                }else{
                    for k in 0..length{
                        match change_direction {
                            1 => { //-Z
                                if k == 0{
                                    self.chunk_grid[i].swap(length-1, 0);

                                    let mut new_position = self.chunk_grid[i][length-1].position.clone();
                                    let mut grid = self.chunk_grid[i][length-1].get_grid().clone();
                                    new_position.x += self.chunk_width as f32;
                                    grid.1 -= 1;

                                    self.chunk_grid[i][0].grid_x = grid.0;
                                    self.chunk_grid[i][0].grid_z = grid.1;
                                    self.chunk_grid[i][0].position = new_position;
                                        
                                    self.unbuilt_models.push((i,0,true, true, false));
                                    self.unbuilt_models.push((i,1,true, false, false));
                                    self.unbuilt_models.push((i,length-1,true, false, false));

                                }else{
                                    if k != length-1{
                                        self.chunk_grid[i].swap(length - k, length - k - 1);
                                    }
                                }
                            },
                            3 =>  { // +Z
                                if k == 0{
                                    self.chunk_grid[i].swap(0, length-1);

                                    let mut new_position = self.chunk_grid[i][0].position.clone();
                                    let mut grid = self.chunk_grid[i][0].get_grid().clone();
                                    new_position.x -= self.chunk_width as f32;
                                    grid.1 += 1; 

                                    self.chunk_grid[i][length-1].grid_x = grid.0;
                                    self.chunk_grid[i][length-1].grid_z = grid.1;
                                    self.chunk_grid[i][length-1].position = new_position;
                                        
                                    self.unbuilt_models.push((i,length-1,true, true, false));
                                    self.unbuilt_models.push((i,length-2,true, false, false));
                                    self.unbuilt_models.push((i,0,true, false, false));

                                }else{
                                    if k != length-1{
                                        self.chunk_grid[i].swap(k, k-1);
                                    }
                                }
                            },
                            _ => println!("KKA")
                        }
                    }
                }
            }
            // stopwatch.stop();
            
            // println!("Move end ms {}", stopwatch.elapsed_ms());
        }
    }

    pub fn destroy_block(&mut self, camera_front: &glm::Vector3<f32>, camera_pos: &glm::Vector3<f32>){
        let (position, mut end, direction) = (camera_pos.clone(), camera_pos.clone(), camera_front.clone());
        while glm::distance(position.clone(), end.clone()) < 6.0 {
            let block_index = get_block(&self, &end);
            if block_index.0 != 9999 && block_index.1 != 9999 && block_index.2 != 9999 && block_index.3 != 9999 && block_index.4 != 9999 {
                
                //Setting block in saved blocks
                let grid = self.chunk_grid[block_index.0][block_index.1].get_grid();
                check_and_set_block(&mut self.set_blocks, grid.0, grid.1, block_index.2, block_index.3, block_index.4, 240);
                
                let mut block = &mut self.chunk_grid[block_index.0][block_index.1].blocks[block_index.2][block_index.3][block_index.4];
                
                block.id  = 240;  //240 is air
                block.visible = false;
                check_blocks_around_block(self, block_index.0, block_index.1, block_index.2, block_index.3, block_index.4);
                break;

            }
            ray_step(&mut end, &direction, 0.1)
        }
    }

    pub fn place_block(&mut self, camera_front: &glm::Vector3<f32>, camera_pos: &glm::Vector3<f32>, selected_block: u8, player_height: f32 ){
        let (position, mut end, direction) = (camera_pos.clone(), camera_pos.clone(), camera_front.clone());
        let mut last_air_block_index: (usize, usize, usize, usize, usize) = (0,0,0,0,0);
        while glm::distance(position.clone(), end.clone()) < 6.0 {
            let block_index = get_block_or_air(&self, &end);
            
            if block_index.0 != 9999 && block_index.1 != 9999 && block_index.2 != 9999 && block_index.3 != 9999 && block_index.4 != 9999 {

                let block = &self.chunk_grid[block_index.0][block_index.1].blocks[block_index.2][block_index.3][block_index.4];

                if Block::is_air(&block) || Block::is_water(&block) {
                    last_air_block_index = block_index.clone();
                }else{
                    if is_player_in_block_location(self, camera_pos, player_height, last_air_block_index.0, last_air_block_index.1, last_air_block_index.2, last_air_block_index.3, last_air_block_index.4){
                        break;
                    }

                    //Setting block in saved blocks
                    let grid = self.chunk_grid[last_air_block_index.0][last_air_block_index.1].get_grid();
                    check_and_set_block(&mut self.set_blocks, grid.0, grid.1, block_index.2, block_index.3, block_index.4, selected_block);

                    let mut block = &mut self.chunk_grid[last_air_block_index.0][last_air_block_index.1].blocks[last_air_block_index.2][last_air_block_index.3][last_air_block_index.4];
                    block.visible = false;
                    block.id = selected_block;
                    check_blocks_around_block(self, last_air_block_index.0, last_air_block_index.1, last_air_block_index.2, last_air_block_index.3, last_air_block_index.4);
                    break;
                }
            }
            ray_step(&mut end, &direction, 0.1)
        }
    }

    // returns values based on what the block type 
    // 0 for nothing
    // 1 for liquid
    // 2 solid block
    pub fn move_to_direction(&self, &desired_position: &glm::Vector3<f32>, player_height: f32, margin_for_player: f32 ) -> usize {
        let mut block_up:usize = 0;
        let mut block_down:usize = 0; 

        let mut block_index = get_block_or_water(self, &desired_position, margin_for_player);
        if block_index.0 != 9999 && block_index.1 != 9999 && block_index.2 != 9999 && block_index.3 != 9999 && block_index.4 != 9999 {
            if self.chunk_grid[block_index.0][block_index.1].blocks[block_index.2][block_index.3][block_index.4].is_water(){
                block_up = 1;
            }else{
                block_up = 2;
            }
        }

        block_index = get_block_or_water(self, &glm::vec3(desired_position.x, desired_position.y - player_height, desired_position.z), margin_for_player);
        if block_index.0 != 9999 && block_index.1 != 9999 && block_index.2 != 9999 && block_index.3 != 9999 && block_index.4 != 9999 {
            if self.chunk_grid[block_index.0][block_index.1].blocks[block_index.2][block_index.3][block_index.4].is_water(){
                block_down = 1;
            }else{
                block_down = 2;
            }
        }


        if block_up == 1 && block_down == 2{
            // println!("In water/ on block");
            return 3;
        }else if block_up == 2 || block_down == 2{
            return 2;
        }else if block_up == 1 || block_down == 1{
            // println!("In water");
            return 1;
        } else {

            return 0;
        }
    }

    // Find where to place player above ground
    // recursion
    // 2 - solid block before
    // 1 - air/no block before
    // 0 - initial passed status
    pub fn get_spawn_location(&self, camera_pos: &glm::Vector3<f32>, status: usize) -> glm::Vector3<f32>{
        let block_index = get_block_or_air(self, &camera_pos);

        if block_index.0 != 9999 && block_index.1 != 9999 && block_index.2 != 9999 && block_index.3 != 9999 && block_index.4 != 9999 {
            let block = &self.chunk_grid[block_index.0][block_index.1].blocks[block_index.2][block_index.3][block_index.4];
            if Block::is_air(&block) {
                //Go down
                if status == 2 {
                    return glm::vec3(camera_pos.x, camera_pos.y+3.0, camera_pos.z)
                }else{
                    self.get_spawn_location(&glm::vec3(camera_pos.x, camera_pos.y-1.0, camera_pos.z), 1 as usize)
                }
            }else{
                //Go up
                if status == 1 {
                    return glm::vec3(camera_pos.x, camera_pos.y+3.0, camera_pos.z)
                }else{
                    self.get_spawn_location(&glm::vec3(camera_pos.x, camera_pos.y+1.0, camera_pos.z), 2 as usize)
                }
            }
        }else{
            //Go down
            if status == 2 {
                return glm::vec3(camera_pos.x, camera_pos.y+3.0, camera_pos.z)
            }else{
                self.get_spawn_location(&glm::vec3(camera_pos.x, camera_pos.y-1.0, camera_pos.z), 1 as usize)
            }
        }
    }
}

fn check_and_set_block(set_blocks: &mut HashMap<String, u8>, grid_x: i32, grid_z: i32, i: usize, k: usize, j: usize, id: u8){
    let key = [grid_x.to_string(), grid_z.to_string(), i.to_string(), k.to_string(), j.to_string()].join("");
    if set_blocks.contains_key(&key) {
        set_blocks.remove_entry(&key);
        set_blocks.insert(key, id);
    }else{
        set_blocks.insert(key, id);
    }
}

fn is_player_in_block_location(world: &World, camera_pos: &glm::Vector3<f32>, player_height: f32,  i: usize, k:usize, j: usize, l: usize, m: usize) -> bool{
    let block_index_up = get_block_or_air(world, &camera_pos);
    let block_index_down = get_block_or_air(world, &glm::vec3(camera_pos.x, camera_pos.y - player_height, camera_pos.z));

    if block_index_up.0 == i && block_index_up.1 == k && block_index_up.2 == j && block_index_up.3 == l && block_index_up.4 == m{
        return true;
    }

    if block_index_down.0 == i && block_index_down.1 == k && block_index_down.2 == j && block_index_down.3 == l && block_index_down.4 == m{
        return true;
    }
    return false;
}

fn get_block(world: &World, end: &glm::Vector3<f32>) -> (usize, usize, usize, usize, usize){
    let mut index_i: usize = 9999;
    let mut index_k: usize = 9999;
    let mut index_j: usize = 9999; 
    let mut index_l: usize = 9999; 
    let mut index_m: usize = 9999;
    let mut min:f32 = 9999.0;
    for i in 0..world.chunk_grid.len(){
        for k in 0..world.chunk_grid[i].len(){
            let position = world.chunk_grid[i][k].position.clone();
            let distance = glm::distance(glm::vec2(position.x, position.z), glm::vec2(end.x.clone(), end.z.clone()));
            let distance_x = (f32::powi(position.x - end.x, 2)).sqrt();
            let distance_y = (f32::powi(position.z - end.z, 2)).sqrt();

            if distance < min && distance_x < world.chunk_width as f32 / 2.0 && distance_y < world.chunk_width as f32 / 2.0{
                min = distance;
                index_i = i;
                index_k = k;
                break;
            }
        }
        if index_i != 9999 {
            break;
        }
    }
    if min != 9999.0{
        min = 9999.0;
        for j in 0..world.chunk_grid[index_i][index_k].blocks.len() {
            for l in 0..world.chunk_grid[index_i][index_k].blocks[j].len() {
                for m in 0..world.chunk_grid[index_i][index_k].blocks[j][l].len() {
                    if world.chunk_grid[index_i][index_k].blocks[j][l][m].visible && world.chunk_grid[index_i][index_k].blocks[j][l][m].id != 3 {
                        let position = world.chunk_grid[index_i][index_k].blocks[j][l][m].position.clone();
                        let distance = glm::distance(position.clone(), end.clone());
                        let distance_x = (f32::powi(position.x - end.x, 2)).sqrt();
                        let distance_y = (f32::powi(position.y - end.y, 2)).sqrt();
                        let distance_z = (f32::powi(position.z - end.z, 2)).sqrt();

                        if distance < min && distance_x < 0.5 && distance_y < 0.5 && distance_z < 0.5{
                            index_j = j; 
                            index_l = l; 
                            index_m = m; 
                            return (index_i,index_k,index_j,index_l,index_m)
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

fn get_block_or_water(world: &World, end: &glm::Vector3<f32>, margin_for_player: f32) -> (usize, usize, usize, usize, usize){
    let mut index_i: usize = 9999;
    let mut index_k: usize = 9999;
    let mut index_j: usize = 9999; 
    let mut index_l: usize = 9999; 
    let mut index_m: usize = 9999;
    let mut min:f32 = 9999.0;
    for i in 0..world.chunk_grid.len(){
        for k in 0..world.chunk_grid[i].len(){
            let position = world.chunk_grid[i][k].position.clone();
            let distance = glm::distance(glm::vec2(position.x, position.z), glm::vec2(end.x.clone(), end.z.clone()));
            let distance_x = (f32::powi(position.x - end.x, 2)).sqrt();
            let distance_y = (f32::powi(position.z - end.z, 2)).sqrt();

            if distance < min && distance_x < world.chunk_width as f32 / 2.0 && distance_y < world.chunk_width as f32 / 2.0{
                min = distance;
                index_i = i;
                index_k = k;
                break;
            }
        }
        if index_i != 9999 {
            break;
        }
    }
    if min != 9999.0{
        min = 9999.0;
        for j in 0..world.chunk_grid[index_i][index_k].blocks.len() {
            for l in 0..world.chunk_grid[index_i][index_k].blocks[j].len() {
                for m in 0..world.chunk_grid[index_i][index_k].blocks[j][l].len() {
                    if world.chunk_grid[index_i][index_k].blocks[j][l][m].visible || world.chunk_grid[index_i][index_k].blocks[j][l][m].is_water(){
                        let position = world.chunk_grid[index_i][index_k].blocks[j][l][m].position.clone();
                        let distance = glm::distance(position.clone(), end.clone());
                        let distance_x = (f32::powi(position.x - end.x, 2)).sqrt();
                        let distance_y = (f32::powi(position.y - end.y, 2)).sqrt();
                        let distance_z = (f32::powi(position.z - end.z, 2)).sqrt();

                        if distance < min && distance_x < 0.5 + margin_for_player && distance_y < 0.5 && distance_z < 0.5 + margin_for_player{
                            index_j = j; 
                            index_l = l; 
                            index_m = m; 
                            return (index_i,index_k,index_j,index_l,index_m)
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

fn get_block_or_air(world: &World, end: &glm::Vector3<f32>) -> (usize, usize, usize, usize, usize){
    let mut index_i: usize = 9999;
    let mut index_k: usize = 9999;
    let mut index_j: usize = 9999; 
    let mut index_l: usize = 9999; 
    let mut index_m: usize = 9999;
    let mut min:f32 = 9999.0;
    for i in 0..world.chunk_grid.len(){
        for k in 0..world.chunk_grid[i].len(){
            let position = world.chunk_grid[i][k].position.clone();
            let distance = glm::distance(glm::vec2(position.x, position.z), glm::vec2(end.x.clone(), end.z.clone()));
            let distance_x = (f32::powi(position.x - end.x, 2)).sqrt();
            let distance_y = (f32::powi(position.z - end.z, 2)).sqrt();

            if distance < min && distance_x < world.chunk_width as f32 / 2.0 && distance_y < world.chunk_width as f32 / 2.0{
                min = distance;
                index_i = i;
                index_k = k;
            }
        }
        if index_i != 9999 {
            break;
        }
    }
    if min != 9999.0{
        min = 9999.0;
        for j in 0..world.chunk_grid[index_i][index_k].blocks.len() {
            for l in 0..world.chunk_grid[index_i][index_k].blocks[j].len() {
                for m in 0..world.chunk_grid[index_i][index_k].blocks[j][l].len() {
                    let position = world.chunk_grid[index_i][index_k].blocks[j][l][m].position.clone();
                    let distance = glm::distance(position.clone(), end.clone());
                    let distance_x = (f32::powi(position.x - end.x, 2)).sqrt();
                    let distance_y = (f32::powi(position.y - end.y, 2)).sqrt();
                    let distance_z = (f32::powi(position.z - end.z, 2)).sqrt();

                    if distance < min && distance_x < 0.5 && distance_y < 0.5 && distance_z < 0.5{
                        index_j = j; 
                        index_l = l; 
                        index_m = m; 
                        return (index_i,index_k,index_j,index_l,index_m)
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


fn generate_chunks(chunk_grid: &mut Vec<Vec<Chunk>>, change_block: &mut Vec<(usize, usize, usize, usize, usize, u8)>, camera_position: &glm::Vector3<f32>, square_chunk_width: &usize, render_out_from_player: &usize, world_gen_seed: &u32, mid_height: &u8, set_blocks: &mut HashMap<String, u8>, underground_height: &u8, sky_height: &u8, noise_resolution: &f32){
    let adjustment = (*render_out_from_player as f32 / 2.0).floor() as f32 * square_chunk_width.clone() as f32 + (*square_chunk_width as f32 / 2.0);
    let mut x_pos = camera_position.x + adjustment;
    let mut z_pos = camera_position.z + adjustment;
    let x_pos_temp = z_pos;

    let chunk_width;
    if render_out_from_player % 2 == 0 {
        chunk_width = render_out_from_player + 1;
    }else {
        chunk_width = *render_out_from_player;
    }

    for i in 0..chunk_width{  //Z line Go from positive to negative
        let collumn: Vec<chunk::Chunk> = vec![];
        chunk_grid.push(collumn);
        for k in 0..chunk_width{  //X line Go from positive to negative
            let chunk = chunk::Chunk::init(change_block, i, k, i as i32, k as i32, glm::vec3(x_pos.clone(), -10.0, z_pos.clone()), square_chunk_width, world_gen_seed, mid_height, set_blocks, underground_height, sky_height, noise_resolution, chunk_width);
            chunk_grid[i].push(chunk);
            x_pos -= *square_chunk_width as f32;
        }
        x_pos = x_pos_temp;
        z_pos -= *square_chunk_width as f32 ;
    }
}

fn distance(max_distance: f32, point1: &glm::Vector3<f32>, point2: &glm::Vector3<f32>) -> bool{
    return (f32::powi(point1.x.clone() - point2.x.clone(), 2)).sqrt().floor() <= max_distance && (f32::powi(point1.z.clone() - point2.z.clone(), 2)).sqrt().floor() <= max_distance
}

fn get_direction(point1: &glm::Vector3<f32>, point2: &glm::Vector3<f32>) -> usize{
    let mut x_distance = point1.x.clone() - point2.x.clone();
    let mut z_distance = point1.z.clone() - point2.z.clone();
    let mut x_negative: bool = false;
    let mut z_negative: bool = false;

    //X is false, z is true
    let mut bigger: bool = false;
    if x_distance < 0.0 {
        x_negative = true;
        x_distance *= -1.0;
    }

    if z_distance < 0.0 {
        z_negative = true;
        z_distance *= -1.0;
    }


    if x_distance < z_distance {
        bigger = true;
    }
    
    if !bigger && !x_negative{ // Positive x
        1
    }else if !bigger && x_negative{ // Negative x
        3
    }else if bigger && !z_negative{ // Positive z
        2
    }else { // Negative z
        4
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
    // let mut stopwatch = Stopwatch::new();

    for i in 0..world.chunk_grid.len() {
        for k in 0..world.chunk_grid[i].len() {
            // stopwatch.reset();
            // stopwatch.start();
            check_chunk_visibility(world, i, k);
            // stopwatch.stop();
            // println!("visibility {} ms", stopwatch.elapsed_ms())
        }
    }
}

fn check_chunk_visibility(world: &mut World, i: usize, k: usize){
    for j in 0..world.chunk_grid[i][k].blocks.len() {
        for l in 0..world.chunk_grid[i][k].blocks[j].len() {
            for m in 0..world.chunk_grid[i][k].blocks[j][l].len() {
                check_block_sides(&mut world.chunk_grid , i.clone(), k.clone(), j.clone(), l.clone(), m.clone(), world.chunk_width as usize);
            }
        }
    }
}

fn check_block_sides(chunk_grid: &mut Vec<Vec<Chunk>>, i: usize, k: usize, j: usize, l: usize, m: usize, chunk_width: usize){
    let block_id = chunk_grid[i][k].blocks[j][l][m].id;
    
    if chunk_grid[i][k].blocks[j][l][m].is_air() {
        chunk_grid[i][k].blocks[j][l][m].visible = false;
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
        if m == 0 { y_block_flag = 0 }else if m == chunk_grid[i][k].blocks[j][l].len()-1 { y_block_flag = 2 } else { y_block_flag = 1 }; //Y axis

        // I = Z chunk, K = X chunk, J = Z block, L = X block, M = Y block
        let mut sides: Vec<bool> = vec![];

        // //Z block go +
        if z_block_flag == 2{
            if z_chunk_flag == 2 || chunk_grid.len()-1 == 0 && z_chunk_flag == 0{
                sides.push(true);
            }else{
                if chunk_grid[i+1][k].blocks[0][l][m].is_air() || block_id != 3 && chunk_grid[i+1][k].blocks[0][l][m].is_water(){ 
                    sides.push(true); 
                }else {
                    sides.push(false); 
                }
            }
        }else{
            if chunk_grid[i][k].blocks[j+1][l][m].is_air() || block_id != 3 && chunk_grid[i][k].blocks[j+1][l][m].is_water() { 
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
                if chunk_grid[i-1][k].blocks[chunk_width-1][l][m].is_air() || block_id != 3 && chunk_grid[i-1][k].blocks[chunk_width-1][l][m].is_water() { 
                    sides.push(true); 
                }else {
                    sides.push(false); 
                }
            }
        }else{
            if chunk_grid[i][k].blocks[j-1][l][m].is_air() || block_id != 3 && chunk_grid[i][k].blocks[j-1][l][m].is_water() { 
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
                if chunk_grid[i][k+1].blocks[j][0][m].is_air() || block_id != 3 && chunk_grid[i][k+1].blocks[j][0][m].is_water()  { 
                    sides.push(true); 
                }else {
                    sides.push(false); 
                }
            }
        }else{
            if chunk_grid[i][k].blocks[j][l+1][m].is_air() || block_id != 3 && chunk_grid[i][k].blocks[j][l+1][m].is_water()  { 
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
                if chunk_grid[i][k-1].blocks[j][chunk_width-1][m].is_air() || block_id != 3 && chunk_grid[i][k-1].blocks[j][chunk_width-1][m].is_water()  { 
                    sides.push(true); 
                }else {
                    sides.push(false); 
                }
            }
        }else{
            if chunk_grid[i][k].blocks[j][l-1][m].is_air() || block_id != 3 && chunk_grid[i][k].blocks[j][l-1][m].is_water() { 
                sides.push(true); 
            }else {
                sides.push(false); 
            }
        }


        // Y block go -
        if y_block_flag == 0{
            sides.push(true);
        }else{
            if chunk_grid[i][k].blocks[j][l][m-1].is_air() || block_id != 3 && chunk_grid[i][k].blocks[j][l][m-1].is_water()  { 
                sides.push(true); 
            }else {
                sides.push(false); 
            }
        }

        // Y block go +
        if y_block_flag == 2 || 0 == chunk_grid[i][k].blocks[j][l].len()-1{
            sides.push(true);
        }else{
            if chunk_grid[i][k].blocks[j][l][m+1].is_air() || block_id != 3 && chunk_grid[i][k].blocks[j][l][m+1].is_water()  { 
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
            chunk_grid[i][k].blocks[j][l][m].visible = true;
            chunk_grid[i][k].blocks[j][l][m].sides = sides;
        }else{
            chunk_grid[i][k].blocks[j][l][m].visible = false;
        }
    } 
}

fn check_blocks_around_block(world: &mut World, i: usize, k: usize, j: usize, l: usize, m: usize){
    check_block_sides(&mut world.chunk_grid, i, k, j, l, m, world.chunk_width as usize);
    //Check up 
    if world.chunk_grid[i][k].blocks[j][l].len() != 1 && m != world.chunk_grid[i][k].blocks[j][l].len() -1  && m < world.chunk_grid[i][k].blocks[j][l].len() + 1 {
        check_block_sides(&mut world.chunk_grid, i, k, j, l, m+1, world.chunk_width as usize);
    }
    //Check down
    if world.chunk_grid[i][k].blocks[j][l].len() != 1 && m != 0 {
        check_block_sides(&mut world.chunk_grid, i, k, j, l, m-1, world.chunk_width as usize);
    }


    //Check left
    if j != 0  {
        check_block_sides(&mut world.chunk_grid, i, k, j-1, l, m, world.chunk_width as usize);
    }
    else if i != 0 {
        check_block_sides(&mut world.chunk_grid, i-1, k, world.chunk_width as usize-1, l, m, world.chunk_width as usize);
        push_unbuilt_to_start((i-1,k,false,false,false), &mut world.unbuilt_models);
    }
    //Check right
    if j != world.chunk_width as usize -1 {
        check_block_sides(&mut world.chunk_grid, i, k, j+1, l, m, world.chunk_width as usize);
    }else if i != world.chunk_grid.len()-1 {
        check_block_sides(&mut world.chunk_grid, i+1, k, 0, l, m, world.chunk_width as usize);
        push_unbuilt_to_start((i+1,k,false,false,false), &mut world.unbuilt_models);
    } 


    //Check front 
    if l != 0  {
        check_block_sides(&mut world.chunk_grid, i, k, j, l-1, m, world.chunk_width as usize);
    }else if k != 0{
        check_block_sides(&mut world.chunk_grid, i, k-1, j, world.chunk_width as usize-1, m, world.chunk_width as usize);
        push_unbuilt_to_start((i,k-1,false,false,false), &mut world.unbuilt_models);

    }
    //Check back
    if l != world.chunk_width as usize -1{
        check_block_sides(&mut world.chunk_grid, i, k, j, l+1, m, world.chunk_width as usize);
    }else if k != world.chunk_grid[i].len()-1 {
        check_block_sides(&mut world.chunk_grid, i, k+1, j, 0, m, world.chunk_width as usize);
        push_unbuilt_to_start((i,k+1,false,false,false), &mut world.unbuilt_models);
    }

    push_unbuilt_to_start((i,k,false,false,false), &mut world.unbuilt_models);
}


fn build_mesh(world: &mut World){
    for i in 0..world.chunk_grid.len() {
        for k in 0..world.chunk_grid[i].len() {
            Chunk::build_mesh(&mut world.chunk_grid[i][k], &world.block_model);
            Chunk::populate_mesh(&mut world.chunk_grid[i][k]);
            let raw_model: (gl::types::GLuint, usize) = get_raw_model(world, i.clone(), k.clone()); 
            let texture_model: (gl::types::GLuint, usize, gl::types::GLuint) = (raw_model.0, raw_model.1, world.loaded_textures);
            world.chunk_grid[i][k].chunk_model = texture_model;

            let raw_transparent_model: (gl::types::GLuint, usize) = get_raw_model_transparent(world, i.clone(), k.clone()); 
            let texture_transparent_model: (gl::types::GLuint, usize, gl::types::GLuint) = (raw_transparent_model.0, raw_transparent_model.1, world.loaded_textures);
            world.chunk_grid[i][k].transparent_chunk_model = texture_transparent_model;
        }
    }
}

fn build_mesh_single(world: &mut World, i: usize, k: usize){
    Chunk::build_mesh(&mut world.chunk_grid[i][k], &world.block_model);
    Chunk::populate_mesh(&mut world.chunk_grid[i][k]);

    let raw_model: (gl::types::GLuint, usize) = get_raw_model(world, i.clone(), k.clone()); 
    let texture_model: (gl::types::GLuint, usize, gl::types::GLuint) = (raw_model.0, raw_model.1, world.loaded_textures);

    world.chunk_grid[i][k].chunk_model = texture_model;

    let raw_transparent_model: (gl::types::GLuint, usize) = get_raw_model_transparent(world, i.clone(), k.clone());
    let texture_transparent_model: (gl::types::GLuint, usize, gl::types::GLuint) = (raw_transparent_model.0, raw_transparent_model.1, world.loaded_textures);
    world.chunk_grid[i][k].transparent_chunk_model = texture_transparent_model
}

// Open gl stuff
fn get_raw_model(world: &mut World, i: usize, k: usize) -> (gl::types::GLuint, usize){
    let vao_id = create_vao();
    
    //Vertices
    let mut vbo_id_vert: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo_id_vert);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id_vert);
        gl::BufferData(gl::ARRAY_BUFFER, (world.chunk_grid[i][k].positions.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, world.chunk_grid[i][k].positions.as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW);
        gl::VertexAttribPointer( 0, 3, gl::FLOAT, gl::FALSE, (3 * std::mem::size_of::<f32>()) as gl::types::GLint, std::ptr::null());
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    //Brightness
    let mut vbo_id_bright: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo_id_bright);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id_bright);
        gl::BufferData(gl::ARRAY_BUFFER, (world.chunk_grid[i][k].brightness.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, world.chunk_grid[i][k].brightness.as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW);
        gl::VertexAttribPointer( 1, 1, gl::FLOAT, gl::FALSE, (1 * std::mem::size_of::<f32>()) as gl::types::GLint, std::ptr::null());
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    //UV's
    let mut vbo_id_tex: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo_id_tex);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id_tex);
        gl::BufferData(gl::ARRAY_BUFFER, (world.chunk_grid[i][k].uvs.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, world.chunk_grid[i][k].uvs.as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW);
        gl::VertexAttribPointer( 2, 2, gl::FLOAT, gl::FALSE, (2 * std::mem::size_of::<f32>()) as gl::types::GLint, std::ptr::null());
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    //Opacity
    let mut vbo_id_opacity: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo_id_opacity);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id_opacity);
        gl::BufferData(gl::ARRAY_BUFFER, (world.chunk_grid[i][k].opacity.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, world.chunk_grid[i][k].opacity.as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW);
        gl::VertexAttribPointer( 3, 1, gl::FLOAT, gl::FALSE, (1 * std::mem::size_of::<f32>()) as gl::types::GLint, std::ptr::null());
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    Chunk::set_vao_vbo(&mut world.chunk_grid[i][k], vao_id, vbo_id_vert, vbo_id_tex, vbo_id_bright, vbo_id_opacity);
    return (vao_id, world.chunk_grid[i][k].opacity.len());
}

fn get_raw_model_transparent(world: &mut World, i: usize, k: usize) -> (gl::types::GLuint, usize){
    let vao_id = create_vao();
    
    //Vertices
    let mut vbo_id_vert: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo_id_vert);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id_vert);
        gl::BufferData(gl::ARRAY_BUFFER, (world.chunk_grid[i][k].transparent_positions.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, world.chunk_grid[i][k].transparent_positions.as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW);
        gl::VertexAttribPointer( 0, 3, gl::FLOAT, gl::FALSE, (3 * std::mem::size_of::<f32>()) as gl::types::GLint, std::ptr::null());
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    //Brightness
    let mut vbo_id_bright: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo_id_bright);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id_bright);
        gl::BufferData(gl::ARRAY_BUFFER, (world.chunk_grid[i][k].transparent_brightness.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, world.chunk_grid[i][k].transparent_brightness.as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW);
        gl::VertexAttribPointer( 1, 1, gl::FLOAT, gl::FALSE, (1 * std::mem::size_of::<f32>()) as gl::types::GLint, std::ptr::null());
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    //UV's
    let mut vbo_id_tex: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo_id_tex);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id_tex);
        gl::BufferData(gl::ARRAY_BUFFER, (world.chunk_grid[i][k].transparent_uvs.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, world.chunk_grid[i][k].transparent_uvs.as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW);
        gl::VertexAttribPointer( 2, 2, gl::FLOAT, gl::FALSE, (2 * std::mem::size_of::<f32>()) as gl::types::GLint, std::ptr::null());
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    //Opacity
    let mut vbo_id_opacity: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo_id_opacity);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id_opacity);
        gl::BufferData(gl::ARRAY_BUFFER, (world.chunk_grid[i][k].transparent_opacity.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, world.chunk_grid[i][k].transparent_opacity.as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW);
        gl::VertexAttribPointer( 3, 1, gl::FLOAT, gl::FALSE, (1 * std::mem::size_of::<f32>()) as gl::types::GLint, std::ptr::null());
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    Chunk::set_transparent_vao_vbo(&mut world.chunk_grid[i][k], vao_id, vbo_id_vert, vbo_id_tex, vbo_id_bright, vbo_id_opacity);
    return (vao_id, world.chunk_grid[i][k].transparent_opacity.len());
}

fn create_vao() -> gl::types::GLuint{
    let mut vao_id: gl::types::GLuint = 0;
    unsafe{
        gl::GenVertexArrays(1, &mut vao_id);
        gl::BindVertexArray(vao_id);
    }
    return vao_id;
}

fn push_unbuilt_to_start(unbuilt: (usize, usize, bool, bool, bool), vector: &mut Vec<(usize, usize, bool, bool, bool)>){
    let mut unbuilt_part: Vec<(usize, usize, bool, bool, bool)> = vec![unbuilt];
    unbuilt_part.append(vector);
    *vector = unbuilt_part
}