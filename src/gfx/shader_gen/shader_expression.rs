use std::{cell::RefCell, fmt::Display};

use anyhow::Result;
use ggmath::prelude::*;

use super::{
    shader_inputs::SHADER_INPUT_PREFIX, shader_parameters::SHADER_UNIFORM_PREFIX,
    shader_type::ShaderType,
};

/// Represents a shader operation within a shader expression.
pub enum ShaderOperation {
    Input(String, ShaderType),
    Uniform(String, ShaderType),
    I32(i32),
    F32(f32),
    Vec2(ShaderExpression, ShaderExpression),
    Vec3(ShaderExpression, ShaderExpression, ShaderExpression),
    Vec4(
        ShaderExpression,
        ShaderExpression,
        ShaderExpression,
        ShaderExpression,
    ),
    Append(ShaderExpression, ShaderExpression),
    Add(ShaderExpression, ShaderExpression),
    Sub(ShaderExpression, ShaderExpression),
    Mul(ShaderExpression, ShaderExpression),
    Div(ShaderExpression, ShaderExpression),
    Pow(ShaderExpression, ShaderExpression),
    Rem(ShaderExpression, ShaderExpression),
    Neg(ShaderExpression),
    Abs(ShaderExpression),
    Sign(ShaderExpression),
    Floor(ShaderExpression),
    Ceil(ShaderExpression),
    Round(ShaderExpression),
    Min(ShaderExpression, ShaderExpression),
    Max(ShaderExpression, ShaderExpression),
    Clamp(ShaderExpression, ShaderExpression, ShaderExpression),
    Mix(ShaderExpression, ShaderExpression, ShaderExpression),
    Dot(ShaderExpression, ShaderExpression),
    Cross(ShaderExpression, ShaderExpression),
    Length(ShaderExpression),
    Normalized(ShaderExpression),
    Sample(ShaderExpression, ShaderExpression, ShaderExpression),
}

/// Represents a shader expression.
pub struct ShaderExpression {
    operation: Box<RefCell<ShaderOperation>>,
}

impl ShaderExpression {
    /// Creates a new shader expression from the given operation.
    pub fn new(operation: ShaderOperation) -> Self {
        ShaderExpression {
            operation: Box::new(RefCell::new(operation)),
        }
    }

    /// Returns the type of the shader expression.
    pub fn shader_type(&self) -> Result<ShaderType> {
        Ok(match &*self.operation.borrow() {
            ShaderOperation::Input(_, value_type) => *value_type,
            ShaderOperation::Uniform(_, value_type) => *value_type,
            ShaderOperation::I32(_) => ShaderType::I32,
            ShaderOperation::F32(_) => ShaderType::F32,
            ShaderOperation::Vec2(_, _) => ShaderType::Vec2,
            ShaderOperation::Vec3(_, _, _) => ShaderType::Vec3,
            ShaderOperation::Vec4(_, _, _, _) => ShaderType::Vec4,
            ShaderOperation::Append(left, right) => match left.shader_type()? {
                ShaderType::I32 | ShaderType::F32 => match right.shader_type()? {
                    ShaderType::I32 | ShaderType::F32 => ShaderType::Vec2,
                    ShaderType::Vec2 => ShaderType::Vec3,
                    ShaderType::Vec3 => ShaderType::Vec4,
                    right => {
                        return Err(anyhow::anyhow!(
                            "Right side of append operation has wrong type: {:?}",
                            right
                        ))
                    }
                },
                ShaderType::Vec2 => match right.shader_type()? {
                    ShaderType::I32 | ShaderType::F32 => ShaderType::Vec3,
                    ShaderType::Vec2 => ShaderType::Vec4,
                    right => {
                        return Err(anyhow::anyhow!(
                            "Right side of append operation has wrong type: {:?}",
                            right
                        ))
                    }
                },
                ShaderType::Vec3 => match right.shader_type()? {
                    ShaderType::I32 | ShaderType::F32 => ShaderType::Vec4,
                    right => {
                        return Err(anyhow::anyhow!(
                            "Right side of append operation has wrong type: {:?}",
                            right
                        ))
                    }
                },
                left => {
                    return Err(anyhow::anyhow!(
                        "Left side of append operation has wrong type: {:?}",
                        left
                    ))
                }
            },
            ShaderOperation::Add(left, _) => left.shader_type()?,
            ShaderOperation::Sub(left, _) => left.shader_type()?,
            ShaderOperation::Mul(left, right) => match left.shader_type()? {
                ShaderType::Mat4 => match right.shader_type()? {
                    ShaderType::Vec4 => ShaderType::Vec4,
                    ShaderType::Mat4 => ShaderType::Mat4,
                    right => {
                        return Err(anyhow::anyhow!(
                            "Right side of mul operation has wrong type: {:?}",
                            right
                        ))
                    }
                },
                left => left,
            },
            ShaderOperation::Div(left, _) => left.shader_type()?,
            ShaderOperation::Pow(left, _) => left.shader_type()?,
            ShaderOperation::Rem(left, _) => left.shader_type()?,
            ShaderOperation::Neg(expr) => expr.shader_type()?,
            ShaderOperation::Abs(expr) => expr.shader_type()?,
            ShaderOperation::Sign(expr) => expr.shader_type()?,
            ShaderOperation::Floor(expr) => expr.shader_type()?,
            ShaderOperation::Ceil(expr) => expr.shader_type()?,
            ShaderOperation::Round(expr) => expr.shader_type()?,
            ShaderOperation::Min(left, _) => left.shader_type()?,
            ShaderOperation::Max(left, _) => left.shader_type()?,
            ShaderOperation::Clamp(left, _, _) => left.shader_type()?,
            ShaderOperation::Mix(left, _, _) => left.shader_type()?,
            ShaderOperation::Dot(_, _) => ShaderType::F32,
            ShaderOperation::Cross(_, _) => ShaderType::Vec3,
            ShaderOperation::Length(_) => ShaderType::F32,
            ShaderOperation::Normalized(expr) => expr.shader_type()?,
            ShaderOperation::Sample(_, _, _) => ShaderType::Vec4,
        })
    }
}

