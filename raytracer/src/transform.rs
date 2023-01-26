use std::ops::Mul;

use serde::Deserialize;
use thiserror::Error;

use crate::{
    float,
    matrix::{self, Matrix},
    tuple::{Point, Vector},
};

#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
#[serde(try_from = "Deserializer")]
pub struct Transform(Matrix<4, 4>);

#[derive(Debug, PartialEq, Error)]
pub enum AntiIsomorphicTransformError {
    #[error("components cannot be scaled to zero")]
    ComponentScaledToZero { x: f64, y: f64, z: f64 },

    #[error(
        "result of `xz * yx * zy + xy * yz * zx - xy * yx - xz * zx - yz * zy` cannot equal `-1`"
    )]
    InvalidRelationBetweenComponents {
        xy: f64,
        xz: f64,
        yx: f64,
        yz: f64,
        zx: f64,
        zy: f64,
    },

    #[error("up direction cannot be null")]
    NullUpVector,

    #[error("`from` and `to` points cannot be equal")]
    EqualFromAndToVectors,

    #[error("`from` and `up` vectors cannot be collinear")]
    CollinearToFromAndUpVectors { to_from: Vector, up: Vector },
}

// This enum exits to facilitate the parsing of the allowed transform variants. The `Transform`
// type itself it's not an enum because of the matrix constructors that have to be used and their
// respective validations. However, using an enum for parsing it's much easier than implementing
// `Deseriliaze` manually on the original type.
// Also note that `PartialEq` is being used here. I don't really care about comparing this type
// beyoond the tests, so floating point comparission doesn't matter here.
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
#[serde(tag = "type")]
enum Deserializer {
    Translation {
        x: f64,
        y: f64,
        z: f64,
    },

    Scaling {
        x: f64,
        y: f64,
        z: f64,
    },

    RotationX {
        degrees: f64,
    },

    RotationY {
        degrees: f64,
    },

    RotationZ {
        degrees: f64,
    },

    Shearing {
        xy: f64,
        xz: f64,
        yx: f64,
        yz: f64,
        zx: f64,
        zy: f64,
    },

    View {
        from: Point,
        to: Point,
        up: Vector,
    },
}

impl TryFrom<Deserializer> for Transform {
    type Error = AntiIsomorphicTransformError;

    fn try_from(value: Deserializer) -> Result<Self, Self::Error> {
        Ok(match value {
            Deserializer::Translation { x, y, z } => Self::translation(x, y, z),
            Deserializer::Scaling { x, y, z } => Self::scaling(x, y, z)?,
            Deserializer::RotationX { degrees } => Self::rotation_x(degrees.to_radians()),
            Deserializer::RotationY { degrees } => Self::rotation_y(degrees.to_radians()),
            Deserializer::RotationZ { degrees } => Self::rotation_z(degrees.to_radians()),
            Deserializer::Shearing {
                xy,
                xz,
                yx,
                yz,
                zx,
                zy,
            } => Self::shearing(xy, xz, yx, yz, zx, zy)?,
            Deserializer::View { from, to, up } => Self::view(from, to, up)?,
        })
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self(matrix::consts::IDENTITY_4X4)
    }
}

