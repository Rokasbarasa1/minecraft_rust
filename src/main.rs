#[macro_use]
#[allow(unused_imports)]
extern crate glium;
mod camera;
use std::io::Cursor;
use glium::{glutin, Surface};
extern crate stopwatch;
pub mod world;
pub mod player;
pub mod skybox; 
use glium::glutin::event::VirtualKeyCode;

//$Env:RUST_BACKTRACE=1
fn main() {
    //Settings
    const WINDOW_WIDTH: u32 = 1280;
    const WINDOW_HEIGHT: u32 = 720;
    
    const SQUARE_CHUNK_WIDTH: usize = 16;           //Values can be: 4,6,10,16,22,28
    const CHUNKS_LAYERS_FROM_PLAYER: usize = 19;    //Odd numbers ONLYYY
    const PLAYER_HEIGHT: f32 = 1.5;

    const WORLD_GEN_SEED: u32 = 60;                 //Any number
    const MID_HEIGHT: u8 = 50;                   //The terrain variation part
    const SKY_HEIGHT: u8 = 10;                   //Works as a buffer for the mid heigt needs to be at least 20 percent of mid size
    const UNDERGROUND_HEIGHT: u8 = 0;            

    let event_loop = glutin::event_loop::EventLoop::new();
    

    let wb = glutin::window::WindowBuilder::new()
    .with_title(format!("Minecraft RS"));
    // . with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT);

    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let image = image::load(Cursor::new(&include_bytes!("../resources/posy.png")),image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    let vertex_shader_block = r#"
        #version 140

        in vec3 position;
        in vec2 tex_coords;
        in float opacity;

        out vec2 v_tex_coords;
        out float v_opacity;

        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;

        void main() {
            mat4 modelView = view * model;
            v_tex_coords = tex_coords;
            v_opacity = opacity;
            gl_Position = perspective * modelView * vec4(position, 0.05);
        }
    "#;

    let fragment_shader_block = r#"
        #version 140

        in vec2 v_tex_coords;
        in float v_opacity;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    let program_block = glium::Program::from_source(&display, vertex_shader_block, fragment_shader_block, None).unwrap();
    // let program_skybox = glium::Program::from_source(&display, vertex_shader_skybox, fragment_shader_skybox, None).unwrap();

    let mut time_increment: f32 = 0.0;
    // let camera_pos = [0.0, 0.0, 0.0];
    let mut camera = camera::CameraState::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
    camera.set_position((0.0, 35.0, -8.0));
    camera.set_direction((0.0, 0.0, 1.0));
    let mut world = world::World::new(
        &display,
        camera.get_position(), 
        &SQUARE_CHUNK_WIDTH, 
        &CHUNKS_LAYERS_FROM_PLAYER, 
        &WORLD_GEN_SEED,
        &MID_HEIGHT,
        &UNDERGROUND_HEIGHT,
        &SKY_HEIGHT,
    );

    let new_camera_position = world.get_spawn_location(&camera.get_position(), 0 as usize);
    camera.set_position((new_camera_position[0], new_camera_position[1], new_camera_position[2]));
    // let mut player: player::Player = player::Player::new(&mut world, PLAYER_HEIGHT, camera_pos);

    let skybox: skybox::Skybox = skybox::Skybox::new(&display);

    const TIME_BETWEEN_FRAMES: u64 = 20;
    let mut stopwatch = stopwatch::Stopwatch::new();

    event_loop.run(move |event, _, control_flow| {
        stopwatch.reset();
        stopwatch.start();
        camera.update();
        if let glutin::event::Event::WindowEvent {
            event: window_event,
            ..
        } = event
        {
            camera.process_input(&window_event);
            match window_event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => (),
            }
        }

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let model = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 2.0, 1.0f32]
        ];
        
        skybox.draw(&mut target, &display, camera.get_view(), camera.get_perspective());

        world.draw(&camera.get_position(), camera.get_view(), camera.get_perspective(), &mut target, &display, &program_block, model);
        
        target.finish().unwrap();

        // world.render_loop();
        // loop{
        //     if (stopwatch.elapsed_ms() as u64) < TIME_BETWEEN_FRAMES {
        //         world.render_loop();
        //     }else{
        //         break;
        //     }
        // }
    });
}