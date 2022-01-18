extern crate gl;
// extern crate sdl2;
extern crate glm;
extern crate stopwatch;
extern crate noise;
use winit::{dpi::LogicalSize, event::{DeviceEvent, Event, KeyboardInput, MouseScrollDelta, VirtualKeyCode, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};
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
    const CHUNKS_LAYERS_FROM_PLAYER: usize = 7;    //Odd numbers ONLYYY
    const PLAYER_HEIGHT: f32 = 1.5;

    const WORLD_GEN_SEED: u32 = 60;                 //Any number
    const MID_HEIGHT: u8 = 50;                      //The terrain variation part
    const SKY_HEIGHT: u8 = 0;                       //Works as a buffer for the mid heigt needs to be at least 20 percent of mid size
    const UNDERGROUND_HEIGHT: u8 = 0;

    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new();
    // .with_inner_size(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
    // .with_title("MinecraftRS");
    window_builder.build(&event_loop).unwrap();

    loop {
        event_loop.run(move |event, _, control_flow| {
            println!("Hello there");
            match event {
                Event::WindowEvent { 
                    window_id, 
                    event: WindowEvent::KeyboardInput { device_id, input, is_synthetic }
                } =>{
                    match input.state{
                        Pressed => {
                            match input.virtual_keycode{
                                Some(VirtualKeyCode::Escape) =>{
                                    close_game = true;
                                },
                                Some(VirtualKeyCode::Q) =>{
                                    unsafe {
                                        if !self.mesh {
                                            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                                            window_builder.mesh = true;
                                        } else{
                                            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                                            self.mesh = false;
                                        }
                                    }
                                },
                                Some(VirtualKeyCode::E) =>{
                                    if !self.flying {
                                        self.flying = true;
                                    } else{
                                        self.flying = false;
                                        self.keyboard_ctrl = true;
                                    }
                                },
                                Some(VirtualKeyCode::Space) =>{
                                    if !self.flying{
                                        if self.in_liquid{
                                            self.keyboard_space = true;
                                            self.touched_ground = false;
                                            self.keyboard_ctrl = false;
                                        }
                                        
                                        if self.touched_ground && !self.in_liquid{
                                            self.keyboard_space = true;
                                            self.keyboard_ctrl = false;
                                            self.touched_ground = false;
                                            self.acceleration_result = 1.5
                                        }
                                    }else{
                                        self.keyboard_space = true;
                                    }
                                },
                                Some(VirtualKeyCode::LControl) =>{
                                    if self.flying{
                                        self.keyboard_ctrl = true;
                                    }
                                },
                                Some(VirtualKeyCode::W) =>{
                                    self.keyboard_w = true;
                                },
                                Some(VirtualKeyCode::A) =>{
                                    self.keyboard_s = true;
                                },
                                Some(VirtualKeyCode::S) =>{
                                    self.keyboard_a = true;
                                },
                                Some(VirtualKeyCode::Down) =>{
                                    self.keyboard_d = true;
                                },
                                Some(VirtualKeyCode::F) =>{
                                    world::World::destroy_block(world, &self.camera_front, &self.camera_pos);
                                },
                                Some(VirtualKeyCode::Key1) =>{
                                    self.selected_block = 0;
                                },
                                Some(VirtualKeyCode::Key2) =>{
                                    self.selected_block = 1;
                                },
                                Some(VirtualKeyCode::Key3) =>{
                                    self.selected_block = 2;
                                },
                                Some(VirtualKeyCode::Key4) =>{
                                    self.selected_block = 3;
                                },
                                Some(VirtualKeyCode::Key5) =>{
                                    self.selected_block = 4;
                                },
                                Some(VirtualKeyCode::Key6) =>{
                                    self.selected_block = 5;
                                },
                                Some(VirtualKeyCode::Key7) =>{
                                    self.selected_block = 6;
                                },
                                _ =>{
                                    println!("Hello there")
                                }
                            }
                        },
                        Released => {
                            match input.virtual_keycode {
                                Some(VirtualKeyCode::W) =>{
                                    self.keyboard_w = false;
                                },
                                Some(VirtualKeyCode::S) =>{
                                    self.keyboard_s = false;
                                },
                                Some(VirtualKeyCode::A) =>{
                                    self.keyboard_a = false;
                                },
                                Some(VirtualKeyCode::D) =>{
                                    self.keyboard_d = false;
                                },
                                Some(VirtualKeyCode::Space) =>{
                                    if self.in_liquid && !self.flying{
                                        // Space up
                                        self.keyboard_space = false;
                                        self.keyboard_ctrl = true;
                                    }else if self.flying{
                                        self.keyboard_space = false;
                                        self.keyboard_ctrl = false;
                                    }
                                },
                                Some(VirtualKeyCode::LControl) =>{
                                    if self.flying{
                                        self.keyboard_ctrl = false;
                                    }
                                },
                                _ =>{
                                    println!("Hello there")
                                }
                            }
                        }
                    }
                    match input.virtual_keycode{
                        Some(VirtualKeyCode::W) =>{
                            self.keyboard_w = false;
                        },
                        Some(VirtualKeyCode::S) =>{
                            self.keyboard_s = false;
                        },
                        Some(VirtualKeyCode::A) =>{
                            self.keyboard_a = false;
                        },
                        Some(VirtualKeyCode::D) =>{
                            self.keyboard_d = false;
                        },
                        Some(VirtualKeyCode::Space) =>{
                            if self.in_liquid && !self.flying{
                                // Space up
                                self.keyboard_space = false;
                                self.keyboard_ctrl = true;
                            }else if self.flying{
                                self.keyboard_space = false;
                                self.keyboard_ctrl = false;
                            }
                        },
                        Some(VirtualKeyCode::LControl) =>{
                            if self.flying{
                                self.keyboard_ctrl = false;
                            }
                        },
                        Some(VirtualKeyCode::A) =>{

                        },
                        Some(VirtualKeyCode::S) =>{

                        },
                        Some(VirtualKeyCode::Down) =>{

                        },
                        Some(VirtualKeyCode::F) =>{

                        },
                        Some(VirtualKeyCode::Key1) =>{

                        },
                        Some(VirtualKeyCode::Key2) =>{

                        },
                        Some(VirtualKeyCode::Key3) =>{

                        },
                        Some(VirtualKeyCode::Key4) =>{

                        },
                        Some(VirtualKeyCode::Key5) =>{

                        },
                        Some(VirtualKeyCode::Key6) =>{

                        },
                        Some(VirtualKeyCode::Key7) =>{

                        },
                        _ =>{
                            println!("Hello there")
                        }
                    }
                },
                Event::DeviceEvent { 
                    device_id, 
                    event  
                } =>{
                    match event{
                        DeviceEvent::MouseMotion { delta } => {
                            let x = delta.0;
                            let y = delta.1;

                            if self.first_mouse{
                                self.last_x = x as f32;
                                self.last_y = y as f32;
                                self.first_mouse = false;
                            }
        
                            let mut xoffset = x as f32 - self.last_x;
                            let mut yoffset = self.last_y - y as f32; // reversed since y-coordinates go from bottom to top
                            self.last_x = x as f32;
                            self.last_y = y as f32;
        
                            let sensitivity = 0.1; // change this value to your liking
                            xoffset *= sensitivity;
                            yoffset *= sensitivity;
        
                            self.yaw += xoffset;
                            self.pitch += yoffset;
        
                            //make sure that when pitch is out of bounds, screen doesn't get flipped
                            if self.pitch > 95.0 {
                                self.pitch = 95.0;
                            }
                            if self.pitch < -89.0 {
                                self.pitch = -89.0;
                            }
        
                            let mut front = glm::vec3(0.0, 0.0, 0.0);
                            front.x = glm::cos(glm::radians(self.yaw)) * glm::cos(glm::radians(self.pitch));
                            front.y = glm::sin(glm::radians(self.pitch));
                            front.z = glm::sin(glm::radians(self.yaw)) * glm::cos(glm::radians(self.pitch));
                            self.camera_front = glm::normalize(front);
                        },
                        DeviceEvent::MouseWheel { delta } => {
                            match delta {
                                winit::event::MouseScrollDelta::LineDelta(x, y) => {
                                    if self.fov >= 1.0 && self.fov <= 90.0 {
                                        self.fov -= y as f32;
                                    }  
                                    if  self.fov < 1.0 {
                                        self.fov = 1.0;
                                    }   
                                    if  self.fov > 90.0 {
                                        self.fov = 90.0;
                                    }       
                                }
                                // winit::event::MouseScrollDelta::PixelDelta(p) => {
                                //     println!("mouse wheel Pixel Delta: ({},{})", p.x, p.y);
                                //     let mut pos = window.outer_position().unwrap();
                                //     pos.x -= p.x as i32;
                                //     pos.y -= p.y as i32;
                                //     window.set_outer_position(pos)
                                // }
                                _ => {},
                            }
                        },
                        DeviceEvent::Button { button, state } => {
                            match state{
                                Pressed => {
                                    if !self.mouse_button_clicked {
                                        if mouse_btn.eq(&sdl2::mouse::MouseButton::Left){
                                            world::World::destroy_block(world, &self.camera_front, &self.camera_pos);
                                        } else {
                                            world::World::place_block(world, &self.camera_front, &self.camera_pos, self.selected_block, self.player_height);
                                        }
                                        self.mouse_button_clicked = true;
                                    }
                                },
                                Released => {
                                    self.mouse_button_clicked = false;
                                }
                            }
                        },
                        _ =>{}
                    }
                },
                _ =>{}
            }
        });
        // window_builder.request_redraw();
    }
    
    // let sdl = sdl2::init().unwrap();
    // let video_subsystem = sdl.video().unwrap();
    // let gl_attr = video_subsystem.gl_attr();
    // gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    // gl_attr.set_context_version(4, 1);
    // let window = video_subsystem
    //     .window("MinecraftRS", WINDOW_WIDTH, WINDOW_HEIGHT)
    //     .opengl()
    //     .resizable()
    //     .build()
    //     .unwrap();
    // let _gl_context = window.gl_create_context().unwrap();
    // let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // //Set mouse to be bound in the window and infinite movement
    // sdl.mouse().capture(true);
    // sdl.mouse().set_relative_mouse_mode(true);

    // // set up block shader
    // let vert_shader = render_gl::Shader::from_vert_source(&CString::new(include_str!("shaders/block.vert")).unwrap()).unwrap();
    // let frag_shader = render_gl::Shader::from_frag_source(&CString::new(include_str!("shaders/block.frag")).unwrap()).unwrap();
    // let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    // // setup skybox shader
    // let vert_shader = render_gl::Shader::from_vert_source(&CString::new(include_str!("shaders/skybox.vert")).unwrap()).unwrap();
    // let frag_shader = render_gl::Shader::from_frag_source(&CString::new(include_str!("shaders/skybox.frag")).unwrap()).unwrap();
    // let skybox_shader = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    // unsafe {
    //     gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);
    //     gl::ClearColor(0.67, 0.79, 1.0, 1.0); // Divide 120 by 255 and you get the color you want. Replace 120 with what you have in rgb.
    //     gl::Enable(gl::DEPTH_TEST);
    //     gl::Enable(gl::CULL_FACE);
    //     gl::Enable(gl::BLEND);
    //     gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    // }

    // let mut time_increment: f32 = 0.0;
    // let camera_pos = glm::vec3(0.0, 0.0, 0.0);
    // let mut world = world::World::new(
    //     &camera_pos, 
    //     &shader_program, 
    //     &SQUARE_CHUNK_WIDTH, 
    //     &CHUNKS_LAYERS_FROM_PLAYER, 
    //     &WORLD_GEN_SEED,
    //     &MID_HEIGHT,
    //     &UNDERGROUND_HEIGHT,
    //     &SKY_HEIGHT,
    // );
    
    // let mut player: player::Player = player::Player::new(&mut world, PLAYER_HEIGHT, camera_pos);

    
    // let skybox: skybox::Skybox = skybox::Skybox::new(skybox_shader.clone());

    // const TIME_BETWEEN_FRAMES: u64 = 20;
    // // let mut event_pump = sdl.event_pump().unwrap();
    // let mut stopwatch = stopwatch::Stopwatch::new();

    
    // 'main: loop {
        
    //     stopwatch.reset();
    //     stopwatch.start();
        // *control_flow = ControlFlow::Poll;
        // event_loop.run(move |event, _, control_flow| {
        //     let mut number = 32;
        // });
        // let close_game: bool = player.handle_events(&mut world, &event_loop);
    //     if close_game {
    //         break 'main
    //     }
    //     //Rendering
    //     unsafe {
    //         gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); 
            
    //         let current_frame = time_increment as f32;
    //         player.delta_time = current_frame - player.last_frame;
    //         player.last_frame = current_frame;
            
    //         shader_program.set_used();
    //         let projection = glm::ext::perspective(glm::radians(player.fov), (WINDOW_WIDTH as f32)/(WINDOW_HEIGHT as f32), 0.1, 5000.0);
    //         let projection_loc = gl::GetUniformLocation(shader_program.id(), "projection".as_ptr() as *const std::os::raw::c_char);
    //         gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, &projection[0][0]);
            
    //         let view = glm::ext::look_at(player.camera_pos, player.camera_pos + player.camera_front, player.camera_up);
    //         let view_loc = gl::GetUniformLocation(shader_program.id(), "view".as_ptr() as *const std::os::raw::c_char);
    //         gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);
            
    //         world::World::draw(&mut world, &player.camera_pos, projection, view);




    //         skybox_shader.set_used();

    //         let mut view = glm::ext::look_at(camera_pos, camera_pos + player.camera_front, player.camera_up);
    //         view[3][0] = 0.0;
    //         view[3][1] = 0.0;
    //         view[3][2] = 0.0;
    //         view[3][3] = 0.0;

    //         view[0][3] = 0.0;
    //         view[1][3] = 0.0;
    //         view[2][3] = 0.0;

    //         let view_loc = gl::GetUniformLocation(skybox_shader.id(), "view".as_ptr() as *const std::os::raw::c_char);

    //         gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);

    //         let projection = glm::ext::perspective(glm::radians(player.fov), (WINDOW_WIDTH as f32)/(WINDOW_HEIGHT as f32), 0.1, 5000.0);
            
    //         let projection_loc = gl::GetUniformLocation(skybox_shader.id(), "projection".as_ptr() as *const std::os::raw::c_char);
    //         gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, &projection[0][0]);

    //         skybox::Skybox::draw(&skybox);
    //         gl::BindVertexArray(0);

    //     }
    //     time_increment += 0.02;
    //     window.gl_swap_window();
        
    //     loop{
    //         if (stopwatch.elapsed_ms() as u64) < TIME_BETWEEN_FRAMES {
    //             world.render_loop();
    //         }else{
    //             break;
    //         }
    //     }
    // }
}