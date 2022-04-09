use glium::glutin;
use crate::world;
extern crate glm;

pub struct CameraState {
    pub camera_pos: [f32;3],
    pub camera_front: [f32;3],
    pub camera_up: [f32;3],
    pub aspect_ratio: f32,

    pub window_width: u32,
    pub window_height: u32,

    pub yaw: f32,
    pub pitch: f32,
    pub fov: f32,

    pub first_mouse: (bool,bool),
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
}

impl CameraState {

    pub fn new(world: &mut world::World, player_height: f32, camera_pos: [f32;3], window_width: u32, window_height: u32) -> CameraState {
        let mut player = CameraState{
            camera_pos: camera_pos,
            camera_front: [0.0, 0.0, -1.0],
            camera_up: [0.0, 1.0, 0.0],
            aspect_ratio: window_width as f32 / window_height as f32,
            window_width: window_width,
            window_height: window_height,

            yaw: -90.0,
            pitch: 0.0,
            fov: 100.0,

            first_mouse: (false,false),
            last_x: window_width as f32 / 2.0,
            last_y: window_height as f32 / 2.0,
            
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

        };

        player.camera_pos = world::World::get_spawn_location(&world, &player.camera_pos, 0 as usize);

        return player;
    }

    pub fn process_input(&mut self, event: &glutin::event::WindowEvent<'_>, world: &mut world::World) {

        match *event {
            glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                let pressed = input.state == glutin::event::ElementState::Pressed;
                let key = match input.virtual_keycode {
                    Some(key) => key,
                    None => return,
                };
                match key {
                    glutin::event::VirtualKeyCode::Space => {
                        if pressed {
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
                        }else if self.keyboard_space {
                            if self.in_liquid && !self.flying{
                                // Space up
                                self.keyboard_space = false;
                                self.keyboard_ctrl = true;
                            }else if self.flying{
                                self.keyboard_space = false;
                                self.keyboard_ctrl = false;
                            }
                        }
                        
                    },
                    glutin::event::VirtualKeyCode::LControl => {
                        if pressed {
                            if self.flying{
                                self.keyboard_ctrl = true;
                            }
                        }else if self.keyboard_ctrl {
                            if self.flying{
                                self.keyboard_ctrl = false;
                            }
                        }
                        
                    },
                    glutin::event::VirtualKeyCode::A => {
                        if pressed {
                            self.keyboard_a = true;
                        }else if self.keyboard_a {
                            self.keyboard_a = false;
                        }
                    },
                    glutin::event::VirtualKeyCode::D => {
                        if pressed {
                            self.keyboard_d = true;
                        }else if self.keyboard_d {
                            self.keyboard_d = false;
                        }
                    },
                    glutin::event::VirtualKeyCode::W => {
                        if pressed {
                            self.keyboard_w = true;
                        }else if self.keyboard_w {
                            self.keyboard_w = false;
                        }
                    },
                    glutin::event::VirtualKeyCode::S => {
                        if pressed {
                            self.keyboard_s = true;
                        }else if self.keyboard_s {
                            self.keyboard_s = false;
                        }
                        
                    },
                    glutin::event::VirtualKeyCode::Q => {
                        println!("IMPLEMENT POLYGON MODE Q");
                        // unsafe {
                        //     if !self.mesh {
                        //         gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                        //         self.mesh = true;
                        //     } else{
                        //         gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                        //         self.mesh = false;
                        //     }
                        // }
                    },
                    glutin::event::VirtualKeyCode::E => {
                        if pressed{
                            if !self.flying {
                                println!("Flying turned on");
                                self.flying = true;
                            } else{
                                println!("Flying turned off");
                                self.flying = false;
                                self.keyboard_ctrl = true;
                            }
                        }  
                    },
                    glutin::event::VirtualKeyCode::F => {
                        world::World::destroy_block(world, &self.camera_front, &self.camera_pos);
                    },
                    glutin::event::VirtualKeyCode::Key1 => {
                        self.selected_block = 0;
                    },
                    glutin::event::VirtualKeyCode::Key2 => {
                        self.selected_block = 1;
                    },
                    glutin::event::VirtualKeyCode::Key3 => {
                        self.selected_block = 2;
                    },
                    glutin::event::VirtualKeyCode::Key4 => {
                        self.selected_block = 3;
                    },
                    glutin::event::VirtualKeyCode::Key5 => {
                        self.selected_block = 4;
                    },
                    glutin::event::VirtualKeyCode::Key6 => {
                        self.selected_block = 5;
                    },
                    glutin::event::VirtualKeyCode::Key7 => {
                        self.selected_block = 6;
                    },
                    _ => (),
                };
            },
            glutin::event::WindowEvent::CursorMoved { position, .. } => {
                let x = position.x;
                let y = position.y;

                if self.first_mouse.0 && !self.first_mouse.1{
                    self.last_x = x as f32;
                    self.last_y = y as f32;
                    self.first_mouse = (true,true);
                }else if !self.first_mouse.0 && !self.first_mouse.1{
                    self.first_mouse = (true,false);
                }

                let mut xoffset = x as f32 - self.last_x;
                let mut yoffset = self.last_y - y as f32; // reversed since y-coordinates go from bottom to top

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

                let mut front = [0.0, 0.0, 0.0];
                front[0] = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
                front[1] = self.pitch.to_radians().sin();
                front[2] = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();

                self.camera_front = normalize(front);
            },
            glutin::event::WindowEvent::MouseInput { state, button, ..} =>{
                
                match state{
                    glutin::event::ElementState::Pressed => {
                        if !self.mouse_button_clicked {
                            match button{
                                glutin::event::MouseButton::Left => {
                                    world::World::destroy_block(world, &self.camera_front, &self.camera_pos);
                                },
                                glutin::event::MouseButton::Right => {
                                    world::World::place_block(world, &self.camera_front, &self.camera_pos, self.selected_block, self.player_height);
                                },
                                _ => return,
                            }
                            self.mouse_button_clicked = true;
                        }
                    },
                    glutin::event::ElementState::Released => {
                        if self.mouse_button_clicked {
                            self.mouse_button_clicked = false;
                        }
                    },
                }
            },
            _ => return,
        };
    }

    pub fn update(&mut self, world: &mut world::World) {
        if self.in_liquid{
            self.liquid_speed_modifyer = 0.45;
        }else{
            self.liquid_speed_modifyer = 1.0;
        }

        if !self.flying{
            if self.keyboard_w {
                let camera_speed = 7.0 * self.delta_time * self.liquid_speed_modifyer;
                let desired_position = add(self.camera_pos, [camera_speed * self.camera_front[0], 0.0, camera_speed * self.camera_front[2]]);
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
                
                let normalized = normalize(cross(self.camera_front, self.camera_up));

                let desired_position = minus(self.camera_pos, [normalized[0] * camera_speed, normalized[1] * camera_speed, normalized[2] * camera_speed]);

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
                let desired_position = minus(self.camera_pos, [camera_speed * self.camera_front[0], 0.0, camera_speed * self.camera_front[2]]);

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
                let normalized = normalize(cross(self.camera_front, self.camera_up));
                let desired_position = add(self.camera_pos, [normalized[0] * camera_speed, normalized[1] * camera_speed, normalized[2] * camera_speed]);            

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
                        let desired_position = add(self.camera_pos, [0.0, camera_speed * self.acceleration_result, 0.0]);
                        
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
                    let desired_position = add(self.camera_pos, [0.0, camera_speed, 0.0]);
                    
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
                    desired_position = minus(self.camera_pos, [0.0, camera_speed, 0.0]);
                }else{
                    desired_position =  minus(self.camera_pos, [0.0, camera_speed * self.acceleration_result, 0.0]);
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
                let desired_position = add(self.camera_pos, [camera_speed * self.camera_front[0], 0.0, camera_speed * self.camera_front[2]]);
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
                let normalized = normalize(cross(self.camera_front, self.camera_up));
                let desired_position = minus(self.camera_pos, [normalized[0] * camera_speed, normalized[1] * camera_speed, normalized[2] * camera_speed]);       
                
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
                let desired_position = minus(self.camera_pos, [camera_speed * self.camera_front[0], 0.0, camera_speed * self.camera_front[2]]);
                
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
                let normalized = normalize(cross(self.camera_front, self.camera_up));
                let desired_position = add(self.camera_pos, [normalized[0] * camera_speed, normalized[1] * camera_speed, normalized[2] * camera_speed]);       

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
                let desired_position = add(self.camera_pos, [0.0, camera_speed, 0.0]);
                
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
                let desired_position = minus(self.camera_pos, [0.0, camera_speed, 0.0]);
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
    }

    pub fn get_view(&self) -> [[f32;4]; 4]{
        let center = add(self.camera_pos, self.camera_front);
        let view_temp = glm::ext::look_at(
            glm::vec3(self.camera_pos[0], self.camera_pos[1], self.camera_pos[2]), 
            glm::vec3(center[0], center[1], center[2]), 
            glm::vec3(self.camera_up[0], self.camera_up[1], self.camera_up[2])
        );

        [
            [view_temp.c0.x, view_temp.c0.y, view_temp.c0.z, view_temp.c0.w],
            [view_temp.c1.x, view_temp.c1.y, view_temp.c1.z, view_temp.c1.w],
            [view_temp.c2.x, view_temp.c2.y, view_temp.c2.z, view_temp.c2.w],
            [view_temp.c3.x, view_temp.c3.y, view_temp.c3.z, view_temp.c3.w]
        ]
    }

    pub fn get_projection(&self) -> [[f32;4]; 4]{
        let projection_temp = glm::ext::perspective(glm::radians(self.fov), (self.window_width as f32)/(self.window_height as f32), 0.1, 5000.0);
        
        [
            [projection_temp.c0.x, projection_temp.c0.y, projection_temp.c0.z, projection_temp.c0.w],
            [projection_temp.c1.x, projection_temp.c1.y, projection_temp.c1.z, projection_temp.c1.w],
            [projection_temp.c2.x, projection_temp.c2.y, projection_temp.c2.z, projection_temp.c2.w],
            [projection_temp.c3.x, projection_temp.c3.y, projection_temp.c3.z, projection_temp.c3.w]
        ]
    }
}

fn cross(arr1: [f32; 3], arr2: [f32; 3]) -> [f32; 3]{
    let mut result: [f32; 3] = [0.0,0.0,0.0];

    let i = arr1[1] * arr2[2] - arr1[2] * arr2[1];
    let j = arr1[2] * arr2[0] - arr1[0] * arr2[2];
    let k = arr1[0] * arr2[1] - arr1[1] * arr2[0];

    result[0] = i;
    result[1] = (-1.0) * j;
    result[2] = k;

    result
}

fn add(arr1: [f32; 3], arr2: [f32; 3]) -> [f32; 3] {
    //Add two vectors
    let mut result: [f32; 3] = [0.0,0.0,0.0];

    result[0] = arr1[0] + arr2[0];
    result[1] = arr1[1] + arr2[1];
    result[2] = arr1[2] + arr2[2];

    result
}

fn minus(arr1: [f32; 3], arr2: [f32; 3]) -> [f32; 3] {
    //Minus two vectors
    let mut result: [f32; 3] = [0.0,0.0,0.0];

    result[0] = arr1[0] - arr2[0];
    result[1] = arr1[1] - arr2[1];
    result[2] = arr1[2] - arr2[2];

    result
}

fn normalize(arr1: [f32; 3]) -> [f32; 3]{
    // Make unit vector 

    let mut result: [f32; 3] = [0.0,0.0,0.0];

    let magnitude = (f32::powi(arr1[0], 2) + f32::powi(arr1[1], 2) + f32::powi(arr1[2], 2)).sqrt();

    result[0] = arr1[0]/magnitude;
    result[1] = arr1[1]/magnitude;
    result[2] = arr1[2]/magnitude;

    result
}

// fn multiply(arr1: [f32; 3], value: f32) -> [f32; 3]{
//     let mut result: [f32; 3] = [0.0,0.0,0.0];

//     result[0] = arr1[0]/value;
//     result[1] = arr1[1]/value;
//     result[2] = arr1[2]/value;

//     result
// }

// fn negative(arr1: [f32; 3]) -> [f32; 3]{
//     let mut result: [f32; 3] = [0.0,0.0,0.0];

//     result[0] = -arr1[0];
//     result[1] = -arr1[1];
//     result[2] = -arr1[2];

//     result
// }

// fn dot(arr1: [f32; 3], arr2: [f32; 3]) -> f32{
//     arr1[0]*arr2[0] + arr1[1]*arr2[1] + arr1[2]*arr2[2] 
// }