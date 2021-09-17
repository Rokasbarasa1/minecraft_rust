extern crate noise;
use glm::ext::powi;
use parking_lot::{Mutex};
use rand::Rng;
use rand::{SeedableRng};
use noise::{Fbm, NoiseFn, Seedable, Worley};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use super::{block_model::BlockModel};
use std::collections::HashMap;
extern crate stopwatch;
pub mod block;
use std::sync::Arc;

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
    pub chunk_distance: usize,
    pub visible_layers: Vec<bool>,
}

impl Chunk{
    pub fn init(change_block: &mut Vec<(usize, usize, usize, usize, usize, u8)>, i: usize, k:usize, grid_x: i32, grid_z: i32, position: glm::Vector3<f32>, square_chunk_width: &usize, world_gen_seed: &u32, mid_height: &u8, set_blocks: &mut HashMap<String, u8>, underground_height: &u8, sky_height: &u8, chunk_distance: usize) -> Chunk{
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

        let end_pos = generate_chunk(change_block, i, k, &mut blocks, *square_chunk_width, position, grid_x, grid_z, *world_gen_seed, *mid_height, *underground_height, *sky_height, false, set_blocks, chunk_distance);
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
            chunk_distance: chunk_distance,
            visible_layers: vec![],
        };
    }

    pub fn regenerate(&mut self, change_block: &mut Vec<(usize, usize, usize, usize, usize, u8)>, i: usize, k:usize, set_blocks: &mut HashMap<String, u8>){
        let half_chunk_width = self.blocks[0].len() as f32 / 2.0;

        let position = glm::vec3(self.position.x + half_chunk_width - 0.5 , self.position.y, self.position.z + half_chunk_width - 0.5);
        let chunk_length = self.blocks[0].len();
        generate_chunk(change_block, i, k, &mut self.blocks, chunk_length, position, self.grid_x, self.grid_z, self.world_gen_seed, self.mid_height, self.underground_height, self.sky_height, true, set_blocks, self.chunk_distance);

    }

    pub fn get_grid(&self) -> (i32, i32){
        (self.grid_x, self.grid_z)
    }

    pub fn build_mesh(&mut self, block_model: &BlockModel) {
        let mut stopwatch = stopwatch::Stopwatch::new();
        stopwatch.start();
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
        for j in 0..self.blocks[0][0].len() {
            if self.visible_layers[j]{
                for i in 0..self.blocks.len() {
                    for k in 0..self.blocks[i].len() {
                        block::Block::get_mesh(&self.blocks[i][k][j], &mut self.vertices, block_model);
                    }
                }
            }
        }
        stopwatch.stop();
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

    pub fn set_vao_vbo(&mut self, vao: gl::types::GLuint, vbo_id_vert: gl::types::GLuint, vbo_id_tex: gl::types::GLuint, vbo_id_bright: gl::types::GLuint, vbo_id_opacity: gl::types::GLuint){
        self.vao = vao;
        self.vbos = (vbo_id_vert, vbo_id_tex, vbo_id_bright, vbo_id_opacity);
    }

    pub fn set_transparent_vao_vbo(&mut self, vao: gl::types::GLuint, vbo_id_vert: gl::types::GLuint, vbo_id_tex: gl::types::GLuint, vbo_id_bright: gl::types::GLuint, vbo_id_opacity: gl::types::GLuint){
        self.transparent_vao = vao;
        self.transparent_vbos = (vbo_id_vert, vbo_id_tex, vbo_id_bright, vbo_id_opacity);
    }

    pub fn set_layer_visibility(&mut self, visible_layers: Vec<bool>){
        self.visible_layers = visible_layers;
    }

    pub fn changed_block_visibility(&mut self, m: usize){
        let mut visible = false;
        for i in 0..self.blocks.len() {
            for k in 0..self.blocks[i].len() {
                if self.blocks[i][k][m].visible{
                    visible = true;
                    break;
                }
            }
        }
        self.visible_layers[m] = visible;
    }
}

