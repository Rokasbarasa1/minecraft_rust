extern crate gl;
extern crate sdl2;
extern crate stb_image;
extern crate image;
extern crate serde_json;
extern crate glm;
extern crate nalgebra_glm;
pub mod render_gl;
mod world;

use std::{ffi::c_void};
use std::ffi::CString;

use image::{ImageBuffer, Rgb};

enum Block{
    Stone_block,
    Dirt_block,
    Grass_block
}


fn main() {
    //Settings
    //Current amount of textures
    const amount_textures: usize =  4;
    let square_chunk_width: u32 = 16;//16;
    let block_radius: f32 = 0.3; 
    let chunks_layers_from_player: u32 = 5;
    let window_width = 1500;
    let window_height = 1000;

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);
    
    let window = video_subsystem
        .window("RSMinecraft", window_width, window_height)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    
    
    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    //Camera
    let mut camera_pos = glm::vec3(0.0, 0.0, 3.0);
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
    let vert_shader = render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap()).unwrap();

    let frag_shader = render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap()).unwrap();

    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();    

    let vertices: Vec<f32> = vec![
        -0.15, -0.15, -0.15, 0.0, 0.0, 
         0.15, -0.15, -0.15, 1.0, 0.0, 
         0.15,  0.15, -0.15, 1.0, 1.0, 
         0.15,  0.15, -0.15, 1.0, 1.0, 
        -0.15,  0.15, -0.15, 0.0, 1.0, 
        -0.15, -0.15, -0.15, 0.0, 0.0, 

        -0.15, -0.15,  0.15, 0.0, 0.0, 
         0.15, -0.15,  0.15, 1.0, 0.0, 
         0.15,  0.15,  0.15, 1.0, 1.0, 
         0.15,  0.15,  0.15, 1.0, 1.0, 
        -0.15,  0.15,  0.15, 0.0, 1.0, 
        -0.15, -0.15,  0.15, 0.0, 0.0, 

        -0.15,  0.15,  0.15, 1.0, 0.0, 
        -0.15,  0.15, -0.15, 1.0, 1.0, 
        -0.15, -0.15, -0.15, 0.0, 1.0, 
        -0.15, -0.15, -0.15, 0.0, 1.0, 
        -0.15, -0.15,  0.15, 0.0, 0.0, 
        -0.15,  0.15,  0.15, 1.0, 0.0, 

         0.15,  0.15,  0.15, 1.0, 0.0, 
         0.15,  0.15, -0.15, 1.0, 1.0, 
         0.15, -0.15, -0.15, 0.0, 1.0, 
         0.15, -0.15, -0.15, 0.0, 1.0, 
         0.15, -0.15,  0.15, 0.0, 0.0, 
         0.15,  0.15,  0.15, 1.0, 0.0,

        -0.15, -0.15, -0.15, 0.0, 1.0, 
         0.15, -0.15, -0.15, 1.0, 1.0, 
         0.15, -0.15,  0.15, 1.0, 0.0, 
         0.15, -0.15,  0.15, 1.0, 0.0, 
        -0.15, -0.15,  0.15, 0.0, 0.0, 
        -0.15, -0.15, -0.15, 0.0, 1.0,

        -0.15,  0.15, -0.15, 0.0, 1.0, 
         0.15,  0.15, -0.15, 1.0, 1.0, 
         0.15,  0.15,  0.15, 1.0, 0.0, 
         0.15,  0.15,  0.15, 1.0, 0.0, 
        -0.15,  0.15,  0.15, 0.0, 0.0, 
        -0.15,  0.15, -0.15, 0.0, 1.0,

    ];

    //Y is height
    //X is sideways in the x axis
    //Z is sideways in the what would normaly be y axis
    
    let mut cube_positions: Vec<glm::Vector3<f32>> = vec!  [
        // glm::vec3( 2.0,  5.0, -15.0),
        // glm::vec3(-1.5, -2.2, -2.5),
        // glm::vec3(-3.8, -2.0, -12.3),
        // glm::vec3( 2.4, -0.4, -3.5),
        // glm::vec3(-1.7,  3.0, -7.5),
        // glm::vec3( 1.3, -2.0, -2.5),
        // glm::vec3( 1.5,  2.0, -2.5),
        // glm::vec3( 1.5,  0.2, -1.5),
        // glm::vec3(-1.3,  1.0, -1.5)
    ];

    for i in 0..square_chunk_width {
        for k in 0..square_chunk_width {
            for j in 0..1 {
                cube_positions.push(glm::vec3( -0.3 * i as f32,  -0.3 * j as f32,  -0.3 * k as f32));
            }
        }
    }
    // set up vertex array object
    let mut vao: gl::types::GLuint = 0;
    let mut vbo: gl::types::GLuint = 0;
    let mut ebo: gl::types::GLuint = 0;
    let mut loaded_textures: [gl::types::GLuint; amount_textures] = [0,0,0,0];
    unsafe {

        gl::GenBuffers(1, &mut vbo);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData( gl::ARRAY_BUFFER, (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, vertices.as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut ebo);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer( 0,3, gl::FLOAT, gl::FALSE, (5 * std::mem::size_of::<f32>()) as gl::types::GLint, std::ptr::null(),);

        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, (5 * std::mem::size_of::<f32>()) as gl::types::GLint, (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,);

        // let mut data = image::open(&std::path::Path::new("C:\\Users\\Rokas\\Desktop\\rust minecraft\\minecraft_rust\\TextureTemplate.png")).unwrap().into_rgb();
        // let number: i32 = 1;
        // for i in 0..amount_textures {
        //     let mut texture: gl::types::GLuint = 0;
        //     gl::GenTextures(number, &mut texture);
        //     gl::BindTexture(gl::TEXTURE_2D, texture);
        //     setup_texture(texture, i, &mut data);
        //     loaded_textures[i] = texture;
        // }

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    unsafe {
        gl::Viewport(0, 0, window_width as i32, window_height as i32);
        gl::ClearColor(0.49, 0.87, 0.96, 1.0); // Divide smth like 120 by 255 and you get the color you want. Replace 120 with what you have in rgb
        gl::Enable(gl::DEPTH_TEST);
    }

    let mut mesh = false;
    let mut time_increment: f32 = 0.0;
    
    let mut world: world::World = world::World::new(&amount_textures, &camera_pos, &square_chunk_width, &block_radius, &shader_program, &chunks_layers_from_player);
    println!("{}", world::World::look_inside(&world));
    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main, 
                
                sdl2::event::Event::KeyDown { timestamp: _, window_id: _, keycode: _, scancode, keymod: _, repeat: _ } => {
                    let camera_speed = 2.5 * delta_time;
                    if scancode.unwrap() == sdl2::keyboard::Scancode::W {
                        camera_pos = camera_pos + glm::vec3(camera_speed * camera_front.x, camera_speed * camera_front.y, camera_speed * camera_front.z);
                    }

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
                    if scancode.unwrap() == sdl2::keyboard::Scancode::A {
                        camera_pos = camera_pos - glm::normalize(glm::cross(camera_front, camera_up)) * camera_speed;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::S {
                        camera_pos = camera_pos - glm::vec3(camera_speed * camera_front.x, camera_speed * camera_front.y, camera_speed * camera_front.z);
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::D {
                        camera_pos = camera_pos + glm::normalize(glm::cross(camera_front, camera_up)) * camera_speed;
                    }
                },
                
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

                    // make sure that when pitch is out of bounds, screen doesn't get flipped
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
                _ => {}
            }
        }     
        
        unsafe {
            
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); 
            shader_program.set_used();

            let current_frame = time_increment as f32;
            delta_time = current_frame - last_frame;
            last_frame = current_frame;


            let projection = glm::ext::perspective(glm::radians(fov), (window_width as f32)/(window_height as f32), 0.1, 100.0);

            let projection_loc = gl::GetUniformLocation(shader_program.id(), "projection".as_ptr() as *const std::os::raw::c_char);
            
            gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, &projection[0][0]);


            let view = glm::ext::look_at(camera_pos, camera_pos + camera_front, camera_up);

            let view_loc = gl::GetUniformLocation(shader_program.id(), "view".as_ptr() as *const std::os::raw::c_char);
            
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);

            gl::BindVertexArray(vao);

            world::World::render(&mut world, &camera_pos, &vao);
            gl::BindVertexArray(0);

        }
        time_increment += 0.02;
        window.gl_swap_window();
        std::thread::sleep(std::time::Duration::from_millis(10));

    }
}