/// Represents the type of a shader expression.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShaderType {
    I32,
    F32,
    Vec2,
    Vec3,
    Vec4,
    Mat4,
    Sampler2D,
}

impl ShaderType {
    /// Get the number of binding locations this type occupies in a shader.
    /// This is usually equal to `components / 4`, rounded up.
    pub fn location_count(self) -> usize {
        match self {
            ShaderType::I32
            | ShaderType::F32
            | ShaderType::Vec2
            | ShaderType::Vec3
            | ShaderType::Vec4
            | ShaderType::Sampler2D => 1,
            ShaderType::Mat4 => 4,
        }
    }

    /// Get the GLSL type name for this type.
    /// This is the name of the type as it appears in GLSL code.
    pub fn glsl_name(self) -> &'static str {
        match self {
            ShaderType::I32 => "int",
            ShaderType::F32 => "float",
            ShaderType::Vec2 => "vec2",
            ShaderType::Vec3 => "vec3",
            ShaderType::Vec4 => "vec4",
            ShaderType::Mat4 => "mat4",
            ShaderType::Sampler2D => "sampler2D",
        }
    }
}