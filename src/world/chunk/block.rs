#[macro_use]
use crate::{world::block_model::BlockModel};

pub enum BlockId{
    
    GRASS = 0,
    STONE = 1,
    DIRT = 2,
    WATER = 3,
    AIR = 240,
}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub opacity: f32,
    pub brightness: f32,
}

implement_vertex!(Vertex, position, tex_coords, opacity, brightness);

pub struct Block {
    pub position: [f32; 3],
    pub id: u8,
    pub visible: bool,
    pub sides: Vec<bool>
}

impl Block {
    pub fn init(position: [f32; 3], id: u8) -> Block{

        return Block{
            position,
            id,
            visible: true,
            sides: vec![]
        };
    }

    pub fn regenerate(&mut self, position: [f32; 3], id: u8){
        self.position = position;
        self.id = id;
    }

    pub fn set_invisible(&mut self){
        self.visible = false;
    }

    pub fn get_mesh(&self, vertices: &mut Vec<Vertex>, block_model: &BlockModel, transparencies: &mut Vec<bool>){

        if self.visible {
            for i in 0..self.sides.len() {
                if self.sides[i] == true{
                    match i {
                        0 => for n in 0..6{
                                vertices.push(Vertex { 
                                    position: add(BlockModel::get_px(block_model, self.id)[n], self.position), 
                                    tex_coords: BlockModel::get_px_uv(block_model)[(self.id as usize * 6) + n], 
                                    opacity: if self.is_water(){0.8}else{1.0},
                                    brightness: BlockModel::get_brightness(block_model)[i]
                                });
                                transparencies.push(self.is_transparent());
                            },
                        1 => for n in 0..6{
                                vertices.push(Vertex { 
                                    position: add(BlockModel::get_nx(block_model, self.id)[n], self.position), 
                                    tex_coords: BlockModel::get_nx_uv(block_model)[(self.id as usize * 6) + n], 
                                    opacity: if self.is_water(){0.8}else{1.0},
                                    brightness: BlockModel::get_brightness(block_model)[i]
                                });
                                transparencies.push(self.is_transparent());
                            },
                        2 =>for n in 0..6{
                                vertices.push(Vertex { 
                                    position: add(BlockModel::get_py(block_model, self.id)[n], self.position), 
                                    tex_coords: BlockModel::get_py_uv(block_model)[(self.id as usize * 6) + n], 
                                    opacity: if self.is_water(){0.8}else{1.0},
                                    brightness: BlockModel::get_brightness(block_model)[i]
                                });
                                transparencies.push(self.is_transparent());
                            },
                        3 => for n in 0..6{
                                vertices.push(Vertex { 
                                    position: add(BlockModel::get_ny(block_model, self.id)[n], self.position), 
                                    tex_coords: BlockModel::get_ny_uv(block_model)[(self.id as usize * 6) + n], 
                                    opacity: if self.is_water(){0.8}else{1.0},
                                    brightness: BlockModel::get_brightness(block_model)[i]
                                });
                                transparencies.push(self.is_transparent());
                            },
                        4 => for n in 0..6{
                                vertices.push(Vertex { 
                                    position: add(BlockModel::get_pz(block_model, self.id)[n], self.position), 
                                    tex_coords: BlockModel::get_pz_uv(block_model)[(self.id as usize * 6) + n], 
                                    opacity: if self.is_water(){0.8}else{1.0},
                                    brightness: BlockModel::get_brightness(block_model)[i]
                                });
                                transparencies.push(self.is_transparent());
                            },
                        5 => for n in 0..6{
                                vertices.push(Vertex { 
                                    position: add(BlockModel::get_nz(block_model, self.id)[n], self.position), 
                                    tex_coords: BlockModel::get_nz_uv(block_model)[(self.id as usize * 6) + n], 
                                    opacity: if self.is_water(){0.8}else{1.0},
                                    brightness: BlockModel::get_brightness(block_model)[i]
                                });
                                transparencies.push(self.is_transparent());
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

fn add(arr1: [f32; 3], arr2: [f32; 3]) -> [f32; 3] {
    //Add two vectors
    let mut result: [f32; 3] = [0.0,0.0,0.0];

    result[0] = arr1[0] + arr2[0];
    result[1] = arr1[1] + arr2[1];
    result[2] = arr1[2] + arr2[2];

    result
}