fn map_value(value: f64, minimum: u8, maximum: u8) -> i32{
    return ((maximum - minimum) as f64 * value).floor() as i32 + minimum as i32;
}

fn generate_chunk(change_block: &mut Vec<(usize, usize, usize, usize, usize, u8)>, chunk_i: usize, chunk_k:usize, blocks: &mut Vec<Vec<Vec<block::Block>>>, square_chunk_width: usize, position: glm::Vec3, grid_x: i32, grid_z: i32, world_gen_seed: u32, mid_height: u8, underground_height: u8, sky_height: u8, overwrite: bool, set_blocks: &mut HashMap<String, u8>, chunk_distance: usize) -> glm::Vec3{
    let mut stopwatch = stopwatch::Stopwatch::new();
    stopwatch.start();

    let water_level: u8 = 11 + underground_height;

    let set_blocks_arc = Arc::new(Mutex::new(set_blocks));
    let mut trees: Vec<(i32, i32, usize, usize, usize)> = vec![];

    let mut collumns: Vec<Vec<(u8, bool, f32, f32, u8)>> = vec![];

    let mut fbm = Fbm::default().set_seed(world_gen_seed);
    let mut worley = Worley::default().set_seed(world_gen_seed);
    worley.frequency = 0.01;


    //Seed with values that will be used for biliniare interpolation
    for i in 0..square_chunk_width{
        collumns.push(vec![]);
        for k in 0..square_chunk_width {
            if square_chunk_width <= 8 && (i == 0 && k == 0 || i == 0 && k == square_chunk_width-1 || i == square_chunk_width-1 && k == 0  || i == square_chunk_width-1 && k == square_chunk_width-1) 
            || square_chunk_width == 10 && ( i % 3 == 0 && k % 3 == 0 )
            || square_chunk_width == 16 && ( i % 3 == 0 && k % 3 == 0 ){

                let z = position.z - 1.0 * i as f32;
                let x = position.x - 1.0 * k as f32;
                
                let value_worley = (worley.get([(z as f64 + grid_z as f64), (x as f64 + grid_x as f64)]) + 1.0)/2.0;
                // 0 normal 
                // 1 Ocean
                // 2 Dessert
                // 3 Mountains
            
                let type_biome: u8; 
                let max: u8;

                if value_worley > 0.97{
                    fbm.octaves = 6;
                    fbm.lacunarity = 2.0;
                    fbm.frequency = 0.02;
                    let value_fbm = (fbm.get([(z as f64 + grid_z as f64), (x as f64 + grid_x as f64)]) + 1.0)/2.0;
                    max = map_value(value_fbm.powf(1.0), 0, mid_height) as u8 + underground_height;
                    type_biome = 3
                }else if value_worley > 0.9{
                    fbm.octaves = 4;
                    fbm.frequency = 0.01;
                    fbm.lacunarity = 2.0;

                    let value_fbm = (fbm.get([(z as f64 + grid_z as f64), (x as f64 + grid_x as f64)]) + 1.0)/2.0;
                    max= map_value(value_fbm.powf(0.5)/2.0, 0, mid_height) as u8 + underground_height;
                    type_biome = 2
                }else if value_worley > 0.5{
                    fbm.octaves = 5;
                    fbm.frequency = 0.01;
                    fbm.lacunarity = 2.0;

                    let value_fbm = (fbm.get([(z as f64 + grid_z as f64), (x as f64 + grid_x as f64)]) + 1.0)/2.0;
                    max= map_value(powi(value_fbm,3), 0, mid_height) as u8 + underground_height;
                    type_biome = 1
                }else {
                    fbm.octaves = 6;
                    fbm.frequency = 0.01;
                    fbm.lacunarity = 2.0;

                    let value_fbm = (fbm.get([(z as f64 + grid_z as f64), (x as f64 + grid_x as f64)]) + 1.0)/2.0;
                    max= map_value(powi(value_fbm,3), 0, mid_height) as u8 + underground_height;
                    type_biome = 0
                }
                let mut has_tree = false;
                let mut rng = rand_xoshiro::SplitMix64::seed_from_u64(world_gen_seed as u64 + powi(x, 2) as u64 +powi(z, 4) as u64);
                if type_biome != 2 && rng.gen_range(1..50) == 1{
                    has_tree = true;
                    if max > water_level+2{
                        trees.push((grid_x, grid_z, i, k, (max + underground_height + 6) as usize));
                    }
                }
                collumns[i].push((max, has_tree, z, x, type_biome));
            }else{
                collumns[i].push((0, false, 0.0, 0.0, 0));
            }
        }
    }



    let coll_temp = collumns.clone();

    for i in 0..square_chunk_width{
        if collumns[i][0].0 == 0{
            continue;
        }
        for k in 0..square_chunk_width {
            if collumns[i][k].0 == 0{
                let z = position.z - 1.0 * i as f32;
                let x = position.x - 1.0 * k as f32;
                
                let value_worley = (worley.get([(z as f64 + grid_z as f64), (x as f64 + grid_x as f64)]) + 1.0)/2.0;
                // 0 normal 
                // 1 Ocean
                // 2 Dessert
                // 3 Mountains
            
                let type_biome: u8; 
                let mut next_distance: f32 = 0.0;
                let mut next_height: f32 = 0.0;
                for j in k+1..square_chunk_width{
                    if coll_temp[i][j].0 != 0{
                        next_distance = (j - k) as f32 + 1.0;
                        next_height = coll_temp[i][j].0 as f32;
                        break;
                    }
                }
                let mut before_distance: f32 = 0.0;
                let mut before_height: f32 = 0.0;
                for j in (0..k).rev(){
                    if coll_temp[i][j].0 != 0{
                        before_distance = (k - j) as f32;
                        before_height = coll_temp[i][j].0 as f32;
                        break;
                    }
                }

                let max: u8 = (next_height * (before_distance/(before_distance+next_distance)) + before_height * (next_distance/(before_distance+next_distance))).round() as u8;

                if value_worley > 0.97{
                    type_biome = 3
                }else if value_worley > 0.9{
                    type_biome = 2
                }else if value_worley > 0.5{
                    type_biome = 6
                }else {
                    type_biome = 0
                }
                let mut has_tree = false;
                let mut rng = rand_xoshiro::SplitMix64::seed_from_u64(world_gen_seed as u64 + powi(x, 2) as u64 +powi(z, 4) as u64);
                if type_biome != 2 && rng.gen_range(1..50) == 1{
                    has_tree = true;
                    if max > water_level+2{
                        trees.push((grid_x, grid_z, i, k, (max + underground_height + 6) as usize));
                    }
                }
                collumns[i][k] = (max, has_tree, z, x, type_biome);
            }

        }
    }
    let coll_temp = collumns.clone();

    for k in 0..square_chunk_width{
        for i in 0..square_chunk_width {
            if coll_temp[i][k].0 == 0{
                let z = position.z - 1.0 * i as f32;
                let x = position.x - 1.0 * k as f32;
                
                let value_worley = (worley.get([(z as f64 + grid_z as f64), (x as f64 + grid_x as f64)]) + 1.0)/2.0;
                // 0 normal 
                // 1 Ocean
                // 2 Dessert
                // 3 Mountains
            
                let type_biome: u8; 
                let mut next_distance: f32 = 0.0;
                let mut next_height: f32 = 0.0;
                for j in i+1..square_chunk_width{
                    if coll_temp[j][k].0 != 0{
                        next_distance = (j - i) as f32 +1.0;
                        next_height = coll_temp[j][k].0 as f32;
                        break;
                    }
                }
                let mut before_distance: f32 = 0.0;
                let mut before_height: f32 = 0.0;
                for j in (0..i).rev(){
                    if coll_temp[j][k].0 != 0{
                        before_distance = (i - j) as f32;
                        before_height = coll_temp[j][k].0 as f32;
                        break;
                    }
                }

                let max: u8 = (next_height * (before_distance/(before_distance+next_distance)) + before_height * (next_distance/(before_distance+next_distance))).round() as u8;
                if value_worley > 0.97{
                    type_biome = 3
                }else if value_worley > 0.9{
                    type_biome = 2
                }else if value_worley > 0.5{
                    type_biome = 6
                }else {
                    type_biome = 0
                }
                let mut has_tree = false;
                let mut rng = rand_xoshiro::SplitMix64::seed_from_u64(world_gen_seed as u64 + powi(x, 2) as u64 +powi(z, 4) as u64);
                if type_biome != 2 && rng.gen_range(1..50) == 1{
                    has_tree = true;
                    if max > water_level+2{
                        trees.push((grid_x, grid_z, i, k, (max + underground_height + 6) as usize));
                    }
                }
                collumns[i][k] = (max, has_tree, z, x, type_biome);
            }
        }
    }
    let collumns = Arc::new(Mutex::new(&collumns));

    if !overwrite { 
        for i in 0..square_chunk_width{
            let collumn: Vec<Vec<block::Block>> = vec![];
            blocks.push(collumn);
            for k in 0..square_chunk_width {
                let row: Vec<block::Block> = vec![];
                blocks[i].push(row);

                for _j in 0..(mid_height + underground_height + sky_height) as usize{
                    blocks[i][k].push(block::Block::init(glm::vec3(0.0, 0.0, 0.0), 1));
                }
            }
        }
    }

    // Fancy way to iterate over the vector. It makes a thread for each of the vectors in the vector
    (blocks).par_iter_mut().enumerate().for_each(|(i, val)|  {
        let collumns_t = Arc::clone(&collumns);
        let set_blocks_arc_t = Arc::clone(&set_blocks_arc);
        let set_blocks_t = set_blocks_arc_t.lock().clone();
        for k in 0..val.len() {
            let collumn_values = collumns_t.lock()[i][k].clone();
            for j in 0..val[k].len(){    
                let mut number: u8;

                number = get_set_block(&set_blocks_t, grid_x, grid_z, i, k, j);
                if number == 7{
                    remove_key(&mut set_blocks_arc_t.lock(), grid_x, grid_z, i, k, j);
                }
                if number == 241{
                    number = get_block_type(j as u8, underground_height + collumn_values.0, water_level, collumn_values.1, underground_height, sky_height, mid_height + underground_height + sky_height, collumn_values.4);
                }
                
                val[k as usize][j as usize].regenerate(glm::vec3(collumn_values.3, position.y + 1.0 * j as f32, collumn_values.2), number);
            }
        }
    });

    for i in 0..trees.len(){
        set_tree(change_block, chunk_i, chunk_k, blocks, &mut set_blocks_arc.lock(), trees[i].0, trees[i].1, trees[i].2, trees[i].3, trees[i].4, chunk_distance, (mid_height + underground_height + sky_height) as usize);
    }

    stopwatch.stop();
    println!("Time ms for chunk: {}", stopwatch.elapsed_ms());

    return blocks[blocks.len()-1][blocks[blocks.len()-1].len()-1][0].position.clone();
}

