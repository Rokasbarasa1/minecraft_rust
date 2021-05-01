extern crate gl;
extern crate sdl2;
extern crate stb_image;
extern crate image;
extern crate serde_json;
extern crate 
pub mod render_gl;


//VERTEX SHADER
// #version 330 core
// layout (location = 0) in vec3 aPos;
// layout (location = 1) in vec3 aColor;
// layout (location = 2) in vec2 aTexCoord;

// out vec3 ourColor;
// out vec2 TexCoord;

// void main()
// {
//     gl_Position = vec4(aPos, 1.0);
//     ourColor = aColor;
//     TexCoord = aTexCoord;
// }




//FRAGMENT SHADER
// #version 330 core

// out vec4 FragColor;

// in vec3 ourColor;
// in vec2 TexCoord;

// uniform sampler2D ourTexture;

// void main()
// {
//     FragColor = texture(ourTexture, TexCoord);
// }
use std::{ffi::c_void, str};

// use self::gl::types::*;
fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // set up shader program

    use std::ffi::CString;
    let vert_shader = render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap()).unwrap();

    let frag_shader = render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap()).unwrap();

    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    // set up vertex buffer object

    let vertices: Vec<f32> = vec![
        // positions      // colors   //TEXTURE
         0.5, -0.5, 0.0,  1.0, 0.0, 0.0,  1.0, 0.0,// bottom right
        -0.5, -0.5, 0.0,  0.0, 1.0, 0.0,  0.0, 0.0,// bottom left
         0.5,  0.5, 0.0,  0.0, 0.0, 1.0,  1.0, 1.0,// top right
        -0.5,  0.5, 0.0,  0.0, 0.0, 1.0,  0.0, 1.0 // top left
    ];

    let indices = [
        0, 1, 2, // first Triangle
        2, 1, 3 // second Triangle
    ];

    let mut vbo: gl::types::GLuint = 0;
    let mut EBO: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,                                                       // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,                               // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        
    }


    // set up vertex array object

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    let mut texture: gl::types::GLuint = 0;
    
    unsafe {
        gl::GenBuffers(1, &mut EBO);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (8 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(),                                     // offset of the first component
        );
        gl::EnableVertexAttribArray(1); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            1,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (8 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid, // offset of the first component
        );

        gl::EnableVertexAttribArray(2); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            2,         // index of the generic vertex attribute ("layout (location = 0)")
            2,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (8 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            (6 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid, // offset of the first component
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
            &indices[0] as *const i32 as *const std::os::raw::c_void,
            gl::STATIC_DRAW,
        );

        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPLACE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT  as i32);

        gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MIN_FILTER, gl::LINEAR  as i32);
        gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MAG_FILTER, gl::LINEAR  as i32);

        let data = image::open(&std::path::Path::new("C:\\Users\\Rokas\\Desktop\\rust minecraft\\minecraft_rust\\texture.png")).unwrap().into_rgb();
        let (width ,height) = data.dimensions();
        
        let img_data = data.into_raw();
        let img_ptr: *const c_void = img_data.as_ptr() as *const c_void;
        
        gl::TexImage2D(
            gl::TEXTURE_2D, 
            0, 
            gl::RGB as i32, 
            width as i32, 
            height as i32, 
            0, 
            gl::RGB, 
            gl::UNSIGNED_BYTE, 
            img_ptr
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    // set up shared state for window

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    // main loop

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // draw triangle

        shader_program.set_used();
        unsafe {
            
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            // gl::DrawArrays(
            //     gl::TRIANGLES, // mode
            //     0,             // starting index in the enabled arrays
            //     6,             // number of indices to be rendered
            // );
            gl::BindVertexArray(0);
        }

        window.gl_swap_window();
    }
}











// extern crate gl;
// extern crate sdl2;

// pub mod render_gl;

// use sdl2::sys::Sint64;

// use self::gl::types::*;

// use std::ffi::CString;
// use std::mem;
// use std::os::raw::c_void;
// use std::ptr;
// use std::str;
// use std::sync::mpsc::Receiver;



// fn main() {
//     //Setup window library
//     let sdl = sdl2::init().unwrap();
//     let video_subsystem = sdl.video().unwrap();

//     //Get gl attribute to initialize gl versions
//     let gl_attr = video_subsystem.gl_attr();

//     gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
//     gl_attr.set_context_version(4, 1);

//     //Init window
//     let window = video_subsystem
//         .window("Game", 900, 700)
//         .opengl()
//         .resizable()
//         .build()
//         .unwrap();

//     //Somewhere this used
//     let _gl_context = window.gl_create_context().unwrap();
//     //Load gl functions
//     let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

//     //Initializing the fragment and vertex shader.
//     use std::ffi::CString;
//     let vert_shader = render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap()).unwrap();

//     let frag_shader = render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap()).unwrap();

//     let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

//     let vert_shader2 = render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap()).unwrap();

//     let frag_shader2 = render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle2.frag")).unwrap()).unwrap();

