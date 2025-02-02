use camera::CameraState;
use glium::{
    backend::glutin::SimpleWindowBuilder,
    glutin::surface::WindowSurface,
    winit::{
        application::ApplicationHandler,
        dpi::{LogicalPosition, Position},
        event::{StartCause, WindowEvent},
        event_loop::{ActiveEventLoop, EventLoop},
        keyboard::{KeyCode, PhysicalKey},
        window::{Window, WindowId},
    },
    Display, Program, Surface,
};
use rand::Rng;
use rand::SeedableRng;
use skybox::Skybox;
use std::time::{SystemTime, UNIX_EPOCH};
use stopwatch::Stopwatch;
use world::World;

use crate::{camera, skybox, world};

//Settings
const WINDOW_WIDTH: u32 = 1600; // For windowed only
const WINDOW_HEIGHT: u32 = 1200; // For windowed only

const SQUARE_CHUNK_WIDTH: usize = 16; //Values can be: 4,6,10,16,22,28
const CHUNKS_LAYERS_FROM_PLAYER: usize = 53; //Odd numbers ONLY
const PLAYER_HEIGHT: f32 = 1.5;

const MID_HEIGHT: u8 = 24; //The terrain variation part
const SKY_HEIGHT: u8 = 4; //Works as a buffer for the mid height needs to be at least 20 percent of mid size
const UNDERGROUND_HEIGHT: u8 = 0;

const TIME_BETWEEN_FRAMES: u64 = 20;

#[derive(Default)]
pub struct App {
    pub window: Option<Window>,
    pub display: Option<Display<WindowSurface>>,
    pub camera: CameraState,
    pub world: World,
    pub skybox: Option<Skybox>,
    pub program_block: Option<Program>,
    pub time_increment: f32,
    pub model: [[f32; 4]; 4],
    pub stopwatch: Stopwatch,
}

impl App {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let mut rng = rand_xoshiro::SplitMix64::seed_from_u64((since_the_epoch.as_millis()) as u64);
        // const WORLD_GEN_SEED: u32 = 60;                 //Any number
        let world_gen_seed: u32 = rng.random_range(1..999999999);

        let (window, display) = SimpleWindowBuilder::new()
            .with_inner_size(WINDOW_WIDTH, WINDOW_HEIGHT)
            .with_title("Minecraft RS")
            .build(event_loop);

        let vertex_shader_block = r#"
            #version 140

            in vec3 position;
            in vec2 tex_coords;
            in float opacity;
            in float brightness;

            out float v_brightness;    
            out vec2 v_tex_coords;
            out float v_opacity;

            uniform mat4 projection;
            uniform mat4 view;
            uniform mat4 model;

            void main() {
                v_brightness = brightness;
                v_tex_coords = tex_coords;
                v_opacity = opacity;
                gl_Position = projection * view * model * vec4(position, 1.0);
            }
        "#;

        let fragment_shader_block = r#"
            #version 140

            in float v_brightness;    
            in vec2 v_tex_coords;
            in float v_opacity;
            out vec4 color;

            uniform sampler2D tex;

            void main() {
                color = texture(tex, v_tex_coords) * vec4(v_brightness, v_brightness, v_brightness, v_opacity);
            }
        "#;

        let program_block =
            Program::from_source(&display, vertex_shader_block, fragment_shader_block, None)
                .unwrap();

        let camera_pos = [0.0, 0.0, 0.0];

        let mut world = world::World::new(
            &display,
            camera_pos,
            &SQUARE_CHUNK_WIDTH,
            &CHUNKS_LAYERS_FROM_PLAYER,
            &world_gen_seed,
            &MID_HEIGHT,
            &UNDERGROUND_HEIGHT,
            &SKY_HEIGHT,
        );

        let camera = camera::CameraState::new(
            &mut world,
            PLAYER_HEIGHT,
            camera_pos,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        );
        let skybox = Skybox::new(&display);

        App {
            window: Some(window),
            display: Some(display),
            camera: camera,
            world: world,
            skybox: Some(skybox),
            program_block: Some(program_block),
            time_increment: 0.0,
            model: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
            stopwatch: stopwatch::Stopwatch::new(),
        }
    }

    fn draw_frame(&mut self, frame_time: u64) {
        self.camera.delta_time = self.time_increment - self.camera.last_frame;
        self.camera.last_frame = self.time_increment;

        self.camera.update(&mut self.world);
        let view = self.camera.get_view();
        let projection = self.camera.get_projection();

        if let Some(display) = self.display.as_ref() {
            let mut target = display.draw();
            target.clear_color_srgb_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

            if let Some(program_block) = self.program_block.as_ref() {
                self.world.draw(
                    &self.camera,
                    view,
                    projection,
                    &mut target,
                    display,
                    program_block,
                    self.model,
                );
            }

            if let Some(skybox) = self.skybox.as_ref() {
                skybox.draw(&mut target, &display, view, projection);
            }

            target.finish().unwrap();
        }

        self.time_increment += 0.02;

        self.world.render_loop();
        loop {
            if (self.stopwatch.elapsed_ms() as u64) < frame_time {
                self.world.render_loop();
            } else {
                break;
            }
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        /*self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );*/
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        self.camera.process_input(&event, &mut self.world);
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => match event.physical_key {
                PhysicalKey::Code(key_code) => {
                    if key_code == KeyCode::Escape {
                        println!("Exit");
                        event_loop.exit();
                    }
                }
                _ => {}
            },
            WindowEvent::RedrawRequested => {
                if let Some(window) = self.window.as_ref() {
                    window.request_redraw();
                }
            }
            WindowEvent::Resized(window_size) => {
                self.camera.window_width = window_size.width;
                self.camera.window_height = window_size.height;
                if let Some(display) = self.display.as_ref() {
                    display.resize(window_size.into());
                }
                //self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        self.stopwatch.reset();
        self.stopwatch.start();
        match cause {
            StartCause::ResumeTimeReached { .. } => (),
            StartCause::Init => (),
            _ => return,
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let position = Position::Logical(LogicalPosition::new(
            self.camera.window_width as f64 / 2.0,
            self.camera.window_height as f64 / 2.0,
        ));
        if let Some(window) = self.window.as_ref() {
            window.set_cursor_position(position).unwrap();
        }
        if let Some(display) = self.display.as_ref() {
            self.draw_frame(TIME_BETWEEN_FRAMES);
        }
    }
}
