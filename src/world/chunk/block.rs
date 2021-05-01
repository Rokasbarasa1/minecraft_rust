extern crate gl;

pub enum BlockId{
    AIR = 0,
    STONE = 1,
    DIRT = 2,
    GRASS = 3
}

pub struct Block {
    position: glm::Vector3<f32>,
    id: u8,
    visible: bool,
    sides: Vec<bool>
}

impl Block {
    pub fn init(position: glm::Vector3<f32>, id: u8) -> Block{
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
    }

    pub fn set_invisiblie(&mut self){
        self.visible = false;
    }

    pub fn render(&self, loaded_textures: &Vec<u32>, program: &gl::types::GLuint){
        if self.visible == true {
            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, loaded_textures[self.id as usize]);

                let mut model = glm::ext::translate(&glm::mat4(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0),  self.position);
                model =  glm::ext::rotate(&model, glm::radians(0.0), glm::vec3(1.0, 0.3, 0.5));
                let model_loc = gl::GetUniformLocation(program.clone(), "model".as_ptr() as *const std::os::raw::c_char);
                gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, &model[0][0]);
                let mut index = 0;
                for i in 0..self.sides.len() {
                    if self.sides[i] == true{
                        if self.id == 3 && i == 5 {
                            gl::BindTexture(gl::TEXTURE_2D, loaded_textures[0]);
                        } else if self.id == 3 && i != 5 && i != 4{
                            gl::BindTexture(gl::TEXTURE_2D, loaded_textures[3]);
                        } else if self.id == 3{
                            gl::BindTexture(gl::TEXTURE_2D, loaded_textures[2]);
                        }
                        
                        gl::DrawArrays(
                            gl::TRIANGLES,
                            index,
                            6,
                        );
                    }
                    index += 6;
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
        return self.id == 0;
    }
}

