use std::ops::Mul;

use serde::Deserialize;
use thiserror::Error;

use crate::{
    float,
    matrix::{self, Matrix},
    tuple::{Point, Vector},
};

/// The error type when trying to create an anti-isomorphic transformation
///
/// A transformation is [isomorphic](https://en.wikipedia.org/wiki/Isomorphism) if and only if it
/// is invertible. A transformation is anti-isomorphic if and only if it is not isomorphic.
///
#[derive(Debug, PartialEq, Error)]
pub enum Error {
    /// The error type when trying to create a scaling transformation that scaled a component to
    /// zero.
    #[error("components cannot be scaled to zero")]
    ComponentScaledToZero { x: f64, y: f64, z: f64 },

    /// The error type when trying to create a shaering transformation that would produce an
    /// anti-isomorphic transformation.
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

    /// The error type when trying to create a view transformation with equal `from` and `to`
    /// vectors.
    ///
    /// This would mean that the camera it's looking at itself.
    ///
    #[error("`from` and `to` points cannot be equal")]
    EqualFromAndToVectors,

    /// The error type when trying to create a view transformation where the result of subtracting
    /// the given `from` and `to` vector is collinear to the given `up` vector.
    ///
    /// This would mean that the camera cannot orient itself, there would be a conflict
    /// between the direction it's looking at and the direction it should consider as "up".
    ///
    #[error("`from` and `up` vectors cannot be collinear")]
    CollinearToFromAndUpVectors { to_from: Vector, up: Vector },

    /// The error type when trying to crate a view transformation with a null `up` vector.
    #[error("up direction cannot be null")]
    NullUpVector,
}

/// An isomorphic linear transformation.
#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
#[serde(try_from = "TransformDeserializer")]
pub struct Transform(Matrix<4, 4>);

#[warn(missing_docs)]
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
#[serde(tag = "type")]
enum TransformDeserializer {
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

impl TryFrom<TransformDeserializer> for Transform {
    type Error = Error;

