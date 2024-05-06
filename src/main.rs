#[macro_use]
#[allow(unused_imports)]
extern crate glium;
mod camera;
use glium::{glutin, Surface};
extern crate stopwatch;
pub mod world;
pub mod player;
pub mod skybox; 
use glutin::dpi::{LogicalPosition, Position};

extern crate rand_xoshiro;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::{SeedableRng};
use rand::Rng;

//$Env:RUST_BACKTRACE=1
fn main() {
    //Settings
    const WINDOW_WIDTH: u32 = 1280; // For windowed only
    const WINDOW_HEIGHT: u32 = 720; // For windowed only
    
    const SQUARE_CHUNK_WIDTH: usize = 16;           //Values can be: 4,6,10,16,22,28
    const CHUNKS_LAYERS_FROM_PLAYER: usize = 33;    //Odd numbers ONLYYY
    const PLAYER_HEIGHT: f32 = 1.5;

    
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let mut rng = rand_xoshiro::SplitMix64::seed_from_u64((since_the_epoch.as_millis()) as u64);
    // const WORLD_GEN_SEED: u32 = 60;                 //Any number
    let WORLD_GEN_SEED: u32 = rng.gen_range(1..999999999);

    const MID_HEIGHT: u8 = 70;                   //The terrain variation part
    const SKY_HEIGHT: u8 = 0;                   //Works as a buffer for the mid heigt needs to be at least 20 percent of mid size
    const UNDERGROUND_HEIGHT: u8 = 0;            

    const TIME_BETWEEN_FRAMES: u64 = 20;

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
    .with_title(format!("Minecraft RS"))
    .with_inner_size(glutin::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
 

    let monitor_handle = display.gl_window().window().available_monitors().next().unwrap();
    let fs = glium::glutin::window::Fullscreen::Borderless(Some(monitor_handle));
    display.gl_window().window().set_fullscreen(Some(fs));
    display.gl_window().window().set_cursor_grab(true);
    display.gl_window().window().set_cursor_visible(false);

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

    let program_block = glium::Program::from_source(&display, vertex_shader_block, fragment_shader_block, None).unwrap();


    let camera_pos = [0.0, 0.0, 0.0];
    
    let mut world = world::World::new(
        &display,
        camera_pos, 
        &SQUARE_CHUNK_WIDTH, 
        &CHUNKS_LAYERS_FROM_PLAYER, 
        &WORLD_GEN_SEED,
        &MID_HEIGHT,
        &UNDERGROUND_HEIGHT,
        &SKY_HEIGHT,
    );
    let mut camera = camera::CameraState::new(&mut world, PLAYER_HEIGHT, camera_pos, WINDOW_WIDTH, WINDOW_HEIGHT);
    let skybox: skybox::Skybox = skybox::Skybox::new(&display);

    let mut time_increment: f32 = 0.0;
    let model = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32]
    ];

    let mut stopwatch = stopwatch::Stopwatch::new();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Poll;

        stopwatch.reset();
        stopwatch.start();
        match event {
            glutin::event::Event::WindowEvent {event, .. } =>{
                camera.process_input(&event, &mut world);

                match event {
                    glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                        let key = match input.virtual_keycode {
                            Some(key) => key,
                            None => return,
                        };
                        match key {
                            glutin::event::VirtualKeyCode::Escape => {
                                *control_flow = glutin::event_loop::ControlFlow::Exit;
                                return;
                            },
                            _ => (),
                        };
                    },
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    }
                    glutin::event::WindowEvent::Resized(dimensions) => {
                        camera.window_width = dimensions.width;
                        camera.window_height = dimensions.height;
                    }
                    _ => (),
                }
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            glutin::event::Event::MainEventsCleared => {
                let position = Position::Logical(LogicalPosition::new(camera.window_width as f64  / 2.0, camera.window_height as f64  / 2.0));
                display.gl_window().window().set_cursor_position(position);
                draw_frame(&display, &mut camera, &skybox, &mut world, &program_block, TIME_BETWEEN_FRAMES, &stopwatch, &mut time_increment, model);
            },
            _ => return,
        }
    });
}

fn draw_frame(display: &glium::Display, camera: &mut camera::CameraState, skybox: &skybox::Skybox, world: &mut world::World, program_block: &glium::Program, frame_time: u64, stopwatch: &stopwatch::Stopwatch, time_increment: &mut f32, model: [[f32;4]; 4], ){
    camera.delta_time = time_increment.clone() - camera.last_frame;
    camera.last_frame = time_increment.clone();
    
    camera.update(world);
    let view = camera.get_view();
    let projection = camera.get_projection();

    let mut target = display.draw();
    target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
 
    world.draw(&camera.camera_pos, view, projection, &mut target, &display, &program_block, model);
    skybox.draw(&mut target, &display, view, projection);

    target.finish().unwrap();

    *time_increment += 0.02;
    
    world.render_loop();
    loop{
        if (stopwatch.elapsed_ms() as u64) < frame_time {
            world.render_loop();
        }else{

            break;
        }
    }
}