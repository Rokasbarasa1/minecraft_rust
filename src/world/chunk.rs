extern crate noise;
use glm::ext::powi;
use rand::Rng;
use rand::rngs::StdRng;
use rand::{SeedableRng};
use noise::{Blend, NoiseFn, RidgedMulti, Seedable, BasicMulti, Value, Fbm};
use super::{block_model::BlockModel};
use std::collections::HashMap;
extern crate stopwatch;
pub mod block;

pub struct Chunk {
    pub position: glm::Vector3<f32>,
    pub blocks: Vec<Vec<Vec<block::Block>>>,
    pub grid_x: i32,
    pub grid_z: i32,
    pub vertices: Vec<(glm::Vec3, glm::Vec2, f32, f32, bool)>,

    pub positions: Vec<f32>,
    pub uvs: Vec<f32>,
    pub brightness: Vec<f32>,
    pub opacity: Vec<f32>,
    pub vao: gl::types::GLuint,
    pub vbos: (gl::types::GLuint, gl::types::GLuint, gl::types::GLuint, gl::types::GLuint),
    pub chunk_model: (gl::types::GLuint, usize, gl::types::GLuint),

    pub transparent_positions: Vec<f32>,
    pub transparent_uvs: Vec<f32>,
    pub transparent_brightness: Vec<f32>,
    pub transparent_opacity: Vec<f32>,
    pub transparent_vao: gl::types::GLuint,
    pub transparent_vbos: (gl::types::GLuint, gl::types::GLuint, gl::types::GLuint, gl::types::GLuint),
    pub transparent_chunk_model: (gl::types::GLuint, usize, gl::types::GLuint),

    pub world_gen_seed: u32,
    pub underground_height: u8,
    pub sky_height: u8,
    pub mid_height: u8,
    pub noise_resolution: f32
}

impl Chunk{
    pub fn init(grid_x: i32, grid_z: i32, position: glm::Vector3<f32>, square_chunk_width: &usize, world_gen_seed: &u32, mid_height: &u8, set_blocks: &mut HashMap<String, u8>, underground_height: &u8, sky_height: &u8, noise_resolution: &f32) -> Chunk{
        let mut blocks: Vec<Vec<Vec<block::Block>>> = vec![];

        // Perlin smooth rolling hills
        // Ridged is just random hills. Maybe good to add at a very low frequency to make interesting terrain
        // FBM is like Perlin wiht more density and random distortion
        // Billow is very distorted with high height variation. Maybe good for islands if made lower frequency
        // Basic multi is like fbm with more interesting terrain. Same distortion maybe to much frequency.
        // Worley is super basic. Very big chunks almost checkerboard. Very big differences in height. Flat. Good for biomes
        // Value is like perlin but even less frequency and a bit more height variaton
        // Super simplex is like perln with more freqnecy and even more height variaton. More than Value
        // Open simplex is very very flat and low heigh variaton
        // Hybrd multi is very extreme. Maybe not use this one

        let end_pos = generate_chunk(&mut blocks, *square_chunk_width, position, grid_x, grid_z, *world_gen_seed, *mid_height, *underground_height, *sky_height, false, set_blocks, *noise_resolution);
        let position_of_chunk = glm::vec3((position.x + end_pos.x) / 2.0, (position.y + end_pos.y) / 2.0, (position.z + end_pos.z) / 2.0);
        
        return Chunk{
            position: position_of_chunk,
            blocks,
            grid_x,
            grid_z,
            vertices:  vec![],
            
            positions:  vec![],
            uvs:  vec![],
            brightness: vec![],
            opacity: vec![],
            vao: 0,
            vbos: (0,0,0,0),
            chunk_model: (0,0,0),

            transparent_positions:  vec![],
            transparent_uvs:  vec![],
            transparent_brightness: vec![],
            transparent_opacity: vec![],
            transparent_vao: 0,
            transparent_vbos: (0,0,0,0),
            transparent_chunk_model: (0,0,0),

            world_gen_seed: *world_gen_seed,
            underground_height: *underground_height,
            sky_height: *sky_height,
            mid_height: *mid_height,
            noise_resolution: *noise_resolution
        };
    }

