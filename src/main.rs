extern crate gl;
extern crate sdl2;
extern crate glm;
extern crate stopwatch;
extern crate noise;
pub mod render_gl;
pub mod world;
pub mod player;
pub mod skybox; 
use std::ffi::CString;
//use std::io::{stdout, Write};


//$Env:RUST_BACKTRACE=1
fn main() {
    //Settings
    //Current amount of textures
    const SQUARE_CHUNK_WIDTH: usize = 10;//16;
    const CHUNKS_LAYERS_FROM_PLAYER: usize = 15; //Odd numbers
    const WINDOW_WIDTH: u32 = 1920;
    const WINDOW_HEIGHT: u32 = 1080;
    const VIEW_DISTANCE: f32 = 200.0;
    const WORLD_GEN_SEED: u32 = 60;
    const MAX_HEIGHT: usize = 15;
    const PLAYER_HEIGHT: f32 = 1.5;
    // const PLAYER_MOVE_SPEED: f32 = 50.0; // Per second

    // let noise = Perlin::new();
    // noise.set_seed(WORLD_GEN_SEED);

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);
    let window = video_subsystem
        .window("MinecraftRS", WINDOW_WIDTH, WINDOW_HEIGHT)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    //Set mouse to be bound in the window and infinite movement
    sdl.mouse().capture(true);
    sdl.mouse().set_relative_mouse_mode(true);

    // set up block shader
    let vert_shader = render_gl::Shader::from_vert_source(&CString::new(include_str!("shaders/block.vert")).unwrap()).unwrap();
    let frag_shader = render_gl::Shader::from_frag_source(&CString::new(include_str!("shaders/block.frag")).unwrap()).unwrap();
    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();    

    // setup skybox shader
    // let vert_shader = render_gl::Shader::from_vert_source(&CString::new(include_str!("shaders/skybox.vert")).unwrap()).unwrap();
    // let frag_shader = render_gl::Shader::from_frag_source(&CString::new(include_str!("shaders/skybox.frag")).unwrap()).unwrap();
    // let skybox_shader = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();    
    unsafe {
        gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);
        gl::ClearColor(0.67, 0.79, 1.0, 1.0); // Divide smth like 120 by 255 and you get the color you want. Replace 120 with what you have in rgb
        //0.67, 0.79, 1.0, 1.0
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    
    let mut time_increment: f32 = 0.0;
    let mut camera_pos = glm::vec3(0.0, 0.0, 0.0);
    let mut world: world::World = world::World::new(
        &camera_pos, 
        &SQUARE_CHUNK_WIDTH, 
        &shader_program, 
        &CHUNKS_LAYERS_FROM_PLAYER, 
        &VIEW_DISTANCE, 
        &WORLD_GEN_SEED,
        &MAX_HEIGHT
    );
    let mut player: player::Player = player::Player::new(&mut world, PLAYER_HEIGHT, camera_pos.clone());

    
    //let skybox: skybox::Skybox = skybox::Skybox::new(skybox_shader.clone());

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {

        let close_game: bool = player.handle_events(&mut world, &mut event_pump);
        
        if close_game {
            break 'main
        }
        //Rendering
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); 
            

            let current_frame = time_increment as f32;
            player.delta_time = current_frame - player.last_frame;
            player.last_frame = current_frame;
            
            shader_program.set_used();
            let projection = glm::ext::perspective(glm::radians(player.fov), (WINDOW_WIDTH as f32)/(WINDOW_HEIGHT as f32), 0.1, 5000.0);
            let projection_loc = gl::GetUniformLocation(shader_program.id(), "projection".as_ptr() as *const std::os::raw::c_char);
            gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, &projection[0][0]);

            let view = glm::ext::look_at(player.camera_pos, player.camera_pos + player.camera_front, player.camera_up);
            let view_loc = gl::GetUniformLocation(shader_program.id(), "view".as_ptr() as *const std::os::raw::c_char);
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);

            world::World::draw(&mut world, &player.camera_pos);




            // skybox_shader.set_used();
            // let mut view = glm::ext::look_at(camera_pos, camera_pos + camera_front, camera_up);
            // view[3][0] = 0.0;
            // view[3][1] = 0.0;
            // view[3][2] = 0.0;
            // view[3][3] = 0.0;

            // view[0][3] = 0.0;
            // view[1][3] = 0.0;
            // view[2][3] = 0.0;

            // let view_loc = gl::GetUniformLocation(skybox_shader.id(), "view".as_ptr() as *const std::os::raw::c_char);

            // gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);

            // let projection = glm::ext::perspective(glm::radians(fov), (WINDOW_WIDTH as f32)/(WINDOW_HEIGHT as f32), 0.1, 5000.0);
            
            // let projection_loc = gl::GetUniformLocation(skybox_shader.id(), "projection".as_ptr() as *const std::os::raw::c_char);
            // gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, &projection[0][0]);

            // skybox::Skybox::draw(&skybox);
            gl::BindVertexArray(0);

        }
        time_increment += 0.02;
        window.gl_swap_window();
        {
            // println!("Position: X:{} Z:{}", camera_pos.x, camera_pos.z);
            // let x_axis = f32::abs(camera_front.x);
            // let y_axis = f32::abs(camera_front.y);
            // let z_axis = f32::abs(camera_front.z);
            // let x_sign = if camera_front.x > 0.0 {"+"} else {"-"};
            // let y_sign = if camera_front.y > 0.0 {"+"} else {"-"};
            // let z_sign = if camera_front.z > 0.0 {"+"} else {"-"};

            // if x_axis > y_axis && x_axis > z_axis {
            //     println!("Axis: {}X",x_sign);
            // }else if y_axis > x_axis && y_axis > z_axis {
            //     println!("Axis: {}Y",y_sign);
            // }else if z_axis > y_axis && z_axis > x_axis {
            //     println!("Axis: {}Z",z_sign);
            // }
        }
        
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}