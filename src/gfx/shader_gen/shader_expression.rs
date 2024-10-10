use std::{cell::RefCell, fmt::Display};

use anyhow::Result;
use ggmath::prelude::*;

use super::{shader_inputs::SHADER_INPUT_PREFIX, shader_type::ShaderType};

/// Represents a shader operation within a shader expression.
pub enum ShaderOperation {
    Input(String, ShaderType),
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
            // _ => unimplemented!("This operation is not implemented yet."),
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

impl std::ops::Add for ShaderExpression {
    type Output = ShaderExpression;

    fn add(self, other: ShaderExpression) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Add(self, other))
    }
}

impl std::ops::Sub for ShaderExpression {
    type Output = ShaderExpression;

    fn sub(self, other: ShaderExpression) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Sub(self, other))
    }
}

impl std::ops::Mul for ShaderExpression {
    type Output = ShaderExpression;

    fn mul(self, other: ShaderExpression) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Mul(self, other))
    }
}

impl std::ops::Div for ShaderExpression {
    type Output = ShaderExpression;

    fn div(self, other: ShaderExpression) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Div(self, other))
    }
}

impl std::ops::Rem for ShaderExpression {
    type Output = ShaderExpression;

    fn rem(self, other: ShaderExpression) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Rem(self, other))
    }
}

impl std::ops::Neg for ShaderExpression {
    type Output = ShaderExpression;

    fn neg(self) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Neg(self))
    }
}

impl std::ops::Add<f32> for ShaderExpression {
    type Output = ShaderExpression;

    fn add(self, other: f32) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Add(self, ShaderExpression::from(other)))
    }
}

impl std::ops::Sub<f32> for ShaderExpression {
    type Output = ShaderExpression;

    fn sub(self, other: f32) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Sub(self, ShaderExpression::from(other)))
    }
}

impl std::ops::Mul<f32> for ShaderExpression {
    type Output = ShaderExpression;

    fn mul(self, other: f32) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Mul(self, ShaderExpression::from(other)))
    }
}

impl std::ops::Div<f32> for ShaderExpression {
    type Output = ShaderExpression;

    fn div(self, other: f32) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Div(self, ShaderExpression::from(other)))
    }
}

impl std::ops::Rem<f32> for ShaderExpression {
    type Output = ShaderExpression;

    fn rem(self, other: f32) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Rem(self, ShaderExpression::from(other)))
    }
}

impl std::ops::Add<i32> for ShaderExpression {
    type Output = ShaderExpression;

    fn add(self, other: i32) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Add(self, ShaderExpression::from(other)))
    }
}

impl std::ops::Sub<i32> for ShaderExpression {
    type Output = ShaderExpression;

    fn sub(self, other: i32) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Sub(self, ShaderExpression::from(other)))
    }
}

impl std::ops::Mul<i32> for ShaderExpression {
    type Output = ShaderExpression;

    fn mul(self, other: i32) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Mul(self, ShaderExpression::from(other)))
    }
}

impl std::ops::Div<i32> for ShaderExpression {
    type Output = ShaderExpression;

    fn div(self, other: i32) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Div(self, ShaderExpression::from(other)))
    }
}

impl std::ops::Rem<i32> for ShaderExpression {
    type Output = ShaderExpression;

    fn rem(self, other: i32) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Rem(self, ShaderExpression::from(other)))
    }
}

pub trait ShaderMath: Into<ShaderExpression> + Sized {
    fn append(self, other: impl Into<ShaderExpression>) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Append(self.into(), other.into()))
    }

    fn add(self, other: impl Into<ShaderExpression>) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Add(self.into(), other.into()))
    }

    fn sub(self, other: impl Into<ShaderExpression>) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Sub(self.into(), other.into()))
    }

    fn mul(self, other: impl Into<ShaderExpression>) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Mul(self.into(), other.into()))
    }

    fn div(self, other: impl Into<ShaderExpression>) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Div(self.into(), other.into()))
    }

    fn pow(self, other: impl Into<ShaderExpression>) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Pow(self.into(), other.into()))
    }

    fn rem(self, other: impl Into<ShaderExpression>) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Rem(self.into(), other.into()))
    }

    fn neg(self) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Neg(self.into()))
    }

    fn abs(self) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Abs(self.into()))
    }

    fn sign(self) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Sign(self.into()))
    }

    fn floor(self) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Floor(self.into()))
    }

    fn ceil(self) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Ceil(self.into()))
    }

    fn round(self) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Round(self.into()))
    }

    fn min(self, other: impl Into<ShaderExpression>) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Min(self.into(), other.into()))
    }

    fn max(self, other: impl Into<ShaderExpression>) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Max(self.into(), other.into()))
    }

    fn clamp(
        self,
        min: impl Into<ShaderExpression>,
        max: impl Into<ShaderExpression>,
    ) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Clamp(self.into(), min.into(), max.into()))
    }

    fn mix(
        self,
        other: impl Into<ShaderExpression>,
        factor: impl Into<ShaderExpression>,
    ) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Mix(
            self.into(),
            other.into(),
            factor.into(),
        ))
    }
}

impl ShaderMath for ShaderExpression {}
impl ShaderMath for f32 {}
impl ShaderMath for i32 {}

pub trait ShaderVector: Into<ShaderExpression> + Sized {
    fn dot(self, other: impl Into<ShaderExpression>) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Dot(self.into(), other.into()))
    }

    fn cross(self, other: impl Into<ShaderExpression>) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Cross(self.into(), other.into()))
    }

    fn length(self) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Length(self.into()))
    }

    fn normalized(self) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Normalized(self.into()))
    }
}

impl ShaderVector for ShaderExpression {}
impl ShaderVector for Vector2<f32> {}
impl ShaderVector for Vector2<i32> {}
impl ShaderVector for Vector3<f32> {}
impl ShaderVector for Vector3<i32> {}
impl ShaderVector for Vector4<f32> {}
impl ShaderVector for Vector4<i32> {}

impl Display for ShaderExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self.operation.borrow() {
            ShaderOperation::Input(name, _) => write!(f, "{}{}", SHADER_INPUT_PREFIX, name),
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
        }
    }
}