fn get_block_type(block_height: u8, max_collumn_height: u8, water_level: u8, has_plant: bool, underground_height: u8, _sky_height: u8,_height_limitt: u8, biome: u8) -> u8 {
    if biome == 2{

        //Check if the j value is below undeground height. Everything underground is stone
        if block_height < underground_height {
            return 1;//Stone
        }
        
        //Check if j above collumn max
        if block_height > max_collumn_height {
            //Check if j is bellow water
            if block_height < water_level{
                return 3;//Water
            }else{
                return 240;// AIR
            }
        }

        if block_height == max_collumn_height {
            return 6; //Sand
        }

        if max_collumn_height - block_height < 3 && block_height > water_level + 2{
            return 6; //Sand
        }

        return 1; // Stone
    }else{
        //Check if the collumn has a plant, j is above water, the max height of the collumn is 2 blocks above water. J is up to 6 blocks bellow max collumn height and that j is aboce max collumn height
        if has_plant && block_height > water_level && water_level + 2 < max_collumn_height && block_height < max_collumn_height + 6 && block_height > max_collumn_height {
            return 5; // Wood log
        }

        //Check if the j value is below undeground height. Everything underground is stone
        if block_height < underground_height {
            return 1;//Stone
        }
        
        //Check if j above collumn max
        if block_height > max_collumn_height {
            //Check if j is bellow water
            if block_height < water_level{
                return 3;//Water
            }else{
                return 240;// AIR
            }
        }

        if block_height == max_collumn_height {
            if block_height > water_level + 2{
                return 0; //Grass
            }else{
                return 6; //Sand
            }
        }

        if max_collumn_height - block_height < 3 && block_height > water_level + 2{
            return 2; // Dirt bellow grass
        }

        return 1; // Stone
    }
}