impl Transform {
    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        Self(Matrix([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Result<Self, AntiIsomorphicTransformError> {
        (!float::approx(x * y * z, 0.0))
            .then_some(Self(Matrix([
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ])))
            .ok_or(AntiIsomorphicTransformError::ComponentScaledToZero { x, y, z })
    }

    pub fn rotation_x(radians: f64) -> Self {
        Self(Matrix([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, radians.cos(), -radians.sin(), 0.0],
            [0.0, radians.sin(), radians.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    pub fn rotation_y(radians: f64) -> Self {
        Self(Matrix([
            [radians.cos(), 0.0, radians.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-radians.sin(), 0.0, radians.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    pub fn rotation_z(radians: f64) -> Self {
        Self(Matrix([
            [radians.cos(), -radians.sin(), 0.0, 0.0],
            [radians.sin(), radians.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    pub fn shearing(
        xy: f64,
        xz: f64,
        yx: f64,
        yz: f64,
        zx: f64,
        zy: f64,
    ) -> Result<Self, AntiIsomorphicTransformError> {
        (!float::approx(
            xz * yx * zy + xy * yz * zx - xy * yx - xz * zx - yz * zy + 1.0,
            0.0,
        ))
        .then_some(Self(Matrix([
            [1.0, xy, xz, 0.0],
            [yx, 1.0, yz, 0.0],
            [zx, zy, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])))
        .ok_or(
            AntiIsomorphicTransformError::InvalidRelationBetweenComponents {
                xy,
                xz,
                yx,
                yz,
                zx,
                zy,
            },
        )
    }

    pub fn view(from: Point, to: Point, up: Vector) -> Result<Self, AntiIsomorphicTransformError> {
        let forward = (to - from)
            .normalize()
            .map_err(|_| AntiIsomorphicTransformError::EqualFromAndToVectors)?;

        let left = forward.cross(
            up.normalize()
                .map_err(|_| AntiIsomorphicTransformError::NullUpVector)?,
        );

        if left == Vector::new(0.0, 0.0, 0.0) {
            return Err(AntiIsomorphicTransformError::CollinearToFromAndUpVectors {
                to_from: to - from,
                up,
            });
        }

        let up = left.cross(forward);

        let orientation = Self(Matrix([
            [left.0.x, left.0.y, left.0.z, 0.0],
            [up.0.x, up.0.y, up.0.z, 0.0],
            [-forward.0.x, -forward.0.y, -forward.0.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]));

        Ok(orientation * Self::translation(-from.0.x, -from.0.y, -from.0.z))
    }

    pub(crate) fn inverse(self) -> Self {
        // Only isomorphic matrices can be constructed through this type's public API. This means that
        // the matrix associated with every transformation is going to be invertible.
        #[allow(clippy::unwrap_used)]
        Self(self.0.inverse().unwrap())
    }

    pub(crate) fn transpose(self) -> Self {
        Self(self.0.transpose())
    }
}

impl Mul for Transform {
    type Output = Self;

    // Again, the fact that one is only able to create isomorphic transformations allows us to
    // claim that any transformation composition is also isomorphic.
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Mul<Point> for Transform {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point(self.0 * rhs.0)
    }
}

impl Mul<Vector> for Transform {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector(self.0 * rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_de_tokens, assert_de_tokens_error, Token};

    use crate::assert_approx;

    use super::*;

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let t = Transform::translation(5.0, -3.0, 2.0);
        let p = Point::new(-3.0, 4.0, 5.0);

        assert_eq!(t * p, Point::new(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let t = Transform::translation(5.0, -3.0, 2.0);
        let inv = t.inverse();
        let p = Point::new(-3.0, 4.0, 5.0);

        assert_eq!(inv * p, Point::new(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let t = Transform::translation(5.0, -3.0, 2.0);
        let v = Vector::new(-3.0, 4.0, 5.0);

        assert_eq!(t * v, v);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_point() {
        let t = Transform::scaling(2.0, 3.0, 4.0).unwrap();
        let p = Point::new(-4.0, 6.0, 8.0);

        assert_eq!(t * p, Point::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_vector() {
        let t = Transform::scaling(2.0, 3.0, 4.0).unwrap();
        let v = Vector::new(-4.0, 6.0, 8.0);

        assert_eq!(t * v, Vector::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let t = Transform::scaling(2.0, 3.0, 4.0).unwrap();
        let inv = t.inverse();
        let v = Vector::new(-4.0, 6.0, 8.0);

        assert_eq!(inv * v, Vector::new(-2.0, 2.0, 2.0));
    }

    #[test]
    fn trying_to_create_an_anti_isomorphic_scaling_transformation() {
        let t = Transform::scaling(0.0, 1.0, 0.0);

        assert_eq!(
            t,
            Err(AntiIsomorphicTransformError::ComponentScaledToZero {
                x: 0.0,
                y: 1.0,
                z: 0.0
            })
        );
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let t = Transform::scaling(-1.0, 1.0, 1.0).unwrap();
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(t * p, Point::new(-2.0, 3.0, 4.0));
    }

    #[test]
    fn converting_from_degrees_to_radians() {
        assert_approx!(0_f64.to_radians(), 0.0);
        assert_approx!(90_f64.to_radians(), std::f64::consts::FRAC_PI_2);
        assert_approx!(180_f64.to_radians(), std::f64::consts::PI);
        assert_approx!(360_f64.to_radians(), 2.0 * std::f64::consts::PI);
        assert_approx!(720_f64.to_radians(), 4.0 * std::f64::consts::PI);
        assert_approx!(-180_f64.to_radians(), -std::f64::consts::PI);
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let p = Point::new(0.0, 1.0, 0.0);

        let half_quarter = Transform::rotation_x(std::f64::consts::FRAC_PI_4);
        let full_quarter = Transform::rotation_x(std::f64::consts::FRAC_PI_2);

        assert_eq!(
            half_quarter * p,
            Point::new(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, Point::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = Point::new(0.0, 1.0, 0.0);

        let half_quarter = Transform::rotation_x(std::f64::consts::FRAC_PI_4);
        let inv = half_quarter.inverse();

        assert_eq!(
            inv * p,
            Point::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = Point::new(0.0, 0.0, 1.0);

        let half_quarter = Transform::rotation_y(std::f64::consts::FRAC_PI_4);
        let full_quarter = Transform::rotation_y(std::f64::consts::FRAC_PI_2);

        assert_eq!(
            half_quarter * p,
            Point::new(2_f64.sqrt() / 2.0, 0.0, 2_f64.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, Point::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = Point::new(0.0, 1.0, 0.0);

        let half_quarter = Transform::rotation_z(std::f64::consts::FRAC_PI_4);
        let full_quarter = Transform::rotation_z(std::f64::consts::FRAC_PI_2);

        assert_eq!(
            half_quarter * p,
            Point::new(-2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0, 0.0)
        );
        assert_eq!(full_quarter * p, Point::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_y() {
        let t = Transform::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0).unwrap();
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(t * p, Point::new(5.0, 3.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_z() {
        let t = Transform::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0).unwrap();
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(t * p, Point::new(6.0, 3.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_x() {
        let t = Transform::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0).unwrap();
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(t * p, Point::new(2.0, 5.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_z() {
        let t = Transform::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(t * p, Point::new(2.0, 7.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_x() {
        let t = Transform::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0).unwrap();
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(t * p, Point::new(2.0, 3.0, 6.0));
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_y() {
        let t = Transform::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0).unwrap();
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(t * p, Point::new(2.0, 3.0, 7.0));
    }

    #[test]
    fn trying_to_create_an_anti_isomorphic_shearing_transformation() {
        let xy = 1.0;
        let xz = 2.0;
        let yx = 1.0 / xy;
        let yz = xz / xy;

        let t = Transform::shearing(xy, xz, yx, yz, 0.0, 0.0);

        assert_eq!(
            t,
            Err(
                AntiIsomorphicTransformError::InvalidRelationBetweenComponents {
                    xy,
                    xz,
                    yx,
                    yz,
                    zx: 0.0,
                    zy: 0.0,
                }
            )
        );
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p0 = Point::new(1.0, 0.0, 1.0);

        let t0 = Transform::rotation_x(std::f64::consts::FRAC_PI_2);
        let t1 = Transform::scaling(5.0, 5.0, 5.0).unwrap();
        let t2 = Transform::translation(10.0, 5.0, 7.0);

        let p1 = t0 * p0;
        let p2 = t1 * p1;
        let p3 = t2 * p2;

        assert_eq!(p1, Point::new(1.0, -1.0, 0.0));
        assert_eq!(p2, Point::new(5.0, -5.0, 0.0));
        assert_eq!(p3, Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = Point::new(1.0, 0.0, 1.0);

        let t0 = Transform::rotation_x(std::f64::consts::FRAC_PI_2);
        let t1 = Transform::scaling(5.0, 5.0, 5.0).unwrap();
        let t2 = Transform::translation(10.0, 5.0, 7.0);

        let t = t2 * t1 * t0;

        assert_eq!(t * p, Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn the_default_transformation() {
        let t = Transform::default();

        assert_eq!(t, Transform(matrix::consts::IDENTITY_4X4));
    }

    #[test]
    fn getting_the_transpose_transformation() {
        let t = Transform::translation(1.0, 2.0, 3.0);

        assert_eq!(
            t.transpose(),
            Transform(Matrix([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [1.0, 2.0, 3.0, 1.0],
            ]))
        );
    }

    #[test]
    fn the_transformation_matrix_for_the_default_orientation() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, -1.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        let t = Transform::view(from, to, up);

        assert_eq!(t, Ok(Transform::default()));
    }

    #[test]
    fn a_view_transformation_matrix_looking_in_positive_z_direction() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, 1.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        let t = Transform::view(from, to, up);

        assert_eq!(t, Ok(Transform::scaling(-1.0, 1.0, -1.0).unwrap()));
    }

    #[test]
    fn the_view_transformation_moves_the_world() {
        let from = Point::new(0.0, 0.0, 8.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        let t = Transform::view(from, to, up);

        assert_eq!(t, Ok(Transform::translation(0.0, 0.0, -8.0)));
    }

    #[test]
    fn an_arbitrary_view_transformation() {
        let from = Point::new(1.0, 3.0, 2.0);
        let to = Point::new(4.0, -2.0, 8.0);
        let up = Vector::new(1.0, 1.0, 0.0);

        let t = Transform::view(from, to, up).unwrap();

        assert_eq!(
            t,
            Transform(Matrix([
                [-0.50709, 0.50709, 0.67612, -2.36643],
                [0.76772, 0.60609, 0.12122, -2.82843],
                [-0.35857, 0.59761, -0.71714, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]))
        );
    }

    #[test]
    fn trying_to_create_a_view_transformation_with_equal_from_and_to_vectors() {
        let from = Point::new(0.0, 0.0, 8.0);
        let to = from;
        let up = Vector::new(1.0, 2.0, 3.0);

        let t = Transform::view(from, to, up);

        assert_eq!(t, Err(AntiIsomorphicTransformError::EqualFromAndToVectors));
    }

    #[test]
    fn trying_to_create_a_view_transformation_with_a_null_up_vector() {
        let from = Point::new(0.0, 0.0, 8.0);
        let to = Point::new(1.0, 2.0, 3.0);
        let up = Vector::new(0.0, 0.0, 0.0);

        let t = Transform::view(from, to, up);

        assert_eq!(t, Err(AntiIsomorphicTransformError::NullUpVector));
    }

    #[test]
    fn trying_to_create_a_view_transformation_with_collinear_direction_and_up_vectors() {
        let from = Point::new(0.0, 2.0, 0.0);
        let to = Point::new(0.0, 1.0, 0.0);
        let up = Vector::new(0.0, -1.0, 0.0);

        let t = Transform::view(from, to, up);

        assert_eq!(
            t,
            Err(AntiIsomorphicTransformError::CollinearToFromAndUpVectors {
                to_from: to - from,
                up,
            })
        );
    }

    #[test]
    fn deserializing_a_translation_transformation() {
        let tokens = [
            Token::Struct {
                name: "Deserializer",
                len: 4,
            },
            Token::Str("type"),
            Token::Str("translation"),
            Token::Str("x"),
            Token::F64(1.0),
            Token::Str("y"),
            Token::F64(-3.0),
            Token::Str("z"),
            Token::F64(0.25),
            Token::StructEnd,
        ];

        assert_de_tokens(
            &Deserializer::Translation {
                x: 1.0,
                y: -3.0,
                z: 0.25,
            },
            &tokens,
        );

        assert_de_tokens(&Transform::translation(1.0, -3.0, 0.25), &tokens);
    }

    #[test]
    fn deserializing_a_scaling_transformation() {
        let tokens = [
            Token::Struct {
                name: "Deserializer",
                len: 4,
            },
            Token::Str("type"),
            Token::Str("scaling"),
            Token::Str("x"),
            Token::F64(1.0),
            Token::Str("y"),
            Token::F64(-3.0),
            Token::Str("z"),
            Token::F64(0.25),
            Token::StructEnd,
        ];

        assert_de_tokens(
            &Deserializer::Scaling {
                x: 1.0,
                y: -3.0,
                z: 0.25,
            },
            &tokens,
        );

        assert_de_tokens(&Transform::scaling(1.0, -3.0, 0.25).unwrap(), &tokens);
    }

    #[test]
    fn trying_to_deserialize_an_invalid_scaling_transform() {
        assert_de_tokens_error::<Transform>(
            &[
                Token::Struct {
                    name: "Deserializer",
                    len: 4,
                },
                Token::Str("type"),
                Token::Str("scaling"),
                Token::Str("x"),
                Token::F64(1.0),
                Token::Str("y"),
                Token::F64(0.0),
                Token::Str("z"),
                Token::F64(0.25),
                Token::StructEnd,
            ],
            "components cannot be scaled to zero",
        );
    }

    #[test]
    fn deserializing_a_rotation_x_transformation() {
        let tokens = [
            Token::Struct {
                name: "Deserializer",
                len: 2,
            },
            Token::Str("type"),
            Token::Str("rotation_x"),
            Token::Str("degrees"),
            Token::F64(60.0),
            Token::StructEnd,
        ];

        assert_de_tokens(&Deserializer::RotationX { degrees: 60.0 }, &tokens);
        assert_de_tokens(&Transform::rotation_x(std::f64::consts::FRAC_PI_3), &tokens);
    }

    #[test]
    fn deserializing_a_rotation_y_transformation() {
        let tokens = [
            Token::Struct {
                name: "Deserializer",
                len: 2,
            },
            Token::Str("type"),
            Token::Str("rotation_y"),
            Token::Str("degrees"),
            Token::F64(120.0),
            Token::StructEnd,
        ];

        assert_de_tokens(&Deserializer::RotationY { degrees: 120.0 }, &tokens);
        assert_de_tokens(&Transform::rotation_y(120_f64.to_radians()), &tokens);
    }

    #[test]
    fn deserializing_a_rotation_z_transformation() {
        let tokens = [
            Token::Struct {
                name: "Deserializer",
                len: 2,
            },
            Token::Str("type"),
            Token::Str("rotation_z"),
            Token::Str("degrees"),
            Token::F64(720.0),
            Token::StructEnd,
        ];

        assert_de_tokens(&Deserializer::RotationZ { degrees: 720.0 }, &tokens);
        assert_de_tokens(&Transform::rotation_z(720_f64.to_radians()), &tokens);
    }

    #[test]
    fn deserializing_a_shearing_transformation() {
        let tokens = [
            Token::Struct {
                name: "Deserializer",
                len: 7,
            },
            Token::Str("type"),
            Token::Str("shearing"),
            Token::Str("xy"),
            Token::F64(1.0),
            Token::Str("xz"),
            Token::F64(-4.25),
            Token::Str("yx"),
            Token::F64(0.0),
            Token::Str("yz"),
            Token::F64(7.89),
            Token::Str("zx"),
            Token::F64(11.1),
            Token::Str("zy"),
            Token::F64(0.001),
            Token::StructEnd,
        ];

        assert_de_tokens(
            &Deserializer::Shearing {
                xy: 1.0,
                xz: -4.25,
                yx: 0.0,
                yz: 7.89,
                zx: 11.1,
                zy: 0.001,
            },
            &tokens,
        );

        assert_de_tokens(
            &Transform::shearing(1.0, -4.25, 0.0, 7.89, 11.1, 0.001).unwrap(),
            &tokens,
        );
    }

    #[test]
    fn trying_to_deserialize_an_invalid_shearing_transform() {
        let xy = 1.0;
        let xz = 2.0;
        let yx = 1.0 / xy;
        let yz = xz / xy;

        assert_de_tokens_error::<Transform>(
            &[
                Token::Struct {
                    name: "Deserializer",
                    len: 7,
                },
                Token::Str("type"),
                Token::Str("shearing"),
                Token::Str("xy"),
                Token::F64(xy),
                Token::Str("xz"),
                Token::F64(xz),
                Token::Str("yx"),
                Token::F64(yx),
                Token::Str("yz"),
                Token::F64(yz),
                Token::Str("zx"),
                Token::F64(11.1),
                Token::Str("zy"),
                Token::F64(0.001),
                Token::StructEnd,
            ],
            "result of `xz * yx * zy + xy * yz * zx - xy * yx - xz * zx - yz * zy` cannot equal `-1`",
        );
    }

    #[test]
    fn deserializing_a_view_transformation() {
        let from = Point::new(1.0, 1.0, 1.0);
        let to = Point::new(0.0, 1.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        let tokens = [
            Token::Struct {
                name: "Deserializer",
                len: 4,
            },
            Token::Str("type"),
            Token::Str("view"),
            // from: Point
            Token::Str("from"),
            Token::Struct {
                name: "Point",
                len: 3,
            },
            Token::Str("x"),
            Token::F64(from.0.x),
            Token::Str("y"),
            Token::F64(from.0.y),
            Token::Str("z"),
            Token::F64(from.0.z),
            Token::StructEnd,
            // to: Point
            Token::Str("to"),
            Token::Struct {
                name: "Point",
                len: 3,
            },
            Token::Str("x"),
            Token::F64(to.0.x),
            Token::Str("y"),
            Token::F64(to.0.y),
            Token::Str("z"),
            Token::F64(to.0.z),
            Token::StructEnd,
            // up: Vector
            Token::Str("up"),
            Token::Struct {
                name: "Vector",
                len: 3,
            },
            Token::Str("x"),
            Token::F64(up.0.x),
            Token::Str("y"),
            Token::F64(up.0.y),
            Token::Str("z"),
            Token::F64(up.0.z),
            Token::StructEnd,
            Token::StructEnd,
        ];

        assert_de_tokens(&Deserializer::View { from, to, up }, &tokens);
        assert_de_tokens(&Transform::view(from, to, up).unwrap(), &tokens);
    }

    #[test]
    fn trying_to_deserialize_an_invalid_view_transformation() {
        let from = Point::new(1.0, 1.0, 1.0);
        let to = from;
        let up = Vector::new(0.0, 1.0, 0.0);

        assert_de_tokens_error::<Transform>(
            &[
                Token::Struct {
                    name: "Deserializer",
                    len: 4,
                },
                Token::Str("type"),
                Token::Str("view"),
                // from: Point
                Token::Str("from"),
                Token::Struct {
                    name: "Point",
                    len: 3,
                },
                Token::Str("x"),
                Token::F64(from.0.x),
                Token::Str("y"),
                Token::F64(from.0.y),
                Token::Str("z"),
                Token::F64(from.0.z),
                Token::StructEnd,
                // to: Point
                Token::Str("to"),
                Token::Struct {
                    name: "Point",
                    len: 3,
                },
                Token::Str("x"),
                Token::F64(to.0.x),
                Token::Str("y"),
                Token::F64(to.0.y),
                Token::Str("z"),
                Token::F64(to.0.z),
                Token::StructEnd,
                // up: Vector
                Token::Str("up"),
                Token::Struct {
                    name: "Vector",
                    len: 3,
                },
                Token::Str("x"),
                Token::F64(up.0.x),
                Token::Str("y"),
                Token::F64(up.0.y),
                Token::Str("z"),
                Token::F64(up.0.z),
                Token::StructEnd,
                Token::StructEnd,
            ],
            "`from` and `to` points cannot be equal",
        );
    }
}
