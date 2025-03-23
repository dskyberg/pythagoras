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

    #[test]
    fn test_ab_c() {
        let a: f32 = 3.0;
        let b: f32 = 4.0;
        let c = ab_c(&a, &b);
        assert_eq!(c.round(), 5.0);
    }

    #[test]
    fn test_bc_a() {
        let b: f32 = 4.0;
        let c: f32 = 5.0;
        let a = bc_a(&b, &c);
        assert_eq!(a.round(), 3.0);
    }

    #[test]
    fn test_ac_b() {
        let a: f32 = 3.0;
        let c: f32 = 5.0;
        let b = ac_b(&a, &c);
        assert_eq!(b.round(), 4.0);
    }

    #[test]
    fn test_ac_r() {
        let a: f32 = 3.0;
        let c: f32 = 5.0;
        let r = ac_r(&a, &c);
        assert_eq!(r.round(), 36.86_f32.to_radians().round());
    }

    #[test]
    fn test_ab_r() {
        let a: f32 = 3.0;
        let b: f32 = 4.0;
        let r = ab_r(&a, &b);
        assert_eq!(r.round(), 36.86_f32.to_radians().round());
    }

    #[test]
    fn test_bc_r() {
        let b: f32 = 4.0;
        let c: f32 = 5.0;
        let r = bc_r(&b, &c);
        assert_eq!(r.round(), 36.86_f32.to_radians().round());
    }

    #[test]
    fn test_ra_b() {
        let r: f32 = 36.86_f32.to_radians();
        let a: f32 = 3.0;
        let b = ra_b(&r, &a);
        assert_eq!(b.round(), 4.0);
    }
    #[test]
    fn test_ra_c() {
        let r: f32 = 36.86_f32.to_radians();
        let a: f32 = 3.0;
        let c = ra_c(&r, &a);
        assert_eq!(c.round(), 5.0);
    }

    #[test]
    fn test_rb_a() {
        let r: f32 = 36.86_f32.to_radians();
        let b: f32 = 4.0;
        let a = rb_a(&r, &b);
        assert_eq!(a.round(), 3.0);
    }

    #[test]
    fn test_rb_c() {
        let r: f32 = 36.86_f32.to_radians();
        let b: f32 = 4.0;
        let c = rb_c(&r, &b);
        assert_eq!(c.round(), 5.0);
    }

    #[test]
    fn test_rc_a() {
        let r: f32 = 36.86_f32.to_radians();
        let c: f32 = 5.0;
        let a = rc_a(&r, &c);
        assert_eq!(a.round(), 3.0);
    }

    #[test]
    fn test_rc_b() {
        let r: f32 = 36.86_f32.to_radians();
        let c: f32 = 5.0;
        let b = rc_b(&r, &c);
        assert_eq!(b.round(), 4.0);
    }

    #[test]
    fn test_rb_ac() {
        let r: f32 = 36.86_f32.to_radians();
        let b: f32 = 4.0;
        let (a, c) = rb_ac(&r, &b);
        assert_eq!(a.round(), 3.0);
        assert_eq!(c.round(), 5.0);
    }

    #[test]
    fn test_ra_bc() {
        let r: f32 = 36.86_f32.to_radians();
        let a: f32 = 3.0;
        let (b, c) = ra_bc(&r, &a);
        assert_eq!(b.round(), 4.0);
        assert_eq!(c.round(), 5.0);
    }

    #[test]
    fn test_rc_ab() {
        let r: f32 = 36.86_f32.to_radians();
        let c: f32 = 5.0;
        let (a, b) = rc_ab(&r, &c);
        assert_eq!(a.round(), 3.0);
        assert_eq!(b.round(), 4.0);
    }
}
