use crate::world;

extern crate glm;

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

    pub mouse_button_clicked: bool,
    pub keyboard_w: bool,
    pub keyboard_a: bool,
    pub keyboard_s: bool,
    pub keyboard_d: bool,

    pub keyboard_space: bool,
    pub keyboard_space_frames: usize,
    pub touched_ground: bool,

    pub keyboard_ctrl: bool,
    pub selected_block: usize,

    pub acceleration_result: f32,
    pub acceleration: f32, // More like bonus speed after adding acceleration

    pub liquid_speed_modifyer: f32,
    pub in_liquid: bool,
}

impl Player{
    pub fn new(world: &mut world::World, player_height: f32, camera_pos: glm::Vector3<f32> ) -> Player {
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
            in_liquid: false
        };

        player.camera_pos = world::World::get_spawn_location(&world, &player.camera_pos, 0 as usize);

        return player;
    }

    pub fn handle_events(&mut self, world: &mut world::World, event_pump: &mut sdl2::EventPump) -> bool{
        let mut close_game = false;
        
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => close_game = true, 
                sdl2::event::Event::KeyDown { timestamp: _, window_id: _, keycode: _, scancode, keymod: _, repeat: _ } => {
                    //Change to polygon mesh mode
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Escape {
                        close_game = true;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Q {
                        unsafe {
                            if !self.mesh {
                                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                                self.mesh = true;
                            } else{
                                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                                self.mesh = false;
                            }
                        }
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Space {
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
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::W {
                        self.keyboard_w = true;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::S {
                        self.keyboard_s = true;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::A {
                        self.keyboard_a = true;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::D {
                        self.keyboard_d = true;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::F {
                        world::World::destroy_block(world, &self.camera_front, &self.camera_pos);
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Num1 {
                        self.selected_block = 0;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Num2 {
                        self.selected_block = 1;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Num3 {
                        self.selected_block = 2;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Num4 {
                        self.selected_block = 3;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Num5 {
                        self.selected_block = 4;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Num6 {
                        self.selected_block = 5;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Num7 {
                        self.selected_block = 6;
                    }
                },
                sdl2::event::Event::KeyUp { timestamp: _, window_id: _, keycode: _, scancode, keymod: _, repeat: _ } => {
                    if scancode.unwrap() == sdl2::keyboard::Scancode::W {
                        self.keyboard_w = false;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::S {
                        self.keyboard_s = false;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::A {
                        self.keyboard_a = false;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::D {
                        self.keyboard_d = false;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Space {
                        if self.in_liquid{
                            // Space up
                            self.keyboard_space = false;
                            self.keyboard_ctrl = true;
                        }
                    }
                },
                sdl2::event::Event::MouseMotion { timestamp: _, window_id: _, which: _, mousestate: _, x, y, xrel: _, yrel: _ } => {
                    if self.first_mouse
                    {
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
                sdl2::event::Event::MouseWheel { timestamp: _, window_id: _, which: _, x: _, y, direction: _ } => {
                    if self.fov >= 1.0 && self.fov <= 90.0 {
                        self.fov -= y as f32;
                    }  
                    if  self.fov < 1.0 {
                        self.fov = 1.0;
                    }   
                    if  self.fov > 90.0 {
                        self.fov = 90.0;
                    }
                },
                sdl2::event::Event::MouseButtonDown { timestamp: _, window_id: _, which: _, mouse_btn, clicks: _, x: _, y: _ } =>{
                    if !self.mouse_button_clicked {
                        if mouse_btn.eq(&sdl2::mouse::MouseButton::Left){
                            world::World::destroy_block(world, &self.camera_front, &self.camera_pos);
                        } else {
                            world::World::place_block(world, &self.camera_front, &self.camera_pos, self.selected_block);
                        }
                        self.mouse_button_clicked = true;
                    }
                },
                sdl2::event::Event::MouseButtonUp { timestamp: _, window_id: _, which: _, mouse_btn: _, clicks: _, x: _, y: _ } =>{
                    self.mouse_button_clicked = false;
                },
                _ => {}
            }
        }     
        
        if self.in_liquid{
            self.liquid_speed_modifyer = 0.45;
        }else{
            self.liquid_speed_modifyer = 1.0;
        }

        {
            if self.keyboard_w {
                let camera_speed = 7.0 * self.delta_time * self.liquid_speed_modifyer;
                let desired_position = self.camera_pos + glm::vec3(camera_speed * self.camera_front.x, 0.0, camera_speed * self.camera_front.z);

                let move_location = world::World::move_to_direction(&world, &desired_position, self.player_height);
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
                
                let move_location = world::World::move_to_direction(&world, &desired_position, self.player_height);
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
                
                let move_location = world::World::move_to_direction(&world, &desired_position, self.player_height);
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
                
                let move_location = world::World::move_to_direction(&world, &desired_position, self.player_height);
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
                    // In not water
                    println!("In not water");
                    self.keyboard_ctrl = false;
                    if self.keyboard_space_frames < 10 {
                        let camera_speed = 7.0 * self.delta_time * self.liquid_speed_modifyer;
                        let desired_position = self.camera_pos + glm::vec3(0.0, camera_speed * self.acceleration_result, 0.0);
                        
                        let move_location = world::World::move_to_direction(&world, &desired_position, self.player_height);
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
                    
                    let move_location = world::World::move_to_direction(&world, &desired_position, self.player_height);
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
                let move_location = world::World::move_to_direction(&world, &desired_position, self.player_height);
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
        }

        // println!("{}", self.in_liquid);
        return close_game;
    }
}