    pub fn regenerate(&mut self, grid_x: i32, grid_z: i32, position: glm::Vector3<f32>, square_chunk_width: &usize, set_blocks: &mut HashMap<String, u8>){
        let half_chunk_width = *square_chunk_width as f32 / 2.0;
        let center_position = position;

        let position = glm::vec3(position.x + half_chunk_width - 0.5 , position.y, position.z + half_chunk_width - 0.5);

        generate_chunk(&mut self.blocks, *square_chunk_width, position, grid_x, grid_z, self.world_gen_seed, self.mid_height, self.underground_height, self.sky_height, true, set_blocks, self.noise_resolution);

        self.grid_x = grid_x;
        self.grid_z = grid_z;
        self.position = center_position;
    }
    
    pub fn get_position(&self) -> &glm::Vector3<f32>{
        return &self.position;
    }

    pub fn get_grid(&self) -> (i32, i32){
        (self.grid_x, self.grid_z)
    }

    pub fn build_mesh(&mut self, block_model: &BlockModel) {
        if self.vao != 0 || self.transparent_vao != 0{
            unsafe {
                gl::DeleteVertexArrays(1, &self.vao);
                gl::DeleteVertexArrays(1, &self.transparent_vao);
            }
        }

        if self.vbos.0 != 0 || self.vbos.1 != 0 || self.transparent_vbos.0 != 0 && self.transparent_vbos.1 != 0{
            unsafe {
                gl::DeleteBuffers(1, &self.vbos.0);
                gl::DeleteBuffers(1, &self.vbos.1);
                gl::DeleteBuffers(1, &self.vbos.2);
                gl::DeleteBuffers(1, &self.vbos.3);

                gl::DeleteBuffers(1, &self.transparent_vbos.0);
                gl::DeleteBuffers(1, &self.transparent_vbos.1);
                gl::DeleteBuffers(1, &self.transparent_vbos.2);
                gl::DeleteBuffers(1, &self.transparent_vbos.3);
            }
        }
        self.vertices.clear();
        self.positions.clear();
        self.uvs.clear();
        self.brightness.clear();
        self.opacity.clear();
        self.vao = 0;
        self.vbos = (0,0,0,0);
        self.chunk_model = (0,0,0);

        self.transparent_positions.clear();
        self.transparent_uvs.clear();
        self.transparent_brightness.clear();
        self.transparent_opacity.clear();
        self.transparent_vao = 0;
        self.transparent_vbos = (0,0,0,0);
        self.transparent_chunk_model = (0,0,0);
        for i in 0..self.blocks.len() {
            for k in 0..self.blocks[i].len() {
                for j in 0..self.blocks[i][k].len() {
                    block::Block::get_mesh(&self.blocks[i][k][j], &mut self.vertices, block_model);
                }
            }
        }
    }
    
    pub fn populate_mesh(&mut self){
        
        for i in 0..self.vertices.len() {
            if self.vertices[i].4 != true{
                self.positions.push(self.vertices[i].0.x);
                self.positions.push(self.vertices[i].0.y);
                self.positions.push(self.vertices[i].0.z);
            }else{
                self.transparent_positions.push(self.vertices[i].0.x);
                self.transparent_positions.push(self.vertices[i].0.y);
                self.transparent_positions.push(self.vertices[i].0.z);
            }
        }

        for i in 0..self.vertices.len() {
            if self.vertices[i].4 != true{
                self.uvs.push(self.vertices[i].1.x);
                self.uvs.push(self.vertices[i].1.y);
            }else{
                self.transparent_uvs.push(self.vertices[i].1.x);
                self.transparent_uvs.push(self.vertices[i].1.y);
            } 
        }

        for i in 0..self.vertices.len() {
            if self.vertices[i].4 != true{
                self.brightness.push(self.vertices[i].2);
            }else{
                self.transparent_brightness.push(self.vertices[i].2);
            }
        }

        for i in 0..self.vertices.len() {
            if self.vertices[i].4 != true{
                self.opacity.push(self.vertices[i].3);
            }else{
                self.transparent_opacity.push(self.vertices[i].3);
            }
        }
        
        self.vertices.clear();
    }

    pub fn get_blocks_vector(&self) -> &Vec<Vec<Vec<block::Block>>> {
        return &self.blocks;
    }

    pub fn get_blocks_vector_mutable(&mut self) -> &mut Vec<Vec<Vec<block::Block>>> {
        return &mut self.blocks;
    }

