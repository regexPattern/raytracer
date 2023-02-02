#![deny(rustdoc::broken_intra_doc_links)]

//! Stochastic ray tracer based on The Ray Tracer Challenge book by Jamis Buck.

mod canvas;
mod float;
mod intersection;
mod matrix;
mod ray;

/// Camera module.
pub mod camera;

/// Colors module.
pub mod color;

/// Light sources for a world.
pub mod light;

/// Materials for shapes.
pub mod material;

/// 3D models module.
pub mod model;

/// Patterns for materials.
pub mod pattern;

/// Geometric shapes module.
pub mod shape;

/// Linear transformations for shapes.
pub mod transform;

/// Tuples module.
pub mod tuple;

/// World module.
pub mod world;
