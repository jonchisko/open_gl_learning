use gl33::{
    global_loader::{
        glAttachShader, glBindBuffer, glBindVertexArray, glBufferData, glClearColor,
        glCompileShader, glCreateProgram, glCreateShader, glDeleteProgram, glDeleteShader,
        glGenBuffers, glGenVertexArrays, glGetProgramInfoLog, glGetProgramiv, glGetShaderInfoLog,
        glGetShaderiv, glLinkProgram, glShaderSource, glUseProgram,
    },
    GLenum, GL_ARRAY_BUFFER, GL_COMPILE_STATUS, GL_ELEMENT_ARRAY_BUFFER, GL_FRAGMENT_SHADER,
    GL_INFO_LOG_LENGTH, GL_LINK_STATUS, GL_VERTEX_SHADER,
};

/// Clear the buffer with the following color.
pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
    // Nothing can go wrong, pub fn is safe
    unsafe { glClearColor(r, g, b, a) };
}

/// Basic wrapper for a vertex array object.
pub struct VertexArray(pub u32);

impl VertexArray {
    /// Creates a new vertex array object.
    pub fn new() -> Option<Self> {
        let mut vao = 0u32;
        unsafe {
            glGenVertexArrays(1, &mut vao);
        }
        if vao != 0 {
            Some(Self(vao))
        } else {
            None
        }
    }

    /// Bind this vertex array as the current vertex array object.
    /// With glBindVertexArray we would make the "vao" (vertex array object) as the active VAO.
    /// This is context wide effect and all functions now operate on this vao.
    pub fn bind(&self) {
        glBindVertexArray(self.0);
    }

    /// Clear the current vertex array object binding.
    pub fn clear_binding() {
        glBindVertexArray(0);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
    /// Array Buffers hold arrays of vertex data for drawing.
    Array,
    /// Element Array Buffers hold indexes of what vertices to use for dawing.
    ElementArray,
}

impl Into<GLenum> for BufferType {
    fn into(self) -> GLenum {
        match self {
            BufferType::Array => GL_ARRAY_BUFFER,
            BufferType::ElementArray => GL_ELEMENT_ARRAY_BUFFER,
        }
    }
}

/// Basic wrapper for a buffer object.
pub struct Buffer(pub u32);

impl Buffer {
    /// Makes a new vertex buffer.
    pub fn new() -> Option<Self> {
        let mut vbo = 0;
        unsafe {
            glGenBuffers(1, &mut vbo);
        }

        if vbo != 0 {
            Some(Self(vbo))
        } else {
            None
        }
    }

    /// Bind this vertex buffer for the given type.
    pub fn bind(&self, buffer_type: BufferType) {
        unsafe {
            glBindBuffer(buffer_type.into(), self.0);
        }
    }

    /// Clear the current vertex buffer binding for the given type.
    pub fn clear_binding(buffer_type: BufferType) {
        unsafe {
            glBindBuffer(buffer_type.into(), 0);
        }
    }
}

/// Places a slice of data into a previously-bound buffer.
pub fn buffer_data(buffer_type: BufferType, data: &[u8], usage: GLenum) {
    unsafe {
        glBufferData(
            buffer_type.into(),
            data.len().try_into().unwrap(),
            data.as_ptr().cast(),
            usage,
        );
    }
}

/// The types of shader object.
pub enum ShaderType {
    /// Vertex shaders determine the position of geometry within the screen.
    Vertex,
    /// Fragment shaders determine the color output of geometry.
    Fragment,
}

impl Into<GLenum> for ShaderType {
    fn into(self) -> GLenum {
        match self {
            ShaderType::Vertex => GL_VERTEX_SHADER,
            ShaderType::Fragment => GL_FRAGMENT_SHADER,
        }
    }
}

/// Basic wrapper of a shader object
pub struct Shader(pub u32);

impl Shader {
    /// Makes a new shader.
    ///
    /// Prefer the [`Shader::from_source`](Shader::from_source) method.
    ///
    /// Possibly skip the direct creation of the shader object and use
    /// [`ShaderProgram::from_vert_frag`](ShaderProgram::from_vert_frag) directly.
    pub fn new(shader_type: ShaderType) -> Option<Self> {
        let shader = glCreateShader(shader_type.into());
        if shader != 0 {
            return Some(Self(shader));
        } else {
            None
        }
    }

    /// Assigns (thus replaces previous) source string to the shader.
    pub fn set_source(&self, src: &str) {
        unsafe {
            glShaderSource(
                self.0,
                1,
                &(src.as_bytes().as_ptr().cast()),
                &(src.len().try_into().unwrap()),
            );
        }
    }

    /// Compiles the shader based on the current source.
    pub fn compile(&self) {
        glCompileShader(self.0);
    }

    /// Check if previously execute compile was successful.
    pub fn compile_success(&self) -> bool {
        let mut compiled = 0;
        unsafe { glGetShaderiv(self.0, GL_COMPILE_STATUS, &mut compiled) }

        compiled != 0
    }