impl From<i32> for ShaderExpression {
    fn from(value: i32) -> Self {
        ShaderExpression::new(ShaderOperation::I32(value))
    }
}

impl From<f32> for ShaderExpression {
    fn from(value: f32) -> Self {
        ShaderExpression::new(ShaderOperation::F32(value))
    }
}

impl From<Vector2<f32>> for ShaderExpression {
    fn from(value: Vector2<f32>) -> Self {
        ShaderExpression::new(ShaderOperation::Vec2(
            ShaderExpression::from(value.x()),
            ShaderExpression::from(value.y()),
        ))
    }
}

impl From<Vector2<i32>> for ShaderExpression {
    fn from(value: Vector2<i32>) -> Self {
        ShaderExpression::new(ShaderOperation::Vec2(
            ShaderExpression::from(value.x()),
            ShaderExpression::from(value.y()),
        ))
    }
}

impl From<Vector3<f32>> for ShaderExpression {
    fn from(value: Vector3<f32>) -> Self {
        ShaderExpression::new(ShaderOperation::Vec3(
            ShaderExpression::from(value.x()),
            ShaderExpression::from(value.y()),
            ShaderExpression::from(value.z()),
        ))
    }
}

impl From<Vector3<i32>> for ShaderExpression {
    fn from(value: Vector3<i32>) -> Self {
        ShaderExpression::new(ShaderOperation::Vec3(
            ShaderExpression::from(value.x()),
            ShaderExpression::from(value.y()),
            ShaderExpression::from(value.z()),
        ))
    }
}

impl From<Vector4<f32>> for ShaderExpression {
    fn from(value: Vector4<f32>) -> Self {
        ShaderExpression::new(ShaderOperation::Vec4(
            ShaderExpression::from(value.x()),
            ShaderExpression::from(value.y()),
            ShaderExpression::from(value.z()),
            ShaderExpression::from(value.w()),
        ))
    }
}

impl From<Vector4<i32>> for ShaderExpression {
    fn from(value: Vector4<i32>) -> Self {
        ShaderExpression::new(ShaderOperation::Vec4(
            ShaderExpression::from(value.x()),
            ShaderExpression::from(value.y()),
            ShaderExpression::from(value.z()),
            ShaderExpression::from(value.w()),
        ))
    }
}

pub trait ShaderMath: Into<ShaderExpression> + Sized {
    /// Appends two values.
    fn append(self, other: impl Into<ShaderExpression>) -> ShaderExpression {
        let a: ShaderExpression = self.into();
        let b: ShaderExpression = other.into();

        // Get the types of the expressions.
        let a_type = a.shader_type().unwrap();
        let b_type = b.shader_type().unwrap();

        // Ensure the types are valid for appending.
        a_type.ensure_math_compatible(b_type, "append").unwrap();

        // Ensure that the total component count is less than or equal to 4.
        let total_components =
            a_type.component_count().unwrap() + b_type.component_count().unwrap();
        if total_components > 4 {
            panic!(
                "Cannot create vector with more than 4 components: {:?} + {:?} = {} components",
                a_type, b_type, total_components
            );
        }

        ShaderExpression::new(ShaderOperation::Append(a, b))
    }

