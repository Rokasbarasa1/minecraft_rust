use glium::glutin;
use crate::world;
pub struct CameraState {
    pub camera_pos: [f32;3],
    pub camera_front: [f32;3],
    pub camera_up: [f32;3],
    pub aspect_ratio: f32,

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
}

impl CameraState {

    pub fn new(world: &mut world::World, player_height: f32, camera_pos: [f32;3], WINDOW_WIDTH: u32, WINDOW_HEIGHT: u32) -> CameraState {
        let mut player = CameraState{
            camera_pos: camera_pos,
            camera_front: [0.0, 0.0, -1.0],
            camera_up: [0.0, 1.0, 0.0],
            aspect_ratio: WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,

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

        };

        // player.camera_pos = world::World::get_spawn_location(&world, &player.camera_pos, 0 as usize);

        return player;
    }

    pub fn get_projection(&self) -> [[f32; 4]; 4] {
        let fov: f32 = 3.141592 / 2.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [f / self.aspect_ratio,    0.0,              0.0              ,   0.0],
            [         0.0         ,     f ,              0.0              ,   0.0],
            [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
            [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
        ]
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {





        
        let f = {
            let f = (self.camera_front[0], self.camera_front[1], self.camera_front[2]);
            let len = f.0 * f.0 + f.1 * f.1 + f.2 * f.2;
            let len = len.sqrt();
            (f.0 / len, f.1 / len, f.2 / len)
        };

        let up = (0.0, 1.0, 0.0);

        let s = (f.1 * up.2 - f.2 * up.1,
                 f.2 * up.0 - f.0 * up.2,
                 f.0 * up.1 - f.1 * up.0);

        let s_norm = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };

        let u = (s_norm.1 * f.2 - s_norm.2 * f.1,
                 s_norm.2 * f.0 - s_norm.0 * f.2,
                 s_norm.0 * f.1 - s_norm.1 * f.0);

        let p = (-self.camera_pos[0] * s.0 - self.camera_pos[1] * s.1 - self.camera_pos[2] * s.2,
                 -self.camera_pos[0] * u.0 - self.camera_pos[1] * u.1 - self.camera_pos[2] * u.2,
                 -self.camera_pos[0] * f.0 - self.camera_pos[1] * f.1 - self.camera_pos[2] * f.2);

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [s_norm.0, u.0, f.0, 0.0],
            [s_norm.1, u.1, f.1, 0.0],
            [s_norm.2, u.2, f.2, 0.0],
            [p.0, p.1,  p.2, 1.0],
        ]
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
                            println!("Flying");

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
                // let crossed = cross(self.camera_front, self.camera_up);

                // println!("Old position: x:{0} y:{1} z:{2}", self.camera_pos[0], self.camera_pos[1], self.camera_pos[2] );

                // println!("Pre cross: (front)x:{0} y:{1} z:{2}   (camera up)x:{3} y:{4} z:{5}", self.camera_front[0], self.camera_front[1], self.camera_front[2], self.camera_up[0], self.camera_up[1], self.camera_up[2] );
                // println!("Cross: x:{0} y:{1} z:{2}", crossed[0], crossed[1], crossed[2] );

                // let normalized = normalize(crossed);

                // println!("normalized: x:{0} y:{1} z:{2}", normalized[0], normalized[1], normalized[2] );

                // let minused = minus(self.camera_pos, normalized);

                // println!("minused with original pos: x:{0} y:{1} z:{2}", minused[0], minused[1], minused[2] );

                // let multi = multiply(minused, camera_speed);

                // println!("Multiplied with camera speed: x:{0} y:{1} z:{2}   camera-speed{3}", multi[0], multi[1], multi[2], camera_speed);
                let normalized = normalize(cross(self.camera_front, self.camera_up));

                let desired_position = minus(self.camera_pos, [normalized[0] * camera_speed, normalized[1] * camera_speed, normalized[2] * camera_speed]);
                
                // println!("Old position: x:{0} y:{1} z:{2}", self.camera_pos[0], self.camera_pos[1], self.camera_pos[2] );
                // println!("Desired position: x:{0} y:{1} z:{2}", desired_position[0], desired_position[1], desired_position[2] );

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
}

fn cross(arr1: [f32; 3], arr2: [f32; 3]) -> [f32; 3]{
    let mut result: [f32; 3] = [0.0,0.0,0.0];

    let i = arr1[1] * arr2[2] - arr1[2] * arr2[1];
    let j = arr1[2] * arr2[0] - arr1[0] * arr2[2];
    let k = arr1[0] * arr2[1] - arr1[1] * arr2[0];

    // let i = arr1[1] * arr2[2] - arr1[2] * arr2[1];
    // let j = arr1[0] * arr2[2] - arr1[2] * arr2[0];
    // let k = arr1[1] * arr2[0] - arr1[0] * arr2[1];

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

fn multiply(arr1: [f32; 3], value: f32) -> [f32; 3]{
    let mut result: [f32; 3] = [0.0,0.0,0.0];

    result[0] = arr1[0]/value;
    result[1] = arr1[1]/value;
    result[2] = arr1[2]/value;

    result
}