    pub fn get_vertices(&self) -> &Vec<f32> {
        &self.positions
    }

    pub fn get_uv(&self) -> &Vec<f32> {
        &self.uvs
    }

    pub fn get_brightnesses(&self) -> &Vec<f32>{
        &self.brightness
    }

    pub fn get_opacity(&self) -> &Vec<f32>{
        &self.opacity
    }

    pub fn get_transparent_vertices(&self) -> &Vec<f32> {
        &self.transparent_positions
    }

    pub fn get_transparent_uv(&self) -> &Vec<f32> {
        &self.transparent_uvs
    }

    pub fn get_transparent_brightnesses(&self) -> &Vec<f32>{
        &self.transparent_brightness
    }

    pub fn get_transparent_opacity(&self) -> &Vec<f32>{
        &self.transparent_opacity
    }

    pub fn set_vao_vbo(&mut self, vao: gl::types::GLuint, vbo_id_vert: gl::types::GLuint, vbo_id_tex: gl::types::GLuint, vbo_id_bright: gl::types::GLuint, vbo_id_opacity: gl::types::GLuint){
        self.vao = vao;
        self.vbos = (vbo_id_vert, vbo_id_tex, vbo_id_bright, vbo_id_opacity);
    }

    pub fn set_transparent_vao_vbo(&mut self, vao: gl::types::GLuint, vbo_id_vert: gl::types::GLuint, vbo_id_tex: gl::types::GLuint, vbo_id_bright: gl::types::GLuint, vbo_id_opacity: gl::types::GLuint){
        self.transparent_vao = vao;
        self.transparent_vbos = (vbo_id_vert, vbo_id_tex, vbo_id_bright, vbo_id_opacity);
    }

    pub fn get_chunk_model(&self) -> &(gl::types::GLuint, usize, gl::types::GLuint){
        return &self.chunk_model;
    }

    pub fn get_transparent_chunk_model(&self) -> &(gl::types::GLuint, usize, gl::types::GLuint){
        return &self.transparent_chunk_model;
    }

    pub fn set_chunk_model(&mut self, chunk_model: (gl::types::GLuint, usize, gl::types::GLuint)){
        self.chunk_model = chunk_model;
    }

    pub fn set_transparent_chunk_model(&mut self, transparent_chunk_model: (gl::types::GLuint, usize, gl::types::GLuint)){
        self.transparent_chunk_model = transparent_chunk_model;
    }
}

fn map_value(value: f64, minimum: u8, maximum: u8) -> i32{
    return ((maximum - minimum) as f64 * value).floor() as i32 + minimum as i32;
}

