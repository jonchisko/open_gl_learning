// Make it windows and not console app. Doesnt open the terminal
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use beryllium::*;
use gl33::{
    global_loader::{
        glAttachShader, glBindBuffer, glBindVertexArray, glBufferData, glClear, glClearColor,
        glCompileShader, glCreateProgram, glCreateShader, glDeleteShader, glDrawArrays,
        glDrawElements, glEnableVertexAttribArray, glGenBuffers, glGenVertexArrays,
        glGetProgramInfoLog, glGetProgramiv, glGetShaderInfoLog, glGetShaderiv, glLinkProgram,
        glShaderSource, glUseProgram, glVertexAttribPointer, load_global_gl,
    },
    *,
};

use std::mem;

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

    unsafe { glClearColor(0.2, 0.3, 0.3, 1.0) };

    // VERTEX ARRAY OBJECT

    let mut vao = 0u32;
    unsafe {
        glGenVertexArrays(1, &mut vao);
    }
    assert!(vao != 0);
    glBindVertexArray(vao);

    // Triangle in Normalized Device Context (NDC).
    const VERTICES: [f32; 12] = [
        0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0, -0.5, 0.5, 0.0,
    ];

    const INDICES: [u32; 6] = [0, 1, 3, 1, 2, 3];

    // VERTEX BUFFER OBJECT

    let mut vbo = 0u32;
    unsafe {
        glGenBuffers(1, &mut vbo);
    }
    assert!(vbo != 0);
    unsafe { glBindBuffer(GL_ARRAY_BUFFER, vbo) };
    unsafe {
        glBufferData(
            GL_ARRAY_BUFFER,
            (VERTICES.len() * mem::size_of::<f32>()).try_into().unwrap(),
            VERTICES.as_ptr().cast(),
            GL_STATIC_DRAW,
        )
    };

    // ELEMENT BUFFER OBJECT

    let mut ebo = 0u32;
    unsafe {
        glGenBuffers(1, &mut ebo);
    }
    assert!(ebo != 0);
    unsafe {
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);
    }
    unsafe {
        glBufferData(
            GL_ELEMENT_ARRAY_BUFFER,
            (INDICES.len() * mem::size_of::<u32>()).try_into().unwrap(),
            INDICES.as_ptr().cast(),
            GL_STATIC_DRAW,
        );
    }

    unsafe {
        glVertexAttribPointer(
            0,                                               // Has to match the shader program later on
            3,        // Number of components in the attribute
            GL_FLOAT, // Element type of the data in the attribute
            0,        // normalized
            (3 * mem::size_of::<f32>()).try_into().unwrap(), // Size in bytes of all the attributes, currently 3 * 4 bytes
            0 as *const _, // Start of the vertext attribute within the buffer
        );
        glEnableVertexAttribArray(0);
    }

    // SHADERS

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

    let vertex_shader = glCreateShader(GL_VERTEX_SHADER);
    assert!(vertex_shader != 0);
    unsafe {
        glShaderSource(
            vertex_shader,
            1,
            &(VERT_SHADER.as_bytes().as_ptr().cast()),
            &(VERT_SHADER.len().try_into().unwrap()),
        );
    };
    glCompileShader(vertex_shader);
    log_error(vertex_shader, true);

    let fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
    assert!(fragment_shader != 0);
    unsafe {
        glShaderSource(
            fragment_shader,
            1,
            &(FRAG_SHADER.as_bytes().as_ptr().cast()),
            &(FRAG_SHADER.len().try_into().unwrap()),
        );
    }
    glCompileShader(fragment_shader);
    log_error(fragment_shader, true);

    // PROGRAM
    let program = glCreateProgram();
    assert!(program != 0);
    glAttachShader(program, vertex_shader);
    glAttachShader(program, fragment_shader);
    glLinkProgram(program);
    log_error(program, false);

    glDeleteShader(vertex_shader);
    glDeleteShader(fragment_shader);

    glUseProgram(program);

    // Enable vsync - swap_window blocks until the image has been presented to the user
    // So we show images at most as fast the display's refresh rate
    let _ = win.set_swap_interval(video::GlSwapInterval::Vsync);

    // Wireframe mode
    /*unsafe {
        glPolygonMode(GL_FRONT_AND_BACK, GL_LINE);
    }*/

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

fn log_error(object_id: u32, is_shader: bool) -> () {
    let mut success = 0;

    unsafe {
        if is_shader {
            glGetShaderiv(object_id, GL_COMPILE_STATUS, &mut success);
        } else {
            glGetProgramiv(object_id, GL_LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut log_len = 0i32;

            if is_shader {
                glGetShaderiv(object_id, GL_INFO_LOG_LENGTH, &mut log_len);
            } else {
                glGetProgramiv(object_id, GL_INFO_LOG_LENGTH, &mut log_len);
            }

            let mut log_message: Vec<u8> = Vec::with_capacity(log_len as usize);

            if is_shader {
                glGetShaderInfoLog(
                    object_id,
                    log_message.capacity() as i32,
                    &mut log_len,
                    log_message.as_mut_ptr().cast(),
                );
            } else {
                glGetProgramInfoLog(
                    object_id,
                    log_message.capacity() as i32,
                    &mut log_len,
                    log_message.as_mut_ptr().cast(),
                );
            }

            log_message.set_len(log_len.try_into().unwrap());

            if is_shader {
                glDeleteShader(object_id);
            }

            panic!(
                "Shader Program Link Error: {}",
                String::from_utf8_lossy(&log_message)
            );
        }
    }
}
