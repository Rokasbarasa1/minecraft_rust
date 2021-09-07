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
use std::sync::{Arc};
use std::thread;
use noise::{Blend, NoiseFn, RidgedMulti, Seedable, BasicMulti, Value, Fbm};
use parking_lot::{Mutex, MutexGuard};

use std::fs::File;
use std::io::{Write};

//$Env:RUST_BACKTRACE=1
fn main() {
    //Settings
    const WINDOW_WIDTH: u32 = 1280;
    const WINDOW_HEIGHT: u32 = 720;
    
    const SQUARE_CHUNK_WIDTH: usize = 16;           //16;
    const CHUNKS_LAYERS_FROM_PLAYER: usize = 15;    //Odd numbers ONLYYY
    const VIEW_DISTANCE: f32 = 200.0;               
    const PLAYER_HEIGHT: f32 = 1.5;

    const WORLD_GEN_SEED: u32 = 60;                 //Any number
    const MID_HEIGHT: u8 = 30;                   //The terrain variation part
    const SKY_HEIGHT: u8 = 0;                   //Works as a buffer for the mid heigt needs to be at least 20 percent of mid size
    const UNDERGROUND_HEIGHT: u8 = 0;            
    const NOISE_RESOLUTION: f32 = 0.019;            //Zoom in - more resolution. Higher - Zoom out


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
        gl::ClearColor(0.67, 0.79, 1.0, 1.0); // Divide 120 by 255 and you get the color you want. Replace 120 with what you have in rgb.
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    // let mut basic_multi = BasicMulti::default().set_seed(60);
    // basic_multi.frequency = 0.1;

    // let mut ridged = RidgedMulti::new().set_seed(60);
    // let mut fbm = Fbm::new().set_seed(60);
    // fbm.persistence = 1.0;
    // fbm.frequency = 0.01;
    // ridged.attenuation = 7.07;
    // ridged.persistence = 2.02;
    // ridged.octaves = 3;
    // ridged.frequency = 7.01 as f64;
    // basic_multi.frequency = 0.000004 as f64;
    // basic_multi.octaves = 3;
    
    // let blend = Blend::new(&fbm, &ridged, &basic_multi);
    // let mut float1:f64 = -1.0;
    // let mut float2:f64 = 1.0;
    // let mut w = File::create("C:/Users/Rokas/Desktop/rust_minecraft/minecraft_rust/test2.txt").unwrap();

    // for i in 0..1000{
    //     let value1: f64 = float2 as f64;
    //     let value2: f64 = float1 as f64;
    //     let value = blend.get([value1, value2]);
    //     writeln!(&mut w, "Value1: {} value2: {} blend: {}", value1, value2, value).unwrap();
    //     float1 = float1 - 1.0;
    //     float2 = float2 + 1.0;
    // }
    // println!("FINISHED");

    // let value1: f64 = ((z_pos - 30.0 + grid_z as f32)* 0.200) as f64;
    // let value2: f64 = ((x_pos - 30.0 + grid_x as f32)* 0.200) as f64;
    // let mut value = blend.get([value1, value2]);
    // if value > 1.0 || value < -1.0{
    //     println!("ValueNoise {} value1: {} value1: {}", value, value1, value2);
    // }



    let mut time_increment: f32 = 0.0;
    let camera_pos = glm::vec3(0.0, 0.0, 0.0);
    let mut world = Arc::new(Mutex::new(world::World::new(
        &camera_pos, 
        &shader_program, 
        &SQUARE_CHUNK_WIDTH, 
        &CHUNKS_LAYERS_FROM_PLAYER, 
        &VIEW_DISTANCE, 
        &WORLD_GEN_SEED,
        &MID_HEIGHT,
        &UNDERGROUND_HEIGHT,
        &SKY_HEIGHT,
        &NOISE_RESOLUTION,
    )));
    let world_player = Arc::clone(&world);

    let mut player: player::Player = player::Player::new(&mut world_player.lock(), PLAYER_HEIGHT, camera_pos);

    
    //let skybox: skybox::Skybox = skybox::Skybox::new(skybox_shader.clone());

    const TIME_BETWEEN_FRAMES: u64 = 20;
    let mut event_pump = sdl.event_pump().unwrap();
    let mut stopwatch = stopwatch::Stopwatch::new();

    let mut thread_keep_alive = true;
    let mut player_thread_waiting = false;
    
    thread::spawn(move || {
        let world_thread = Arc::clone(&world);
        loop{
            let mut guard = world_thread.lock();
            // println!("Got lock");
            guard.render_loop();
            MutexGuard::unlock_fair(guard);
        }
    });


    'main: loop {
        stopwatch.reset();
        stopwatch.start();

        // println!("Player waiting");
        let mut guard = world_player.lock();
        // println!("Player got lock");


        let close_game: bool = player.handle_events(&mut guard, &mut event_pump);
        if close_game {
            thread_keep_alive = false;
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

            world::World::draw(&mut guard, &player.camera_pos);
            MutexGuard::unlock_fair(guard);




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
            // println!("Position: X:{} Z:{}", player.camera_pos.x, player.camera_pos.z);

            // let x_axis = f32::abs(player.camera_front.x);
            // let y_axis = f32::abs(player.camera_front.y);
            // let z_axis = f32::abs(player.camera_front.z);
            // let x_sign = if player.camera_front.x > 0.0 {"+"} else {"-"};
            // let y_sign = if player.camera_front.y > 0.0 {"+"} else {"-"};
            // let z_sign = if player.camera_front.z > 0.0 {"+"} else {"-"};

            // if x_axis > y_axis && x_axis > z_axis {
            //     println!("Axis: {}X",x_sign);
            // }else if y_axis > x_axis && y_axis > z_axis {
            //     println!("Axis: {}Y",y_sign);
            // }else if z_axis > y_axis && z_axis > x_axis {
            //     println!("Axis: {}Z",z_sign);
            // }
        }

        if (stopwatch.elapsed_ms() as u64) < TIME_BETWEEN_FRAMES {
            std::thread::sleep(std::time::Duration::from_millis(TIME_BETWEEN_FRAMES - stopwatch.elapsed_ms() as u64));
        }
    }
}