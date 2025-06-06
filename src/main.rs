// Make it windows and not console app. Doesnt open the terminal
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use beryllium::{events::{SDLK_a, SDLK_d, SDLK_s, SDLK_w, SDLK_6, SDLK_UP}, *};
use gl33::{
    global_loader::{
        glActiveTexture, glAttachShader, glBindBuffer, glBindTexture, glBindVertexArray, glBufferData, glClear, glClearColor, glCompileShader, glCreateProgram, glCreateShader, glDeleteBuffers, glDeleteProgram, glDeleteShader, glDeleteVertexArrays, glDisableVertexAttribArray, glDrawArrays, glDrawElements, glEnable, glEnableVertexAttribArray, glGenBuffers, glGenTextures, glGenVertexArrays, glGenerateMipmap, glGetIntegerv, glGetProgramInfoLog, glGetProgramiv, glGetShaderInfoLog, glGetShaderiv, glGetUniformLocation, glLinkProgram, glShaderSource, glTexImage2D, glTexParameteri, glUniform1i, glUniform4f, glUniformMatrix4fv, glUseProgram, glVertexAttribPointer, load_global_gl
    },
    *,
};
use glam::vec4;

use std::{
    f32::consts::PI, ffi::CString, mem, time::SystemTime
};

use image::ImageReader;

#[rustfmt::skip]
fn get_vertices() -> [f32; 180] {
    [ // I coppied the data from learnopengl, 1.5 -> 1.0 and 0.5 to 0.0 but i am too lazy
    -0.5, -0.5, -0.5,  0.0, 0.0,
     0.5, -0.5, -0.5,  1.0, 0.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
    -0.5,  0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 0.0,

    -0.5, -0.5,  0.5,  0.5, 0.5,
     0.5, -0.5,  0.5,  1.5, 0.5,
     0.5,  0.5,  0.5,  1.5, 1.5,
     0.5,  0.5,  0.5,  1.5, 1.5,
    -0.5,  0.5,  0.5,  0.5, 1.5,
    -0.5, -0.5,  0.5,  0.5, 0.5,

    -0.5,  0.5,  0.5,  1.5, 0.5,
    -0.5,  0.5, -0.5,  1.5, 1.5,
    -0.5, -0.5, -0.5,  0.5, 1.5,
    -0.5, -0.5, -0.5,  0.5, 1.5,
    -0.5, -0.5,  0.5,  0.5, 0.5,
    -0.5,  0.5,  0.5,  1.5, 0.5,

     0.5,  0.5,  0.5,  1.5, 0.5,
     0.5,  0.5, -0.5,  1.5, 1.5,
     0.5, -0.5, -0.5,  0.5, 1.5,
     0.5, -0.5, -0.5,  0.5, 1.5,
     0.5, -0.5,  0.5,  0.5, 0.5,
     0.5,  0.5,  0.5,  1.5, 0.5,

    -0.5, -0.5, -0.5,  0.5, 1.5,
     0.5, -0.5, -0.5,  1.5, 1.5,
     0.5, -0.5,  0.5,  1.5, 0.5,
     0.5, -0.5,  0.5,  1.5, 0.5,
    -0.5, -0.5,  0.5,  0.5, 0.5,
    -0.5, -0.5, -0.5,  0.5, 1.5,

    -0.5,  0.5, -0.5,  0.5, 1.5,
     0.5,  0.5, -0.5,  1.5, 1.5,
     0.5,  0.5,  0.5,  1.5, 0.5,
     0.5,  0.5,  0.5,  1.5, 0.5,
    -0.5,  0.5,  0.5,  0.5, 0.5,
    -0.5,  0.5, -0.5,  0.5, 1.5
    ]
}