//     let shader_program2 = render_gl::Program::from_shaders(&[vert_shader2, frag_shader2]).unwrap();

//     // set up vertex buffer object

//     let vertices: [f32; 12] = [
//          0.5,  0.5, 0.0,  // top right
//          0.5, -0.5, 0.0, // bottom right
//         -0.5, -0.5, 0.0, // bottom left
//         -0.5,  0.5, 0.0,  // top left
//     ];

//     let vertices2: [f32; 9] = [
//          0.9,  0.9, 0.0,  // top right
//          0.9, -0.0, 0.0, // bottom right
//          0.6,  0.0, 0.0, // bottom left
//     ];
//     let indices = [
//         0, 1, 3, // first Triangle
//         1, 2, 3, // second Triangle
//         4, 5, 6
//     ];

//     let mut EBO: gl::types::GLuint = 0;
//     let mut VBO: gl::types::GLuint = 0;
//     let mut VAO: gl::types::GLuint = 0;
//     let mut VBO2: gl::types::GLuint = 0;
//     let mut VAO2: gl::types::GLuint = 0;
//     unsafe {
//         //binding space on the video card
//         gl::GenVertexArrays(1, &mut VAO);
//         gl::GenVertexArrays(1, &mut VAO2);
//         gl::GenBuffers(1, &mut VBO2);  
//         gl::GenBuffers(1, &mut VBO);  
//         gl::GenBuffers(1, &mut EBO);
//         gl::BindVertexArray(VAO);

//         //VbO SPECIFIES DATA
//         //We bind to the buffer and we upload the data
//         gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
//         gl::BufferData(
//             gl::ARRAY_BUFFER, 
//             (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, 
//             vertices.as_ptr() as *const gl::types::GLvoid, 
//             gl::STATIC_DRAW, 
//         );
//         gl::BindBuffer(gl::ARRAY_BUFFER, 0); //Unbind the buffer


        
//         gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
//         gl::BufferData(
//             gl::ELEMENT_ARRAY_BUFFER,
//             (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
//             &indices[0] as *const i32 as *const c_void,
//             gl::STATIC_DRAW,
//         );
            



        
        



//         //VAO SPECIFIES LAYOUT OF DATA
//         gl::BindBuffer(gl::ARRAY_BUFFER, VBO); // We bind VBO to create relationship with VAO
//          // this is "layout (location = 0)" in vertex shader
//         gl::VertexAttribPointer(
//             0,         // index of the generic vertex attribute ("layout (location = 0)")
//             3,         // the number of components per generic vertex attribute
//             gl::FLOAT, // data type
//             gl::FALSE, // normalized (int-to-float conversion)
//             (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
//             std::ptr::null(),                                     // offset of the first component
//         );
//         gl::EnableVertexAttribArray(0); // WHICH LAYOUT WE CHOOSE

//         //UNBIND BOTH
//         gl::BindBuffer(gl::ARRAY_BUFFER, 0);
//         gl::BindVertexArray(0);




//         //CREATING ANOTHER SET OF binds
//         gl::BindBuffer(gl::ARRAY_BUFFER, VBO2);
//         gl::BufferData(
//             gl::ARRAY_BUFFER, 
//             (vertices2.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, 
//             vertices2.as_ptr() as *const gl::types::GLvoid, 
//             gl::STATIC_DRAW, 
//         );
//         gl::BindBuffer(gl::ARRAY_BUFFER, 0); //Unbind the buffer
        
//         gl::BindVertexArray(VAO2);
//         gl::BindBuffer(gl::ARRAY_BUFFER, VBO2); // We bind VBO to create relationship with VAO
//         gl::VertexAttribPointer(
//             0,         // index of the generic vertex attribute ("layout (location = 0)")
//             3,         // the number of components per generic vertex attribute
//             gl::FLOAT, // data type
//             gl::FALSE, // normalized (int-to-float conversion)
//             (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
//             std::ptr::null(),                                     // offset of the first component
//         );
//         gl::EnableVertexAttribArray(0); // WHICH LAYOUT WE CHOOSE

//     }

//     unsafe {
//         gl::Viewport(0, 0, 900, 700);
//         gl::ClearColor(0.3, 0.3, 0.5, 1.0);
//         gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
//     }

//     let mut event_pump = sdl.event_pump().unwrap();
//     'main: loop {
//         for event in event_pump.poll_iter() {
//             match event {
//                 sdl2::event::Event::Quit { .. } => break 'main,
//                 _ => {}
//             }
//         }

//         unsafe {
//             gl::Clear(gl::COLOR_BUFFER_BIT);
//         }

//         // draw triangle

//         shader_program.set_used();
//         unsafe {
//             gl::BindVertexArray(VAO);
//             //gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
//             //gl::DrawElements(gl::TRIANGLES, 9, gl::UNSIGNED_INT, ptr::null());
//             gl::DrawArrays(
//                 gl::TRIANGLES, // mode
//                 0,             // starting index in the enabled arrays
//                 3,             // number of indices to be rendered
//             );

            
//             shader_program2.set_used();
//             gl::BindVertexArray(VAO2);

