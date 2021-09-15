extern crate glm;

pub struct BlockModel{
    px_pos: Vec<glm::Vec3>,
    nx_pos: Vec<glm::Vec3>,
    py_pos: Vec<glm::Vec3>,
    ny_pos: Vec<glm::Vec3>,
    pz_pos: Vec<glm::Vec3>,
    nz_pos: Vec<glm::Vec3>,
    px_pos_water: Vec<glm::Vec3>,
    nx_pos_water: Vec<glm::Vec3>,
    py_pos_water: Vec<glm::Vec3>,
    ny_pos_water: Vec<glm::Vec3>,
    pz_pos_water: Vec<glm::Vec3>,
    nz_pos_water: Vec<glm::Vec3>,
    px_uv: Vec<glm::Vec2>,
    nx_uv: Vec<glm::Vec2>,
    py_uv: Vec<glm::Vec2>,
    ny_uv: Vec<glm::Vec2>,
    pz_uv: Vec<glm::Vec2>,
    nz_uv: Vec<glm::Vec2>,
    brightness: Vec<f32>
}


impl BlockModel {

    pub fn init() -> BlockModel{
        let px_pos: Vec<glm::Vec3> = vec![
            glm::vec3(-0.5, -0.5, -0.5),
            glm::vec3( 0.5,  0.5, -0.5),
            glm::vec3(0.5, -0.5, -0.5),
            glm::vec3(0.5,  0.5, -0.5),
            glm::vec3(-0.5, -0.5, -0.5),
            glm::vec3(-0.5,  0.5, -0.5)
        ];

        let nx_pos: Vec<glm::Vec3> = vec![
            glm::vec3(-0.5, -0.5,  0.5),
            glm::vec3(0.5, -0.5,  0.5),
            glm::vec3(0.5,  0.5,  0.5),
            glm::vec3(0.5,  0.5,  0.5),
            glm::vec3(-0.5,  0.5,  0.5),
            glm::vec3(-0.5, -0.5,  0.5)
        ];

        let py_pos: Vec<glm::Vec3> = vec![
            glm::vec3(-0.5,  0.5,  0.5),
            glm::vec3(-0.5,  0.5, -0.5),
            glm::vec3(-0.5, -0.5, -0.5),
            glm::vec3(-0.5, -0.5, -0.5),
            glm::vec3(-0.5, -0.5,  0.5),
            glm::vec3(-0.5,  0.5,  0.5)
        ];

        let ny_pos: Vec<glm::Vec3> = vec![
            glm::vec3(0.5,  0.5,  0.5),
            glm::vec3(0.5, -0.5, -0.5),
            glm::vec3(0.5,  0.5, -0.5),
            glm::vec3(0.5, -0.5, -0.5),
            glm::vec3(0.5,  0.5,  0.5),
            glm::vec3(0.5, -0.5,  0.5)
        ];

        let pz_pos: Vec<glm::Vec3> = vec![
            glm::vec3(-0.5, -0.5, -0.5),
            glm::vec3(0.5, -0.5, -0.5),
            glm::vec3(0.5, -0.5,  0.5),
            glm::vec3(0.5, -0.5,  0.5),
            glm::vec3(-0.5, -0.5,  0.5),
            glm::vec3(-0.5, -0.5, -0.5)
        ];

        let nz_pos: Vec<glm::Vec3> = vec![
            glm::vec3(-0.5,  0.5, -0.5),
            glm::vec3(0.5,  0.5,  0.5),
            glm::vec3(0.5,  0.5, -0.5),
            glm::vec3( 0.5,  0.5,  0.5),
            glm::vec3(-0.5,  0.5, -0.5),
            glm::vec3(-0.5,  0.5,  0.5)
        ];




        let px_pos_water: Vec<glm::Vec3> = vec![
            glm::vec3(-0.5, -0.5, -0.5),
            glm::vec3( 0.5,  0.40, -0.5),
            glm::vec3(0.5, -0.5, -0.5),
            glm::vec3(0.5,  0.40, -0.5),
            glm::vec3(-0.5, -0.5, -0.5),
            glm::vec3(-0.5,  0.40, -0.5)
        ];

        let nx_pos_water: Vec<glm::Vec3> = vec![
            glm::vec3(-0.5, -0.5,  0.5),
            glm::vec3(0.5, -0.5,  0.5),
            glm::vec3(0.5,  0.40,  0.5),
            glm::vec3(0.5,  0.40,  0.5),
            glm::vec3(-0.5,  0.40,  0.5),
            glm::vec3(-0.5, -0.5,  0.5)
        ];

        let py_pos_water: Vec<glm::Vec3> = vec![
            glm::vec3(-0.5,  0.40,  0.5),
            glm::vec3(-0.5,  0.40, -0.5),
            glm::vec3(-0.5, -0.5, -0.5),
            glm::vec3(-0.5, -0.5, -0.5),
            glm::vec3(-0.5, -0.5,  0.5),
            glm::vec3(-0.5,  0.40,  0.5)
        ];

        let ny_pos_water: Vec<glm::Vec3> = vec![
            glm::vec3(0.5,  0.40,  0.5),
            glm::vec3(0.5, -0.5, -0.5),
            glm::vec3(0.5,  0.40, -0.5),
            glm::vec3(0.5, -0.5, -0.5),
            glm::vec3(0.5,  0.40,  0.5),
            glm::vec3(0.5, -0.5,  0.5)
        ];

        let pz_pos_water: Vec<glm::Vec3> = vec![
            glm::vec3(-0.5, -0.5, -0.5),
            glm::vec3(0.5, -0.5, -0.5),
            glm::vec3(0.5, -0.5,  0.5),
            glm::vec3(0.5, -0.5,  0.5),
            glm::vec3(-0.5, -0.5,  0.5),
            glm::vec3(-0.5, -0.5, -0.5)
        ];

        let nz_pos_water: Vec<glm::Vec3> = vec![
            glm::vec3(-0.5,  0.5, -0.5),
            glm::vec3(0.5,  0.5,  0.5),
            glm::vec3(0.5,  0.5, -0.5),
            glm::vec3( 0.5,  0.5,  0.5),
            glm::vec3(-0.5,  0.5, -0.5),
            glm::vec3(-0.5,  0.5,  0.5)
        ];






        let px_uv: Vec<glm::Vec2> = vec![
            //GRASS
            glm::vec2(4.0 / 16.0, 1.0 / 16.0),
            glm::vec2(3.0 / 16.0, 0.0 / 16.0),
            glm::vec2(3.0 / 16.0, 1.0 / 16.0),
            glm::vec2(3.0 / 16.0, 0.0 / 16.0),
            glm::vec2(4.0 / 16.0, 1.0 / 16.0),
            glm::vec2(4.0 / 16.0, 0.0 / 16.0),
            //STONE
            glm::vec2(2.0 / 16.0, 1.0 / 16.0),
            glm::vec2(1.0 / 16.0, 0.0 / 16.0),
            glm::vec2(1.0 / 16.0, 1.0 / 16.0),
            glm::vec2(1.0 / 16.0, 0.0 / 16.0),
            glm::vec2(2.0 / 16.0, 1.0 / 16.0),
            glm::vec2(2.0 / 16.0, 0.0 / 16.0),
            //DIRT
            glm::vec2(3.0 / 16.0, 1.0 / 16.0),
            glm::vec2(2.0 / 16.0, 0.0 / 16.0),
            glm::vec2(2.0 / 16.0, 1.0 / 16.0),
            glm::vec2(2.0 / 16.0, 0.0 / 16.0),
            glm::vec2(3.0 / 16.0, 1.0 / 16.0),
            glm::vec2(3.0 / 16.0, 0.0 / 16.0),
            //WATER
            glm::vec2(5.0 / 16.0, 1.0 / 16.0),
            glm::vec2(4.0 / 16.0, 0.0 / 16.0),
            glm::vec2(4.0 / 16.0, 1.0 / 16.0),
            glm::vec2(4.0 / 16.0, 0.0 / 16.0),
            glm::vec2(5.0 / 16.0, 1.0 / 16.0),
            glm::vec2(5.0 / 16.0, 0.0 / 16.0),
            //WOOD PLANKS
            glm::vec2(6.0 / 16.0, 1.0 / 16.0),
            glm::vec2(5.0 / 16.0, 0.0 / 16.0),
            glm::vec2(5.0 / 16.0, 1.0 / 16.0),
            glm::vec2(5.0 / 16.0, 0.0 / 16.0),
            glm::vec2(6.0 / 16.0, 1.0 / 16.0),
            glm::vec2(6.0 / 16.0, 0.0 / 16.0),
            //LOG SIDE
            glm::vec2(7.0 / 16.0, 1.0 / 16.0),
            glm::vec2(6.0 / 16.0, 0.0 / 16.0),
            glm::vec2(6.0 / 16.0, 1.0 / 16.0),
            glm::vec2(6.0 / 16.0, 0.0 / 16.0),
            glm::vec2(7.0 / 16.0, 1.0 / 16.0),
            glm::vec2(7.0 / 16.0, 0.0 / 16.0),
            //SAND
            glm::vec2(9.0 / 16.0, 1.0 / 16.0),
            glm::vec2(8.0 / 16.0, 0.0 / 16.0),
            glm::vec2(8.0 / 16.0, 1.0 / 16.0),
            glm::vec2(8.0 / 16.0, 0.0 / 16.0),
            glm::vec2(9.0 / 16.0, 1.0 / 16.0),
            glm::vec2(9.0 / 16.0, 0.0 / 16.0),
            //LEAVES
            glm::vec2(10.0 / 16.0, 1.0 / 16.0),
            glm::vec2(9.0 / 16.0, 0.0 / 16.0),
            glm::vec2(9.0 / 16.0, 1.0 / 16.0),
            glm::vec2(9.0 / 16.0, 0.0 / 16.0),
            glm::vec2(10.0 / 16.0, 1.0 / 16.0),
            glm::vec2(10.0 / 16.0, 0.0 / 16.0),

            glm::vec2(1.0, 1.0),
            glm::vec2(0.0, 0.0),
            glm::vec2(0.0, 1.0),
            glm::vec2(0.0, 0.0),
            glm::vec2(1.0, 1.0),
            glm::vec2(1.0, 0.0),

        ];

        let nx_uv: Vec<glm::Vec2> = vec![
//GRASS
            glm::vec2(4.0 / 16.0, 1.0 / 16.0),
            glm::vec2(3.0 / 16.0, 1.0 / 16.0),
            glm::vec2(3.0 / 16.0, 0.0 / 16.0),
            glm::vec2(3.0 / 16.0, 0.0 / 16.0),
            glm::vec2(4.0 / 16.0, 0.0 / 16.0),
            glm::vec2(4.0 / 16.0, 1.0 / 16.0),
//STONE
            glm::vec2(2.0 / 16.0, 1.0 / 16.0),
            glm::vec2(1.0 / 16.0, 1.0 / 16.0),
            glm::vec2(1.0 / 16.0, 0.0 / 16.0),
            glm::vec2(1.0 / 16.0, 0.0 / 16.0),
            glm::vec2(2.0 / 16.0, 0.0 / 16.0),
            glm::vec2(2.0 / 16.0, 1.0 / 16.0),
//DIRT
            glm::vec2(3.0 / 16.0, 1.0 / 16.0),
            glm::vec2(2.0 / 16.0, 1.0 / 16.0),
            glm::vec2(2.0 / 16.0, 0.0 / 16.0),
            glm::vec2(2.0 / 16.0, 0.0 / 16.0),
            glm::vec2(3.0 / 16.0, 0.0 / 16.0),
            glm::vec2(3.0 / 16.0, 1.0 / 16.0),
//WATER
            glm::vec2(5.0 / 16.0, 1.0 / 16.0),
            glm::vec2(4.0 / 16.0, 1.0 / 16.0),
            glm::vec2(4.0 / 16.0, 0.0 / 16.0),
            glm::vec2(4.0 / 16.0, 0.0 / 16.0),
            glm::vec2(5.0 / 16.0, 0.0 / 16.0),
            glm::vec2(5.0 / 16.0, 1.0 / 16.0),
//WOOD PLANKS
            glm::vec2(6.0 / 16.0, 1.0 / 16.0),
            glm::vec2(5.0 / 16.0, 1.0 / 16.0),
            glm::vec2(5.0 / 16.0, 0.0 / 16.0),
            glm::vec2(5.0 / 16.0, 0.0 / 16.0),
            glm::vec2(6.0 / 16.0, 0.0 / 16.0),
            glm::vec2(6.0 / 16.0, 1.0 / 16.0),
//LOG SIDE
            glm::vec2(7.0 / 16.0, 1.0 / 16.0),
            glm::vec2(6.0 / 16.0, 1.0 / 16.0),
            glm::vec2(6.0 / 16.0, 0.0 / 16.0),
            glm::vec2(6.0 / 16.0, 0.0 / 16.0),
            glm::vec2(7.0 / 16.0, 0.0 / 16.0),
            glm::vec2(7.0 / 16.0, 1.0 / 16.0),
//SAND
            glm::vec2(9.0 / 16.0, 1.0 / 16.0),
            glm::vec2(8.0 / 16.0, 1.0 / 16.0),
            glm::vec2(8.0 / 16.0, 0.0 / 16.0),
            glm::vec2(8.0 / 16.0, 0.0 / 16.0),
            glm::vec2(9.0 / 16.0, 0.0 / 16.0),
            glm::vec2(9.0 / 16.0, 1.0 / 16.0),
//LEAVES
            glm::vec2(10.0 / 16.0, 1.0 / 16.0),
            glm::vec2(9.0 / 16.0, 1.0 / 16.0),
            glm::vec2(9.0 / 16.0, 0.0 / 16.0),
            glm::vec2(9.0 / 16.0, 0.0 / 16.0),
            glm::vec2(10.0 / 16.0, 0.0 / 16.0),
            glm::vec2(10.0 / 16.0, 1.0 / 16.0),

            glm::vec2(4.0, 1.0),
            glm::vec2(3.0, 1.0),
            glm::vec2(3.0, 0.0),
            glm::vec2(3.0, 0.0),
            glm::vec2(4.0, 0.0),
            glm::vec2(4.0, 1.0),

        ];

        let py_uv: Vec<glm::Vec2> = vec![
//GRASS
            glm::vec2(4.0 / 16.0, 0.0 / 16.0),
            glm::vec2(3.0 / 16.0, 0.0 / 16.0),
            glm::vec2(3.0 / 16.0, 1.0 / 16.0),
            glm::vec2(3.0 / 16.0, 1.0 / 16.0),
            glm::vec2(4.0 / 16.0, 1.0 / 16.0),
            glm::vec2(4.0 / 16.0, 0.0 / 16.0),
//STONE
            glm::vec2(2.0 / 16.0, 0.0 / 16.0),
            glm::vec2(1.0 / 16.0, 0.0 / 16.0),
            glm::vec2(1.0 / 16.0, 1.0 / 16.0),
            glm::vec2(1.0 / 16.0, 1.0 / 16.0),
            glm::vec2(2.0 / 16.0, 1.0 / 16.0),
            glm::vec2(2.0 / 16.0, 0.0 / 16.0),
//DIRT
            glm::vec2(3.0 / 16.0, 0.0 / 16.0),
            glm::vec2(2.0 / 16.0, 0.0 / 16.0),
            glm::vec2(2.0 / 16.0, 1.0 / 16.0),
            glm::vec2(2.0 / 16.0, 1.0 / 16.0),
            glm::vec2(3.0 / 16.0, 1.0 / 16.0),
            glm::vec2(3.0 / 16.0, 0.0 / 16.0),
//WATER
            glm::vec2(5.0 / 16.0, 0.0 / 16.0),
            glm::vec2(4.0 / 16.0, 0.0 / 16.0),
            glm::vec2(4.0 / 16.0, 1.0 / 16.0),
            glm::vec2(4.0 / 16.0, 1.0 / 16.0),
            glm::vec2(5.0 / 16.0, 1.0 / 16.0),
            glm::vec2(5.0 / 16.0, 0.0 / 16.0),
//WOOD PLANKS
            glm::vec2(6.0 / 16.0, 0.0 / 16.0),
            glm::vec2(5.0 / 16.0, 0.0 / 16.0),
            glm::vec2(5.0 / 16.0, 1.0 / 16.0),
            glm::vec2(5.0 / 16.0, 1.0 / 16.0),
            glm::vec2(6.0 / 16.0, 1.0 / 16.0),
            glm::vec2(6.0 / 16.0, 0.0 / 16.0),
//LOG SIDE
            glm::vec2(7.0 / 16.0, 0.0 / 16.0),
            glm::vec2(6.0 / 16.0, 0.0 / 16.0),
            glm::vec2(6.0 / 16.0, 1.0 / 16.0),
            glm::vec2(6.0 / 16.0, 1.0 / 16.0),
            glm::vec2(7.0 / 16.0, 1.0 / 16.0),
            glm::vec2(7.0 / 16.0, 0.0 / 16.0),
//SAND
            glm::vec2(9.0 / 16.0, 0.0 / 16.0),
            glm::vec2(8.0 / 16.0, 0.0 / 16.0),
            glm::vec2(8.0 / 16.0, 1.0 / 16.0),
            glm::vec2(8.0 / 16.0, 1.0 / 16.0),
            glm::vec2(9.0 / 16.0, 1.0 / 16.0),
            glm::vec2(9.0 / 16.0, 0.0 / 16.0),
//Leaves
            glm::vec2(10.0 / 16.0, 0.0 / 16.0),
            glm::vec2(9.0 / 16.0, 0.0 / 16.0),
            glm::vec2(9.0 / 16.0, 1.0 / 16.0),
            glm::vec2(9.0 / 16.0, 1.0 / 16.0),
            glm::vec2(10.0 / 16.0, 1.0 / 16.0),
            glm::vec2(10.0 / 16.0, 0.0 / 16.0),

            glm::vec2(1.0, 0.0),
            glm::vec2(0.0, 0.0),
            glm::vec2(0.0, 1.0),
            glm::vec2(0.0, 1.0),
            glm::vec2(1.0, 1.0),
            glm::vec2(1.0, 0.0),
        ];

        let ny_uv: Vec<glm::Vec2> = vec![
//GRASS
            glm::vec2(3.0 / 16.0, 0.0 / 16.0),
            glm::vec2(4.0 / 16.0, 1.0 / 16.0),
            glm::vec2(4.0 / 16.0, 0.0 / 16.0),
            glm::vec2(4.0 / 16.0, 1.0 / 16.0),
            glm::vec2(3.0 / 16.0, 0.0 / 16.0),
            glm::vec2(3.0 / 16.0, 1.0 / 16.0),
//STONE
            glm::vec2(1.0 / 16.0, 0.0 / 16.0),
            glm::vec2(2.0 / 16.0, 1.0 / 16.0),
            glm::vec2(2.0 / 16.0, 0.0 / 16.0),
            glm::vec2(2.0 / 16.0, 1.0 / 16.0),
            glm::vec2(1.0 / 16.0, 0.0 / 16.0),
            glm::vec2(1.0 / 16.0, 1.0 / 16.0),
//DIRT
            glm::vec2(2.0 / 16.0, 0.0 / 16.0),
            glm::vec2(3.0 / 16.0, 1.0 / 16.0),
            glm::vec2(3.0 / 16.0, 0.0 / 16.0),
            glm::vec2(3.0 / 16.0, 1.0 / 16.0),
            glm::vec2(2.0 / 16.0, 0.0 / 16.0),
            glm::vec2(2.0 / 16.0, 1.0 / 16.0),
//WATER
            glm::vec2(4.0 / 16.0, 0.0 / 16.0),
            glm::vec2(5.0 / 16.0, 1.0 / 16.0),
            glm::vec2(5.0 / 16.0, 0.0 / 16.0),
            glm::vec2(5.0 / 16.0, 1.0 / 16.0),
            glm::vec2(4.0 / 16.0, 0.0 / 16.0),
            glm::vec2(4.0 / 16.0, 1.0 / 16.0),
//WOOD PLANKS
            glm::vec2(5.0 / 16.0, 0.0 / 16.0),
            glm::vec2(6.0 / 16.0, 1.0 / 16.0),
            glm::vec2(6.0 / 16.0, 0.0 / 16.0),
            glm::vec2(6.0 / 16.0, 1.0 / 16.0),
            glm::vec2(5.0 / 16.0, 0.0 / 16.0),
            glm::vec2(5.0 / 16.0, 1.0 / 16.0),
//LOG SIDE
            glm::vec2(6.0 / 16.0, 0.0 / 16.0),
            glm::vec2(7.0 / 16.0, 1.0 / 16.0),
            glm::vec2(7.0 / 16.0, 0.0 / 16.0),
            glm::vec2(7.0 / 16.0, 1.0 / 16.0),
            glm::vec2(6.0 / 16.0, 0.0 / 16.0),
            glm::vec2(6.0 / 16.0, 1.0 / 16.0),
//SAND
            glm::vec2(8.0 / 16.0, 0.0 / 16.0),
            glm::vec2(9.0 / 16.0, 1.0 / 16.0),
            glm::vec2(9.0 / 16.0, 0.0 / 16.0),
            glm::vec2(9.0 / 16.0, 1.0 / 16.0),
            glm::vec2(8.0 / 16.0, 0.0 / 16.0),
            glm::vec2(8.0 / 16.0, 1.0 / 16.0),
//Leaves
            glm::vec2(9.0  / 16.0, 0.0 / 16.0),
            glm::vec2(10.0 / 16.0, 1.0 / 16.0),
            glm::vec2(10.0 / 16.0, 0.0 / 16.0),
            glm::vec2(10.0 / 16.0, 1.0 / 16.0),
            glm::vec2(9.0  / 16.0, 0.0 / 16.0),
            glm::vec2(9.0  / 16.0, 1.0 / 16.0),

            glm::vec2(0.0, 0.0),
            glm::vec2(1.0, 1.0),
            glm::vec2(1.0, 0.0),
            glm::vec2(1.0, 1.0),
            glm::vec2(0.0, 0.0),
            glm::vec2(0.0, 1.0)
            
        ];

        let pz_uv: Vec<glm::Vec2> = vec![
//GRASS
            glm::vec2(3.0 / 16.0, 0.0 / 16.0),
            glm::vec2(2.0 / 16.0, 0.0 / 16.0),
            glm::vec2(2.0 / 16.0, 1.0 / 16.0),
            glm::vec2(2.0 / 16.0, 1.0 / 16.0),
            glm::vec2(3.0 / 16.0, 1.0 / 16.0),
            glm::vec2(3.0 / 16.0, 0.0 / 16.0),

//STONE
            glm::vec2(2.0 / 16.0, 0.0 / 16.0),
            glm::vec2(1.0 / 16.0, 0.0 / 16.0),
            glm::vec2(1.0 / 16.0, 1.0 / 16.0),
            glm::vec2(1.0 / 16.0, 1.0 / 16.0),
            glm::vec2(2.0 / 16.0, 1.0 / 16.0),
            glm::vec2(2.0 / 16.0, 0.0 / 16.0),

//DIRT
            glm::vec2(3.0 / 16.0, 0.0 / 16.0),
            glm::vec2(2.0 / 16.0, 0.0 / 16.0),
            glm::vec2(2.0 / 16.0, 1.0 / 16.0),
            glm::vec2(2.0 / 16.0, 1.0 / 16.0),
            glm::vec2(3.0 / 16.0, 1.0 / 16.0),
            glm::vec2(3.0 / 16.0, 0.0 / 16.0),

//WATER
            glm::vec2(5.0 / 16.0, 0.0 / 16.0),
            glm::vec2(4.0 / 16.0, 0.0 / 16.0),
            glm::vec2(4.0 / 16.0, 1.0 / 16.0),
            glm::vec2(4.0 / 16.0, 1.0 / 16.0),
            glm::vec2(5.0 / 16.0, 1.0 / 16.0),
            glm::vec2(5.0 / 16.0, 0.0 / 16.0),

//WOOD PLANKS
            glm::vec2(6.0 / 16.0, 0.0 / 16.0),
            glm::vec2(5.0 / 16.0, 0.0 / 16.0),
            glm::vec2(5.0 / 16.0, 1.0 / 16.0),
            glm::vec2(5.0 / 16.0, 1.0 / 16.0),
            glm::vec2(6.0 / 16.0, 1.0 / 16.0),
            glm::vec2(6.0 / 16.0, 0.0 / 16.0),
//LOG TOP
            glm::vec2(8.0 / 16.0, 0.0 / 16.0),
            glm::vec2(7.0 / 16.0, 0.0 / 16.0),
            glm::vec2(7.0 / 16.0, 1.0 / 16.0),
            glm::vec2(7.0 / 16.0, 1.0 / 16.0),
            glm::vec2(8.0 / 16.0, 1.0 / 16.0),
            glm::vec2(8.0 / 16.0, 0.0 / 16.0),
//SAND
            glm::vec2(9.0 / 16.0, 0.0 / 16.0),
            glm::vec2(8.0 / 16.0, 0.0 / 16.0),
            glm::vec2(8.0 / 16.0, 1.0 / 16.0),
            glm::vec2(8.0 / 16.0, 1.0 / 16.0),
            glm::vec2(9.0 / 16.0, 1.0 / 16.0),
            glm::vec2(9.0 / 16.0, 0.0 / 16.0),
//Leaves
            glm::vec2(10.0 / 16.0, 0.0 / 16.0),
            glm::vec2(9.0 / 16.0, 0.0 / 16.0),
            glm::vec2(9.0 / 16.0, 1.0 / 16.0),
            glm::vec2(9.0 / 16.0, 1.0 / 16.0),
            glm::vec2(10.0 / 16.0, 1.0 / 16.0),
            glm::vec2(10.0 / 16.0, 0.0 / 16.0),

            glm::vec2(1.0, 0.0),
            glm::vec2(0.0, 0.0),
            glm::vec2(0.0, 1.0),
            glm::vec2(1.0, 0.0),
            glm::vec2(0.0, 0.0),
            glm::vec2(0.0, 1.0)
        ];

        let nz_uv: Vec<glm::Vec2> = vec![
//GRASS
            glm::vec2(1.0 / 16.0, 1.0 / 16.0),
            glm::vec2(0.0 / 16.0, 0.0 / 16.0),
            glm::vec2(0.0 / 16.0, 1.0 / 16.0),
            glm::vec2(0.0 / 16.0, 0.0 / 16.0),
            glm::vec2(1.0 / 16.0, 1.0 / 16.0),
            glm::vec2(1.0 / 16.0, 0.0 / 16.0),
//STONE
            glm::vec2(2.0 / 16.0, 1.0 / 16.0),
            glm::vec2(1.0 / 16.0, 0.0 / 16.0),
            glm::vec2(1.0 / 16.0, 1.0 / 16.0),
            glm::vec2(1.0 / 16.0, 0.0 / 16.0),
            glm::vec2(2.0 / 16.0, 1.0 / 16.0),
            glm::vec2(2.0 / 16.0, 0.0 / 16.0),
//DIRT
            glm::vec2(3.0 / 16.0, 1.0 / 16.0),
            glm::vec2(2.0 / 16.0, 0.0 / 16.0),
            glm::vec2(2.0 / 16.0, 1.0 / 16.0),
            glm::vec2(2.0 / 16.0, 0.0 / 16.0),
            glm::vec2(3.0 / 16.0, 1.0 / 16.0),
            glm::vec2(3.0 / 16.0, 0.0 / 16.0),
//WATER
            glm::vec2(5.0 / 16.0, 1.0 / 16.0),
            glm::vec2(4.0 / 16.0, 0.0 / 16.0),
            glm::vec2(4.0 / 16.0, 1.0 / 16.0),
            glm::vec2(4.0 / 16.0, 0.0 / 16.0),
            glm::vec2(5.0 / 16.0, 1.0 / 16.0),
            glm::vec2(5.0 / 16.0, 0.0 / 16.0),
//WOOD PLANKS
            glm::vec2(6.0 / 16.0, 1.0 / 16.0),
            glm::vec2(5.0 / 16.0, 0.0 / 16.0),
            glm::vec2(5.0 / 16.0, 1.0 / 16.0),
            glm::vec2(5.0 / 16.0, 0.0 / 16.0),
            glm::vec2(6.0 / 16.0, 1.0 / 16.0),
            glm::vec2(6.0 / 16.0, 0.0 / 16.0),
//LOG TOP
            glm::vec2(8.0 / 16.0, 1.0 / 16.0),
            glm::vec2(7.0 / 16.0, 0.0 / 16.0),
            glm::vec2(7.0 / 16.0, 1.0 / 16.0),
            glm::vec2(7.0 / 16.0, 0.0 / 16.0),
            glm::vec2(8.0 / 16.0, 1.0 / 16.0),
            glm::vec2(8.0 / 16.0, 0.0 / 16.0),
//SAND
            glm::vec2(9.0 / 16.0, 1.0 / 16.0),
            glm::vec2(8.0 / 16.0, 0.0 / 16.0),
            glm::vec2(8.0 / 16.0, 1.0 / 16.0),
            glm::vec2(8.0 / 16.0, 0.0 / 16.0),
            glm::vec2(9.0 / 16.0, 1.0 / 16.0),
            glm::vec2(9.0 / 16.0, 0.0 / 16.0),
//Leaves
            glm::vec2(10.0 / 16.0, 1.0 / 16.0),
            glm::vec2(9.0 / 16.0, 0.0 / 16.0),
            glm::vec2(9.0 / 16.0, 1.0 / 16.0),
            glm::vec2(9.0 / 16.0, 0.0 / 16.0),
            glm::vec2(10.0 / 16.0, 1.0 / 16.0),
            glm::vec2(10.0 / 16.0, 0.0 / 16.0),


            glm::vec2(0.0, 1.0),
            glm::vec2(1.0, 0.0),
            glm::vec2(1.0, 1.0),   
            glm::vec2(1.0, 0.0),
            glm::vec2(0.0, 1.0),
            glm::vec2(0.0, 0.0)
            
        ];




        let brightness: Vec<f32> = vec![
            //Back
            0.6,
            //Front
            0.8,
            //Left
            0.65,
            //Right
            0.75,
            //Bottom
            0.45,
            //Top
            0.95
        ];

        return BlockModel{
            px_pos,
            nx_pos,
            py_pos,
            ny_pos,
            pz_pos,
            nz_pos,

            px_pos_water,
            nx_pos_water,
            py_pos_water,
            ny_pos_water,
            pz_pos_water,
            nz_pos_water,

            px_uv,
            nx_uv,
            py_uv,
            ny_uv,
            pz_uv,
            nz_uv,
            brightness
        }
    }
    
