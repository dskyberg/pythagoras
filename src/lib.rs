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
//!
//! #Example
//! ```
//! use pythagoras::right_angle::*;
//! use pythagoras::utils::*;
//!
//! // Using the standard 3,4,5 right triangle
//! const A:f32 = 3.0;
//! const B:f32 = 4.0;
//! const C:f32 = 5.0;
//! const R: f32 = 0.6435011;
//!
//! // Get the hypotenuse (c)
//! let c = ab_c(&A,&B);
//! assert_eq!(c, C);
//!
//! // Now get the angle (r) in radians
//! let r = ab_r(&A,&B);
//! assert_eq!(r, R);
//!
//! // Convert the angle to degrees (using std::f32::to_degrees)
//! let r_degrees = r.to_degrees();
//! assert_eq!(r_degrees.round(), 37.0);
//!
//! // Using the angle (r) and one side, get another side
//! let b = rc_b(&r,&C);
//! assert_eq!(b.round(), B.round());
//!
//! let a = rc_a(&r,&C);
//! assert_eq!(a.round(), A.round());
//!
//! // Using the angle (r), and one side (C), get the remaining two sides
//! let (a,b) = rc_ab(&r,&C);
//! assert_eq!(a, A);
//! assert_eq!(b, B);
//!
//! ```

pub use right_angle::*;
pub use utils::*;

pub mod right_angle;
pub mod utils;
