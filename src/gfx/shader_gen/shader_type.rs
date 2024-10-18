use anyhow::Result;

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

    /// Get the Rust type name for this type.
    /// This is the name of the type as it appears in Rust code.
    pub fn rust_name(self) -> &'static str {
        match self {
            ShaderType::I32 => "i32",
            ShaderType::F32 => "f32",
            ShaderType::Vec2 => "Vector2<f32>",
            ShaderType::Vec3 => "Vector3<f32>",
            ShaderType::Vec4 => "Vector4<f32>",
            ShaderType::Mat4 => "Matrix4x4<f32>",
            ShaderType::Sampler2D => "TextureView",
        }
    }

    /// Get the component count for this type.
    pub fn component_count(self) -> Option<usize> {
        match self {
            ShaderType::I32 | ShaderType::F32 => Some(1),
            ShaderType::Vec2 => Some(2),
            ShaderType::Vec3 => Some(3),
            ShaderType::Vec4 => Some(4),
            ShaderType::Mat4 => Some(16),
            ShaderType::Sampler2D => None,
        }
    }

    /// Get the component type for this type (or the type itself if it is a scalar).
    pub fn component_type(self) -> Option<ShaderType> {
        match self {
            ShaderType::I32 | ShaderType::F32 => Some(self),
            ShaderType::Vec2 | ShaderType::Vec3 | ShaderType::Vec4 | ShaderType::Mat4 => {
                Some(ShaderType::F32)
            }
            ShaderType::Sampler2D => None,
        }
    }

    /// Get the component type for this type (or the type itself if it is a scalar).
    /// Returns an error if the type is not a vector or scalar.
    /// The error message will be decorated with the given name in `origin_object`.
    pub fn ensure_vector_or_scalar(self, origin_object: impl AsRef<str>) -> Result<ShaderType> {
        self.component_type().ok_or_else(|| {
            anyhow::anyhow!("{} is not a vector or scalar type", origin_object.as_ref())
        })
    }

    /// Returns an error if the type is not a float vector or scalar.
    /// The error message will be decorated with the given name in `origin_object`.
    pub fn ensure_vector_or_scalar_f32(self, origin_object: impl AsRef<str>) -> Result<ShaderType> {
        if self.component_count().is_some() && self.component_type() == Some(ShaderType::F32) {
            Ok(self)
        } else {
            Err(anyhow::anyhow!(
                "{} is not {} or a Vector type",
                origin_object.as_ref(),
                ShaderType::F32.rust_name()
            ))
        }
    }

    /// Returns an error if the type is not a vector.
    /// The error message will be decorated with the given name in `origin_object`.
    pub fn ensure_vector(self, origin_object: impl AsRef<str>) -> Result<ShaderType> {
        if let Some(count) = self.component_count() {
            if count > 1 {
                Ok(self)
            } else {
                Err(anyhow::anyhow!(
                    "{} is not a Vector type",
                    origin_object.as_ref()
                ))
            }
        } else {
            Err(anyhow::anyhow!(
                "{} is not a Vector type",
                origin_object.as_ref()
            ))
        }
    }

    /// Returns an error if the type is not a float vector.
    /// The error message will be decorated with the given name in `origin_object`.
    pub fn ensure_vector_f32(self, origin_object: impl AsRef<str>) -> Result<ShaderType> {
        if let Some(count) = self.component_count() {
            if count > 1 && self.component_type() == Some(ShaderType::F32) {
                Ok(self)
            } else {
                Err(anyhow::anyhow!(
                    "{} is not a Vector type of {}",
                    origin_object.as_ref(),
                    ShaderType::F32.rust_name()
                ))
            }
        } else {
            Err(anyhow::anyhow!(
                "{} is not a Vector type of {}",
                origin_object.as_ref(),
                ShaderType::F32.rust_name()
            ))
        }
    }

    /// Returns an error if the type is not a scalar.
    /// The error message will be decorated with the given name in `origin_object`.
    pub fn ensure_scalar(self, origin_object: impl AsRef<str>) -> Result<ShaderType> {
        if self.component_count() == Some(1) {
            Ok(self)
        } else {
            Err(anyhow::anyhow!(
                "{} is not a scalar type",
                origin_object.as_ref()
            ))
        }
    }

    /// Returns an error if the two types do not match.
    /// The error message will be decorated with the given name in `origin_pair`.
    pub fn ensure_matches(self, other: ShaderType, origin_pair: impl AsRef<str>) -> Result<()> {
        if self == other {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "{} do not match: {} and {}",
                origin_pair.as_ref(),
                self.rust_name(),
                other.rust_name()
            ))
        }
    }

    /// Returns an error if the two types are not compatible for a binary math operation.
    /// The error message will be decorated with the given name in `origin_operation`.
    pub fn ensure_math_compatible(
        self,
        other: ShaderType,
        origin_operation: impl AsRef<str>,
    ) -> Result<()> {
        let origin_operation = origin_operation.as_ref();

        // Generate the names for error messages
        let left_name = format!("left side of '{}'", origin_operation);
        let right_name = format!("right side of '{}'", origin_operation);
        let pair_name = format!("left and right sides of '{}'", origin_operation);

        // Ensure the types are vectors or scalars
        let self_component = self.ensure_vector_or_scalar(left_name)?;
        let other_component = other.ensure_vector_or_scalar(right_name)?;

        // Ensure the components match
        self_component.ensure_matches(other_component, pair_name)?;

        // Ensure the component counts for both sides are valid.
        // Both sides can be scalars, both sides can be vectors of the same length, or one side can be a scalar and the other a vector.
        let self_count = self.component_count().unwrap_or(1);
        let other_count = other.component_count().unwrap_or(1);
        match (self_count, other_count) {
            (1, _) | (_, 1) => Ok(()),
            (a, b) if a == b => Ok(()),
            _ => Err(anyhow::anyhow!(
                "Left and right sides of '{}' have invalid types: {} and {}",
                origin_operation,
                self.rust_name(),
                other.rust_name()
            )),
        }
    }

    /// Returns an error if this type is not found in the given list.
    /// The error message will be decorated with the given name in `origin_object`.
    /// The error message will also include the list of expected types.
    pub fn ensure_in_list(self, list: &[ShaderType], origin_object: impl AsRef<str>) -> Result<()> {
        if list.contains(&self) {
            Ok(())
        } else {
            let list_str = list
                .iter()
                .cloned()
                .map(ShaderType::rust_name)
                .collect::<Vec<_>>()
                .join(", ");
            Err(anyhow::anyhow!(
                "{} was type {} but expected one of: {}",
                origin_object.as_ref(),
                self.rust_name(),
                list_str,
            ))
        }
    }

    /// Returns an error if this type is not the `expected` type.
    /// The error message will be decorated with the given name in `origin_object`.
    pub fn ensure_type(self, expected: ShaderType, origin_object: impl AsRef<str>) -> Result<()> {
        if self == expected {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "{} was type {} but expected type {}",
                origin_object.as_ref(),
                self.rust_name(),
                expected.rust_name(),
            ))
        }
    }
}
