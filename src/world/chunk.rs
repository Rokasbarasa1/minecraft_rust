extern crate glm;
pub mod block;
pub struct Chunk {
    position: glm::Vector3<f32>,
    grid_x: i32,
    grid_z: i32,
    blocks: Vec<Vec<Vec<block::Block>>>,
}

impl Chunk{
    pub fn init(grid_x: i32, grid_z: i32, position: glm::Vector3<f32>, square_chunk_width: &u32, block_radius: &f32) -> Chunk{
        let end_position = glm::vec3(position.x - (*square_chunk_width as f32 /2.0 * block_radius), position.y, position.z - (*square_chunk_width as f32 /2.0 * block_radius));
        let mut x_pos = position.x.clone();
        let mut z_pos = position.z.clone();
        let mut y_pos = position.y.clone();
        let x_pos_temp = position.x.clone();
        let y_pos_temp = position.y.clone();

        let mut blocks: Vec<Vec<Vec<block::Block>>> = vec![];
        for i in 0..*square_chunk_width as usize {  //Z line Go from positive to negative
            let collumn: Vec<Vec<block::Block>> = vec![];
            blocks.push(collumn);
            for k in 0..*square_chunk_width as usize { //X line go from positive to negative
                let row: Vec<block::Block> = vec![];
                blocks[i].push(row);
                for j in 0..10{
                    //Maybe later do air rendering 
                    let mut number = j%4;//k % 4;

                    blocks[i][k].push(block::Block::init(glm::vec3(x_pos * block_radius, y_pos * block_radius, z_pos * block_radius), number as u32));
                    y_pos += 1.0;
                }
                y_pos = y_pos_temp;
                x_pos -= 1.0;
                
            }
            x_pos = x_pos_temp;
            z_pos -= 1.0;
        }

        return Chunk{
            position: end_position,
            grid_x,
            grid_z,
            blocks
        };
    }

    pub fn render(&self, loaded_textures: &Vec<u32>, program: &gl::types::GLuint){
        for i in 0..self.blocks.len() {
            for k in 0..self.blocks[i].len() {
                for j in 0..self.blocks[i][k].len() {
                    block::Block::render(&self.blocks[i][k][j], loaded_textures, program);
                }
            }
        }
    }

    pub fn get_position(&self) -> &glm::Vector3<f32>{
        return &self.position;
    }

    pub fn copy(&self) -> Chunk{
        let mut copy_chunk = Chunk {
            position: self.position.clone(),
            grid_x: self.grid_x.clone(),
            grid_z: self.grid_x.clone(),
            blocks: vec![],
        };

        for i in 0..self.blocks.len() {
            copy_chunk.blocks.push(vec![]);
            for k in 0..self.blocks[i].len() {
                copy_chunk.blocks[i].push(vec![]);
                for j in 0..self.blocks[i][k].len(){
                    copy_chunk.blocks[i][k].push(block::Block::copy(&self.blocks[i][k][j]));
                }
            }
        }

        return copy_chunk;
    }

    pub fn compare(&self, other_chunk: &Chunk) -> bool{
        if self.position.x == other_chunk.position.x && self.position.y == other_chunk.position.y && self.position.z == other_chunk.position.z {
            return true;
        } else {
            return false;
        }
    }

    pub fn get_blocks_vector(&self) -> &Vec<Vec<Vec<block::Block>>> {
        return &self.blocks;
    }

    pub fn get_blocks_vector_mutable(&mut self) -> &mut Vec<Vec<Vec<block::Block>>> {
        return &mut self.blocks;
    }
}