fn main() {
    // Specify you will be using open GL before creating the window
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);

    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_minor_version(3).unwrap();
    // Core is a subset of all the features the OpenGL provides
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();
    sdl.set_relative_mouse_mode(true).unwrap();
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

    let vertices = get_vertices();

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

    unsafe {
        glVertexAttribPointer(
            0,        // Has to match the shader program later on
            3,        // Number of components in the attribute
            GL_FLOAT, // Element type of the data in the attribute
            0,        // normalized
            ((3 + 2) * mem::size_of::<f32>()).try_into().unwrap(), // Size in bytes of all the attributes, currently 3 * 4 bytes
            0 as *const _, // Start of the vertext attribute within the buffer
        );
        glEnableVertexAttribArray(0);

        glVertexAttribPointer(
            1,
            2,
            GL_FLOAT,
            0,
            ((3 + 2) * mem::size_of::<f32>()).try_into().unwrap(),
            (3 * mem::size_of::<f32>()) as *const _,
        );
        glEnableVertexAttribArray(1);
    }

    glBindVertexArray(0);
    unsafe {
        glDisableVertexAttribArray(0);
    }
    unsafe {
        glDisableVertexAttribArray(1);
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
        layout (location = 1) in vec2 textureCoord;

        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 projection;

        out vec2 texCoord;

        void main() {
            //gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
            gl_Position = projection * view * model * vec4(pos, 1.0);
            texCoord = textureCoord;
        }
    "#;

    const FRAG_SHADER: &str = r#"#version 330 core
        out vec4 final_color;

        //uniform vec4 ourColor;
        // built in datatype for texture objects
        uniform sampler2D texture1;
        uniform sampler2D texture2;

        in vec2 texCoord;

        void main() {
            final_color = mix(texture(texture1, texCoord), texture(texture2, vec2(texCoord.x, 1.0 - texCoord.y)), 0.2);
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

    // CUBE POSITIONS
    let cube_positions = vec![
        glam::vec3(0.0, 0.0, 0.0),
        glam::vec3(2.0, 5.0, -15.0),
        glam::vec3(1.0, 3.0, -2.0),
        glam::vec3(3.0, -2.0, -10.0),
        glam::vec3(-2.4, 3.0, -3.0),
        glam::vec3(-1.3, -2.5, -11.0),
        glam::vec3(1.0, 0.5, -8.0),
        glam::vec3(-1.5, 1.0, -4.0),
    ];


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

    let model = CString::new("model").unwrap();
    let location_model = unsafe { glGetUniformLocation(program, model.as_ptr().cast()) };
    assert!(location_model >= 0);

    let view = CString::new("view").unwrap();
    let location_view = unsafe { glGetUniformLocation(program, view.as_ptr().cast())};
    assert!(location_view >= 0);

    let projection = CString::new("projection").unwrap();
    let location_projection = unsafe {glGetUniformLocation(program, projection.as_ptr().cast())};
    assert!(location_projection >= 0);

    unsafe { glEnable(GL_DEPTH_TEST) };

    let mut delta_time = 0.0;
    let mut last_frame = 0.0;

    let camera_speed = 10.0;
    let mut camera_front = glam::Vec3::new(0.0, 0.0, -1.0);
    let mut camera_pos = glam::Vec3::new(0.0, 0.0, 0.0);
    let global_up = glam::Vec3::new(0.0, 1.0, 0.0);

    let mut yaw = 0.0;
    let mut pitch = 0.0;
    let mut fov = 45.0;

    // Processing events - we have to, OS otherwise thinks the application has stalled
    'main_loop: loop {

        // DELTA TIME
        let current_frame = now.elapsed().unwrap().as_secs_f32();
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        // Handle events this frame
        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => break 'main_loop,
                (events::Event::Key { win_id: _, pressed: true, repeat: _, scancode: _, keycode: SDLK_w, modifiers: _ }, _) => {
                    camera_pos += camera_front * camera_speed * delta_time;
                },
                (events::Event::Key { win_id: _, pressed: true, repeat: _, scancode: _, keycode: SDLK_s, modifiers: _ }, _) => {
                    camera_pos -= camera_front * camera_speed * delta_time;
                },
                (events::Event::Key { win_id: _, pressed: true, repeat: _, scancode: _, keycode: SDLK_a, modifiers: _ }, _) => {
                    camera_pos -= camera_front.cross(global_up).normalize() * camera_speed * delta_time;
                },
                (events::Event::Key { win_id: _, pressed: true, repeat: _, scancode: _, keycode: SDLK_d, modifiers: _ }, _) => {
                    camera_pos += camera_front.cross(global_up).normalize() * camera_speed * delta_time;
                },
                (events::Event::MouseMotion { win_id: _, mouse_id: _, button_state: _, x_win, y_win, x_delta, y_delta }, _) => {
                    yaw += x_delta as f32 * 0.1;
                    pitch -= y_delta as f32 * 0.1;

                    if pitch >= 89.0 {
                        pitch = 89.0;
                    }
                    if pitch <= -89.0 {
                        pitch = -89.0;
                    }

                    let mut look_direction = glam::Vec3::ZERO;
                    look_direction.x = yaw.to_radians().cos() * pitch.to_radians().cos();
                    look_direction.y = pitch.to_radians().sin();
                    look_direction.z = yaw.to_radians().sin() * pitch.to_radians().cos();
                    camera_front = look_direction.normalize();
                },
                (events::Event::MouseWheel { win_id: _, mouse_id: _, x: _, y }, _) => {
                    fov -= y as f32;
                    if fov < 1.0 {
                        fov = 1.0;
                    }
                    if fov > 45.0 {
                        fov = 45.0
                    }
                },
                _ => (),
            }
        }
        // Now events are clear

        // Here is the spot to change the world state and draw

        unsafe {
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT );

            //glDrawArrays(GL_TRIANGLES, 0, 3);

            // The default texture unit for a texture is 0 which is the default active texture unit
            // so we didn't need to assign a location in the previous section
            glActiveTexture(GL_TEXTURE0);
            glBindTexture(GL_TEXTURE_2D, texture_wooden_crate);
            glActiveTexture(GL_TEXTURE1);
            glBindTexture(GL_TEXTURE_2D, texture_face);

            // Compute matrix

            let time_value = now.elapsed().unwrap().as_secs_f32();


            //glUniformMatrix4fv(location_model, 1, 0, model_matrix.to_cols_array().as_ptr());

            // CAMERA SETUP

            let camera_target = camera_pos + camera_front;
            let camera_direction = (camera_pos - camera_target).normalize();
            let camera_right = global_up.cross(camera_direction).normalize();

            let camera_up = camera_direction.cross(camera_right);

            let view_matrix_3 = glam::mat3(camera_right, camera_up, camera_direction).transpose();
            let mut transpose_matrix = glam::Mat4::IDENTITY;
            transpose_matrix.w_axis = glam::Vec4::from_array([-camera_pos[0], -camera_pos[1], -camera_pos[2], 1.0]);
            let view_matrix = glam::Mat4::from_mat3(view_matrix_3) * transpose_matrix;

            //let view_matrix = glam::Mat4::look_at_rh(camera_pos, camera_target, global_up);

            glUniformMatrix4fv(location_view, 1, 0, view_matrix.to_cols_array().as_ptr());

            let mut projection_matrix = glam::Mat4::IDENTITY;
            projection_matrix = projection_matrix * glam::Mat4::perspective_rh_gl(fov.to_radians(), 800.0/600.0, 0.1, 100.0);
            glUniformMatrix4fv(location_projection, 1, 0, projection_matrix.to_cols_array().as_ptr());

            glBindVertexArray(vao);

            for i in 0..cube_positions.len() {
                let model_matrix = glam::Mat4::from_translation(cube_positions[i]) * glam::Mat4::from_rotation_x(-PI/3.0 * time_value);
                glUniformMatrix4fv(location_model, 1, 0, model_matrix.to_cols_array().as_ptr());

                glDrawArrays(GL_TRIANGLES, 0, 36);    
            }

            //glDrawArrays(GL_TRIANGLES, 0, 36);

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
