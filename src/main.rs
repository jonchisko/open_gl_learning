// Make it windows and not console app. Doesnt open the terminal
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use beryllium::{
    events::{SDLK_a, SDLK_d, SDLK_s, SDLK_w, SDLK_6, SDLK_UP},
    *,
};
use gl33::{
    global_loader::{
        glActiveTexture, glAttachShader, glBindBuffer, glBindTexture, glBindVertexArray,
        glBufferData, glClear, glClearColor, glCompileShader, glCreateProgram, glCreateShader,
        glDeleteBuffers, glDeleteProgram, glDeleteShader, glDeleteVertexArrays,
        glDisableVertexAttribArray, glDrawArrays, glDrawElements, glEnable,
        glEnableVertexAttribArray, glGenBuffers, glGenTextures, glGenVertexArrays,
        glGenerateMipmap, glGetIntegerv, glGetProgramInfoLog, glGetProgramiv, glGetShaderInfoLog,
        glGetShaderiv, glGetUniformLocation, glLinkProgram, glShaderSource, glTexImage2D,
        glTexParameteri, glUniform1i, glUniform3f, glUniform4f, glUniformMatrix4fv, glUseProgram,
        glVertexAttribPointer, load_global_gl,
    },
    *,
};
use glam::{vec4, Vec4, Vec4Swizzles};

use std::{
    f32::consts::PI,
    ffi::{CStr, CString},
    mem,
    time::SystemTime,
};

use image::ImageReader;

#[rustfmt::skip]
fn get_vertices() -> [f32; 108] {
    [ // I coppied the data from learnopengl, 1.5 -> 1.0 and 0.5 to 0.0 but i am too lazy
    -0.5, -0.5, -0.5,
     0.5, -0.5, -0.5,
     0.5,  0.5, -0.5,
     0.5,  0.5, -0.5,
    -0.5,  0.5, -0.5,
    -0.5, -0.5, -0.5,
    -0.5, -0.5,  0.5,
     0.5, -0.5,  0.5,
     0.5,  0.5,  0.5,
     0.5,  0.5,  0.5,
    -0.5,  0.5,  0.5,
    -0.5, -0.5,  0.5,
    -0.5,  0.5,  0.5,
    -0.5,  0.5, -0.5,
    -0.5, -0.5, -0.5,
    -0.5, -0.5, -0.5,
    -0.5, -0.5,  0.5,
    -0.5,  0.5,  0.5,
     0.5,  0.5,  0.5,
     0.5,  0.5, -0.5,
     0.5, -0.5, -0.5,
     0.5, -0.5, -0.5,
     0.5, -0.5,  0.5,
     0.5,  0.5,  0.5,
    -0.5, -0.5, -0.5,
     0.5, -0.5, -0.5,
     0.5, -0.5,  0.5,
     0.5, -0.5,  0.5,
    -0.5, -0.5,  0.5,
    -0.5, -0.5, -0.5,
    -0.5,  0.5, -0.5,
     0.5,  0.5, -0.5,
     0.5,  0.5,  0.5,
     0.5,  0.5,  0.5,
    -0.5,  0.5,  0.5,
    -0.5,  0.5, -0.5,
    ]
}

