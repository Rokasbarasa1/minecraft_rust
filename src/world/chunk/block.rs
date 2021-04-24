extern crate gl;
extern crate sdl2;
extern crate stb_image;
extern crate image;
extern crate serde_json;
extern crate glm;
extern crate nalgebra_glm;
extern crate imageproc;

pub enum BlockId{
    AIR = 0,
    STONE = 1,
    DIRT = 2,
    GRASS = 3
}

pub struct Block {
    position: glm::Vector3<f32>,
    id: u32,
}

impl Block {
    pub fn init(position: glm::Vector3<f32>, id: u32) -> Block{
        return Block{
            position,
            id
        };
    }

    pub fn render(&self, loaded_textures: &Vec<u32>, program: &gl::types::GLuint){
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, loaded_textures[0]);

            let mut model = glm::ext::translate(&glm::mat4(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0),  self.position);
            model =  glm::ext::rotate(&model, glm::radians(0.0), glm::vec3(1.0, 0.3, 0.5));
            let model_loc = gl::GetUniformLocation(program.clone(), "model".as_ptr() as *const std::os::raw::c_char);
            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, &model[0][0]);
            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                36,
            );
        }
    }

    pub fn copy(&self) -> Block{
        return Block {
            position: self.position.clone(),
            id: self.id.clone(), 
        };
    }
}