    fn try_from(value: TransformDeserializer) -> Result<Self, Self::Error> {
        Ok(match value {
            TransformDeserializer::Translation { x, y, z } => Self::translation(x, y, z),
            TransformDeserializer::Scaling { x, y, z } => Self::scaling(x, y, z)?,
            TransformDeserializer::RotationX { degrees } => Self::rotation_x(degrees.to_radians()),
            TransformDeserializer::RotationY { degrees } => Self::rotation_y(degrees.to_radians()),
            TransformDeserializer::RotationZ { degrees } => Self::rotation_z(degrees.to_radians()),
            TransformDeserializer::Shearing {
                xy,
                xz,
                yx,
                yz,
                zx,
                zy,
            } => Self::shearing(xy, xz, yx, yz, zx, zy)?,
            TransformDeserializer::View { from, to, up } => Self::view(from, to, up)?,
        })
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self(matrix::consts::IDENTITY_4X4)
    }
}

impl Transform {
    /// Constructs a translation transformation.
    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        Self(Matrix([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    /// Constructs a scaling transformation.
    ///
    /// # Errors
    ///
    /// Fails when a component is scaled to zero. This is because scaling a component to zero would
    /// make that components' original value irrecoverable, producing an anti-isomorphic matrix.
    ///
    pub fn scaling(x: f64, y: f64, z: f64) -> Result<Self, Error> {
        (!float::approx(x * y * z, 0.0))
            .then_some(Self(Matrix([
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ])))
            .ok_or(Error::ComponentScaledToZero { x, y, z })
    }

    /// Constructs a rotation transformation with respect to the `x` axis.
    pub fn rotation_x(radians: f64) -> Self {
        Self(Matrix([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, radians.cos(), -radians.sin(), 0.0],
            [0.0, radians.sin(), radians.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    /// Constructs a rotation transformation with respect to the `y` axis.
    pub fn rotation_y(radians: f64) -> Self {
        Self(Matrix([
            [radians.cos(), 0.0, radians.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-radians.sin(), 0.0, radians.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    /// Constructs a rotation transformation with respect to the `z` axis.
    pub fn rotation_z(radians: f64) -> Self {
        Self(Matrix([
            [radians.cos(), -radians.sin(), 0.0, 0.0],
            [radians.sin(), radians.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    /// Constructs a [shearing](https://en.wikipedia.org/wiki/Shear_matrix) transformation.
    ///
    /// This transformation basically translates a component based on the value of the some of the
    /// other components.
    ///
    /// # Errors
    /// Fails if the passed values fulfill the equation: `xz * yx * zy + xy * yz * zx - xy * yx -
    /// xz * zx - yz * zy = -1`. As with the scaling transformation, this would create an
    /// anti-isomorphic transformation.
    ///
    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Result<Self, Error> {
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
        .ok_or(Error::InvalidRelationBetweenComponents {
            xy,
            xz,
            yx,
            yz,
            zx,
            zy,
        })
    }

    /// Constructs a view transformation.
    ///
    /// This transformation is mainly used for positioning the camera relative to a world's origin.
    ///
    /// # Arguments
    ///
    /// * `from` - Point where the camera is going to be positioned.
    /// * `to` - Point where the center of camera is going to be looking at.
    /// * `up` - Vector that indicated the direction considered at "up". This orientates the camera
    /// so that your image is not upside-down.
    ///
    /// # Errors
    ///
    /// * Fails when the `from` and `to` vectors are the same vectors. This would mean that the
    /// camera it's looking at itself.
    ///
    /// * Fails when the resulting vector of subtracting `to - from` is collinear with the `up`
    /// vector. This would mean that the camera cannot orient itself, there would be a conflict
    /// between the direction it's looking at and the direction it should consider as "up".
    ///
    /// * Fails when the `up` vector is null.
    ///
    pub fn view(from: Point, to: Point, up: Vector) -> Result<Self, Error> {
        let forward = (to - from)
            .normalize()
            .map_err(|_| Error::EqualFromAndToVectors)?;

        let left = forward.cross(up.normalize().map_err(|_| Error::NullUpVector)?);

        if left == Vector::new(0.0, 0.0, 0.0) {
            return Err(Error::CollinearToFromAndUpVectors {
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
        let transform = Transform::translation(5.0, -3.0, 2.0);
        let point = Point::new(-3.0, 4.0, 5.0);

        assert_eq!(transform * point, Point::new(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = Transform::translation(5.0, -3.0, 2.0);
        let inverse = transform.inverse();
        let point = Point::new(-3.0, 4.0, 5.0);

        assert_eq!(inverse * point, Point::new(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = Transform::translation(5.0, -3.0, 2.0);
        let vector = Vector::new(-3.0, 4.0, 5.0);

        assert_eq!(transform * vector, vector);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_point() {
        let transform = Transform::scaling(2.0, 3.0, 4.0).unwrap();
        let point = Point::new(-4.0, 6.0, 8.0);

        assert_eq!(transform * point, Point::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_vector() {
        let transform = Transform::scaling(2.0, 3.0, 4.0).unwrap();
        let vector = Vector::new(-4.0, 6.0, 8.0);

        assert_eq!(transform * vector, Vector::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = Transform::scaling(2.0, 3.0, 4.0).unwrap();
        let inverse = transform.inverse();
        let vector = Vector::new(-4.0, 6.0, 8.0);

        assert_eq!(inverse * vector, Vector::new(-2.0, 2.0, 2.0));
    }

    #[test]
    fn trying_to_create_an_anti_isomorphic_scaling_transformation() {
        let transform = Transform::scaling(0.0, 1.0, 0.0);

        assert_eq!(
            transform,
            Err(Error::ComponentScaledToZero {
                x: 0.0,
                y: 1.0,
                z: 0.0
            })
        );
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let transform = Transform::scaling(-1.0, 1.0, 1.0).unwrap();
        let point = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * point, Point::new(-2.0, 3.0, 4.0));
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
        let point = Point::new(0.0, 1.0, 0.0);

        let half_quarter = Transform::rotation_x(std::f64::consts::FRAC_PI_4);
        let full_quarter = Transform::rotation_x(std::f64::consts::FRAC_PI_2);

        assert_eq!(
            half_quarter * point,
            Point::new(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * point, Point::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let point = Point::new(0.0, 1.0, 0.0);

        let half_quarter = Transform::rotation_x(std::f64::consts::FRAC_PI_4);
        let inverse = half_quarter.inverse();

        assert_eq!(
            inverse * point,
            Point::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let point = Point::new(0.0, 0.0, 1.0);

        let half_quarter = Transform::rotation_y(std::f64::consts::FRAC_PI_4);
        let full_quarter = Transform::rotation_y(std::f64::consts::FRAC_PI_2);

        assert_eq!(
            half_quarter * point,
            Point::new(2_f64.sqrt() / 2.0, 0.0, 2_f64.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * point, Point::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let point = Point::new(0.0, 1.0, 0.0);

        let half_quarter = Transform::rotation_z(std::f64::consts::FRAC_PI_4);
        let full_quarter = Transform::rotation_z(std::f64::consts::FRAC_PI_2);

        assert_eq!(
            half_quarter * point,
            Point::new(-2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0, 0.0)
        );
        assert_eq!(full_quarter * point, Point::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = Transform::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0).unwrap();
        let point = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * point, Point::new(5.0, 3.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = Transform::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0).unwrap();
        let point = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * point, Point::new(6.0, 3.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = Transform::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0).unwrap();
        let point = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * point, Point::new(2.0, 5.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = Transform::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();
        let point = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * point, Point::new(2.0, 7.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = Transform::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0).unwrap();
        let point = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * point, Point::new(2.0, 3.0, 6.0));
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = Transform::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0).unwrap();
        let point = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform * point, Point::new(2.0, 3.0, 7.0));
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
            Err(Error::InvalidRelationBetweenComponents {
                xy,
                xz,
                yx,
                yz,
                zx: 0.0,
                zy: 0.0,
            })
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
        let point = Point::new(1.0, 0.0, 1.0);

        let t0 = Transform::rotation_x(std::f64::consts::FRAC_PI_2);
        let t1 = Transform::scaling(5.0, 5.0, 5.0).unwrap();
        let t2 = Transform::translation(10.0, 5.0, 7.0);

        let transform = t2 * t1 * t0;

        assert_eq!(transform * point, Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn the_default_transformation() {
        let transform = Transform::default();

        assert_eq!(transform, Transform(matrix::consts::IDENTITY_4X4));
    }

    #[test]
    fn getting_the_transpose_transformation() {
        let transform = Transform::translation(1.0, 2.0, 3.0);

        assert_eq!(
            transform.transpose(),
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

        let transform = Transform::view(from, to, up);

        assert_eq!(transform, Ok(Transform::default()));
    }

    #[test]
    fn a_view_transformation_matrix_looking_in_positive_z_direction() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, 1.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        let transform = Transform::view(from, to, up);

        assert_eq!(transform, Ok(Transform::scaling(-1.0, 1.0, -1.0).unwrap()));
    }

    #[test]
    fn the_view_transformation_moves_the_world() {
        let from = Point::new(0.0, 0.0, 8.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        let transform = Transform::view(from, to, up);

        assert_eq!(transform, Ok(Transform::translation(0.0, 0.0, -8.0)));
    }

    #[test]
    fn an_arbitrary_view_transformation() {
        let from = Point::new(1.0, 3.0, 2.0);
        let to = Point::new(4.0, -2.0, 8.0);
        let up = Vector::new(1.0, 1.0, 0.0);

        let transform = Transform::view(from, to, up).unwrap();

        assert_eq!(
            transform,
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

        let transform = Transform::view(from, to, up);

        assert_eq!(transform, Err(Error::EqualFromAndToVectors));
    }

    #[test]
    fn trying_to_create_a_view_transformation_with_a_null_up_vector() {
        let from = Point::new(0.0, 0.0, 8.0);
        let to = Point::new(1.0, 2.0, 3.0);
        let up = Vector::new(0.0, 0.0, 0.0);

        let transform = Transform::view(from, to, up);

        assert_eq!(transform, Err(Error::NullUpVector));
    }

    #[test]
    fn trying_to_create_a_view_transformation_with_collinear_direction_and_up_vectors() {
        let from = Point::new(0.0, 2.0, 0.0);
        let to = Point::new(0.0, 1.0, 0.0);
        let up = Vector::new(0.0, -1.0, 0.0);

        let transform = Transform::view(from, to, up);

        assert_eq!(
            transform,
            Err(Error::CollinearToFromAndUpVectors {
                to_from: to - from,
                up,
            })
        );
    }

    #[test]
    fn deserializing_a_translation_transformation() {
        let tokens = [
            Token::Struct {
                name: "TransformDeserializer",
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
            &TransformDeserializer::Translation {
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
                name: "TransformDeserializer",
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
            &TransformDeserializer::Scaling {
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
                    name: "TransformDeserializer",
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
                name: "TransformDeserializer",
                len: 2,
            },
            Token::Str("type"),
            Token::Str("rotation_x"),
            Token::Str("degrees"),
            Token::F64(60.0),
            Token::StructEnd,
        ];

        assert_de_tokens(&TransformDeserializer::RotationX { degrees: 60.0 }, &tokens);
        assert_de_tokens(&Transform::rotation_x(std::f64::consts::FRAC_PI_3), &tokens);
    }

    #[test]
    fn deserializing_a_rotation_y_transformation() {
        let tokens = [
            Token::Struct {
                name: "TransformDeserializer",
                len: 2,
            },
            Token::Str("type"),
            Token::Str("rotation_y"),
            Token::Str("degrees"),
            Token::F64(120.0),
            Token::StructEnd,
        ];

        assert_de_tokens(
            &TransformDeserializer::RotationY { degrees: 120.0 },
            &tokens,
        );
        assert_de_tokens(&Transform::rotation_y(120_f64.to_radians()), &tokens);
    }

    #[test]
    fn deserializing_a_rotation_z_transformation() {
        let tokens = [
            Token::Struct {
                name: "TransformDeserializer",
                len: 2,
            },
            Token::Str("type"),
            Token::Str("rotation_z"),
            Token::Str("degrees"),
            Token::F64(720.0),
            Token::StructEnd,
        ];

        assert_de_tokens(
            &TransformDeserializer::RotationZ { degrees: 720.0 },
            &tokens,
        );
        assert_de_tokens(&Transform::rotation_z(720_f64.to_radians()), &tokens);
    }

    #[test]
    fn deserializing_a_shearing_transformation() {
        let tokens = [
            Token::Struct {
                name: "TransformDeserializer",
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
            &TransformDeserializer::Shearing {
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
                    name: "TransformDeserializer",
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
                name: "TransformDeserializer",
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

        assert_de_tokens(&TransformDeserializer::View { from, to, up }, &tokens);
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
                    name: "TransformDeserializer",
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