fn generate_chunk(blocks: &mut Vec<Vec<Vec<block::Block>>>, square_chunk_width: usize, position: glm::Vec3, grid_x: i32, grid_z: i32, world_gen_seed: u32, mid_height: u8, underground_height: u8, sky_height: u8, overwrite: bool, set_blocks: &mut HashMap<String, u8>, resolution: f32) -> glm::Vec3{
    let mut stopwatch = stopwatch::Stopwatch::new();
    stopwatch.start();
    let mut x_pos = position.x;
    let mut z_pos = position.z;
    let mut y_pos = position.y;
    let x_pos_temp = position.x;
    let y_pos_temp = position.y;

    let mut basic_multi = BasicMulti::default().set_seed(60);
    basic_multi.frequency = 0.1;

    let mut ridged = RidgedMulti::new().set_seed(60);
    let mut fbm = Fbm::new().set_seed(60);
    // fbm.persistence = 1.0;
    fbm.frequency = 0.01;
    ridged.attenuation = 7.07;
    ridged.persistence = 2.02;
    ridged.octaves = 3;
    ridged.frequency = 7.01 as f64;
    basic_multi.frequency = 0.000004 as f64;
    basic_multi.octaves = 3;
    
    let mut trees: Vec<(i32, i32, usize, usize, usize)> = vec![];

    let blend = Blend::new(&fbm, &ridged, &basic_multi);
    // blend.control // Negative value for more value 1 positive for more value 2
    let mut rng = StdRng::seed_from_u64(world_gen_seed as u64 + powi(grid_x as f64, 2).sqrt() as u64 + powi(grid_z as f64, 2).sqrt() as u64);    
    let water_level: u8 = 11 + underground_height;

    for i in 0..square_chunk_width{
        if !overwrite {
            let collumn: Vec<Vec<block::Block>> = vec![];
            blocks.push(collumn);
        }
        
        for k in 0..square_chunk_width {
            if !overwrite {
                let row: Vec<block::Block> = vec![];
                blocks[i].push(row);
            }
            
            let value1: f64 = (z_pos + grid_z as f32) as f64;
            let value2: f64 = (x_pos + grid_x as f32) as f64;
            let mut value = blend.get([value1, value2]);
            if value > 1.0 || value < -1.0{
                println!("ValueNoise {} value1: {} value1: {}", value, value1, value2);
            }
            value = (value + 1.0)/2.0;
            let max_int = map_value(value, 0, mid_height);
            let max: u8;
            if max_int < 0 {
                max = (max_int * -1) as u8 + underground_height;
            }else{
                max = max_int as u8 + underground_height;
            }

            let has_plant;
            if rng.gen_range(1..500) == 1 {
                has_plant = true;
                if max> water_level{
                    trees.push((grid_x, grid_z, i, k, (max + underground_height + 6) as usize));
                }
            }else{
                has_plant = false;
            }

            for j in 0..(mid_height + underground_height + sky_height) as usize{
                let mut number: u8;

                // CHUNK TESTING BLOCK BREAKING
                // if k == square_chunk_width as usize -1 && i == square_chunk_width as usize -1 {
                //     number = 0;
                // }else if k == square_chunk_width as usize -1 || k == 0 || i == 0 || i == square_chunk_width as usize -1 {
                //     number = 1;
                // }else{
                //     number = 2;
                // }
                
                number = get_set_block(set_blocks, grid_x, grid_z, i, k, j);
                if number == 241{
                    number = get_block_type(j as u8, underground_height+max, water_level, has_plant, underground_height, sky_height, mid_height + underground_height + sky_height);
                }
                
                if !overwrite{
                    blocks[i as usize][k as usize].push(block::Block::init(glm::vec3(x_pos, y_pos, z_pos), number));
                }else{
                    blocks[i as usize][k as usize][j as usize].regenerate(glm::vec3(x_pos, y_pos, z_pos), number);
                }
                y_pos += 1.0;
            }
            y_pos = y_pos_temp;
            x_pos -= 1.0;
        }
        x_pos = x_pos_temp;
        z_pos -= 1.0;
    }

    for i in 0..trees.len(){
        set_tree(blocks, set_blocks, trees[i].0, trees[i].1, trees[i].2, trees[i].3, trees[i].4);
    }
    stopwatch.stop();
    println!("Time ms for chunk: {}", stopwatch.elapsed_ms());

    return block::Block::get_position(&blocks[blocks.len()-1][blocks[blocks.len()-1].len()-1][0]).clone();
}

fn get_block_type(block_height: u8, max_collumn_height: u8, water_level: u8, has_plant: bool, underground_height: u8, sky_height: u8, height_limit: u8) -> u8 {
    
    if block_height < underground_height {
        return 2;
    }

    if has_plant && block_height > water_level && water_level < max_collumn_height && block_height < max_collumn_height + 6 && block_height > max_collumn_height {
        return 5; // Wood log
    }
    
    if block_height > max_collumn_height {
        if block_height < water_level{
            return 3;//block_id = 3; //Water
        }else{
            return 240;// AIR
        }
    }
    
    if block_height <= 7 {
        return 1; // Dirt
    }

    if block_height == max_collumn_height {
        if block_height > water_level + 2{
            return 0; //Grass
        }else{
            return 6; //Sand
        }
    }
    
    if block_height >= 8  {
        return 2; //Stone
    }  else {
        return 240; //AIR
    }

}

fn get_set_block(set_blocks: &mut HashMap<String, u8>, grid_x: i32, grid_z: i32, i: usize, k: usize, j: usize) -> u8{
    let key = [grid_x.to_string(), grid_z.to_string(), i.to_string(), k.to_string(), j.to_string()].join("");

    match set_blocks.get(&key){
        Some(value) => return *value,
        None => return 241,
    }
}

