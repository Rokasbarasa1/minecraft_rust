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

//$Env:RUST_BACKTRACE=1
fn main() {
    //Settings
    const WINDOW_WIDTH: u32 = 1280;
    const WINDOW_HEIGHT: u32 = 720;
    
    const SQUARE_CHUNK_WIDTH: usize = 10;           //Values can be: 4,6,10,16,22,28
    const CHUNKS_LAYERS_FROM_PLAYER: usize = 11;    //Odd numbers ONLYYY
    const PLAYER_HEIGHT: f32 = 1.5;

    const WORLD_GEN_SEED: u32 = 60;                 //Any number
    const MID_HEIGHT: u8 = 50;                   //The terrain variation part
    const SKY_HEIGHT: u8 = 0;                   //Works as a buffer for the mid heigt needs to be at least 20 percent of mid size
    const UNDERGROUND_HEIGHT: u8 = 0;            


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
    let vert_shader = render_gl::Shader::from_vert_source(&CString::new(include_str!("shaders/skybox.vert")).unwrap()).unwrap();
    let frag_shader = render_gl::Shader::from_frag_source(&CString::new(include_str!("shaders/skybox.frag")).unwrap()).unwrap();
    let skybox_shader = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    unsafe {
        gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);
        gl::ClearColor(0.67, 0.79, 1.0, 1.0); // Divide 120 by 255 and you get the color you want. Replace 120 with what you have in rgb.
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    let mut time_increment: f32 = 0.0;
    let camera_pos = glm::vec3(0.0, 0.0, 0.0);
    let mut world = world::World::new(
        &camera_pos, 
        &shader_program, 
        &SQUARE_CHUNK_WIDTH, 
        &CHUNKS_LAYERS_FROM_PLAYER, 
        &WORLD_GEN_SEED,
        &MID_HEIGHT,
        &UNDERGROUND_HEIGHT,
        &SKY_HEIGHT,
    );
    
    let mut player: player::Player = player::Player::new(&mut world, PLAYER_HEIGHT, camera_pos);

    
    let skybox: skybox::Skybox = skybox::Skybox::new(skybox_shader.clone());

    const TIME_BETWEEN_FRAMES: u64 = 20;
    let mut event_pump = sdl.event_pump().unwrap();
    let mut stopwatch = stopwatch::Stopwatch::new();

    'main: loop {
        stopwatch.reset();
        stopwatch.start();

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
            
            world::World::draw(&mut world, &player.camera_pos, projection, view);




            skybox_shader.set_used();

            let mut view = glm::ext::look_at(camera_pos, camera_pos + player.camera_front, player.camera_up);
            view[3][0] = 0.0;
            view[3][1] = 0.0;
            view[3][2] = 0.0;
            view[3][3] = 0.0;

            view[0][3] = 0.0;
            view[1][3] = 0.0;
            view[2][3] = 0.0;

            let view_loc = gl::GetUniformLocation(skybox_shader.id(), "view".as_ptr() as *const std::os::raw::c_char);

            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);

            let projection = glm::ext::perspective(glm::radians(player.fov), (WINDOW_WIDTH as f32)/(WINDOW_HEIGHT as f32), 0.1, 5000.0);
            
            let projection_loc = gl::GetUniformLocation(skybox_shader.id(), "projection".as_ptr() as *const std::os::raw::c_char);
            gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, &projection[0][0]);

            skybox::Skybox::draw(&skybox);
            gl::BindVertexArray(0);

        }
        time_increment += 0.02;
        window.gl_swap_window();
        
        loop{
            if (stopwatch.elapsed_ms() as u64) < TIME_BETWEEN_FRAMES {
                world.render_loop(&camera_pos);
            }else{
                break;
            }
        }
    }
}