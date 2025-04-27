// Make it windows and not console app. Doesnt open the terminal
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use beryllium::*;
use gl33::{
    global_loader::{
        glAttachShader, glBindBuffer, glBindVertexArray, glBufferData, glClear, glClearColor, glCompileShader, glCreateProgram, glCreateShader, glDeleteShader, glDrawArrays, glEnableVertexAttribArray, glGenBuffers, glGenVertexArrays, glGetProgramiv, glGetShaderInfoLog, glGetShaderiv, glLinkProgram, glShaderSource, glUseProgram, glVertexAttribPointer, load_global_gl
    },
    *,
};
use std::mem;

type Vertex = [f32; 3];

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
    // Clear the buffer with the following color
    unsafe {
        glClearColor(0.2, 0.3, 0.3, 1.0);
    }

    let mut vao = 0; // Pointer to the generated vertex array
    unsafe {
        glGenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0); // If zero was returned -> error
    }
    // with glBindVertexArray we would make the "vao" (vertex array object) as the active VAO
    // This is context wide effect and all functions now operate on this vao
    glBindVertexArray(vao);

    // VBO, vertex buffer object
    let mut vbo = 0;
    unsafe {
        glGenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);
    }

    // Binding to the binding target
    unsafe {
        glBindBuffer(GL_ARRAY_BUFFER, vbo);
    }

    // Triangle in Normalized Device Context (NDC).
    const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];
    // We operate on GL_ARRAY_BUFFER, the vbo we bound above
    // we need the size of the vertices, which is 3 (vertices) * 3 (floats) * 4 bytes (size of f32) = 36 bytes
    // Void ptr to the data
    // Hint on how we use the data so GPU/CPU can use the faster/slower mem. Static -> sending the data
    // once with multiple redraws
    unsafe {
        glBufferData(
            GL_ARRAY_BUFFER,
            mem::size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
            GL_STATIC_DRAW,
        );
    }

    unsafe {
        glVertexAttribPointer(
            0,                                            // Has to match the shader program later on
            3,                                            // Number of components in the attribute
            GL_FLOAT, // Element type of the data in the attribute
            0,        // normalized
            mem::size_of::<Vertex>().try_into().unwrap(), // Size in bytes of the data type, 3 * 4 bytes
            0 as *const _, // Start of the vertext attribute within the buffer
        );
        glEnableVertexAttribArray(0);
    }

    let vertex_shader = glCreateShader(GL_VERTEX_SHADER);
    assert_ne!(vertex_shader, 0);

    const VERT_SHADER: &str = r#"#version 330 core
        layout (location = 0) in vec3 pos;
        void main() {
            gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
        }
    "#;

    unsafe {
        glShaderSource(
            vertex_shader,
            1,
            &(VERT_SHADER.as_bytes().as_ptr().cast()),
            &(VERT_SHADER.len().try_into().unwrap()),
        );
    }
    glCompileShader(vertex_shader);

    // Check for compilation error
    unsafe {
        let mut success = 0;
        glGetShaderiv(vertex_shader, GL_COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024); // We assume msg is less than 1024 bytes
            let mut log_len = 0i32;
            glGetShaderInfoLog(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            
            assert!(v.capacity() >= log_len as usize);
            v.set_len(log_len.try_into().unwrap());
            
            panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
        }
    }

    // Creating the fragment shader
    let fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
    assert_ne!(fragment_shader, 0);

    const FRAG_SHADER: &str = r#"#version 330 core
        out vec4 final_color;

        void main() {
            final_color = vec4(1.0, 0.5, 0.2, 1.0);
        }
    "#;

    unsafe {
        glShaderSource(fragment_shader, 1, &(FRAG_SHADER.as_bytes().as_ptr().cast()), &(FRAG_SHADER.len().try_into().unwrap()));
    }
    glCompileShader(fragment_shader);

    unsafe {
        let mut success = 0;
        glGetShaderiv(fragment_shader, GL_COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0i32;
            glGetShaderInfoLog(fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());

            assert!(v.capacity() >= log_len as usize);
            v.set_len(log_len.try_into().unwrap());

            panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
        }
    }

    // Program combines several shader stages and creates a complete graphics pipeline
    let shader_program = glCreateProgram();
    glAttachShader(shader_program, vertex_shader);
    glAttachShader(shader_program, fragment_shader);
    glLinkProgram(shader_program);

    unsafe {
        let mut success = 0;
        glGetProgramiv(shader_program, GL_LINK_STATUS, &mut success);

        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0i32;
            glGetShaderInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());

            assert!(v.capacity() >= log_len as usize);
            v.set_len(log_len.try_into().unwrap());

            panic!("Shader Program Link Error: {}", String::from_utf8_lossy(&v));
        }
    }

    // They get deleted after we unattach them from the defined program
    glDeleteShader(vertex_shader);
    glDeleteShader(fragment_shader);

    glUseProgram(shader_program);


    // Enable vsync - swap_window blocks until the image has been presented to the user
    // So we show images at most as fast the display's refresh rate
    let _ = win.set_swap_interval(video::GlSwapInterval::Vsync);

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
            glDrawArrays(GL_TRIANGLES, 0, 3);
            win.swap_window();
        }
    }
}