fn set_tree(blocks: &mut Vec<Vec<Vec<block::Block>>>, set_blocks: &mut HashMap<String, u8>, grid_x: i32, grid_z: i32, i: usize, k: usize, j: usize){
    // UP is +X
    // Left is +Z
    // Right is -Z
    // Bottom is -X

    //       X
    //      XXX
    //       X     5

    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize,   k as isize,   j);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize +1,k as isize,   j,);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize,   k as isize +1,j);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize,   k as isize -1,j);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize -1,k as isize   ,j);

    //      XXX
    //     XXXXX
    //     XX XX
    //     XXXXX
    //      XXX    20

    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize +1,k as isize,      j-1);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize +1,k as isize +1,   j-1);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize +1,k as isize -1,   j-1);

    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize,   k as isize +1,   j-1);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize,   k as isize -1,   j-1);

    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize -1,k as isize   ,   j-1);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize -1,k as isize +1  , j-1);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize -1,k as isize -1  , j-1);

    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize +2,k as isize,      j-2);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize +2,k as isize +1,   j-2);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize +2,k as isize -1,   j-2);

    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize +2,k as isize,      j-1);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize +2,k as isize +1,   j-1);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize +2,k as isize -1,   j-1);

    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize -2,k as isize,      j-1);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize -2,k as isize +1,   j-1);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize -2,k as isize -1,   j-1);

    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize   ,k as isize -2,   j-1);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize -1,k as isize -2,   j-1);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize +1,k as isize -2,   j-1);

    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize   ,k as isize +2,   j-1);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize -1,k as isize +2,   j-1);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize +1,k as isize +2,   j-1);

    //     XXXXX
    //     XXXXX
    //     XX XX
    //     XXXXX
    //     XXXXX   20

    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize +1,k as isize,      j-2);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize +1,k as isize +1,   j-2);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize +1,k as isize -1,   j-2);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize,   k as isize +1,   j-2);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize,   k as isize -1,   j-2);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize -1,k as isize   ,   j-2);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize -1,k as isize +1  , j-2);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize -1,k as isize -1  , j-2);

    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize +2,k as isize,      j-2);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize +2,k as isize +1,   j-2);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize +2,k as isize -1,   j-2);


    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize -2,k as isize,      j-2);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize -2,k as isize +1,   j-2);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize -2,k as isize -1,   j-2);

    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize   ,k as isize -2,   j-2);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize -1,k as isize -2,   j-2);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize +1,k as isize -2,   j-2);

    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize   ,k as isize +2,   j-2);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize -1,k as isize +2,   j-2);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize +1,k as isize +2,   j-2);

    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize +2,k as isize +2,   j-2);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize -2,k as isize +2,   j-2);

    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize +2,k as isize -2,   j-2);
    set_tree_block(blocks, set_blocks, grid_x, grid_z, i as isize -2,k as isize -2,   j-2);
}

fn set_tree_block(blocks: &mut Vec<Vec<Vec<block::Block>>>, set_blocks: &mut HashMap<String, u8>, grid_x: i32, grid_z: i32, i: isize, k: isize, j: usize){
    if i < 0 || k < 0 || i > (blocks[0].len() - 1) as isize || k > (blocks[0].len() - 1) as isize || j > (blocks[0][0].len() - 1){
        let mut grid_x_set = grid_x;
        let mut grid_z_set = grid_z;
        let mut i_set = i;
        let mut k_set = k;
        
        if i < 0 {
            i_set = blocks[0].len() as isize;
            grid_x_set = grid_x_set - i as i32;
        }

        if k < 0 {
            k_set = blocks[0].len() as isize;
            grid_z_set = grid_z_set - k as i32;
        }

        if i >= blocks[0].len() as isize {
            i_set = 0;
            grid_x_set = grid_x_set + (i as usize - (blocks[0].len() - 1)) as i32;

        }

        if k >= blocks[0].len() as isize {
            k_set = 0;
            grid_z_set = grid_z_set + (k as usize - (blocks[0].len() - 1)) as i32;
        }
        
        let key = [grid_x_set.to_string(), grid_z_set.to_string(), i_set.to_string(), k_set.to_string(), j.to_string()].join("");

        if !set_blocks.contains_key(&key){
            set_blocks.insert(key, 7);
        }

    }else{
        blocks[i as usize][k as usize][j].id = 7;
    }
}
