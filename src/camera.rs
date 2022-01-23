use glium::glutin;

pub struct CameraState {
    aspect_ratio: f32,
    position: (f32, f32, f32),
    direction: (f32, f32, f32),

    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,

    first_mouse: bool,
    last_x: f32,
    last_y: f32,
    yaw: f32,
    pitch: f32,
}

impl CameraState {

    pub fn new(width: f32, height: f32) -> CameraState {
        CameraState {
            aspect_ratio: width / height,
            position: (0.1, 0.1, 1.0),
            direction: (0.0, 0.0, -1.0),
            moving_up: false,
            moving_left: false,
            moving_down: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,

            first_mouse: true,
            last_x: 800.0 / 2.0,
            last_y: 600.0 / 2.0,
            yaw: -90.0,
            pitch: 0.0,
        }
    }

    pub fn set_position(&mut self, pos: (f32, f32, f32)) {
        self.position = pos;
    }

    pub fn set_direction(&mut self, dir: (f32, f32, f32)) {
        self.direction = dir;
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
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
            let f = self.direction;
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

        let p = (-self.position.0 * s.0 - self.position.1 * s.1 - self.position.2 * s.2,
                 -self.position.0 * u.0 - self.position.1 * u.1 - self.position.2 * u.2,
                 -self.position.0 * f.0 - self.position.1 * f.1 - self.position.2 * f.2);

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [s_norm.0, u.0, f.0, 0.0],
            [s_norm.1, u.1, f.1, 0.0],
            [s_norm.2, u.2, f.2, 0.0],
            [p.0, p.1,  p.2, 1.0],
        ]
    }

    pub fn update(&mut self) {
        let f = {
            let f = self.direction;
            let len = f.0 * f.0 + f.1 * f.1 + f.2 * f.2;
            let len = len.sqrt();
            (f.0 / len, f.1 / len, f.2 / len)
        };

        let up = (0.0, 1.0, 0.0);

        let s = (f.1 * up.2 - f.2 * up.1,
                 f.2 * up.0 - f.0 * up.2,
                 f.0 * up.1 - f.1 * up.0);

        let s = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };

        let u = (s.1 * f.2 - s.2 * f.1,
                 s.2 * f.0 - s.0 * f.2,
                 s.0 * f.1 - s.1 * f.0);

        if self.moving_up {
            self.position.0 += u.0 * 0.01;
            self.position.1 += u.1 * 0.01;
            self.position.2 += u.2 * 0.01;
        }

        if self.moving_left {
            self.position.0 -= s.0 * 0.01;
            self.position.1 -= s.1 * 0.01;
            self.position.2 -= s.2 * 0.01;
        }

        if self.moving_down {
            self.position.0 -= u.0 * 0.01;
            self.position.1 -= u.1 * 0.01;
            self.position.2 -= u.2 * 0.01;
        }

        if self.moving_right {
            self.position.0 += s.0 * 0.01;
            self.position.1 += s.1 * 0.01;
            self.position.2 += s.2 * 0.01;
        }

        if self.moving_forward {
            self.position.0 += f.0 * 0.01;
            self.position.1 += f.1 * 0.01;
            self.position.2 += f.2 * 0.01;
        }

        if self.moving_backward {
            self.position.0 -= f.0 * 0.01;
            self.position.1 -= f.1 * 0.01;
            self.position.2 -= f.2 * 0.01;
        }
    }

    pub fn get_position(&self) -> [f32; 3]{

        [self.position.0, self.position.1, self.position.2]
    }

    pub fn process_input(&mut self, event: &glutin::event::WindowEvent<'_>) {

        match *event {
            glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                let pressed = input.state == glutin::event::ElementState::Pressed;
                let key = match input.virtual_keycode {
                    Some(key) => key,
                    None => return,
                };
                match key {
                    glutin::event::VirtualKeyCode::Up => self.moving_up = pressed,
                    glutin::event::VirtualKeyCode::Down => self.moving_down = pressed,
                    glutin::event::VirtualKeyCode::A => self.moving_left = pressed,
                    glutin::event::VirtualKeyCode::D => self.moving_right = pressed,
                    glutin::event::VirtualKeyCode::W => self.moving_forward = pressed,
                    glutin::event::VirtualKeyCode::S => self.moving_backward = pressed,
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

                let normalized_front = normalize(front);
                self.direction = (normalized_front[0], normalized_front[1], normalized_front[2]);
            },
            _ => return,
        };
        
    }
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