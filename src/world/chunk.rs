extern crate noise;
use rand::Rng;
use rand::rngs::StdRng;
use rand::{SeedableRng};
use noise::{Blend, Fbm, NoiseFn, Perlin, RidgedMulti, Seedable, Billow, BasicMulti, Worley, Value, SuperSimplex, OpenSimplex, HybridMulti};

use super::{block_model::BlockModel};
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

    pub world_gen_seed: u32
}

impl Chunk{
    pub fn init(grid_x: i32, grid_z: i32, position: glm::Vector3<f32>, square_chunk_width: &usize, world_gen_seed: &u32, max_height: &usize) -> Chunk{

        let mut x_pos = position.x.clone();
        let mut z_pos = position.z.clone();
        let mut y_pos = position.y.clone();
        let x_pos_temp = position.x.clone();
        let y_pos_temp = position.y.clone();

        let mut blocks: Vec<Vec<Vec<block::Block>>> = vec![];

        //TODO: ADD NOISE FUNC TO STRUCT PARAMETERS

        let perlin = Perlin::new().set_seed(*world_gen_seed);
        let ridged = RidgedMulti::new().set_seed(*world_gen_seed);
        let fbm = Fbm::new().set_seed(*world_gen_seed);
        let blend = Blend::new(&perlin, &ridged, &fbm);
        // let billow = Billow::default().set_seed(*world_gen_seed);
        // let basic_multi = BasicMulti::default().set_seed(*world_gen_seed);
        // let blend = Blend::new(&blend1, &billow, &billow);
        // let worley = Worley::default().set_seed(*world_gen_seed);   
        // let value = Value::default().set_seed(*world_gen_seed);   
        // let super_simplex = SuperSimplex::default().set_seed(*world_gen_seed);   
        // let open_simplex = OpenSimplex::default().set_seed(*world_gen_seed);   
        // let hybrid_multi = HybridMulti::default().set_seed(*world_gen_seed);   


        let mut rng = StdRng::seed_from_u64(*world_gen_seed as u64);

        for i in 0..*square_chunk_width{  //Z line Go from positive to negative
            let collumn: Vec<Vec<block::Block>> = vec![];
            blocks.push(collumn);
            for k in 0..*square_chunk_width { //X line go from positive to negative
                let row: Vec<block::Block> = vec![];
                blocks[i].push(row);

                let value1: f64 = ((z_pos - 30.0 + grid_z as f32)* 0.0190) as f64;
                let value2: f64 = ((x_pos - 30.0 + grid_x as f32)* 0.0190) as f64;
                let mut value = blend.get([value1, value2]);
                
                value = (value + 1.0)/2.0;
                let max_int = map_value(value, 0, *max_height);
                let max: usize;
                if max_int < 0 {
                    max = (max_int * -1) as usize;
                }else{
                    max = max_int as usize
                }

                let has_plant;
                if rng.gen_range(1..100) == 1 {
                    has_plant = true;
                }else{
                    has_plant = false;
                }

                for j in 0..*max_height{//{
                    let number: usize;
                    const WATER_LEVEL: usize = 11;
                    // CHUNK TESTING BLOCK BREAKING
                    // if k == *square_chunk_width as usize -1 && i == *square_chunk_width as usize -1 {
                    //     number = 0;
                    // }else if k == *square_chunk_width as usize -1 || k == 0 || i == 0 || i == *square_chunk_width as usize -1 {
                    //     number = 1;
                    // }else{
                    //     number = 2;
                    // }

                    number = get_block_type(j, max, WATER_LEVEL, has_plant);
                    
                    blocks[i][k].push(block::Block::init(glm::vec3(x_pos, y_pos, z_pos), number));
                    y_pos += 1.0;
                }
                y_pos = y_pos_temp;
                x_pos -= 1.0;
                
            }
            x_pos = x_pos_temp;
            z_pos -= 1.0;

        }

        let end_pos = block::Block::get_position(&blocks[blocks.len()-1][blocks[blocks.len()-1].len()-1][0]);
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

            world_gen_seed: *world_gen_seed
        };
    }

    pub fn regenerate(&mut self, grid_x: i32, grid_z: i32, position: glm::Vector3<f32>, square_chunk_width: &usize){
        let half_chunk_width = *square_chunk_width as f32 / 2.0;
        let center_position = position;

        let position = glm::vec3(position.x + half_chunk_width - 0.5 , position.y, position.z + half_chunk_width - 0.5);
        
        let mut x_pos = position.x;
        let mut z_pos = position.z;
        let mut y_pos = position.y;
        let x_pos_temp = position.x;
        let y_pos_temp = position.y;

        let perlin = Perlin::new().set_seed(self.world_gen_seed);
        let ridged = RidgedMulti::new().set_seed(self.world_gen_seed);
        let fbm = Fbm::new().set_seed(self.world_gen_seed);
        let blend = Blend::new(&perlin, &ridged, &fbm);

        let max_height = self.blocks[0][0].len();

        let mut rng = StdRng::seed_from_u64(self.world_gen_seed as u64);

        for i in 0..self.blocks.len(){  //Z line Go from positive to negative
            for k in 0..self.blocks[i].len() { //X line go from positive to negative


                let value1: f64 = ((z_pos - 30.0 + grid_z as f32)* 0.0190) as f64;
                let value2: f64 = ((x_pos - 30.0 + grid_x as f32)* 0.0190) as f64;
                let mut  value = blend.get([value1, value2]);
                
                value = (value + 1.0)/2.0;
                let max_int = map_value(value, 0, max_height);
                let max: usize;
                if max_int < 0 {
                    max = (max_int * -1) as usize;
                }else{
                    max = max_int as usize
                }

                let has_plant;
                if rng.gen_range(1..100) == 1 {
                    has_plant = true;
                }else{
                    has_plant = false;
                }

                for j in 0..max_height{
                    let number: usize;
                    const WATER_LEVEL: usize = 11;
                    // CHUNK TESTING BLOCK BREAKING
                    // let square_chunk_width = self.blocks.len();
                    // if k == square_chunk_width as usize -1 && i == square_chunk_width as usize -1 {
                    //     number = 0;
                    // }else if k == square_chunk_width as usize -1 || k == 0 || i == 0 || i == square_chunk_width as usize -1 {
                    //     number = 1;
                    // }else{
                    //     number = 2;
                    // }

                    
                    number = get_block_type(j, max, WATER_LEVEL, has_plant);
                    
                    self.blocks[i][k][j].regenerate(glm::vec3(x_pos, y_pos, z_pos), number);
                    
                    y_pos += 1.0;
                }
                y_pos = y_pos_temp;
                x_pos -= 1.0;
            }
            x_pos = x_pos_temp;
            z_pos -= 1.0;
        }

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

fn map_value(value: f64, minimum: usize, maximum: usize) -> i32{
    return ((maximum - minimum) as f64 * value).floor() as i32 + minimum as i32;
}

fn get_block_type(block_height: usize, max_collumn_height: usize, water_level: usize, has_plant: bool) -> usize {
    let block_id;
    if has_plant && block_height > water_level && water_level < max_collumn_height && block_height < max_collumn_height + 7 {
        return 5;
    }else if block_height > max_collumn_height {
        if block_height < water_level{
            block_id = 3; //Water
        }else{
            block_id = 240;// AIR
        }
    }else{
        if block_height <= 7 {
            block_id = 1; // Dirt
        }else if block_height == max_collumn_height {
            if block_height > water_level + 2{
                block_id = 0; //Grass
            }else{
                block_id = 6; //Sand
            }
        }else if block_height >= 8  {
            block_id = 2; //Stone
        }  else {
            block_id = 240; //AIR
        }
    }

    return block_id;
}