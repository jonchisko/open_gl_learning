// Make it windows and not console app. Doesnt open the terminal
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use beryllium::*;
use gl33::{
    global_loader::{
        glActiveTexture, glAttachShader, glBindBuffer, glBindTexture, glBindVertexArray, glBufferData, glClear, glClearColor, glCompileShader, glCreateProgram, glCreateShader, glDeleteBuffers, glDeleteProgram, glDeleteShader, glDeleteVertexArrays, glDisableVertexAttribArray, glDrawArrays, glDrawElements, glEnableVertexAttribArray, glGenBuffers, glGenTextures, glGenVertexArrays, glGenerateMipmap, glGetIntegerv, glGetProgramInfoLog, glGetProgramiv, glGetShaderInfoLog, glGetShaderiv, glGetUniformLocation, glLinkProgram, glShaderSource, glTexImage2D, glTexParameteri, glUniform1i, glUniform4f, glUniformMatrix4fv, glUseProgram, glVertexAttribPointer, load_global_gl
    },
    *,
};
use glam::vec4;

use std::{
    ffi::CString, mem, time::SystemTime
};

use image::ImageReader;

#[rustfmt::skip]
fn get_vertices() -> [f32; 32] {
    // Triangle in Normalized Device Context (NDC).
    [
        // pos                // col              // tex coord 
        -0.5, -0.5, 0.0,      1.0, 0.0, 0.0,      0.0, 0.0,  
        0.5, -0.5, 0.0,       0.0, 1.0, 0.0,      1.0, 0.0,
        0.5, 0.5, 0.0,        0.0, 0.0, 1.0,      1.0, 1.0,
        -0.5, 0.5, 0.0,       1.0, 1.0, 0.0,      0.0, 1.0,
    ]
}

