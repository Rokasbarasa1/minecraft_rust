use crate::world;

extern crate glm;
use winit::event_loop::EventLoop;
use winit::event::{DeviceEvent, Event, MouseButton, VirtualKeyCode, WindowEvent};

pub struct Player {
    pub camera_pos: glm::Vector3<f32>,
    pub camera_front: glm::Vector3<f32>,
    pub camera_up: glm::Vector3<f32>,

    pub yaw: f32,
    pub pitch: f32,
    pub fov: f32,

    pub first_mouse: bool,
    pub last_x: f32,
    pub last_y: f32,

    pub delta_time: f32,
    pub last_frame: f32,

    pub player_height: f32,
    pub mesh: bool,
    pub flying: bool,

    pub mouse_button_clicked: bool,
    pub keyboard_w: bool,
    pub keyboard_a: bool,
    pub keyboard_s: bool,
    pub keyboard_d: bool,

    pub keyboard_space: bool,
    pub keyboard_space_frames: usize,
    pub touched_ground: bool,

    pub keyboard_ctrl: bool,
    pub selected_block: u8,

    pub acceleration_result: f32,
    pub acceleration: f32, // More like bonus speed after adding acceleration

    pub liquid_speed_modifyer: f32,
    pub in_liquid: bool,
    pub margin_for_player: f32,
    pub event_loop: EventLoop<()>

}

impl Player{
    pub fn new(world: &mut world::World, player_height: f32, camera_pos: glm::Vector3<f32>, event_loop: EventLoop<()>) -> Player {
        let mut player = Player{
            camera_pos: camera_pos,
            camera_front: glm::vec3(0.0, 0.0, -1.0),
            camera_up: glm::vec3(0.0, 1.0, 0.0),

            yaw: -90.0,
            pitch: 0.0,
            fov: 90.0,

            first_mouse: true,
            last_x: 800.0 / 2.0,
            last_y: 600.0 / 2.0,

            delta_time: 0.0,
            last_frame: 0.0,

            player_height: player_height,
            mesh: false,
            flying: false,
        
            mouse_button_clicked: false,
            keyboard_w: false,
            keyboard_a: false,
            keyboard_s: false,
            keyboard_d: false,
        
            keyboard_space: false,
            keyboard_space_frames: 0,
            touched_ground: false,
        
            keyboard_ctrl: true,
            selected_block: 4,

            acceleration_result: 0.0,
            acceleration: 0.15,
            liquid_speed_modifyer: 1.0,
            in_liquid: false,
            margin_for_player: 0.25,
            event_loop: event_loop

        };

        player.camera_pos = world::World::get_spawn_location(&world, &player.camera_pos, 0 as usize);

        return player;
    }

