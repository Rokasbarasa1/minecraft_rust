extern crate noise;

use noise::{NoiseFn, Perlin};

use super::{block_model::BlockModel};
pub mod block;
use noise::{ Blend, Fbm, RidgedMulti};

pub struct Chunk {
    position: glm::Vector3<f32>,
    blocks: Vec<Vec<Vec<block::Block>>>,
    grid_x: i32,
    grid_z: i32,
    vertices: Vec<(glm::Vec3, glm::Vec2, f32, f32, bool)>,

    positions: Vec<f32>,
    uvs: Vec<f32>,
    brightness: Vec<f32>,
    opacity: Vec<f32>,
    vao: gl::types::GLuint,
    vbos: (gl::types::GLuint, gl::types::GLuint, gl::types::GLuint, gl::types::GLuint),
    chunk_model: (gl::types::GLuint, usize, gl::types::GLuint),

    transparent_positions: Vec<f32>,
    transparent_uvs: Vec<f32>,
    transparent_brightness: Vec<f32>,
    transparent_opacity: Vec<f32>,
    transparent_vao: gl::types::GLuint,
    transparent_vbos: (gl::types::GLuint, gl::types::GLuint, gl::types::GLuint, gl::types::GLuint),
    transparent_chunk_model: (gl::types::GLuint, usize, gl::types::GLuint),

}

impl Chunk{
    pub fn init(grid_x: i32, grid_z: i32, position: glm::Vector3<f32>, square_chunk_width: &usize, world_gen_seed: &u32, max_height: &usize) -> Chunk{

        let mut x_pos = position.x.clone();
        let mut z_pos = position.z.clone();
        let mut y_pos = position.y.clone();
        let x_pos_temp = position.x.clone();
        let y_pos_temp = position.y.clone();

        let mut blocks: Vec<Vec<Vec<block::Block>>> = vec![];

        let perlin = Perlin::default();
        let ridged = RidgedMulti::new();
        let fbm = Fbm::new();
        let blend = Blend::new(&perlin, &ridged, &fbm);

        for i in 0..*square_chunk_width{  //Z line Go from positive to negative
            let collumn: Vec<Vec<block::Block>> = vec![];
            blocks.push(collumn);
            for k in 0..*square_chunk_width { //X line go from positive to negative
                let row: Vec<block::Block> = vec![];
                blocks[i].push(row);

                let value1: f64 = ((z_pos - 30.0 + grid_z as f32)* 0.0190) as f64;
                let value2: f64 = ((x_pos - 30.0 + grid_x as f32)* 0.0190) as f64;
                let mut  value = blend.get([value1, value2]);
                
                value = (value + 1.0)/2.0;
                let max_int = map_value(value, 0, *max_height);
                let max: usize;
                if max_int < 0 {
                    max = (max_int * -1) as usize;
                }else{
                    max = max_int as usize
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

                    //ACTUAL TERRAIn
                    if j > max {
                        if j < WATER_LEVEL{
                            number = 3;
                        }else{
                            number = 240;
                        }
                        
                    }else{
                        
                        if j <= 7 {
                            number = 1;
                        }else if j == max {
                            if j > WATER_LEVEL + 2{
                                number = 0;
                            }else{
                                number = 6;
                            }
                        }else if j >= 8  {
                            number = 2;
                        }  else {
                            number = 240;
                        }
                    }
                    

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
            transparent_chunk_model: (0,0,0)
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

        let perlin = Perlin::default();
        let ridged = RidgedMulti::new();
        let fbm = Fbm::new();
        let blend = Blend::new(&perlin, &ridged, &fbm);

        let max_height = self.blocks[0][0].len();
        let square_chunk_width = self.blocks.len();

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

                for j in 0..max_height{//{
                    let number: usize;
                    const WATER_LEVEL: usize = 11;
                    // CHUNK TESTING BLOCK BREAKING
                    // if k == square_chunk_width as usize -1 && i == square_chunk_width as usize -1 {
                    //     number = 0;
                    // }else if k == square_chunk_width as usize -1 || k == 0 || i == 0 || i == square_chunk_width as usize -1 {
                    //     number = 1;
                    // }else{
                    //     number = 2;
                    // }

                    //ACTUAL TERRAIn
                    if j > max {
                        if j < WATER_LEVEL{
                            number = 3;
                        }else{
                            number = 240;
                        }
                        
                    }else{
                        
                        if j <= 7 {
                            number = 1;
                        }else if j == max {
                            if j > WATER_LEVEL + 2{
                                number = 0;
                            }else{
                                number = 6;
                            }
                        }else if j >= 8  {
                            number = 2;
                        }  else {
                            number = 240;
                        }
                    }
                    
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