    /// Adds two values.
    fn add(self, other: impl Into<ShaderExpression>) -> ShaderExpression {
        let a: ShaderExpression = self.into();
        let b: ShaderExpression = other.into();

        // Ensure the types are valid for addition.
        let a_type = a.shader_type().unwrap();
        let b_type = b.shader_type().unwrap();
        a_type.ensure_math_compatible(b_type, "add").unwrap();

        ShaderExpression::new(ShaderOperation::Add(a, b))
    }

    /// Subtracts two values.
    fn sub(self, other: impl Into<ShaderExpression>) -> ShaderExpression {
        let a: ShaderExpression = self.into();
        let b: ShaderExpression = other.into();

        // Ensure the types are valid for subtraction.
        let a_type = a.shader_type().unwrap();
        let b_type = b.shader_type().unwrap();
        a_type.ensure_math_compatible(b_type, "sub").unwrap();

        ShaderExpression::new(ShaderOperation::Sub(a, b))
    }

    /// Multiplies two values.
    fn mul(self, other: impl Into<ShaderExpression>) -> ShaderExpression {
        let a: ShaderExpression = self.into();
        let b: ShaderExpression = other.into();

        // Ensure the types are valid for multiplication.
        let a_type = a.shader_type().unwrap();
        let b_type = b.shader_type().unwrap();
        a_type.ensure_math_compatible(b_type, "mul").unwrap();

        ShaderExpression::new(ShaderOperation::Mul(a, b))
    }

    /// Divides two values.
    fn div(self, other: impl Into<ShaderExpression>) -> ShaderExpression {
        let a: ShaderExpression = self.into();
        let b: ShaderExpression = other.into();

        // Ensure the types are valid for division.
        let a_type = a.shader_type().unwrap();
        let b_type = b.shader_type().unwrap();
        a_type.ensure_math_compatible(b_type, "div").unwrap();

        ShaderExpression::new(ShaderOperation::Div(a, b))
    }

    /// Raises the left side to the power of the right side.
    fn pow(self, other: impl Into<ShaderExpression>) -> ShaderExpression {
        let a: ShaderExpression = self.into();
        let b: ShaderExpression = other.into();

        // Ensure the types are valid for exponentiation.
        let a_type = a.shader_type().unwrap();
        let b_type = b.shader_type().unwrap();
        a_type
            .ensure_vector_or_scalar_f32("left side of 'pow'")
            .unwrap();
        b_type
            .ensure_type(ShaderType::F32, "right side of 'pow'")
            .unwrap();

        ShaderExpression::new(ShaderOperation::Pow(a, b))
    }

    /// Returns the remainder of the left side divided by the right side.
    fn rem(self, other: impl Into<ShaderExpression>) -> ShaderExpression {
        let a: ShaderExpression = self.into();
        let b: ShaderExpression = other.into();

        // Ensure the types are valid for remainder.
        let a_type = a.shader_type().unwrap();
        let b_type = b.shader_type().unwrap();
        let a_component = a_type
            .ensure_vector_or_scalar("left side of 'rem'")
            .unwrap();
        a_component
            .ensure_matches(
                b_type,
                "component/scalar of left side and right side of 'rem'",
            )
            .unwrap();

        ShaderExpression::new(ShaderOperation::Rem(a, b))
    }

    /// Negates the value.
    fn neg(self) -> ShaderExpression {
        let a: ShaderExpression = self.into();

        // Ensure the type is valid for negation.
        let a_type = a.shader_type().unwrap();
        a_type.ensure_vector_or_scalar("operand of 'neg'").unwrap();

        ShaderExpression::new(ShaderOperation::Neg(a))
    }

    /// Returns the absolute value of the value.
    fn abs(self) -> ShaderExpression {
        let a: ShaderExpression = self.into();

        // Ensure the type is valid for absolute value.
        let a_type = a.shader_type().unwrap();
        a_type.ensure_vector_or_scalar("operand of 'abs'").unwrap();

        ShaderExpression::new(ShaderOperation::Abs(a))
    }

