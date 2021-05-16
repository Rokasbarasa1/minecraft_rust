use crate::world::block_model::BlockModel;

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

    pub fn set_visibility_vector(&mut self, cube_sides: Vec<bool>){
        self.sides = cube_sides;
        self.visible = true;
    }

    pub fn set_invisiblie(&mut self){
        self.visible = false;
    }

    pub fn get_mesh(&self, vertices: &mut Vec<(glm::Vec3, glm::Vec2, glm::Vec3)>, block_model: &BlockModel){
        if self.id != 240 && self.visible {
            for i in 0..self.sides.len() {
                if self.sides[i] == true{
                    match i {
                        0 => for n in 0..6{
                                vertices.push(
                                    (
                                        glm::vec3(
                                            BlockModel::get_px(block_model)[n].x + self.position.x, 
                                            BlockModel::get_px(block_model)[n].y + self.position.y, 
                                            BlockModel::get_px(block_model)[n].z + self.position.z
                                        ),
                                        BlockModel::get_px_uv(block_model)[(self.id * 6) + n],
                                        BlockModel::get_normals(block_model)[n]
                                    )
                                )
                            },
                        1 => for n in 0..6{
                                vertices.push(
                                    (
                                        glm::vec3(
                                            BlockModel::get_nx(block_model)[n].x + self.position.x, 
                                            BlockModel::get_nx(block_model)[n].y + self.position.y, 
                                            BlockModel::get_nx(block_model)[n].z + self.position.z
                                        ),
                                        BlockModel::get_nx_uv(block_model)[(self.id * 6) + n],
                                        BlockModel::get_normals(block_model)[n]
                                    )
                                )
                            },
                        2 =>for n in 0..6{
                                vertices.push(
                                    (
                                        glm::vec3(
                                            BlockModel::get_py(block_model)[n].x + self.position.x, 
                                            BlockModel::get_py(block_model)[n].y + self.position.y, 
                                            BlockModel::get_py(block_model)[n].z + self.position.z
                                        ),
                                        BlockModel::get_py_uv(block_model)[(self.id * 6) + n],
                                        BlockModel::get_normals(block_model)[n]
                                    )
                                )
                            },
                        3 => for n in 0..6{
                                vertices.push(
                                    (
                                        glm::vec3(
                                            BlockModel::get_ny(block_model)[n].x + self.position.x, 
                                            BlockModel::get_ny(block_model)[n].y + self.position.y, 
                                            BlockModel::get_ny(block_model)[n].z + self.position.z
                                        ),
                                        BlockModel::get_ny_uv(block_model)[(self.id * 6) + n],
                                        BlockModel::get_normals(block_model)[n]
                                    )
                                )
                            },
                        4 => for n in 0..6{
                                vertices.push(
                                    (
                                        glm::vec3(
                                            BlockModel::get_pz(block_model)[n].x + self.position.x, 
                                            BlockModel::get_pz(block_model)[n].y + self.position.y, 
                                            BlockModel::get_pz(block_model)[n].z + self.position.z
                                        ),
                                        BlockModel::get_pz_uv(block_model)[(self.id * 6) + n],
                                        BlockModel::get_normals(block_model)[n]
                                    )
                                )
                            },
                        5 => for n in 0..6{
                                vertices.push(
                                    (
                                        glm::vec3(
                                            BlockModel::get_nz(block_model)[n].x + self.position.x, 
                                            BlockModel::get_nz(block_model)[n].y + self.position.y, 
                                            BlockModel::get_nz(block_model)[n].z + self.position.z
                                        ),
                                        BlockModel::get_nz_uv(block_model)[(self.id * 6) + n],
                                        BlockModel::get_normals(block_model)[n]
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
}