fn get_set_block(set_blocks: &HashMap<String, u8>, grid_x: i32, grid_z: i32, i: usize, k: usize, j: usize) -> u8{
    let key = [grid_x.to_string(), grid_z.to_string(), i.to_string(), k.to_string(), j.to_string()].join("");

    match set_blocks.get(&key){
        Some(value) => return *value,
        None => return 241,
    }
}

fn remove_key(set_blocks: &mut HashMap<String, u8>, grid_x: i32, grid_z: i32, i: usize, k: usize, j: usize){
    let key = [grid_x.to_string(), grid_z.to_string(), i.to_string(), k.to_string(), j.to_string()].join("");
    set_blocks.remove(&key);
}

fn set_tree(change_block: &mut Vec<(usize, usize, usize, usize, usize, u8)>, chunk_i: usize, chunk_k:usize, blocks: &mut Vec<Vec<Vec<block::Block>>>, set_blocks: &mut HashMap<String, u8>, grid_x: i32, grid_z: i32, i: usize, k: usize, j: usize, chunk_distance: usize, j_max: usize){
    // UP is +X
    // Left is +Z
    // Right is -Z
    // Bottom is -X

    //       X
    //      XXX
    //       X     5

    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize,   k as isize,   j, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize +1,k as isize,   j, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize,   k as isize +1,j, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize,   k as isize -1,j, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize -1,k as isize   ,j, chunk_distance, j_max);

    //      XXX
    //     XXXXX
    //     XX XX
    //     XXXXX
    //      XXX    20

    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize +1,k as isize,      j-1, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize +1,k as isize +1,   j-1, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize +1,k as isize -1,   j-1, chunk_distance, j_max);

    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize,   k as isize +1,   j-1, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize,   k as isize -1,   j-1, chunk_distance, j_max);

    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize -1,k as isize   ,   j-1, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize -1,k as isize +1  , j-1, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize -1,k as isize -1  , j-1, chunk_distance, j_max);

    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize +2,k as isize,      j-2, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize +2,k as isize +1,   j-2, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize +2,k as isize -1,   j-2, chunk_distance, j_max);

    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize +2,k as isize,      j-1, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize +2,k as isize +1,   j-1, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize +2,k as isize -1,   j-1, chunk_distance, j_max);

    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize -2,k as isize,      j-1, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize -2,k as isize +1,   j-1, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize -2,k as isize -1,   j-1, chunk_distance, j_max);

    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize   ,k as isize -2,   j-1, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize -1,k as isize -2,   j-1, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize +1,k as isize -2,   j-1, chunk_distance, j_max);

    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize   ,k as isize +2,   j-1, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize -1,k as isize +2,   j-1, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize +1,k as isize +2,   j-1, chunk_distance, j_max);

    //     XXXXX
    //     XXXXX
    //     XX XX
    //     XXXXX
    //     XXXXX   20

    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize +1,k as isize,      j-2, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize +1,k as isize +1,   j-2, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize +1,k as isize -1,   j-2, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize,   k as isize +1,   j-2, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize,   k as isize -1,   j-2, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize -1,k as isize   ,   j-2, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize -1,k as isize +1  , j-2, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize -1,k as isize -1  , j-2, chunk_distance, j_max);

    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize +2,k as isize,      j-2, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize +2,k as isize +1,   j-2, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize +2,k as isize -1,   j-2, chunk_distance, j_max);


    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize -2,k as isize,      j-2, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize -2,k as isize +1,   j-2, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize -2,k as isize -1,   j-2, chunk_distance, j_max);

    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize   ,k as isize -2,   j-2, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize -1,k as isize -2,   j-2, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize +1,k as isize -2,   j-2, chunk_distance, j_max);

    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize   ,k as isize +2,   j-2, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize -1,k as isize +2,   j-2, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize +1,k as isize +2,   j-2, chunk_distance, j_max);

    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize +2,k as isize +2,   j-2, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize -2,k as isize +2,   j-2, chunk_distance, j_max);

    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize +2,k as isize -2,   j-2, chunk_distance, j_max);
    set_tree_block(change_block, chunk_i, chunk_k, blocks, set_blocks, grid_x, grid_z, i as isize -2,k as isize -2,   j-2, chunk_distance, j_max);
}

