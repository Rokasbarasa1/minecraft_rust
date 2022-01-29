#[macro_use]
#[allow(unused_imports)]
extern crate glium;
extern crate glm;
mod camera;
use glium::{glutin, Surface};
use skybox::Skybox;
extern crate stopwatch;
pub mod world;
pub mod player;
pub mod skybox; 
use glutin::dpi::{self, LogicalPosition, Position};
//$Env:RUST_BACKTRACE=1
fn main() {
    //Settings
    const WINDOW_WIDTH: u32 = 1280;
    const WINDOW_HEIGHT: u32 = 720;
    
    const SQUARE_CHUNK_WIDTH: usize = 16;           //Values can be: 4,6,10,16,22,28
    const CHUNKS_LAYERS_FROM_PLAYER: usize = 9;    //Odd numbers ONLYYY
    const PLAYER_HEIGHT: f32 = 1.5;

    const WORLD_GEN_SEED: u32 = 60;                 //Any number
    const MID_HEIGHT: u8 = 50;                   //The terrain variation part
    const SKY_HEIGHT: u8 = 0;                   //Works as a buffer for the mid heigt needs to be at least 20 percent of mid size
    const UNDERGROUND_HEIGHT: u8 = 0;            

    const TIME_BETWEEN_FRAMES: u64 = 20;

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
    .with_title(format!("Minecraft RS"))
    .with_inner_size(glutin::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));
    // .grab_cursor(true);
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
 

    let monitor_handle = display.gl_window().window().available_monitors().next().unwrap();
    let fs = glium::glutin::window::Fullscreen::Borderless(Some(monitor_handle));
    display.gl_window().window().set_fullscreen(Some(fs));
    display.gl_window().window().set_cursor_grab(true);
    // display.gl_window().window().set_cursor_visible(false);

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
                        // camera.update_screen();
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
    
    let mut target = display.draw();
    target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

    let center = add(camera.camera_pos, camera.camera_front);
    let view_temp = glm::ext::look_at(
        glm::vec3(camera.camera_pos[0], camera.camera_pos[1], camera.camera_pos[2]), 
        glm::vec3(center[0], center[1], center[2]), 
        glm::vec3(camera.camera_up[0], camera.camera_up[1], camera.camera_up[2])
    );
    let view: [[f32; 4]; 4] = [
        [view_temp.c0.x, view_temp.c0.y, view_temp.c0.z, view_temp.c0.w],
        [view_temp.c1.x, view_temp.c1.y, view_temp.c1.z, view_temp.c1.w],
        [view_temp.c2.x, view_temp.c2.y, view_temp.c2.z, view_temp.c2.w],
        [view_temp.c3.x, view_temp.c3.y, view_temp.c3.z, view_temp.c3.w]
    ];


    let projection_temp = glm::ext::perspective(glm::radians(camera.fov), (camera.window_width as f32)/(camera.window_height as f32), 0.1, 5000.0);
    let projection: [[f32; 4]; 4] = [
        [projection_temp.c0.x, projection_temp.c0.y, projection_temp.c0.z, projection_temp.c0.w],
        [projection_temp.c1.x, projection_temp.c1.y, projection_temp.c1.z, projection_temp.c1.w],
        [projection_temp.c2.x, projection_temp.c2.y, projection_temp.c2.z, projection_temp.c2.w],
        [projection_temp.c3.x, projection_temp.c3.y, projection_temp.c3.z, projection_temp.c3.w]
    ];

    world.draw(&camera.camera_pos, view, projection, &mut target, &display, &program_block, model);
    skybox.draw(&mut target, &display, view, projection);

    target.finish().unwrap();

    *time_increment += 0.02;
    loop{
        if (stopwatch.elapsed_ms() as u64) < frame_time {
            world.render_loop();
        }else{

            break;
        }
    }
}

fn add(arr1: [f32; 3], arr2: [f32; 3]) -> [f32; 3] {
    let mut result: [f32; 3] = [0.0,0.0,0.0];

    result[0] = arr1[0] + arr2[0];
    result[1] = arr1[1] + arr2[1];
    result[2] = arr1[2] + arr2[2];

    result
}