    /// Returns the sign of the value.
    fn sign(self) -> ShaderExpression {
        let a: ShaderExpression = self.into();

        // Ensure the type is valid for sign.
        let a_type = a.shader_type().unwrap();
        a_type.ensure_vector_or_scalar("operand of 'sign'").unwrap();

        ShaderExpression::new(ShaderOperation::Sign(a))
    }

    /// Rounds the value down.
    fn floor(self) -> ShaderExpression {
        let a: ShaderExpression = self.into();

        // Ensure the type is valid for floor.
        let a_type = a.shader_type().unwrap();
        a_type
            .ensure_vector_or_scalar_f32("operand of 'floor'")
            .unwrap();

        ShaderExpression::new(ShaderOperation::Floor(a))
    }

    /// Rounds the value up.
    fn ceil(self) -> ShaderExpression {
        let a: ShaderExpression = self.into();

        // Ensure the type is valid for ceil.
        let a_type = a.shader_type().unwrap();
        a_type
            .ensure_vector_or_scalar_f32("operand of 'ceil'")
            .unwrap();

        ShaderExpression::new(ShaderOperation::Ceil(a))
    }

    /// Rounds the value to the nearest integer.
    fn round(self) -> ShaderExpression {
        let a: ShaderExpression = self.into();

        // Ensure the type is valid for round.
        let a_type = a.shader_type().unwrap();
        a_type
            .ensure_vector_or_scalar_f32("operand of 'round'")
            .unwrap();

        ShaderExpression::new(ShaderOperation::Round(a))
    }

    /// Returns the minimum of the two values.
    fn min(self, other: impl Into<ShaderExpression>) -> ShaderExpression {
        let a: ShaderExpression = self.into();
        let b: ShaderExpression = other.into();

        // Ensure the types are valid for min.
        let a_type = a.shader_type().unwrap();
        let b_type = b.shader_type().unwrap();
        a_type.ensure_math_compatible(b_type, "min").unwrap();

        ShaderExpression::new(ShaderOperation::Min(a, b))
    }

    /// Returns the maximum of the two values.
    fn max(self, other: impl Into<ShaderExpression>) -> ShaderExpression {
        let a: ShaderExpression = self.into();
        let b: ShaderExpression = other.into();

        // Ensure the types are valid for max.
        let a_type = a.shader_type().unwrap();
        let b_type = b.shader_type().unwrap();
        a_type.ensure_math_compatible(b_type, "max").unwrap();

        ShaderExpression::new(ShaderOperation::Max(a, b))
    }

    /// Clamps a value between the minimum and maximum values.
    fn clamp(
        self,
        min: impl Into<ShaderExpression>,
        max: impl Into<ShaderExpression>,
    ) -> ShaderExpression {
        let a: ShaderExpression = self.into();
        let b: ShaderExpression = min.into();
        let c: ShaderExpression = max.into();

        // Ensure the types are valid for clamp.
        // TODO: Make this accept more types.
        let a_type = a.shader_type().unwrap();
        let b_type = b.shader_type().unwrap();
        let c_type = c.shader_type().unwrap();
        a_type
            .ensure_vector_or_scalar("argument 'self' of 'clamp'")
            .unwrap();
        a_type
            .ensure_matches(b_type, "arguments 'self' and 'min' of 'clamp'")
            .unwrap();
        a_type
            .ensure_matches(c_type, "arguments 'self' and 'max' of 'clamp'")
            .unwrap();

        ShaderExpression::new(ShaderOperation::Clamp(a, b, c))
    }

    /// Mixes two values based on the factor.
    fn mix(
        self,
        other: impl Into<ShaderExpression>,
        factor: impl Into<ShaderExpression>,
    ) -> ShaderExpression {
        let a: ShaderExpression = self.into();
        let b: ShaderExpression = other.into();
        let c: ShaderExpression = factor.into();

        // Ensure the types are valid for mix.
        // TODO: Make this accept more types.
        let a_type = a.shader_type().unwrap();
        let b_type = b.shader_type().unwrap();
        let c_type = c.shader_type().unwrap();
        a_type
            .ensure_vector_or_scalar_f32("argument 'self' of 'mix'")
            .unwrap();
        a_type
            .ensure_matches(b_type, "arguments 'self' and 'other' of 'mix'")
            .unwrap();
        c_type
            .ensure_type(ShaderType::F32, "argument 'factor' of 'mix'")
            .unwrap();

        ShaderExpression::new(ShaderOperation::Mix(a, b, c))
    }
}

