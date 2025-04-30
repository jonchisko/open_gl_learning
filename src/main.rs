// Make it windows and not console app. Doesnt open the terminal
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use beryllium::*;
use gl33::{
    global_loader::{
        glAttachShader, glBindBuffer, glBindVertexArray, glBufferData, glClear, glClearColor,
        glCompileShader, glCreateProgram, glCreateShader, glDeleteBuffers, glDeleteProgram,
        glDeleteShader, glDeleteVertexArrays, glDisableVertexAttribArray, glDrawArrays,
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
    // I assume it also does the glViewport, which sets the data for the NDC -> screen-space coord.
    let win = sdl
        .create_gl_window(win_args)
        .expect("Could not make a window and context.");

    // Load up every OpenGL function
    unsafe {
        load_global_gl(&|f_name| win.get_proc_address(f_name));
    }

    unsafe { glClearColor(0.2, 0.3, 0.3, 1.0) };

    // Triangle in Normalized Device Context (NDC).
    const V_TRIANGLE1: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.5, 0.5, 0.0];
    const V_TRIANGLE2: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, 0.5, 0.0, -0.5, 0.5, 0.0];

    const INDICES1: [u32; 3] = [0, 1, 2];
    const INDICES2: [u32; 3] = [0, 1, 2];

    // VERTEX ARRAY OBJECT

    let mut vao1 = 0u32;
    let mut vao2 = 0u32;
    unsafe {
        // Could also use n == 2 and pass in an array
        glGenVertexArrays(1, &mut vao1);
        glGenVertexArrays(1, &mut vao2);
    }
    assert!(vao1 != 0);
    assert!(vao2 != 0);
    glBindVertexArray(vao1);

    // VERTEX BUFFER OBJECT

    let mut vbo1 = 0u32;
    let mut vbo2 = 0u32;
    unsafe {
        glGenBuffers(1, &mut vbo1);
        glGenBuffers(1, &mut vbo2);
    }
    assert!(vbo1 != 0);
    assert!(vbo2 != 0);
    unsafe { glBindBuffer(GL_ARRAY_BUFFER, vbo1) };
    unsafe {
        glBufferData(
            GL_ARRAY_BUFFER,
            (V_TRIANGLE1.len() * mem::size_of::<f32>())
                .try_into()
                .unwrap(),
            V_TRIANGLE1.as_ptr().cast(),
            GL_STATIC_DRAW,
        )
    };

    // ELEMENT BUFFER OBJECT

    let mut ebo1 = 0u32;
    let mut ebo2 = 0u32;
    unsafe {
        glGenBuffers(1, &mut ebo1);
        glGenBuffers(1, &mut ebo2);
    }
    assert!(ebo1 != 0);
    assert!(ebo2 != 0);
    unsafe {
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo1);
    }
    unsafe {
        glBufferData(
            GL_ELEMENT_ARRAY_BUFFER,
            (INDICES1.len() * mem::size_of::<u32>()).try_into().unwrap(),
            INDICES1.as_ptr().cast(),
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

    glBindVertexArray(0);
    unsafe { glDisableVertexAttribArray(0) };
    unsafe { glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0) };
    unsafe { glBindBuffer(GL_ARRAY_BUFFER, 0) };

    glBindVertexArray(vao2);
    unsafe { glBindBuffer(GL_ARRAY_BUFFER, vbo2) };
    unsafe {
        glBufferData(
            GL_ARRAY_BUFFER,
            (V_TRIANGLE2.len() * mem::size_of::<f32>())
                .try_into()
                .unwrap(),
            V_TRIANGLE2.as_ptr().cast(),
            GL_STATIC_DRAW,
        );
    }

    unsafe { glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo2) };
    unsafe {
        glBufferData(
            GL_ELEMENT_ARRAY_BUFFER,
            (INDICES2.len() * mem::size_of::<f32>()).try_into().unwrap(),
            INDICES2.as_ptr().cast(),
            GL_STATIC_DRAW,
        );
    }

    unsafe {
        glVertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            0,
            (3 * mem::size_of::<f32>()).try_into().unwrap(),
            0 as *const _,
        );
        glEnableVertexAttribArray(0);
    }

    glBindVertexArray(0);
    unsafe {
        glDisableVertexAttribArray(0);
        glBindBuffer(GL_ARRAY_BUFFER, 0);
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
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

    const FRAG_SHADER2: &str = r#"#version 330 core
        out vec4 final_color;

        void main() {
            final_color = vec4(0.8, 0.5, 0.0, 1.0);
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

    let fragment_shader2 = glCreateShader(GL_FRAGMENT_SHADER);
    assert!(fragment_shader != 0);
    unsafe {
        glShaderSource(
            fragment_shader2,
            1,
            &(FRAG_SHADER2.as_bytes().as_ptr().cast()),
            &(FRAG_SHADER2.len().try_into().unwrap()),
        );
    }
    glCompileShader(fragment_shader2);
    log_error(fragment_shader2, true);

    // PROGRAM
    let program = glCreateProgram();
    assert!(program != 0);
    glAttachShader(program, vertex_shader);
    glAttachShader(program, fragment_shader);
    glLinkProgram(program);
    log_error(program, false);

    glDeleteShader(vertex_shader);
    glDeleteShader(fragment_shader);

    let program2 = glCreateProgram();
    assert!(program2 != 0);
    glAttachShader(program2, vertex_shader);
    glAttachShader(program2, fragment_shader2);
    glLinkProgram(program2);
    log_error(program2, false);

    glDeleteShader(fragment_shader2);

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
            glUseProgram(program);

            glBindVertexArray(vao1);
            glDrawElements(GL_TRIANGLES, 3, GL_UNSIGNED_INT, 0 as *const _);

            glUseProgram(program2);
            glBindVertexArray(vao2);
            glDrawElements(GL_TRIANGLES, 3, GL_UNSIGNED_INT, 0 as *const _);

            win.swap_window();
        }
    }

    unsafe {
        glDeleteVertexArrays(1, &vao1);
        glDeleteVertexArrays(1, &vao2);

        glDeleteBuffers(1, &vbo1);
        glDeleteBuffers(1, &vbo2);

        glDeleteProgram(program);
        glDeleteProgram(program2);
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