    /// Marks a shader for deletion.
    ///
    /// This only marks the shader for deletion. It deletes it only after it is unattached.
    pub fn delete(self) {
        glDeleteShader(self.0);
    }

    /// Takes a shader type and source string and produces either the compiled shader or an error message.
    ///
    /// Prefer [`ShaderProgram::from_vert_frag`](ShaderProgram::from_vert_frag).
    pub fn from_source(shader_type: ShaderType, source: &str) -> Result<Self, String> {
        let id =
            Self::new(shader_type).ok_or_else(|| "Could not allocate new shader".to_string())?;

        id.set_source(source);
        id.compile();

        if id.compile_success() {
            Ok(id)
        } else {
            let out = id.get_info_log();
            id.delete();
            Err(out)
        }
    }
}

impl InfoLog for Shader {
    fn get_info_length(&self) -> i32 {
        let mut needed_len = 0;
        unsafe {
            glGetShaderiv(self.0, GL_INFO_LOG_LENGTH, &mut needed_len);
        }

        needed_len
    }

    fn write_info_log(&self, buffer: &mut Vec<u8>) -> i32 {
        let mut len_written = 0i32;

        unsafe {
            glGetShaderInfoLog(
                self.0,
                buffer.capacity().try_into().unwrap(),
                &mut len_written,
                buffer.as_mut_ptr().cast(),
            );
        }

        len_written
    }
}

/// Basic wrapper for shader program.
pub struct ShaderProgram(pub u32);

impl ShaderProgram {
    /// Allocates a new program object.
    ///
    /// Prefer [`ShaderProgram::from_vert_frag`](ShaderProgram::from_vert_frag),
    /// it makes a complete program from the vertex and fragment sources all at
    /// once.
    pub fn new() -> Option<Self> {
        let prog = glCreateProgram();
        if prog != 0 {
            Some(Self(prog))
        } else {
            None
        }
    }

    /// Attaches a shader object to this program object.
    pub fn attach_shader(&self, shader: &Shader) {
        glAttachShader(self.0, shader.0);
    }

    /// Links the various attached, compiled shader objects into a usable program.
    pub fn link_program(&self) {
        glLinkProgram(self.0);
    }

    /// Checks if the last linking operation was successful.
    pub fn link_success(&self) -> bool {
        let mut success = 0;
        unsafe { glGetProgramiv(self.0, GL_LINK_STATUS, &mut success) };

        success != 0
    }

    /// Sets the program as the program to use when drawing.
    pub fn use_program(&self) {
        glUseProgram(self.0);
    }

    /// Marks the program for deletion.
    ///
    /// Note: This _does not_ immediately delete the program. If the program is
    /// currently in use it won't be deleted until it's not the active program.
    /// When a program is finally deleted and attached shaders are unattached.
    pub fn delete(self) {
        glDeleteProgram(self.0);
    }

    /// Takes a vertex shader source string and a fragment shader source string
    /// and either gets you a working program object or gets you an error message.
    ///
    /// This is the preferred way to create a simple shader program in the common
    /// case. It's just less error prone than doing all the steps yourself.
    pub fn from_vert_frag(vert: &str, frag: &str) -> Result<Self, String> {
        let p = Self::new().ok_or_else(|| "Couldn't allocate a program".to_string())?;

        let v = Shader::from_source(ShaderType::Vertex, vert)
            .map_err(|e| format!("Vertex Compile Error: {}", e))?;

        let f = Shader::from_source(ShaderType::Fragment, frag)
            .map_err(|e| format!("Fragment Compile Error: {}", e))?;

        p.attach_shader(&v);
        p.attach_shader(&f);
        p.link_program();

        v.delete();
        f.delete();

        if p.link_success() {
            Ok(p)
        } else {
            let out = format!("Program Link Error: {}", p.get_info_log());
            p.delete();
            Err(out)
        }
    }
}

impl InfoLog for ShaderProgram {
    fn get_info_length(&self) -> i32 {
        let mut needed_len = 0;
        unsafe {
            glGetProgramiv(self.0, GL_INFO_LOG_LENGTH, &mut needed_len);
        }

        needed_len
    }

    fn write_info_log(&self, buffer: &mut Vec<u8>) -> i32 {
        let mut len_written = 0;
        unsafe {
            glGetProgramInfoLog(
                self.0,
                buffer.capacity().try_into().unwrap(),
                &mut len_written,
                buffer.as_mut_ptr().cast(),
            );
        }

        len_written
    }
}

pub trait InfoLog {
    fn get_info_length(&self) -> i32;
    fn write_info_log(&self, buffer: &mut Vec<u8>) -> i32;

    fn get_info_log(&self) -> String {
        let info_length = self.get_info_length();
        assert!(info_length >= 0);
        let info_length = info_length as usize;

        let mut v: Vec<u8> = Vec::with_capacity(info_length);

        let len_written = self.write_info_log(&mut v);
        assert!(len_written >= 0);
        let len_written = len_written as usize;

        // elements are initialized till len_written, check for capacity
        unsafe {
            assert!(len_written <= v.capacity());
            v.set_len(len_written);
        }

        String::from_utf8_lossy(&v).into_owned()
    }
}
