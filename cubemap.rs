#[macro_use]
extern crate glium;
extern crate image;

use std::io::Cursor;
use glium::{DisplayBuild, Surface};
use glium::glutin;
use glium::index::PrimitiveType;

mod camera;

fn main() {
    let display = glutin::WindowBuilder::new()
        .with_vsync()
        .with_depth_buffer(24)
        .with_dimensions(800, 600)
	.with_title(format!("Glium CubeMap"))
        .build_glium()
        .unwrap();


    let image = image::load(Cursor::new(&include_bytes!("images/posx512.jpg")[..]),
                        image::JPEG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
    let tex_posx = glium::Texture2d::new(&display, image).unwrap();

    let image = image::load(Cursor::new(&include_bytes!("images/negx512.jpg")[..]),
                        image::JPEG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
    let tex_negx = glium::Texture2d::new(&display, image).unwrap();

    let image = image::load(Cursor::new(&include_bytes!("images/posy512.jpg")[..]),
                        image::JPEG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
    let tex_posy = glium::Texture2d::new(&display, image).unwrap();

    let image = image::load(Cursor::new(&include_bytes!("images/negy512.jpg")[..]),
                        image::JPEG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
    let tex_negy = glium::Texture2d::new(&display, image).unwrap();

    let image = image::load(Cursor::new(&include_bytes!("images/posz512.jpg")[..]),
                        image::JPEG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
    let tex_posz = glium::Texture2d::new(&display, image).unwrap();

    let image = image::load(Cursor::new(&include_bytes!("images/negz512.jpg")[..]),
                        image::JPEG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
    let tex_negz = glium::Texture2d::new(&display, image).unwrap();

    let cubemap = glium::texture::Cubemap::empty(&display, 512).unwrap();

    // skybox
    let skybox_vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 3],
        }

        implement_vertex!(Vertex, position);

        let side2: f32 = 50.0 / 2.0;

        glium::VertexBuffer::new(&display,
            &[
                // Front
    		Vertex { position: [-side2, -side2,  side2] },
    		Vertex { position: [ side2, -side2,  side2] },
    		Vertex { position: [ side2,  side2,  side2] },
                Vertex { position: [-side2,  side2,  side2] },
    		// Right
    		Vertex { position: [ side2, -side2,  side2] },
    		Vertex { position: [ side2, -side2, -side2] },
    		Vertex { position: [ side2,  side2, -side2] },
                Vertex { position: [ side2,  side2,  side2] },
    		// Back
    		Vertex { position: [-side2, -side2, -side2] },
    		Vertex { position: [-side2,  side2, -side2] },
    		Vertex { position: [ side2,  side2, -side2] },
                Vertex { position: [ side2, -side2, -side2] },
    		// Left
    		Vertex { position: [-side2, -side2,  side2] },
    		Vertex { position: [-side2,  side2,  side2] },
                Vertex { position: [-side2,  side2, -side2] },
                Vertex { position: [-side2, -side2, -side2] },
                // Bottom
    		Vertex { position: [-side2, -side2,  side2] },
    		Vertex { position: [-side2, -side2, -side2] },
    		Vertex { position: [ side2, -side2, -side2] },
                Vertex { position: [ side2, -side2,  side2] },
    		// Top
                Vertex { position: [-side2,  side2,  side2] },
    		Vertex { position: [ side2,  side2,  side2] },
    		Vertex { position: [ side2,  side2, -side2] },
                Vertex { position: [-side2,  side2, -side2] },
    	    ]
        ).unwrap()
    };

    let skybox_index_buffer = glium::IndexBuffer::new(&display,
            glium::index::PrimitiveType::TrianglesList,
            &[
                // Front
                0u16, 2, 1, 0, 3, 2,
                // Right
                4, 6, 5, 4, 7, 6,
                // Back
                8, 10, 9, 8, 11, 10,
                // Left
                12, 14, 13, 12, 15, 14,
                // Bottom
                16, 18, 17, 16, 19, 18,
                // Top
                20, 22, 21, 20, 23, 22,
            ]).unwrap();

    let skybox_program = glium::Program::from_source(&display,
        " #version 140

            in vec3 position;
            out vec3 ReflectDir;

            uniform mat4 model;
            uniform mat4 view;
            uniform mat4 perspective;

            void main() {
                ReflectDir = position;
                gl_Position = perspective * view * model * vec4(position, 1.0);
            }
        ",
        " #version 140
            in vec3 ReflectDir;
            out vec4 color;

            uniform samplerCube cubetex;

            void main() {
                color = texture(cubetex, ReflectDir);
            }
        ",
        None).unwrap();

    //model
    let model_vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 3],
            normal:  [f32; 3],
        }

        implement_vertex!(Vertex, position, normal);

        let side2: f32 = 2.0 / 2.0;

        glium::VertexBuffer::new(&display,
            &[
                // Front
    		Vertex { position: [-side2, -side2,  side2], normal: [ 0.0,  0.0,  1.0] },
    		Vertex { position: [ side2, -side2,  side2], normal: [ 0.0,  0.0,  1.0] },
    		Vertex { position: [ side2,  side2,  side2], normal: [ 0.0,  0.0,  1.0] },
                Vertex { position: [-side2,  side2,  side2], normal: [ 0.0,  0.0,  1.0] },
    		// Right
    		Vertex { position: [ side2, -side2,  side2], normal: [ 1.0,  0.0,  0.0] },
    		Vertex { position: [ side2, -side2, -side2], normal: [ 1.0,  0.0,  0.0] },
    		Vertex { position: [ side2,  side2, -side2], normal: [ 1.0,  0.0,  0.0] },
                Vertex { position: [ side2,  side2,  side2], normal: [ 1.0,  0.0,  0.0] },
    		// Back
    		Vertex { position: [-side2, -side2, -side2], normal: [ 0.0,  0.0, -1.0] },
    		Vertex { position: [-side2,  side2, -side2], normal: [ 0.0,  0.0, -1.0] },
    		Vertex { position: [ side2,  side2, -side2], normal: [ 0.0,  0.0, -1.0] },
                Vertex { position: [ side2, -side2, -side2], normal: [ 0.0,  0.0, -1.0] },
    		// Left
    		Vertex { position: [-side2, -side2,  side2], normal: [-1.0,  0.0,  0.0] },
    		Vertex { position: [-side2,  side2,  side2], normal: [-1.0,  0.0,  0.0] },
                Vertex { position: [-side2,  side2, -side2], normal: [-1.0,  0.0,  0.0] },
                Vertex { position: [-side2, -side2, -side2], normal: [-1.0,  0.0,  0.0] },
                // Bottom
    		Vertex { position: [-side2, -side2,  side2], normal: [ 0.0, -1.0,  0.0] },
    		Vertex { position: [-side2, -side2, -side2], normal: [ 0.0, -1.0,  0.0] },
    		Vertex { position: [ side2, -side2, -side2], normal: [ 0.0, -1.0,  0.0] },
                Vertex { position: [ side2, -side2,  side2], normal: [ 0.0, -1.0,  0.0] },
    		// Top
                Vertex { position: [-side2,  side2,  side2], normal: [ 0.0,  1.0,  0.0] },
    		Vertex { position: [ side2,  side2,  side2], normal: [ 0.0,  1.0,  0.0] },
    		Vertex { position: [ side2,  side2, -side2], normal: [ 0.0,  1.0,  0.0] },
                Vertex { position: [-side2,  side2, -side2], normal: [ 0.0,  1.0,  0.0] },
    		]
        ).unwrap()
    };

    let model_index_buffer = glium::IndexBuffer::new(&display,
            glium::index::PrimitiveType::TrianglesList,
            &[
                // Front
                0u16, 2, 1, 0, 3, 2,
                // Right
                4, 6, 5, 4, 7, 6,
                // Back
                8, 10, 9, 8, 11, 10,
                // Left
                12, 14, 13, 12, 15, 14,
                // Bottom
                16, 18, 17, 16, 19, 18,
                // Top
                20, 22, 21, 20, 23, 22,
            ]).unwrap();

    let model_program = glium::Program::from_source(&display,
        " #version 140

            in vec3 position;
            in vec3 normal;
            out vec4 v_position;
            out vec3 v_normal;

            uniform mat4 model;
            uniform mat4 view;
            uniform mat4 perspective;

            void main() {
                mat4 modelviewMatrix = view * model;
                mat3 normalMatrix = mat3(modelviewMatrix);

                v_position = modelviewMatrix * vec4(position, 1.0);
                v_normal = normalMatrix * normal;
                gl_Position = perspective * v_position;
            }
        ",
        " #version 140
            in vec4 v_position;
            in vec3 v_normal;
            out vec4 f_color;

            uniform samplerCube cubetex;
            uniform float ReflectFactor;
            uniform vec4 MaterialColor;
            uniform vec3 WorldCameraPosition;

            void main() {
                vec3 s = normalize(v_normal);
                vec3 v = normalize(WorldCameraPosition - v_position.xyz);
                vec3 ReflectDir = reflect(v, s);
                vec4 cubeMapColor = texture(cubetex, ReflectDir);
                f_color = mix(MaterialColor, cubeMapColor, ReflectFactor);
            }
        ",
        None).unwrap();

    let dest_rect1 = glium::BlitTarget {
        left: 0,
        bottom: 0,
        width: 512,
        height: 512,
    };

    let mut camera = camera::CameraState::new();

    let scale: f32 = 1.0;
    let scale2: f32 = 1.0;
    let mut t: f32 = 0.0;

    // main loop
    loop {
	t += 0.002;

        let  framebuffer1 = glium::framebuffer::SimpleFrameBuffer::new(&display,
                        cubemap.main_level().image(glium::texture::CubeLayer::PositiveX)).unwrap();
        let  framebuffer2 = glium::framebuffer::SimpleFrameBuffer::new(&display,
                        cubemap.main_level().image(glium::texture::CubeLayer::NegativeX)).unwrap();
        let  framebuffer3 = glium::framebuffer::SimpleFrameBuffer::new(&display,
                        cubemap.main_level().image(glium::texture::CubeLayer::PositiveY)).unwrap();
        let  framebuffer4 = glium::framebuffer::SimpleFrameBuffer::new(&display,
                        cubemap.main_level().image(glium::texture::CubeLayer::NegativeY)).unwrap();
        let  framebuffer5 = glium::framebuffer::SimpleFrameBuffer::new(&display,
                        cubemap.main_level().image(glium::texture::CubeLayer::PositiveZ)).unwrap();
        let  framebuffer6 = glium::framebuffer::SimpleFrameBuffer::new(&display,
                        cubemap.main_level().image(glium::texture::CubeLayer::NegativeZ)).unwrap();

        tex_posx.as_surface().blit_whole_color_to(&framebuffer1, &dest_rect1,
                        glium::uniforms::MagnifySamplerFilter::Linear);
        tex_negx.as_surface().blit_whole_color_to(&framebuffer2, &dest_rect1,
                        glium::uniforms::MagnifySamplerFilter::Linear);
        tex_negy.as_surface().blit_whole_color_to(&framebuffer3, &dest_rect1,
                        glium::uniforms::MagnifySamplerFilter::Linear);
        tex_posy.as_surface().blit_whole_color_to(&framebuffer4, &dest_rect1,
                        glium::uniforms::MagnifySamplerFilter::Linear);
        tex_posz.as_surface().blit_whole_color_to(&framebuffer5, &dest_rect1,
                        glium::uniforms::MagnifySamplerFilter::Linear);
        tex_negz.as_surface().blit_whole_color_to(&framebuffer6, &dest_rect1,
                        glium::uniforms::MagnifySamplerFilter::Linear);

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let model = [
        	[ t.cos()*scale,      0.0 , t.sin()*scale, 0.0],
        	[          0.0 , 1.0*scale,           0.0, 0.0],
        	[-t.sin()*scale,      0.0 , t.cos()*scale, 0.0],
        	[           0.0,      0.0 ,           0.0, 1.0f32],
        ];

        let camera_position: [f32; 3]= [0.0, 0.0, -8.0];
        camera.set_position((0.0, 0.0, -8.0));
        camera.set_direction((0.0, 0.0, 1.0));
        let view = camera.get_view();
        let perspective = camera.get_perspective();

        let material_color: [f32; 4] = [0.9, 0.9, 0.9, 1.0];
        let reflect_factor: f32 = 0.9;

        let skybox_uniforms = uniform! {
             model: model,
             view: view,
             perspective: perspective,
	     cubetex: cubemap.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
        };

        let model_uniforms = uniform! {
    	     model: [
                  [ t.cos()*scale2,       0.0 , t.sin()*scale2, 0.0],
                  [            0.0, 1.0*scale2,            0.0, 0.0],
                  [-t.sin()*scale2,       0.0 , t.cos()*scale2, 0.0],
                  [            0.0,       0.0 ,            0.0, 1.0f32]
             ],
             view: view,
             perspective: perspective,
             cubetex: cubemap.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
             ReflectFactor: reflect_factor,
             MaterialColor: material_color,
             WorldCameraPosition: camera_position,
    	};

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        target.draw(&skybox_vertex_buffer, &skybox_index_buffer, &skybox_program,
                    &skybox_uniforms, &params).unwrap();
        target.draw(&model_vertex_buffer, &model_index_buffer, &model_program,
                    &model_uniforms, &params).unwrap();

        target.finish().unwrap();

        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}