    pub fn handle_events(&mut self, world: &mut world::World) -> bool{
        let mut close_game = false;
        let mut mesh = self.mesh;
        self.event_loop.run(move |event, _, control_flow| {
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
                                        if !mesh {
                                            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                                            mesh = true;
                                        } else{
                                            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                                            mesh = false;
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
                                    // world::World::destroy_block(world, &self.camera_front, &self.camera_pos);
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
                        _ =>{}
                    }
                },
                Event::WindowEvent { 
                    window_id, 
                    event: WindowEvent::MouseInput { 
                        device_id, 
                        state, 
                        button, 
                        modifiers }
                } =>{
                    match state{
                        Pressed => {
                            if !self.mouse_button_clicked {
                                if button == winit::event::MouseButton::Left{
                                    world::World::destroy_block(world, &self.camera_front, &self.camera_pos);
                                } else if button == winit::event::MouseButton::Right{
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
        });

        self.mesh = mesh;
        
        if self.in_liquid{
            self.liquid_speed_modifyer = 0.45;
        }else{
            self.liquid_speed_modifyer = 1.0;
        }

        if !self.flying{
            if self.keyboard_w {
                let camera_speed = 7.0 * self.delta_time * self.liquid_speed_modifyer;
                let desired_position = self.camera_pos + glm::vec3(camera_speed * self.camera_front.x, 0.0, camera_speed * self.camera_front.z);
                let move_location = world::World::move_to_direction(&world, &desired_position, self.player_height, self.margin_for_player);
                if move_location == 0 || move_location == 1 {
                    self.camera_pos = desired_position;
                    if move_location == 1{
                        self.in_liquid = true;
                    }else{
                        if self.in_liquid {
                            self.touched_ground = false;
                        }
                        self.in_liquid = false;
                    }
                }
            }

            if self.keyboard_a {
                let camera_speed = 7.0 * self.delta_time * self.liquid_speed_modifyer;
                let desired_position = self.camera_pos - glm::normalize(glm::cross(self.camera_front, self.camera_up)) * camera_speed;
                
                let move_location = world::World::move_to_direction(&world, &desired_position, self.player_height, self.margin_for_player);
                if move_location == 0 || move_location == 1 {
                    self.camera_pos = desired_position;
                    if move_location == 1{
                        self.in_liquid = true;
                    }else{
                        if self.in_liquid {
                            self.touched_ground = false;
                        }
                        self.in_liquid = false;
                    }
                }
            }

            if self.keyboard_s {
                let camera_speed = 7.0 * self.delta_time * self.liquid_speed_modifyer;
                let desired_position = self.camera_pos - glm::vec3(camera_speed * self.camera_front.x, 0.0, camera_speed * self.camera_front.z);
                
                let move_location = world::World::move_to_direction(&world, &desired_position, self.player_height, self.margin_for_player);
                if move_location == 0 || move_location == 1 {
                    self.camera_pos = desired_position;
                    if move_location == 1{
                        self.in_liquid = true;
                    }else{
                        if self.in_liquid {
                            self.touched_ground = false;
                        }
                        self.in_liquid = false;
                    }
                }
            }

            if self.keyboard_d {
                let camera_speed = 7.0 * self.delta_time * self.liquid_speed_modifyer;
                let desired_position = self.camera_pos + glm::normalize(glm::cross(self.camera_front, self.camera_up)) * camera_speed;
                
                let move_location = world::World::move_to_direction(&world, &desired_position, self.player_height, self.margin_for_player);
                if move_location == 0 || move_location == 1 {
                    self.camera_pos = desired_position;
                    if move_location == 1{
                        self.in_liquid = true;
                    }else{
                        if self.in_liquid {
                            self.touched_ground = false;
                        }
                        self.in_liquid = false;
                    }
                }
            }

            if self.keyboard_space {
                
                if !self.in_liquid{
                    self.keyboard_ctrl = false;
                    if self.keyboard_space_frames < 10 {
                        let camera_speed = 7.0 * self.delta_time * self.liquid_speed_modifyer;
                        let desired_position = self.camera_pos + glm::vec3(0.0, camera_speed * self.acceleration_result, 0.0);
                        
                        let move_location = world::World::move_to_direction(&world, &desired_position, self.player_height, self.margin_for_player);
                        if move_location == 0 || move_location == 1 {
                            self.camera_pos = desired_position;
                            self.keyboard_space_frames += 1;
                            self.acceleration_result += self.acceleration * (-1.0);
                            if move_location == 1{
                                self.in_liquid = true;
                            }else{
                                if self.in_liquid {
                                    self.touched_ground = false;
                                }
                                self.in_liquid = false;
                            }
                        }else{
                            self.keyboard_space = false;
                            self.keyboard_ctrl = true;
                            self.keyboard_space_frames = 0;
                            self.acceleration_result = 0.2;
                        }
                    }else{
                        self.keyboard_space = false;
                        self.keyboard_ctrl = true;
                        self.keyboard_space_frames = 0;
                        self.acceleration_result = 0.2
                    }
                }else if self.in_liquid{
                    let camera_speed = 7.0 * self.delta_time * self.liquid_speed_modifyer;
                    let desired_position = self.camera_pos + glm::vec3(0.0, camera_speed, 0.0);
                    
                    let move_location = world::World::move_to_direction(&world, &desired_position, self.player_height, self.margin_for_player);
                    if move_location == 0 || move_location == 1 {
                        
                        if move_location == 3{
                            self.in_liquid = true;
                        }else {
                            self.camera_pos = desired_position;
                            if move_location == 1{
                                self.in_liquid = true;
                                self.camera_pos = desired_position;
                            }else{
                                if self.in_liquid {
                                    self.touched_ground = false;
                                }
                                self.keyboard_space = false;
                                self.keyboard_ctrl = true;
                                self.in_liquid = false;
                            }
                        }
                    }
                }
            }

            if self.keyboard_ctrl {
                
                let camera_speed = 7.0 * self.delta_time * self.liquid_speed_modifyer;
                let desired_position;
                if self.in_liquid{
                    desired_position = self.camera_pos - glm::vec3(0.0, camera_speed, 0.0);
                }else{
                    desired_position = self.camera_pos - glm::vec3(0.0, camera_speed * self.acceleration_result, 0.0);
                }
                let move_location = world::World::move_to_direction(&world, &desired_position, self.player_height, self.margin_for_player);
                if move_location == 0 || move_location == 1 {
                    self.camera_pos = desired_position;
                    if self.acceleration_result < 5.0 && !self.in_liquid{
                        self.acceleration_result += self.acceleration
                    }

                    if move_location == 1{
                        self.in_liquid = true;
                    }else{
                        self.in_liquid = false;
                    }
                }else{
                    self.touched_ground = true;
                    self.acceleration_result = 0.0;
                }
            }
        }else{
            if self.keyboard_w {
                let camera_speed = 14.0 * self.delta_time * self.liquid_speed_modifyer;
                let desired_position = self.camera_pos + glm::vec3(camera_speed * self.camera_front.x, 0.0, camera_speed * self.camera_front.z);
                let move_location = world::World::move_to_direction(&world, &desired_position, self.player_height, self.margin_for_player);
                if move_location == 0 || move_location == 1 {
                    self.camera_pos = desired_position;
                    if move_location == 1{
                        self.in_liquid = true;
                    }else{
                        if self.in_liquid {
                            self.touched_ground = false;
                        }
                        self.in_liquid = false;
                    }
                }
            }

            if self.keyboard_a {
                let camera_speed = 7.0 * self.delta_time * self.liquid_speed_modifyer;
                let desired_position = self.camera_pos - glm::normalize(glm::cross(self.camera_front, self.camera_up)) * camera_speed;
                
                let move_location = world::World::move_to_direction(&world, &desired_position, self.player_height, self.margin_for_player);
                if move_location == 0 || move_location == 1 {
                    self.camera_pos = desired_position;
                    if move_location == 1{
                        self.in_liquid = true;
                    }else{
                        if self.in_liquid {
                            self.touched_ground = false;
                        }
                        self.in_liquid = false;
                    }
                }
            }

            if self.keyboard_s {
                let camera_speed = 7.0 * self.delta_time * self.liquid_speed_modifyer;
                let desired_position = self.camera_pos - glm::vec3(camera_speed * self.camera_front.x, 0.0, camera_speed * self.camera_front.z);
                
                let move_location = world::World::move_to_direction(&world, &desired_position, self.player_height, self.margin_for_player);
                if move_location == 0 || move_location == 1 {
                    self.camera_pos = desired_position;
                    if move_location == 1{
                        self.in_liquid = true;
                    }else{
                        if self.in_liquid {
                            self.touched_ground = false;
                        }
                        self.in_liquid = false;
                    }
                }
            }

            if self.keyboard_d {
                let camera_speed = 7.0 * self.delta_time * self.liquid_speed_modifyer;
                let desired_position = self.camera_pos + glm::normalize(glm::cross(self.camera_front, self.camera_up)) * camera_speed;
                
                let move_location = world::World::move_to_direction(&world, &desired_position, self.player_height, self.margin_for_player);
                if move_location == 0 || move_location == 1 {
                    self.camera_pos = desired_position;
                    if move_location == 1{
                        self.in_liquid = true;
                    }else{
                        if self.in_liquid {
                            self.touched_ground = false;
                        }
                        self.in_liquid = false;
                    }
                }
            }

            if self.keyboard_space {
                let camera_speed = 7.0 * self.delta_time * self.liquid_speed_modifyer;
                let desired_position = self.camera_pos + glm::vec3(0.0, camera_speed, 0.0);
                
                let move_location = world::World::move_to_direction(&world, &desired_position, self.player_height, self.margin_for_player);
                if move_location == 0 || move_location == 1 {
                    self.camera_pos = desired_position;
                    if move_location == 1{
                        self.in_liquid = true;
                    }else{
                        self.in_liquid = false;
                    }
                }
            }

            if self.keyboard_ctrl {
                
                let camera_speed = 7.0 * self.delta_time * self.liquid_speed_modifyer;
                let desired_position = self.camera_pos - glm::vec3(0.0, camera_speed, 0.0);
                let move_location = world::World::move_to_direction(&world, &desired_position, self.player_height, self.margin_for_player);
                if move_location == 0 || move_location == 1 {
                    self.camera_pos = desired_position;
                    if move_location == 1{
                        self.in_liquid = true;
                    }else{
                        self.in_liquid = false;
                    }
                }
            }
        }

        return close_game;
    }
}