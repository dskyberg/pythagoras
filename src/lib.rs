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
//! use pythagoras::*;
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
pub mod right_angle;

/// Returns the hypotenuse (c) of a right triangle given the rise (a) and run (b).
#[inline(always)]
pub fn ab_c(rise: &f32, run: &f32) -> f32 {
    (rise.powi(2) + run.powi(2)).sqrt()
}

/// Returns the opposite side (a) given the hypotenuse (c) and adjacent side (b).
#[inline(always)]
pub fn bc_a(b: &f32, c: &f32) -> f32 {
    (c.powi(2) - b.powi(2)).sqrt()
}

/// Returns the adjacent side (b) given the hypotenuse (c) and opposite side (a).
#[inline(always)]
pub fn ac_b(a: &f32, c: &f32) -> f32 {
    (c.powi(2) - a.powi(2)).sqrt()
}

/// Returns radians (r) given the opposite side (a) and hypotenuse (c).
#[inline(always)]
pub fn ac_r(a: &f32, c: &f32) -> f32 {
    (a / c).asin()
}

/// Returns the radians (r) given the opposite side (a) and adjacent side (b).
#[inline(always)]
pub fn ab_r(a: &f32, b: &f32) -> f32 {
    (a / b).atan()
}

/// Returns the radians (r) given the adjacent side (b) and hypotenuse (c).
#[inline(always)]
pub fn bc_r(b: &f32, c: &f32) -> f32 {
    (b / c).acos()
}

/// Returns the adjacent side (b) given the radians (r) and the opposite side (a).
#[inline(always)]
pub fn ra_b(r: &f32, a: &f32) -> f32 {
    a / r.tan()
}

/// Returns the hypotenuse (c) given the radians (r) and the opposite side (a).
#[inline(always)]
pub fn ra_c(r: &f32, a: &f32) -> f32 {
    a / r.sin()
}

/// Returns the opposite side(a) given the radians (r) and the adjacent side (b).
#[inline(always)]
pub fn rb_a(r: &f32, b: &f32) -> f32 {
    r.tan() * b
}

/// Returns the hypotenuse (c) given the radians (r) and the adjacent side (b).
#[inline(always)]
pub fn rb_c(r: &f32, b: &f32) -> f32 {
    b / r.cos()
}

/// Given the radians and the hypotenuse (c), return the opposite side (a)
#[inline(always)]
pub fn rc_a(r: &f32, c: &f32) -> f32 {
    c * r.sin()
}

/// Given the radians and the hypotenuse (c), return the adjacent side (b)
#[inline(always)]
pub fn rc_b(r: &f32, c: &f32) -> f32 {
    c * r.cos()
}

/// Given radians and the adjacent (b), calculate the opposite (a) and hypotenuse (c).
pub fn rb_ac(r: &f32, b: &f32) -> (f32, f32) {
    let a = rb_a(r, b);
    let c = ab_c(&a, b);
    (a, c)
}

/// Given radians and the opposite (a), calculate the adjacent (c) and hypotenuse (c).
pub fn ra_bc(r: &f32, a: &f32) -> (f32, f32) {
    let b = ra_b(r, a);
    let c = ab_c(a, &b);
    (b, c)
}

/// Given radians and the hypotenuse (c), calculate the opposite (a) and adjacent (b).
pub fn rc_ab(r: &f32, c: &f32) -> (f32, f32) {
    let a = rc_a(r, c);
    let b = rc_b(r, c);
    (a, b)
}

#[cfg(test)]
mod tests {
    use super::*;
    const RADIANS_345: f32 = 0.6435011;
    const A_345: f32 = 3.0;
    const B_345: f32 = 4.0;
    const C_345: f32 = 5.0;

    #[test]
    fn test_ab_c() {
        assert_eq!(ab_c(&A_345, &B_345), C_345);
    }

    #[test]
    fn test_bc_a() {
        assert_eq!(bc_a(&B_345, &C_345), A_345);
    }

    #[test]
    fn test_ac_b() {
        assert_eq!(ac_b(&A_345, &C_345), B_345);
    }

    #[test]
    fn test_ac_r() {
        assert_eq!(
            format!("{:.6}", ac_r(&A_345, &C_345)),
            format!("{:.6}", RADIANS_345)
        );
    }

    #[test]
    fn test_ab_r() {
        assert_eq!(
            format!("{:.6}", ab_r(&A_345, &B_345)),
            format!("{:.6}", RADIANS_345)
        );
    }

    #[test]
    fn test_bc_r() {
        assert_eq!(
            format!("{:.6}", bc_r(&B_345, &C_345)),
            format!("{:.6}", RADIANS_345)
        );
    }

    #[test]
    fn test_ra_b() {
        assert_eq!(ra_b(&RADIANS_345, &A_345), B_345);
    }
    #[test]
    fn test_ra_c() {
        assert_eq!(ra_c(&RADIANS_345, &A_345), C_345);
    }

    #[test]
    fn test_rb_a() {
        assert_eq!(rb_a(&RADIANS_345, &B_345), A_345);
    }

    #[test]
    fn test_rb_c() {
        assert_eq!(rb_c(&RADIANS_345, &B_345), C_345);
    }

    #[test]
    fn test_rc_a() {
        assert_eq!(rc_a(&RADIANS_345, &C_345), A_345);
    }

    #[test]
    fn test_rc_b() {
        assert_eq!(rc_b(&RADIANS_345, &C_345), B_345);
    }

    #[test]
    fn test_rb_ac() {
        let result = rb_ac(&RADIANS_345, &B_345);
        assert_eq!(result, (A_345, C_345));
    }

    #[test]
    fn test_ra_bc() {
        let result = ra_bc(&RADIANS_345, &A_345);
        assert_eq!(result, (B_345, C_345));
    }

    #[test]
    fn test_rc_ab() {
        let result = rc_ab(&RADIANS_345, &C_345);
        assert_eq!(result, (3.0, 4.0));
    }
}