fn set_tree_block(change_block: &mut Vec<(usize, usize, usize, usize, usize, u8)>, chunk_i: usize, chunk_k: usize, blocks: &mut Vec<Vec<Vec<block::Block>>>, set_blocks: &mut HashMap<String, u8>, grid_x: i32, grid_z: i32, i: isize, k: isize, j: usize, chunk_distance: usize, j_max: usize){
    if j_max > j{
        if i < 0 || k < 0 || i > (blocks[0].len() - 1) as isize || k > (blocks[0].len() - 1) as isize || j > (blocks[0][0].len() - 1){
            let mut chunk_i_mut = chunk_i as i32;
            let mut chunk_k_mut = chunk_k as i32; 
            let mut grid_x_set = grid_x;
            let mut grid_z_set = grid_z;
            let mut i_set = i;
            let mut k_set = k;
            
            if i < 0 {
                i_set = blocks[0].len() as isize + i;
                grid_x_set = grid_x_set - 1;
                chunk_i_mut = chunk_i_mut - 1;
            } else if i > blocks[0].len() as isize - 1{
                i_set = i - blocks[0].len() as isize;
                grid_x_set = grid_x_set + 1;
                chunk_i_mut = chunk_i_mut + 1;
            }

            if k < 0 {
                k_set = blocks[0].len() as isize + k;
                grid_z_set = grid_z_set - 1;
                chunk_k_mut = chunk_k_mut - 1;
            }else if k > blocks[0].len() as isize - 1 {
                k_set = k - blocks[0].len() as isize;
                grid_z_set = grid_z_set + 1;
                chunk_k_mut = chunk_k_mut + 1;
            } 
            
            // If block is outisde the chunk border that is currently rendered add it as set block
            // If the block is inside the currently rendered chunk border put it in change blocks

            if (chunk_distance as i32 - 1) >= chunk_k_mut && (chunk_distance as i32 - 1) >= chunk_i_mut && 0 <= chunk_k_mut && 0 <= chunk_i_mut{
                
                change_block.push((chunk_i_mut as usize, chunk_k_mut as usize, i_set as usize, k_set as usize, j, 7))

            }else{

                let key = [grid_x_set.to_string(), grid_z_set.to_string(), i_set.to_string(), k_set.to_string(), j.to_string()].join("");

                if !set_blocks.contains_key(&key){
                    set_blocks.insert(key, 7);
                }
            }

        }else{
            blocks[i as usize][k as usize][j].id = 7;
        }
    }
}

