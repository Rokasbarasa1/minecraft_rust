use crate::{world::block_model::BlockModel};

extern crate gl;
pub enum BlockId{
    
    GRASS = 0,
    STONE = 1,
    DIRT = 2,
    WATER = 3,
    AIR = 240,
}

pub struct Block {
    pub position: glm::Vector3<f32>,
    pub id: u8,
    pub visible: bool,
    pub sides: Vec<bool>
}

impl Block {
    pub fn init(position: glm::Vector3<f32>, id: u8) -> Block{
        return Block{
            position,
            id,
            visible: true,
            sides: vec![]
        };
    }

    pub fn regenerate(&mut self, position: glm::Vector3<f32>, id: u8){
        self.position = position;
        self.id = id;
    }

    pub fn set_invisiblie(&mut self){
        self.visible = false;
    }

    pub fn get_mesh(&self, vertices: &mut Vec<(glm::Vec3, glm::Vec2, f32, f32, bool)>, block_model: &BlockModel){
        if self.visible {
            for i in 0..self.sides.len() {
                if self.sides[i] == true{
                    match i {
                        0 => for n in 0..6{
                                vertices.push(
                                    (
                                        glm::vec3(
                                            BlockModel::get_px(block_model, self.id)[n].x + self.position.x, 
                                            BlockModel::get_px(block_model, self.id)[n].y + self.position.y, 
                                            BlockModel::get_px(block_model, self.id)[n].z + self.position.z
                                        ),
                                        BlockModel::get_px_uv(block_model)[(self.id as usize * 6) + n],
                                        BlockModel::get_brightness(block_model)[i],
                                        if self.is_water(){0.8}else{1.0},
                                        Block::is_transparent(&self)
                                    )
                                )
                            },
                        1 => for n in 0..6{
                                vertices.push(
                                    (
                                        glm::vec3(
                                            BlockModel::get_nx(block_model, self.id)[n].x + self.position.x, 
                                            BlockModel::get_nx(block_model, self.id)[n].y + self.position.y, 
                                            BlockModel::get_nx(block_model, self.id)[n].z + self.position.z
                                        ),
                                        BlockModel::get_nx_uv(block_model)[(self.id as usize * 6) + n],
                                        BlockModel::get_brightness(block_model)[i],
                                        if self.is_water(){0.8}else{1.0},
                                        Block::is_transparent(&self)
                                    )
                                )
                            },
                        2 =>for n in 0..6{
                                vertices.push(
                                    (
                                        glm::vec3(
                                            BlockModel::get_py(block_model, self.id)[n].x + self.position.x, 
                                            BlockModel::get_py(block_model, self.id)[n].y + self.position.y, 
                                            BlockModel::get_py(block_model, self.id)[n].z + self.position.z
                                        ),
                                        BlockModel::get_py_uv(block_model)[(self.id as usize * 6) + n],
                                        BlockModel::get_brightness(block_model)[i],
                                        if self.is_water(){0.8}else{1.0},
                                        Block::is_transparent(&self)
                                    )
                                )
                            },
                        3 => for n in 0..6{
                                vertices.push(
                                    (
                                        glm::vec3(
                                            BlockModel::get_ny(block_model, self.id)[n].x + self.position.x, 
                                            BlockModel::get_ny(block_model, self.id)[n].y + self.position.y, 
                                            BlockModel::get_ny(block_model, self.id)[n].z + self.position.z
                                        ),
                                        BlockModel::get_ny_uv(block_model)[(self.id as usize * 6) + n],
                                        BlockModel::get_brightness(block_model)[i],
                                        if self.is_water(){0.8}else{1.0},
                                        Block::is_transparent(&self)
                                    )
                                )
                            },
                        4 => for n in 0..6{
                                vertices.push(
                                    (
                                        glm::vec3(
                                            BlockModel::get_pz(block_model, self.id)[n].x + self.position.x, 
                                            BlockModel::get_pz(block_model, self.id)[n].y + self.position.y, 
                                            BlockModel::get_pz(block_model, self.id)[n].z + self.position.z
                                        ),
                                        BlockModel::get_pz_uv(block_model)[(self.id as usize * 6) + n],
                                        BlockModel::get_brightness(block_model)[i],
                                        if self.is_water(){0.8}else{1.0},
                                        Block::is_transparent(&self)
                                    )
                                )
                            },
                        5 => for n in 0..6{
                                vertices.push(
                                    (
                                        glm::vec3(
                                            BlockModel::get_nz(block_model, self.id)[n].x + self.position.x, 
                                            BlockModel::get_nz(block_model, self.id)[n].y + self.position.y - ((self.is_water()) as i32 as f32 * 0.1), 
                                            BlockModel::get_nz(block_model, self.id)[n].z + self.position.z
                                        ),
                                        BlockModel::get_nz_uv(block_model)[(self.id as usize * 6) + n],
                                        BlockModel::get_brightness(block_model)[i],
                                        if self.is_water(){0.8}else{1.0},
                                        Block::is_transparent(&self)
                                    )
                                )
                            },
                        _ => println!("Bad block")
                    }
                }
            }
        }
    }
    
    pub fn is_air(&self) -> bool{
        return self.id == 240;
    }

    pub fn is_water(&self) -> bool{
        return self.id == 3;
    }

    fn is_transparent(&self) -> bool {
        if self.is_water(){
            return true;
        }else{
            return false;
        }
    }
}