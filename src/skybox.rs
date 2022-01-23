

extern crate glium;
use std::io::Cursor;
use glium::Surface;
pub struct Skybox {
    program: glium::Program,
    skybox_vao: glium::VertexBuffer<Vertex>,
    skybox_indices: glium::IndexBuffer<u16>,
    tex_posx: glium::Texture2d,
    tex_negx: glium::Texture2d,
    tex_posy: glium::Texture2d,
    tex_negy: glium::Texture2d,
    tex_posz: glium::Texture2d,
    tex_negz: glium::Texture2d,
    cubemap: glium::texture::Cubemap,
    dest_rect1: glium::BlitTarget
}
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}

implement_vertex!(Vertex, position);

impl Skybox{
    pub fn new(display: &glium::Display) -> Skybox{

        let image = image::load(Cursor::new(&include_bytes!("../resources/sides.png")),image::ImageFormat::Png).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let tex_posx = glium::Texture2d::new(display, image).unwrap();

        let image = image::load(Cursor::new(&include_bytes!("../resources/sides.png")),image::ImageFormat::Png).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let tex_negx = glium::Texture2d::new(display, image).unwrap();

        let image = image::load(Cursor::new(&include_bytes!("../resources/posy.png")),image::ImageFormat::Png).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let tex_posy = glium::Texture2d::new(display, image).unwrap();

        let image = image::load(Cursor::new(&include_bytes!("../resources/negy.png")),image::ImageFormat::Png).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let tex_negy = glium::Texture2d::new(display, image).unwrap();

        let image = image::load(Cursor::new(&include_bytes!("../resources/sides.png")),image::ImageFormat::Png).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let tex_posz = glium::Texture2d::new(display, image).unwrap();

        let image = image::load(Cursor::new(&include_bytes!("../resources/sides.png")),image::ImageFormat::Png).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let tex_negz = glium::Texture2d::new(display, image).unwrap();

        let cubemap = glium::texture::Cubemap::empty(display, 512).unwrap();

        // skybox
        let skybox_vertex_buffer = {
            

            let side2: f32 = 50.0 / 2.0;

            glium::VertexBuffer::new(display,
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

        let skybox_index_buffer = glium::IndexBuffer::new(display,
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

        let skybox_program = glium::Program::from_source(display,
            " #version 140

                in vec3 position;
                out vec3 ReflectDir;

                uniform mat4 model;
                uniform mat4 view;
                uniform mat4 perspective;

                void main() {
                    ReflectDir = position;
                    gl_Position = perspective * view * vec4(position, 0.1);
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


        let dest_rect1 = glium::BlitTarget {
            left: 0,
            bottom: 0,
            width: 512,
            height: 512,
        };

        return Skybox{
            program: skybox_program,
            skybox_vao: skybox_vertex_buffer,
            skybox_indices: skybox_index_buffer,
            tex_posx: tex_posx,
            tex_negx: tex_negx,
            tex_posy: tex_posy,
            tex_negy: tex_negy,
            tex_posz: tex_posz,
            tex_negz: tex_negz,
            cubemap: cubemap,
            dest_rect1: dest_rect1
        };
    }

    pub fn draw(&self, target: &mut glium::Frame, display: &glium::Display, view: [[f32; 4]; 4], perspective: [[f32; 4]; 4]){
        let  framebuffer1 = glium::framebuffer::SimpleFrameBuffer::new(display,
            self.cubemap.main_level().image(glium::texture::CubeLayer::PositiveX)).unwrap();
        let  framebuffer2 = glium::framebuffer::SimpleFrameBuffer::new(display,
            self.cubemap.main_level().image(glium::texture::CubeLayer::NegativeX)).unwrap();
        let  framebuffer3 = glium::framebuffer::SimpleFrameBuffer::new(display,
            self.cubemap.main_level().image(glium::texture::CubeLayer::PositiveY)).unwrap();
        let  framebuffer4 = glium::framebuffer::SimpleFrameBuffer::new(display,
            self.cubemap.main_level().image(glium::texture::CubeLayer::NegativeY)).unwrap();
        let  framebuffer5 = glium::framebuffer::SimpleFrameBuffer::new(display,
            self.cubemap.main_level().image(glium::texture::CubeLayer::PositiveZ)).unwrap();
        let  framebuffer6 = glium::framebuffer::SimpleFrameBuffer::new(display,
            self.cubemap.main_level().image(glium::texture::CubeLayer::NegativeZ)).unwrap();

        self.tex_posx.as_surface().blit_whole_color_to(&framebuffer1, &self.dest_rect1,
                    glium::uniforms::MagnifySamplerFilter::Linear);
        self.tex_negx.as_surface().blit_whole_color_to(&framebuffer2, &self.dest_rect1,
                    glium::uniforms::MagnifySamplerFilter::Linear);
        self.tex_negy.as_surface().blit_whole_color_to(&framebuffer3, &self.dest_rect1,
                    glium::uniforms::MagnifySamplerFilter::Linear);
        self.tex_posy.as_surface().blit_whole_color_to(&framebuffer4, &self.dest_rect1,
                    glium::uniforms::MagnifySamplerFilter::Linear);
        self.tex_posz.as_surface().blit_whole_color_to(&framebuffer5, &self.dest_rect1,
                    glium::uniforms::MagnifySamplerFilter::Linear);
        self.tex_negz.as_surface().blit_whole_color_to(&framebuffer6, &self.dest_rect1,
                    glium::uniforms::MagnifySamplerFilter::Linear);

        

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        let skybox_uniforms = uniform! {
            view: view,
            perspective: perspective,
            cubetex: self.cubemap.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
       };

        // target.draw(&self.skybox_vao, &self.skybox_indices, &self.program,
        //     &skybox_uniforms, &params).unwrap();

        target.draw(&self.skybox_vao, &self.skybox_indices, &self.program,
            &skybox_uniforms, &params).unwrap();
        // target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
        //         &Default::default()).unwrap();
    }
}