fn main() {
    // Specify you will be using open GL before creating the window
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);

    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_minor_version(3).unwrap();
    // Core is a subset of all the features the OpenGL provides
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();
    //sdl.set_relative_mouse_mode(true).unwrap();
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

    let mut vao = 0u32;
    unsafe {
        glGenVertexArrays(1, &mut vao);
    }
    assert!(vao != 0);
    glBindVertexArray(vao);

    let vertices = get_vertices();

    let mut vbo = 0u32;
    unsafe {
        glGenBuffers(1, &mut vbo);
    }
    assert!(vbo != 0);
    unsafe {
        glBindBuffer(GL_ARRAY_BUFFER, vbo);
        glBufferData(
            GL_ARRAY_BUFFER,
            (std::mem::size_of::<f32>() * vertices.len())
                .try_into()
                .unwrap(),
            vertices.as_ptr().cast(),
            GL_STATIC_READ,
        )
    };

    unsafe {
        glVertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            0,
            (3 * std::mem::size_of::<f32>()).try_into().unwrap(),
            0 as *const _,
        );
        glEnableVertexAttribArray(0);
    }

    glBindVertexArray(0);

    // Setup Light Source Object
    let mut vao_light_source = 0u32;
    unsafe {
        glGenVertexArrays(1, &mut vao_light_source);
    }
    assert!(vao_light_source != 0);
    glBindVertexArray(vao_light_source);

    unsafe { glBindBuffer(GL_ARRAY_BUFFER, vbo) };
    unsafe {
        glVertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            0,
            (3 * std::mem::size_of::<f32>()).try_into().unwrap(),
            0 as *const _,
        );
        glEnableVertexAttribArray(0);
    }

    glBindVertexArray(0);

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

        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 projection;

        void main() {
            gl_Position = projection * view * model * vec4(pos, 1.0);
        }
    "#;

    const FRAG_SHADER: &str = r#"#version 330 core
        out vec4 frag_color;

        uniform vec3 object_color;
        uniform vec3 light_color;

        void main() {
            frag_color = vec4(light_color * object_color, 1.0);
        }
    
    "#;

    const FRAG_SHADER_LIGHT_SOURCE: &str = r#"#version 330 core
        out vec4 frag_color;

        void main() {
            frag_color = vec4(1.0);
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
    }
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

    let fragment_shader_light_source = glCreateShader(GL_FRAGMENT_SHADER);
    assert!(fragment_shader_light_source != 0);
    unsafe {
        glShaderSource(
            fragment_shader_light_source,
            1,
            &(FRAG_SHADER_LIGHT_SOURCE.as_bytes().as_ptr().cast()),
            &(FRAG_SHADER_LIGHT_SOURCE.len().try_into().unwrap()),
        );
    }
    glCompileShader(fragment_shader_light_source);
    log_error(fragment_shader_light_source, true);

    let program_object = glCreateProgram();
    assert!(program_object != 0);
    glAttachShader(program_object, vertex_shader);
    glAttachShader(program_object, fragment_shader);
    glLinkProgram(program_object);
    log_error(program_object, false);

    let program_light = glCreateProgram();
    assert!(program_light != 0);
    glAttachShader(program_light, vertex_shader);
    glAttachShader(program_light, fragment_shader_light_source);
    glLinkProgram(program_light);
    log_error(program_light, false);

    glDeleteShader(vertex_shader);
    glDeleteShader(fragment_shader);
    glDeleteShader(fragment_shader_light_source);

    let _ = win.set_swap_interval(video::GlSwapInterval::Vsync);

    glUseProgram(program_object);

    let object_color = CString::new("object_color").unwrap();
    let light_color = CString::new("light_color").unwrap();

    let location_object_color =
        unsafe { glGetUniformLocation(program_object, object_color.as_ptr().cast()) };
    let location_light_color =
        unsafe { glGetUniformLocation(program_object, light_color.as_ptr().cast()) };
    assert!(location_object_color >= 0);
    assert!(location_light_color >= 0);

    unsafe {
        glUniform3f(location_object_color, 1.0, 0.5, 0.31);
        glUniform3f(location_light_color, 1.0, 1.0, 1.0);
    }

    unsafe {
        glEnable(GL_DEPTH_TEST);
    }

    'main: loop {
        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => break 'main,
                _ => (),
            }

            unsafe {
                glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

                glUseProgram(program_object);

                let model = CString::new("model").unwrap();
                let view = CString::new("view").unwrap();
                let projection = CString::new("projection").unwrap();

                let location_model =
                    unsafe { glGetUniformLocation(program_object, model.as_ptr().cast()) };
                let location_view =
                    unsafe { glGetUniformLocation(program_object, view.as_ptr().cast()) };
                let location_projection =
                    unsafe { glGetUniformLocation(program_object, projection.as_ptr().cast()) };
                assert!(location_model >= 0);
                assert!(location_view >= 0);
                assert!(location_projection >= 0);

                let position = glam::Vec3::new(-2.0, 1.0, 2.0);
                let translation = glam::Mat4::from_translation(position);
                let rotation = glam::Mat4::IDENTITY;
                let scale = glam::Mat4::IDENTITY * 4.0;
                let model_matrix = translation * rotation * scale;
                glUniformMatrix4fv(location_model, 1, 0, model_matrix.to_cols_array().as_ptr());

                // CAMERA SETUP
                let view_matrix = glam::Mat4::look_at_rh(
                    glam::Vec3::new(0.0, 2.0, -10.0),
                    glam::Vec3::new(0.0, 0.0, 0.0),
                    glam::Vec3::new(0.0, 1.0, 0.0),
                );
                glUniformMatrix4fv(location_view, 1, 0, view_matrix.to_cols_array().as_ptr());

                let mut projection_matrix = glam::Mat4::perspective_rh_gl(
                    (45.0f32).to_radians(),
                    800.0 / 600.0,
                    0.1,
                    100.0,
                );
                glUniformMatrix4fv(
                    location_projection,
                    1,
                    0,
                    projection_matrix.to_cols_array().as_ptr(),
                );

                glBindVertexArray(vao);
                glDrawArrays(GL_TRIANGLES, 0, 36);

                glUseProgram(program_light);

                let model = CString::new("model").unwrap();
                let view = CString::new("view").unwrap();
                let projection = CString::new("projection").unwrap();

                let location_model =
                    unsafe { glGetUniformLocation(program_light, model.as_ptr().cast()) };
                let location_view =
                    unsafe { glGetUniformLocation(program_light, view.as_ptr().cast()) };
                let location_projection =
                    unsafe { glGetUniformLocation(program_light, projection.as_ptr().cast()) };
                assert!(location_model >= 0);
                assert!(location_view >= 0);
                assert!(location_projection >= 0);

                let position = glam::Vec3::new(2.0, -1.0, 0.0);
                let translation = glam::Mat4::from_translation(position);

                let rotation = glam::Mat4::IDENTITY;
                let scale = glam::Mat4::IDENTITY * 1.0;
                let model_matrix = translation * rotation * scale;
                glUniformMatrix4fv(location_model, 1, 0, model_matrix.to_cols_array().as_ptr());
                // CAMERA SETUP
                let view_matrix = glam::Mat4::look_at_rh(
                    glam::Vec3::new(0.0, 2.0, -10.0),
                    glam::Vec3::new(0.0, 0.0, 0.0),
                    glam::Vec3::new(0.0, 1.0, 0.0),
                );
                glUniformMatrix4fv(location_view, 1, 0, view_matrix.to_cols_array().as_ptr());

                let mut projection_matrix = glam::Mat4::perspective_rh_gl(
                    (45.0f32).to_radians(),
                    800.0 / 600.0,
                    0.1,
                    100.0,
                );
                glUniformMatrix4fv(
                    location_projection,
                    1,
                    0,
                    projection_matrix.to_cols_array().as_ptr(),
                );

                glBindVertexArray(vao_light_source);
                glDrawArrays(GL_TRIANGLES, 0, 36);

                win.swap_window();
            }
        }
    }

    unsafe {
        glDeleteVertexArrays(1, &vao);
        glDeleteVertexArrays(1, &vao_light_source);
        glDeleteBuffers(1, &vbo);
        glDeleteProgram(program_light);
        glDeleteProgram(program_object);
    }

    /*


    // Processing events - we have to, OS otherwise thinks the application has stalled
    'main_loop: loop {

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
    }*/
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