//             gl::DrawArrays(
//                 gl::TRIANGLES, // mode
//                 0,             // starting index in the enabled arrays
//                 3,             // number of indices to be rendered
//             );

//             gl::BindVertexArray(0);
//         }

//         window.gl_swap_window();
//     }
// }


// //Old code that i made
// // unsafe {
// //         //VERTEX ARRAY ObJECT
// //         let mut VAO: u32= 0;
// //         gl::GenVertexArrays(1,&mut VAO);
// //         assert_ne!(VAO, 0);
// //         gl::BindVertexArray(VAO);
// //         println!("Made it PAST VAO");
// //         //CREATING memory space on gpu to store verteces
// //         let mut VBO: u32 = 0;
// //         gl::GenBuffers(1, &mut VBO);
// //         assert_ne!(VBO, 0);
// //         //We are binding this buffer object o a specific buffer type array_buffer so that it doesnt interfere with othe buffer types
// //         //All cals we make on ARRAY_nUFFER will talk to VBO
// //         gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
// //         //COPies vertex data into buffer.
// //         //GL STATIC DRAW Places the vertex in memory that is not changing and will stay drawed. 
// //         //GL dynamic draw lets us change it
// //         gl::BufferData(gl::ARRAY_BUFFER, std::mem::size_of_val(&vertices) as isize, vertices.as_ptr().cast(),gl::STATIC_DRAW);
        
// //         println!("Made it PAST CREATE VBO");



// //         //CREATE VETTEX SHADER
// //         //Creating a shader object
// //         let mut vertexShader: u32;
// //         //Shader of type vertex shader FIRST STEP SHADER
// //         vertexShader = gl::CreateShader(gl::VERTEX_SHADER);
        
// //         gl::ShaderSource(vertexShader, 1, &(vertexShaderSource.as_bytes().as_ptr().cast()), std::ptr::null());
// //         gl::CompileShader(vertexShader);

// //         //Check status
// //         let mut success: gl::types::GLint = 1;
// //         gl::GetShaderiv(vertexShader, gl::COMPILE_STATUS, &mut success);
// //         //CHeck if error happened
// //         if success == 0 {
// //             println!("Error in function create vertex shader source")
// //         }
// //         println!("Made it PAST CREATE VERTEX SHADER");




// //         //CREATE FRAGMENT SHADER
// //         let mut fragmentShader: u32;
// //         //Shader of type vertex shader FIRST STEP SHADER
// //         fragmentShader = gl::CreateShader(gl::FRAGMENT_SHADER);
        
// //         gl::ShaderSource(fragmentShader, 1, &(fragmentShaderSource.as_bytes().as_ptr().cast()), std::ptr::null());
// //         gl::CompileShader(fragmentShader);

// //         //Check status
// //         let mut success: gl::types::GLint = 1;
// //         gl::GetShaderiv(fragmentShader, gl::COMPILE_STATUS, &mut success);
// //         //CHeck if error happened
// //         if success == 0 {
// //             println!("Error in function create fragment shader source")
// //         }






// //         //CREATING THE SHADER PROGRAM that is gonna store our shaders
// //         let mut shaderProgram: u32;
// //         shaderProgram = gl::CreateProgram();

// //         //Linking the shaders to the program
// //         gl::AttachShader(shaderProgram, vertexShader);
// //         gl::AttachShader(shaderProgram, fragmentShader);
// //         gl::LinkProgram(shaderProgram);
// //         let mut success: gl::types::GLint = 1;
// //         gl::GetProgramiv(shaderProgram, gl::LINK_STATUS, &mut success);

// //         if success == 0 {
// //             println!("Error in function create fragment shader source")
// //         }
// //         gl::UseProgram(shaderProgram);

// //         gl::DeleteShader(vertexShader);
// //         gl::DeleteShader(fragmentShader);
        
// //         //Setting vertex type (index), dimensions values(3), type of values (float), rounding to int(false), stride - how much does one shader take in array, and starting index as void
// //         gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32> as i32, 0 as *mut std::ffi::c_void );
// //         gl::EnableVertexAttribArray(0);

// //         //                  // 0. copy our vertices array in a buffer for OpenGL to use
// //         //                  glBindBuffer(GL_ARRAY_BUFFER, VBO);
// //         //                  glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);
// //         //                  // 1. then set the vertex attributes pointers
// //         //                  glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(float),
// //         //                  (void*)0);
// //         //                  glEnableVertexAttribArray(0);
// //         //                  // 2. use our shader program when we want to render an object
// //         //                  glUseProgram(shaderProgram);
// //         //                  // 3. now draw the object
// //         //                  someOpenGLFunctionThatDrawsOurTriangle();

// //         gl::BindVertexArray(VAO);

// //         gl::DrawArrays(gl::TRIANGLES, 0, 3);
        
// //     }