impl ShaderMath for ShaderExpression {}
impl ShaderMath for f32 {}
impl ShaderMath for i32 {}

pub trait ShaderVector: Into<ShaderExpression> + Sized {
    /// Returns the dot product of the two vectors.
    fn dot(self, other: impl Into<ShaderExpression>) -> ShaderExpression {
        let a: ShaderExpression = self.into();
        let b: ShaderExpression = other.into();

        // Ensure the types are valid for dot product.
        let a_type = a.shader_type().unwrap();
        let b_type = b.shader_type().unwrap();
        a_type
            .ensure_vector_f32("argument 'self' of 'dot'")
            .unwrap();
        a_type
            .ensure_matches(b_type, "arguments 'self' and 'other' of 'dot'")
            .unwrap();

        ShaderExpression::new(ShaderOperation::Dot(a, b))
    }

    /// Returns the cross product of the two vectors.
    fn cross(self, other: impl Into<ShaderExpression>) -> ShaderExpression {
        let a: ShaderExpression = self.into();
        let b: ShaderExpression = other.into();

        // Ensure the types are valid for cross product.
        let a_type = a.shader_type().unwrap();
        let b_type: ShaderType = b.shader_type().unwrap();
        a_type
            .ensure_vector_f32("argument 'self' of 'cross'")
            .unwrap();
        a_type
            .ensure_matches(b_type, "arguments 'self' and 'other' of 'cross'")
            .unwrap();

        ShaderExpression::new(ShaderOperation::Cross(a, b))
    }

    /// Returns the length of the vector.
    fn length(self) -> ShaderExpression {
        let a: ShaderExpression = self.into();

        // Ensure the type is valid for length.
        let a_type = a.shader_type().unwrap();
        a_type
            .ensure_vector_f32("argument 'self' of 'length'")
            .unwrap();

        ShaderExpression::new(ShaderOperation::Length(a))
    }

    /// Returns the normalized vector.
    fn normalized(self) -> ShaderExpression {
        let a: ShaderExpression = self.into();

        // Ensure the type is valid for normalization.
        let a_type = a.shader_type().unwrap();
        a_type
            .ensure_vector_f32("argument 'self' of 'normalized'")
            .unwrap();

        ShaderExpression::new(ShaderOperation::Normalized(a))
    }
}

impl ShaderVector for ShaderExpression {}
impl ShaderVector for Vector2<f32> {}
impl ShaderVector for Vector2<i32> {}
impl ShaderVector for Vector3<f32> {}
impl ShaderVector for Vector3<i32> {}
impl ShaderVector for Vector4<f32> {}
impl ShaderVector for Vector4<i32> {}

pub trait ShaderTexture: Into<ShaderExpression> + Sized {
    /// Samples the texture at the given UV coordinates.
    fn sample(
        self,
        uv: impl Into<ShaderExpression>,
        level: impl Into<ShaderExpression>,
    ) -> ShaderExpression {
        let a = self.into();
        let b = uv.into();
        let c = level.into();

        // Ensure the types are valid for sampling.
        let a_type = a.shader_type().unwrap();
        let b_type = b.shader_type().unwrap();
        let c_type = c.shader_type().unwrap();
        a_type
            .ensure_type(ShaderType::Sampler2D, "argument 'self' of 'sample'")
            .unwrap();
        // TODO: Make this accept more dimensions of UV coordinates.
        b_type
            .ensure_type(ShaderType::Vec2, "argument 'uv' of 'sample'")
            .unwrap();
        c_type
            .ensure_type(ShaderType::F32, "argument 'level' of 'sample'")
            .unwrap();

        ShaderExpression::new(ShaderOperation::Sample(a, b, c))
    }

    /// Get the minimum region coordinates (Vector3).
    /// The X and Y components correspond to the UV coordinates.
    /// The Z component corresponds to the LOD level.
    fn min_uv(self) -> ShaderExpression {
        let a = self.into();

        // Ensure that self's operation is a Sampler2D uniform and get the name.
        let name = match a.operation.into_inner() {
            ShaderOperation::Uniform(name, value_type) => {
                // Ensure the type is a Sampler2D.
                value_type.ensure_type(ShaderType::Sampler2D, "argument 'self' of 'min_uv'").unwrap();

                name
            },
            _ => panic!("Expected a uniform"),
        };

        // Create a new expression for the min UV coordinates.
        ShaderExpression::new(ShaderOperation::Uniform(
            format!("{}_min", name),
            ShaderType::Vec3,
        ))
    }

