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
    position: glm::Vector3<f32>,
    id: usize,
    visible: bool,
    sides: Vec<bool>
}

impl Block {
    pub fn init(position: glm::Vector3<f32>, id: usize) -> Block{
        let cube_sides: Vec<bool> = vec![];
        return Block{
            position,
            id,
            visible: true,
            sides: cube_sides
        };
    }

    pub fn regenerate(&mut self, position: glm::Vector3<f32>, id: usize){
        self.position = position;
        self.id = id;
        self.visible = true;
        self.sides = vec![]
    }

    pub fn set_visibility_vector(&mut self, cube_sides: Vec<bool>){
        self.sides = cube_sides;
        self.visible = true;
    }

    pub fn set_invisiblie(&mut self){
        self.visible = false;
    }

    pub fn get_mesh(&self, vertices: &mut Vec<(glm::Vec3, glm::Vec2, glm::Vec3, f32, f32, bool)>, block_model: &BlockModel){
        if self.id != 240 && self.visible {
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
                                        BlockModel::get_px_uv(block_model)[(self.id * 6) + n],
                                        BlockModel::get_normals(block_model)[n],
                                        BlockModel::get_brightness(block_model)[i],
                                        if self.id == 3{0.8}else{1.0},
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
                                        BlockModel::get_nx_uv(block_model)[(self.id * 6) + n],
                                        BlockModel::get_normals(block_model)[n],
                                        BlockModel::get_brightness(block_model)[i],
                                        if self.id == 3{0.8}else{1.0},
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
                                        BlockModel::get_py_uv(block_model)[(self.id * 6) + n],
                                        BlockModel::get_normals(block_model)[n],
                                        BlockModel::get_brightness(block_model)[i],
                                        if self.id == 3{0.8}else{1.0},
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
                                        BlockModel::get_ny_uv(block_model)[(self.id * 6) + n],
                                        BlockModel::get_normals(block_model)[n],
                                        BlockModel::get_brightness(block_model)[i],
                                        if self.id == 3{0.8}else{1.0},
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
                                        BlockModel::get_pz_uv(block_model)[(self.id * 6) + n],
                                        BlockModel::get_normals(block_model)[n],
                                        BlockModel::get_brightness(block_model)[i],
                                        if self.id == 3{0.8}else{1.0},
                                        Block::is_transparent(&self)
                                    )
                                )
                            },
                        5 => for n in 0..6{
                                vertices.push(
                                    (
                                        glm::vec3(
                                            BlockModel::get_nz(block_model, self.id)[n].x + self.position.x, 
                                            BlockModel::get_nz(block_model, self.id)[n].y + self.position.y - ((self.id == 3) as i32 as f32 * 0.1), 
                                            BlockModel::get_nz(block_model, self.id)[n].z + self.position.z
                                        ),
                                        BlockModel::get_nz_uv(block_model)[(self.id * 6) + n],
                                        BlockModel::get_normals(block_model)[n],
                                        BlockModel::get_brightness(block_model)[i],
                                        if self.id == 3{0.8}else{1.0},
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

    pub fn copy(&self) -> Block{
        return Block {
            position: self.position.clone(),
            id: self.id.clone(), 
            visible: self.visible.clone(),
            sides: self.sides.clone()
        };
    }
    
    pub fn is_air(&self) -> bool{
        return self.id == 240;
    }

    pub fn is_water(&self) -> bool{
        return self.id == 3;
    }

    pub fn is_air_or_water(&self) -> bool{
        return self.id == 240 || self.id == 3;
    }

    pub fn is_visible(&self) -> bool{
        return self.visible;
    }

    pub fn get_position(&self) -> &glm::Vector3<f32>{
        return &self.position;
    }

    pub fn set_block_id(&mut self, new_id: usize){
        self.id = new_id;
    }

    pub fn set_visible(&mut self){
        self.visible = true;
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    fn is_transparent(&self) -> bool {
        if self.id == 3{
            return true;
        }else{
            return false;
        }
    }
}

