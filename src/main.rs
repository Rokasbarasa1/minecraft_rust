extern crate gl;
extern crate sdl2;
extern crate glm;
extern crate stopwatch;
extern crate noise;
pub mod render_gl;
pub mod world;
use std::ffi::CString;
//use std::io::{stdout, Write};


//$Env:RUST_BACKTRACE=1
fn main() {
    //Settings
    //Current amount of textures
    const SQUARE_CHUNK_WIDTH: u32 = 16;//16;
    const CHUNKS_LAYERS_FROM_PLAYER: u32 = 7; //Odd numbers
    const WINDOW_WIDTH: u32 = 1920;
    const WINDOW_HEIGHT: u32 = 1080;
    const VIEW_DISTANCE: f32 = 200.0;
    const WORLD_GEN_SEED: u32 = 60;
    const MAX_HEIGHT: usize = 20;

    //Some booleans that in game keys control
    let mut mesh = false;
    let mut mouse_button_clicked = false;
    let mut keyboard_w = false;
    let mut keyboard_a = false;
    let mut keyboard_s = false;
    let mut keyboard_d = false;
    let mut keyboard_space = false;
    let mut keyboard_ctrl = false;
    let mut selected_block: usize = 4;

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

    //Camera
    let mut camera_pos = glm::vec3(0.0, 0.0, 0.0);
    let mut camera_front = glm::vec3(0.0, 0.0, -1.0);
    let camera_up = glm::vec3(0.0, 1.0, 0.0);

    let mut yaw = -90.0;
    let mut pitch = 0.0;
    let mut fov = 85.0;

    //Mouse state
    let mut first_mouse = true;
    let mut last_x = 800.0 / 2.0;
    let mut last_y = 600.0 / 2.0;

    //Timing
    let mut delta_time = 0.0;
    let mut last_frame = 0.0;

    //Set mouse to be bound in the window and infinite movement
    sdl.mouse().capture(true);
    sdl.mouse().set_relative_mouse_mode(true);

    // set up shader program
    let vert_shader = render_gl::Shader::from_vert_source(&CString::new(include_str!("shaders/triangle.vert")).unwrap()).unwrap();

    let frag_shader = render_gl::Shader::from_frag_source(&CString::new(include_str!("shaders/triangle.frag")).unwrap()).unwrap();

    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();    

    unsafe {
        gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);
        gl::ClearColor(0.49, 0.87, 0.96, 1.0); // Divide smth like 120 by 255 and you get the color you want. Replace 120 with what you have in rgb
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    
    let mut time_increment: f32 = 0.0;

    let mut world: world::World = world::World::new(
        &camera_pos, 
        &SQUARE_CHUNK_WIDTH, 
        &shader_program, 
        &CHUNKS_LAYERS_FROM_PLAYER, 
        &VIEW_DISTANCE, 
        &WORLD_GEN_SEED,
        &MAX_HEIGHT
    );

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main, 
                sdl2::event::Event::KeyDown { timestamp: _, window_id: _, keycode: _, scancode, keymod: _, repeat: _ } => {
                    //Change to polygon mesh mode
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Q {
                        unsafe {
                            if !mesh {
                                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                                mesh = true;
                            } else{
                                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                                mesh = false;
                            }
                        }
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Escape {
                        break 'main;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Space {
                        keyboard_space = true;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::LCtrl {
                        keyboard_ctrl = true;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::W {
                        keyboard_w = true;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::S {
                        keyboard_s = true;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::A {
                        keyboard_a = true;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::D {
                        keyboard_d = true;
                    }

                    if scancode.unwrap() == sdl2::keyboard::Scancode::F {
                        world::World::destroy_block(&mut world, &camera_front, &camera_pos);
                    }
                    
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Num1 {
                        selected_block = 0;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Num2 {
                        selected_block = 1;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Num3 {
                        selected_block = 2;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Num4 {
                        selected_block = 3;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Num5 {
                        selected_block = 4;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Num6 {
                        selected_block = 5;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Num7 {
                        selected_block = 6;
                    }
                    // if scancode.unwrap() == sdl2::keyboard::Scancode::Num8 {
                    //     selected_block = 7;
                    // }
                    // if scancode.unwrap() == sdl2::keyboard::Scancode::Num9 {
                    //     selected_block = 8;
                    // }
                    // if scancode.unwrap() == sdl2::keyboard::Scancode::Num0 {
                    //     selected_block = 9;
                    // }

                    
                },
                sdl2::event::Event::KeyUp { timestamp: _, window_id: _, keycode: _, scancode, keymod: _, repeat: _ } => {
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Space {
                        keyboard_space = false;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::LCtrl {
                        keyboard_ctrl = false;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::W {
                        keyboard_w = false;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::S {
                        keyboard_s = false;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::A {
                        keyboard_a = false;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::D {
                        keyboard_d = false;
                    }
                }
                
                sdl2::event::Event::MouseMotion { timestamp: _, window_id: _, which: _, mousestate: _, x, y, xrel: _, yrel: _ } => {
                    if first_mouse
                    {
                        last_x = x as f32;
                        last_y = y as f32;
                        first_mouse = false;
                    }

                    let mut xoffset = x as f32 - last_x;
                    let mut yoffset = last_y - y as f32; // reversed since y-coordinates go from bottom to top
                    last_x = x as f32;
                    last_y = y as f32;

                    let sensitivity = 0.1; // change this value to your liking
                    xoffset *= sensitivity;
                    yoffset *= sensitivity;

                    yaw += xoffset;
                    pitch += yoffset;

                    //make sure that when pitch is out of bounds, screen doesn't get flipped
                    if pitch > 89.0 {
                        pitch = 89.0;
                    }
                    if pitch < -89.0 {
                        pitch = -89.0;
                    }

                    let mut front = glm::vec3(0.0, 0.0, 0.0);
                    front.x = glm::cos(glm::radians(yaw)) * glm::cos(glm::radians(pitch));
                    front.y = glm::sin(glm::radians(pitch));
                    front.z = glm::sin(glm::radians(yaw)) * glm::cos(glm::radians(pitch));
                    camera_front = glm::normalize(front);
                },
                sdl2::event::Event::MouseWheel { timestamp: _, window_id: _, which: _, x: _, y, direction: _ } => {
                    if fov >= 1.0 && fov <= 90.0 {
                        fov -= y as f32;
                    }  
                    if  fov < 1.0 {
                        fov = 1.0;
                    }   
                    if  fov > 90.0 {
                        fov = 90.0;
                    }
                },
                sdl2::event::Event::MouseButtonDown { timestamp: _, window_id: _, which: _, mouse_btn, clicks: _, x: _, y: _ } =>{
                    if !mouse_button_clicked {
                        if mouse_btn.eq(&sdl2::mouse::MouseButton::Left){
                            world::World::destroy_block(&mut world, &camera_front, &camera_pos);
                            println!("Clicked left button ")
                        } else {
                            world::World::place_block(&mut world, &camera_front, &camera_pos, selected_block);
                            println!("Clicked right button ")
                        }
                        mouse_button_clicked = true;
                    }
                },
                sdl2::event::Event::MouseButtonUp { timestamp: _, window_id: _, which: _, mouse_btn: _, clicks: _, x: _, y: _ } =>{
                    mouse_button_clicked = false;
                },
                _ => {}
            }
        }     
        //Key control
        {
            if keyboard_w {
                let camera_speed = 7.0 * delta_time;
                camera_pos = camera_pos + glm::vec3(camera_speed * camera_front.x, 0.0, camera_speed * camera_front.z);
            }

            if keyboard_a {
                let camera_speed = 7.0 * delta_time;
                camera_pos = camera_pos - glm::normalize(glm::cross(camera_front, camera_up)) * camera_speed;
            }

            if keyboard_s {
                let camera_speed = 7.0 * delta_time;
                camera_pos = camera_pos - glm::vec3(camera_speed * camera_front.x, 0.0, camera_speed * camera_front.z);
            }

            if keyboard_d {
                let camera_speed = 7.0 * delta_time;
                camera_pos = camera_pos + glm::normalize(glm::cross(camera_front, camera_up)) * camera_speed;
            }

            if keyboard_space {
                let camera_speed = 7.0 * delta_time;
                camera_pos = camera_pos + glm::vec3(0.0, camera_speed, 0.0);
            }

            if keyboard_ctrl {
                let camera_speed = 7.0 * delta_time;
                camera_pos = camera_pos - glm::vec3(0.0, camera_speed, 0.0);
            }
        }
        
        //Rendering
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); 

            let current_frame = time_increment as f32;
            delta_time = current_frame - last_frame;
            last_frame = current_frame;

            let projection = glm::ext::perspective(glm::radians(fov), (WINDOW_WIDTH as f32)/(WINDOW_HEIGHT as f32), 0.1, 100.0);
            let projection_loc = gl::GetUniformLocation(shader_program.id(), "projection".as_ptr() as *const std::os::raw::c_char);
            gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, &projection[0][0]);

            let view = glm::ext::look_at(camera_pos, camera_pos + camera_front, camera_up);
            let view_loc = gl::GetUniformLocation(shader_program.id(), "view".as_ptr() as *const std::os::raw::c_char);
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);

            world::World::draw(&mut world,&camera_pos);
            gl::BindVertexArray(0);

        }
        time_increment += 0.04;
        window.gl_swap_window();
        {
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
        
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

/*
TODO:
Make efficient block rendering without lag.
Make terrain look good
Make block desturction and placement possible.
Make water
Make lighting




*/