    pub fn get_px(&self, id: u8) -> &Vec<glm::Vec3>{
        if id == 3{
            &self.px_pos_water
        }else{
            &self.px_pos
        }
    }

    pub fn get_nx(&self, id: u8) -> &Vec<glm::Vec3>{
        if id == 3{
            &self.nx_pos_water
        }else{
            &self.nx_pos
        }
    }

    pub fn get_py(&self, id: u8) -> &Vec<glm::Vec3>{
        if id == 3{
            &self.py_pos_water
        }else{
            &self.py_pos
        }
    }

    pub fn get_ny(&self, id: u8) -> &Vec<glm::Vec3>{
        if id == 3{
            &self.ny_pos_water
        }else{
            &self.ny_pos
        }
    }

    pub fn get_pz(&self, id: u8) -> &Vec<glm::Vec3>{
        if id == 3{
            &self.pz_pos_water
        }else{
            &self.pz_pos
        }
    }

    pub fn get_nz(&self, id: u8) -> &Vec<glm::Vec3>{
        if id == 3{
            &self.nz_pos_water
        }else{
            &self.nz_pos
        }
    }


    //UV FOR EACH BLOCK

    pub fn get_px_uv(&self) -> &Vec<glm::Vec2>{
        &self.px_uv
    }

    pub fn get_nx_uv(&self) -> &Vec<glm::Vec2>{
        &self.nx_uv
    }

    pub fn get_py_uv(&self) -> &Vec<glm::Vec2>{
        &self.py_uv
    }

    pub fn get_ny_uv(&self) -> &Vec<glm::Vec2>{
        &self.ny_uv
    }

    pub fn get_pz_uv(&self) -> &Vec<glm::Vec2>{
        &self.pz_uv
    }

    pub fn get_nz_uv(&self) -> &Vec<glm::Vec2>{
        &self.nz_uv
    }

    pub fn get_brightness(&self) -> &Vec<f32>{
        &self.brightness
    }
    
}