#[rustfmt::skip]
fn get_indices() -> [u32; 6] {
    [
        // Triangle 1
        0, 1, 2,
        // Triangle 2
        0, 2, 3,
    ]
}

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

    // VERTEX ARRAY OBJECT

    let mut vao = 0u32;
    unsafe {
        glGenVertexArrays(1, &mut vao);
    }
    assert!(vao != 0);
    glBindVertexArray(vao);

    // Triangle in Normalized Device Context (NDC).
    let vertices = get_vertices();

    let indices = get_indices();

    // Load texture image
    let wooden_crate_texture = ImageReader::open("./assets/wall.jpg")
        .expect("Could not open image")
        .decode()
        .expect("Could not decode the image");
    let (wooden_width, wooden_height) =
        (wooden_crate_texture.width(), wooden_crate_texture.height());
    let wooden_crate_texture = wooden_crate_texture.as_bytes();

    let face_texture = ImageReader::open("./assets/awesomeface.png")
        .expect("Could not open image")
        .decode()
        .expect("Could not decode the image");
    let (face_width, face_height) = (face_texture.width(), face_texture.height());
    let face_texture = face_texture.as_bytes();

    // TEXTURE GENERATION
    let mut texture_wooden_crate = 0u32;
    unsafe { glGenTextures(1, &mut texture_wooden_crate) };
    assert!(texture_wooden_crate != 0);
    unsafe { glBindTexture(GL_TEXTURE_2D, texture_wooden_crate) };

    unsafe { glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT.0 as i32) };
    unsafe { glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT.0 as i32) };
    unsafe {
        glTexParameteri(
            GL_TEXTURE_2D,
            GL_TEXTURE_MIN_FILTER,
            GL_LINEAR_MIPMAP_LINEAR.0 as i32,
        )
    }
    unsafe { glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR.0 as i32) }

    unsafe {
        glTexImage2D(
            GL_TEXTURE_2D,
            0,
            GL_RGB.0 as i32, // format in the opengl/gpu
            wooden_width as i32,
            wooden_height as i32,
            0,
            GL_RGB, // original format, so our byte slice
            GL_UNSIGNED_BYTE,
            wooden_crate_texture.as_ptr().cast(),
        );
    }
    unsafe {
        glGenerateMipmap(GL_TEXTURE_2D);
    }
    unsafe {
        glBindTexture(GL_TEXTURE_2D, 0);
    }

    // Face texture
    let mut texture_face = 0u32;
    unsafe {
        glGenTextures(1, &mut texture_face);
    }
    assert!(texture_face != 0);
    unsafe {
        glBindTexture(GL_TEXTURE_2D, texture_face);
    }

    unsafe {
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT.0 as i32);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT.0 as i32);
        glTexParameteri(
            GL_TEXTURE_2D,
            GL_TEXTURE_MIN_FILTER,
            GL_LINEAR_MIPMAP_LINEAR.0 as i32,
        );
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR.0 as i32);
    }

    unsafe {
        glTexImage2D(
            GL_TEXTURE_2D,
            0,
            GL_RGB.0 as i32,
            face_width as i32,
            face_height as i32,
            0,
            GL_RGBA,
            GL_UNSIGNED_BYTE,
            face_texture.as_ptr().cast(),
        );
        glGenerateMipmap(GL_TEXTURE_2D);

        glBindTexture(GL_TEXTURE_2D, 0);
    }

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
            (vertices.len() * mem::size_of::<f32>()).try_into().unwrap(),
            vertices.as_ptr().cast(),
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
            (indices.len() * mem::size_of::<u32>()).try_into().unwrap(),
            indices.as_ptr().cast(),
            GL_STATIC_DRAW,
        );
    }

    unsafe {
        glVertexAttribPointer(
            0,        // Has to match the shader program later on
            3,        // Number of components in the attribute
            GL_FLOAT, // Element type of the data in the attribute
            0,        // normalized
            ((2 * 3 + 2) * mem::size_of::<f32>()).try_into().unwrap(), // Size in bytes of all the attributes, currently 3 * 4 bytes
            0 as *const _, // Start of the vertext attribute within the buffer
        );
        glEnableVertexAttribArray(0);

        glVertexAttribPointer(
            1,
            3,
            GL_FLOAT,
            0,
            ((2 * 3 + 2) * mem::size_of::<f32>()).try_into().unwrap(),
            (3 * mem::size_of::<f32>()) as *const _,
        );
        glEnableVertexAttribArray(1);

        glVertexAttribPointer(
            2,
            2,
            GL_FLOAT,
            0,
            ((2 * 3 + 2) * mem::size_of::<f32>()).try_into().unwrap(),
            (6 * mem::size_of::<f32>()) as *const _,
        );
        glEnableVertexAttribArray(2);
    }

    glBindVertexArray(0);
    unsafe {
        glDisableVertexAttribArray(0);
    }
    unsafe {
        glDisableVertexAttribArray(1);
    }
    unsafe {
        glDisableVertexAttribArray(2);
    }
    unsafe {
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
    }
    unsafe {
        glBindBuffer(GL_ARRAY_BUFFER, 0);
    }

    // SHADERS

    let mut max_attribute_number = 0i32;
    unsafe {
        glGetIntegerv(GL_MAX_VERTEX_ATTRIBS, &mut max_attribute_number);
    };
    println!(
        "Max number of vertex attributes (input variable) for vertex shader: {}",
        max_attribute_number
    );

    const VERT_SHADER: &str = r#"#version 330 core
        layout (location = 0) in vec3 pos;
        layout (location = 1) in vec3 color;
        layout (location = 2) in vec2 textureCoord;

        uniform mat4 transform;

        out vec4 vertexColor;
        out vec2 texCoord;

        void main() {
            //gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
            gl_Position = transform * vec4(pos, 1.0);
            vertexColor = vec4(color, 1.0);
            texCoord = textureCoord;
        }
    "#;

    const FRAG_SHADER: &str = r#"#version 330 core
        out vec4 final_color;

        //uniform vec4 ourColor;
        // built in datatype for texture objects
        uniform sampler2D texture1;
        uniform sampler2D texture2;

        in vec4 vertexColor;
        in vec2 texCoord;

        void main() {
            final_color = mix(texture(texture1, texCoord), texture(texture2, texCoord), 0.2);
            //final_color = texture(texture1, texCoord) * vertexColor;
            //final_color = ourColor;
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

    // Enable vsync - swap_window blocks until the image has been presented to the user
    // So we show images at most as fast the display's refresh rate
    let _ = win.set_swap_interval(video::GlSwapInterval::Vsync);

    // Wireframe mode
    /*unsafe {
        glPolygonMode(GL_FRONT_AND_BACK, GL_LINE);
    }*/

    let now = SystemTime::now();

    glUseProgram(program);
    let texture1 = CString::new("texture1").unwrap();
    let texture2 = CString::new("texture2").unwrap();

    let location_texture1 = unsafe { glGetUniformLocation(program, texture1.as_ptr().cast()) };
    let location_texture2 = unsafe { glGetUniformLocation(program, texture2.as_ptr().cast()) };
    assert!(location_texture1 >= 0);
    assert!(location_texture2 >= 0);
    unsafe {
        glUniform1i(location_texture1, 0);
        glUniform1i(location_texture2, 1);
    }

    let transform = CString::new("transform").unwrap();
    let location_transform = unsafe { glGetUniformLocation(program, transform.as_ptr().cast()) };
    assert!(location_transform >= 0);

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

            // The default texture unit for a texture is 0 which is the default active texture unit
            // so we didn't need to assign a location in the previous section
            glActiveTexture(GL_TEXTURE0);
            glBindTexture(GL_TEXTURE_2D, texture_wooden_crate);
            glActiveTexture(GL_TEXTURE1);
            glBindTexture(GL_TEXTURE_2D, texture_face);

            // Compute matrix

            //let rotation_matrix = glam::Mat4::from_rotation_z(std::f32::consts::PI/2.0);
            let mut rotation_translation_matrix = glam::Mat4::from_rotation_z(now.elapsed().unwrap().as_secs_f32() % std::f32::consts::TAU);
            rotation_translation_matrix.w_axis = glam::vec4(0.5, -0.5, 0.0, 1.0);

            glUniformMatrix4fv(location_transform, 1, 0, rotation_translation_matrix.to_cols_array().as_ptr());

            glBindVertexArray(vao);

            /*let time_value = now.elapsed().unwrap().as_secs_f32();
            let green_value = f32::sin(time_value) / 2.0 + 0.5;

            let uniform_name = CString::new("ourColor").unwrap();
            let vertex_color_location = glGetUniformLocation(program, uniform_name.as_ptr().cast());
            assert!(vertex_color_location >= 0);
            glUniform4f(vertex_color_location, 0.0, green_value, 0.0, 1.0);*/

            glDrawElements(
                GL_TRIANGLES,
                indices.len() as i32,
                GL_UNSIGNED_INT,
                0 as *const _,
            );
            win.swap_window();
        }
    }

    unsafe {
        glDeleteVertexArrays(1, &vao);

        glDeleteBuffers(1, &vbo);

        glDeleteProgram(program);
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
