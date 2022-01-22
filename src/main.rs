#[macro_use]
extern crate glium;

use std::io::Cursor;

#[allow(unused_imports)]
use glium::{glutin, Surface};
// extern crate gl;
// extern crate sdl2;
// extern crate glm;
extern crate stopwatch;
pub mod render_gl;
pub mod world;
pub mod player;
pub mod skybox; 
use std::ffi::CString;

// #![allow(dead_code)]
// use std::time::{Duration, Instant};
// use glium::glutin::event_loop::{EventLoop, ControlFlow};
// use glium::glutin::event::{Event, StartCause};
use glium::glutin::event::VirtualKeyCode;

//$Env:RUST_BACKTRACE=1
fn main() {
    //Settings
    const WINDOW_WIDTH: u32 = 1280;
    const WINDOW_HEIGHT: u32 = 720;
    
    const SQUARE_CHUNK_WIDTH: usize = 10;           //Values can be: 4,6,10,16,22,28
    const CHUNKS_LAYERS_FROM_PLAYER: usize = 31;    //Odd numbers ONLYYY
    const PLAYER_HEIGHT: f32 = 1.5;

    const WORLD_GEN_SEED: u32 = 60;                 //Any number
    const MID_HEIGHT: u8 = 50;                   //The terrain variation part
    const SKY_HEIGHT: u8 = 0;                   //Works as a buffer for the mid heigt needs to be at least 20 percent of mid size
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

    // #[derive(Copy, Clone)]
    // struct Vertex {
    //     position: [f32; 3],
    //     tex_coords: [f32; 2],
    //     opacity: f32,
    // }

    // implement_vertex!(Vertex, position, tex_coords, opacity);

    // // SIDE PX
    // let vertex_px1 = Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 0.0], opacity: 0.1};
    // let vertex_px2 = Vertex { position: [ 0.5,  0.5, -0.5], tex_coords: [0.0, 1.0], opacity: 0.1};
    // let vertex_px3 = Vertex { position: [ 0.5, -0.5, -0.5], tex_coords: [1.0, 0.0], opacity: 0.1};
    
    // let shape1 = vec![vertex_px1, vertex_px2, vertex_px3];
    // let vertex_buffer1 = glium::VertexBuffer::new(&display, &shape1).unwrap();
    // let indices1 = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // let vertex_px4 = Vertex { position: [0.5,  0.5, -0.5], tex_coords: [0.0, 0.0], opacity: 0.1};
    // let vertex_px5 = Vertex { position: [ 0.5,  0.5, -0.5], tex_coords: [0.0, 1.0], opacity: 0.1};
    // let vertex_px6 = Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [1.0, 0.0], opacity: 0.1};

    // let shape2 = vec![vertex_px4, vertex_px5, vertex_px6];
    // let vertex_buffer2 = glium::VertexBuffer::new(&display, &shape2).unwrap();
    // let indices2 = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);


    // // SIDE NX
    // let vertex_nx1 = Vertex { position: [-0.5, -0.5,  0.5], tex_coords: [0.0, 0.0], opacity: 0.1};
    // let vertex_nx2 = Vertex { position: [ 0.5, -0.5,  0.5], tex_coords: [0.0, 1.0], opacity: 0.1};
    // let vertex_nx3 = Vertex { position: [ 0.5,  0.5,  0.5], tex_coords: [1.0, 0.0], opacity: 0.1};
    
    // let shape3 = vec![vertex_nx1, vertex_nx2, vertex_nx3];
    // let vertex_buffer3 = glium::VertexBuffer::new(&display, &shape3).unwrap();
    // let indices3 = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // let vertex_nx4 = Vertex { position: [0.5,  0.5,  0.5], tex_coords: [0.0, 0.0], opacity: 0.1};
    // let vertex_nx5 = Vertex { position: [-0.5,  0.5,  0.5], tex_coords: [0.0, 1.0], opacity: 0.1};
    // let vertex_nx6 = Vertex { position: [-0.5, -0.5,  0.5], tex_coords: [1.0, 0.0], opacity: 0.1};

    // let shape4 = vec![vertex_nx4, vertex_nx5, vertex_nx6];
    // let vertex_buffer4 = glium::VertexBuffer::new(&display, &shape4).unwrap();
    // let indices4 = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);


    // // SIDE PY
    // let vertex_py1 = Vertex { position: [-0.5,  0.5,  0.5], tex_coords: [0.0, 0.0], opacity: 0.1};
    // let vertex_py2 = Vertex { position: [-0.5,  0.5, -0.5], tex_coords: [0.0, 1.0], opacity: 0.1};
    // let vertex_py3 = Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [1.0, 0.0], opacity: 0.1};
    
    // let shape5 = vec![vertex_py1, vertex_py2, vertex_py3];
    // let vertex_buffer5 = glium::VertexBuffer::new(&display, &shape5).unwrap();
    // let indices5 = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // let vertex_py4 = Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 0.0], opacity: 0.1};
    // let vertex_py5 = Vertex { position: [-0.5, -0.5,  0.5], tex_coords: [0.0, 1.0], opacity: 0.1};
    // let vertex_py6 = Vertex { position: [-0.5,  0.5,  0.5], tex_coords: [1.0, 0.0], opacity: 0.1};

    // let shape6 = vec![vertex_py4, vertex_py5, vertex_py6];
    // let vertex_buffer6 = glium::VertexBuffer::new(&display, &shape6).unwrap();
    // let indices6 = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);


    // // SIDE NY
    // let vertex_ny1 = Vertex { position: [0.5,  0.5,  0.5], tex_coords: [0.0, 0.0], opacity: 0.1};
    // let vertex_ny2 = Vertex { position: [0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], opacity: 0.1};
    // let vertex_ny3 = Vertex { position: [0.5,  0.5, -0.5], tex_coords: [1.0, 0.0], opacity: 0.1};
    
    // let shape7 = vec![vertex_ny1, vertex_ny2, vertex_ny3];
    // let vertex_buffer7 = glium::VertexBuffer::new(&display, &shape7).unwrap();
    // let indices7 = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // let vertex_ny4 = Vertex { position: [0.5, -0.5, -0.5], tex_coords: [0.0, 0.0], opacity: 0.1};
    // let vertex_ny5 = Vertex { position: [0.5,  0.5,  0.5], tex_coords: [0.0, 1.0], opacity: 0.1};
    // let vertex_ny6 = Vertex { position: [0.5, -0.5,  0.5], tex_coords: [1.0, 0.0], opacity: 0.1};

    // let shape8 = vec![vertex_ny4, vertex_ny5, vertex_ny6];
    // let vertex_buffer8 = glium::VertexBuffer::new(&display, &shape8).unwrap();
    // let indices8 = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

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
            gl_Position = perspective * modelView * vec4(position, 0.01);
        }
    "#;

    let fragment_shader_block = r#"
        #version 140

        in vec2 v_tex_coords;
        in float v_opacity;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords) * vec4(1.0, 1.0, 1.0, v_opacity);
        }
    "#;

    let vertex_shader_skybox = r#"
        #version 140

        in vec3 position;

        out vec3 v_tex_coords;

        uniform mat4 projection;
        uniform mat4 view;

        void main() {
            v_tex_coords = position;
            vec4 pos = projection * view * vec4(position, 1.0)
            gl_Position = pos.xyww;
        }
    "#;

    let fragment_shader_skybox = r#"
        #version 140

        in vec3 v_tex_coords;

        out vec4 color;

        uniform samplerCube skybox;

        void main() {
            color = texture(skybox, v_tex_coords);
        }
    "#;

    let program_block = glium::Program::from_source(&display, vertex_shader_block, fragment_shader_block, None).unwrap();
    let program_skybox = glium::Program::from_source(&display, vertex_shader_skybox, fragment_shader_skybox, None).unwrap();

    let mut time_increment: f32 = 0.0;
    let camera_pos = [0.0, 0.0, 0.0];
    let mut world = world::World::new(
        &camera_pos, 
        program_block, 
        &SQUARE_CHUNK_WIDTH, 
        &CHUNKS_LAYERS_FROM_PLAYER, 
        &WORLD_GEN_SEED,
        &MID_HEIGHT,
        &UNDERGROUND_HEIGHT,
        &SKY_HEIGHT,
    );

    let mut player: player::Player = player::Player::new(&mut world, PLAYER_HEIGHT, camera_pos);

    let skybox: skybox::Skybox = skybox::Skybox::new(program_skybox, &display);

    const TIME_BETWEEN_FRAMES: u64 = 20;
    let mut stopwatch = stopwatch::Stopwatch::new();


    event_loop.run(move |event, _, control_flow| {
        if let glutin::event::Event::WindowEvent {
            event: window_event,
            ..
        } = event
        {
            match window_event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::KeyboardInput {
                    input:
                        glutin::event::KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::K),
                            ..
                        },
                    ..
                } => println!("Pressed K"),
                glutin::event::WindowEvent::KeyboardInput {
                    input:
                        glutin::event::KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::W),
                            ..
                        },
                    ..
                } => println!("Pressed W"),
                glutin::event::WindowEvent::KeyboardInput {
                    input:
                        glutin::event::KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::S),
                            ..
                        },
                    ..
                } => println!("Pressed s"),
                _ => (),
            }
        }

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);



        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let view = view_matrix(&[2.0, -1.0, 1.0], &[-2.0, 1.0, 1.0], &[0.0, 1.0, 0.0]);

        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ]
        };

        let model = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 2.0, 1.0f32]
        ];

        let uniforms = uniform! {
            tex: &texture,
            model: model,
            view: view,
            perspective: perspective

        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };
        

        // target.draw(&vertex_buffer1, &indices1, &program, &uniforms, &params).unwrap();
        

        // target.draw(&vertex_buffer1, &indices1, &program_block, &uniforms, &params).unwrap();
        // target.draw(&vertex_buffer2, &indices2, &program_block, &uniforms, &params).unwrap();
        // target.draw(&vertex_buffer3, &indices3, &program_block, &uniforms, &params).unwrap();
        // target.draw(&vertex_buffer4, &indices4, &program_block, &uniforms, &params).unwrap();
        // target.draw(&vertex_buffer5, &indices5, &program_block, &uniforms, &params).unwrap();
        // target.draw(&vertex_buffer6, &indices6, &program_block, &uniforms, &params).unwrap();
        // target.draw(&vertex_buffer7, &indices7, &program_block, &uniforms, &params).unwrap();
        // target.draw(&vertex_buffer8, &indices8, &program_block, &uniforms, &params).unwrap();

        // target.draw(&vertex_buffer2, &indices2, &program, &uniforms, &params).unwrap();
        skybox.draw();
        target.finish().unwrap();

    });

    // let _gl_context = window.gl_create_context().unwrap();
    // let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    //Set mouse to be bound in the window and infinite movement
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
    // let mut event_pump = sdl.event_pump().unwrap();
    // let mut stopwatch = stopwatch::Stopwatch::new();

    // 'main: loop {
    //     stopwatch.reset();
    //     stopwatch.start();

    //     let close_game: bool = player.handle_events(&mut world, &mut event_pump);
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

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}
