// Make it windows and not console app. Doesnt open the terminal
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use beryllium::*;
use gl33::{
    global_loader::{
        glClear, glDrawArrays, glDrawElements, glEnableVertexAttribArray, glPolygonMode, glVertexAttribPointer, load_global_gl
    },
    *,
};
use opengl_chrno::learn_opengl::{self as learn, Buffer, BufferType, ShaderProgram, VertexArray};

use std::mem;

type Vertex = [f32; 3];
type TriIndices = [u32; 3];

fn main() {
    // Specify you will be using open GL before creating the window
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);

    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_minor_version(3).unwrap();
    // Core is a subset of all the features the OpenGL provides
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();
    #[cfg(target_os = "macos")]
    {
        // For Mac OS -> FC basically makes all deperecated but available functions unavailable
        // Necessary for Mac OS to use Core feature set
        sdl.set_gl_context_flags(video::GlContextFlags::FORWARD_COMPATIBLE)
            .unwrap();
    }

    let win_args = video::CreateWinArgs {
        title: "LEARN OPENGL",
        width: 800,
        height: 600,
        allow_high_dpi: true,
        borderless: false,
        resizable: false,
    };

    // Beryllium sticks the window and GL context together as a single thing
    let win = sdl
        .create_gl_window(win_args)
        .expect("Could not make a window and context.");

    // Load up every OpenGL function
    unsafe {
        load_global_gl(&|f_name| win.get_proc_address(f_name));
    }

    learn::clear_color(0.2, 0.3, 0.3, 1.0);

    let vao = VertexArray::new().expect("Could not make a VAO");
    vao.bind();

    // Triangle in Normalized Device Context (NDC).
    const VERTICES: [Vertex; 4] = [[0.5, 0.5, 0.0], [0.5, -0.5, 0.0], [-0.5, -0.5, 0.0], [-0.5, 0.5, 0.0]];
    const INDICES: [TriIndices; 2] = [[0, 1, 3], [1, 2, 3]];

    let vbo = Buffer::new().expect("Could not make a VBO");
    vbo.bind(BufferType::Array);
    learn::buffer_data(
        BufferType::Array,
        bytemuck::cast_slice(&VERTICES),
        GL_STATIC_DRAW,
    );

    let ebo = Buffer::new().expect("Could not make the element buffer");
    ebo.bind(BufferType::ElementArray);
    learn::buffer_data(BufferType::ElementArray, bytemuck::cast_slice(&INDICES), GL_STATIC_DRAW);


    unsafe {
        glVertexAttribPointer(
            0,                                            // Has to match the shader program later on
            3,                                            // Number of components in the attribute
            GL_FLOAT, // Element type of the data in the attribute
            0,        // normalized
            mem::size_of::<Vertex>().try_into().unwrap(), // Size in bytes of all the attributes, currently 3 * 4 bytes
            0 as *const _, // Start of the vertext attribute within the buffer
        );
        glEnableVertexAttribArray(0);
    }

    const VERT_SHADER: &str = r#"#version 330 core
        layout (location = 0) in vec4 pos;
        void main() {
            //gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
            gl_Position = pos;
        }
    "#;

    const FRAG_SHADER: &str = r#"#version 330 core
        out vec4 final_color;

        void main() {
            final_color = vec4(1.0, 0.5, 0.2, 1.0);
        }
    "#;

    let shader_program = ShaderProgram::from_vert_frag(VERT_SHADER, FRAG_SHADER).unwrap();
    shader_program.use_program();

    // Enable vsync - swap_window blocks until the image has been presented to the user
    // So we show images at most as fast the display's refresh rate
    let _ = win.set_swap_interval(video::GlSwapInterval::Vsync);

    // Wireframe mode
    unsafe {
        glPolygonMode(GL_FRONT_AND_BACK, GL_LINE);
    }

    // Processing events - we have to, OS otherwise thinks the application has stalled
    'main_loop: loop {
        // Handle events this frame
        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => break 'main_loop,
                _ => (),
            }
        }
        // Now events are clear

        // Here is the spot to change the world state and draw

        unsafe {
            glClear(GL_COLOR_BUFFER_BIT);
            //glDrawArrays(GL_TRIANGLES, 0, 3);
            glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, 0 as *const _);
            win.swap_window();
        }
    }
}