    /// Get the maximum region coordinates (Vector3).
    /// The X and Y components correspond to the UV coordinates.
    /// The Z component corresponds to the LOD level.
    fn max_uv(self) -> ShaderExpression {
        let a = self.into();

        // Ensure that self's operation is a Sampler2D uniform and get the name.
        let name = match a.operation.into_inner() {
            ShaderOperation::Uniform(name, value_type) => {
                // Ensure the type is a Sampler2D.
                value_type.ensure_type(ShaderType::Sampler2D, "argument 'self' of 'max_uv'").unwrap();

                name
            },
            _ => panic!("Expected a uniform"),
        };

        // Create a new expression for the max UV coordinates.
        ShaderExpression::new(ShaderOperation::Uniform(
            format!("{}_max", name),
            ShaderType::Vec3,
        ))
    }
}

impl ShaderTexture for ShaderExpression {}

impl Display for ShaderExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self.operation.borrow() {
            ShaderOperation::Input(name, _) => write!(f, "{}{}", SHADER_INPUT_PREFIX, name),
            ShaderOperation::Uniform(name, _) => write!(f, "{}{}", SHADER_UNIFORM_PREFIX, name),
            ShaderOperation::I32(value) => write!(f, "{}", value),
            ShaderOperation::F32(value) => write!(f, "{}", value),
            ShaderOperation::Vec2(x, y) => write!(f, "vec2({}, {})", x, y),
            ShaderOperation::Vec3(x, y, z) => write!(f, "vec3({}, {}, {})", x, y, z),
            ShaderOperation::Vec4(x, y, z, w) => write!(f, "vec4({}, {}, {}, {})", x, y, z, w),
            ShaderOperation::Append(left, right) => match self.shader_type().unwrap() {
                ShaderType::Vec2 => write!(f, "vec2({}, {})", left, right),
                ShaderType::Vec3 => write!(f, "vec3({}, {})", left, right),
                ShaderType::Vec4 => write!(f, "vec4({}, {})", left, right),
                _ => unimplemented!(),
            },
            ShaderOperation::Add(left, right) => write!(f, "({} + {})", left, right),
            ShaderOperation::Sub(left, right) => write!(f, "({} - {})", left, right),
            ShaderOperation::Mul(left, right) => write!(f, "({} * {})", left, right),
            ShaderOperation::Div(left, right) => write!(f, "({} / {})", left, right),
            ShaderOperation::Pow(left, right) => write!(f, "pow({}, {})", left, right),
            ShaderOperation::Rem(left, right) => write!(f, "mod({}, {})", left, right),
            ShaderOperation::Neg(expr) => write!(f, "(-{})", expr),
            ShaderOperation::Abs(expr) => write!(f, "abs({})", expr),
            ShaderOperation::Sign(expr) => write!(f, "sign({})", expr),
            ShaderOperation::Floor(expr) => write!(f, "floor({})", expr),
            ShaderOperation::Ceil(expr) => write!(f, "ceil({})", expr),
            ShaderOperation::Round(expr) => write!(f, "round({})", expr),
            ShaderOperation::Min(left, right) => write!(f, "min({}, {})", left, right),
            ShaderOperation::Max(left, right) => write!(f, "max({}, {})", left, right),
            ShaderOperation::Clamp(left, min, max) => {
                write!(f, "clamp({}, {}, {})", left, min, max)
            }
            ShaderOperation::Mix(left, right, factor) => {
                write!(f, "mix({}, {}, {})", left, right, factor)
            }
            ShaderOperation::Dot(left, right) => write!(f, "dot({}, {})", left, right),
            ShaderOperation::Cross(left, right) => write!(f, "cross({}, {})", left, right),
            ShaderOperation::Length(expr) => write!(f, "length({})", expr),
            ShaderOperation::Normalized(expr) => write!(f, "normalize({})", expr),
            ShaderOperation::Sample(texture, uv, lod) => {
                match &*texture.operation.borrow() {
                    ShaderOperation::Uniform(name, _) => write!(f, "textureLod({0}{1}, {0}{1}_min.xy + ({0}{1}_max.xy - {0}{1}_min.xy) * {2}, int({0}{1}_min.z + ({0}{1}_max.z - {0}{1}_min.z) * {3}))", SHADER_UNIFORM_PREFIX, name, uv, lod),
                    _ => unimplemented!(),
                }
            }
        }
    }
}
