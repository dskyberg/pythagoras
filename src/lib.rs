//! Pythagorean functions for right triangles.
//! Simple helper functions to calculate sides and angles.
//!
//! The methods in this library use a common theme in which the components of
//! right triangles are consistenly referred to as:
//! - a: Opposite side, or the rise, using rise/run language
//! - b: Adjacent side, or the run, using rise/run language
//! - c: Hypotenuse, or the diagonal, using rise/run language
//! - r: The angle in radians
//!
//! Method signatures are formed by adding the inputs (from the list above) to the desired outputs.
//! Thus, if you have a and b, and you need c, call `let c = ab_c(&a,&b);`
//!
//! Note, if you have an angle in degrees, simply pass in `angle.to_radians()`.
//! And, to convert radians to degrees, use `radians.to_degrees()`.

pub use right_angle::*;
pub use utils::*;

pub mod right_angle;
pub mod utils;
