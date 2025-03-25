//! [RightAngle] is designed to enable completion of right angle data based on the provided input.
//! It provides a single method, [TryFrom<&RightAngleInput>].  Fill in whaever data you have, and [RightAngle] will
//! fill in the rest.
//!
//! <br />
//!
//! [RightAngle] (and [RightAngleInput]) supports [serde](https://docs.rs/serde/latest/serde/).  So you can `Deserialize`
//! [RightAngleInput] and  `Serialize` [RightAngle]. Perfect for API applications!
//!
//! <br />
//!
//! # Example
//! ```rust
//! use pythagoras::right_angle::{RightAngle, RightAngleInput};
//!
//! // Using the standard 3,4,5 right triangle
//! const A:f32 = 3.0;
//! const B:f32 = 4.0;
//! const C:f32 = 5.0;
//! const R: f32 = 0.6435011;
//!
//! const RIGHT_ANGLE: RightAngle = RightAngle {
//!     rise: A,
//!     run: B,
//!     diagonal: C,
//!     radians: R,
//! };
//!
//! // Use one side and the angle to compete the rest of the right angle
//! let input = RightAngleInput {
//!     radians: Some(R),
//!     rise: Some(A),
//!     run: None,
//!     diagonal: None,
//! };
//!
//! let result = RightAngle::try_from(&input).expect("Failed to create RightAngle");
//! assert_eq!(result, RIGHT_ANGLE);
//!
//!
//! // Use two sides to complete the rest of the right angle
//! let input = RightAngleInput {
//!    radians: None,
//!    rise: None,
//!    run: Some(B),
//!    diagonal: Some(C),
//! };
//!
//! let result = RightAngle::try_from(&input).expect("Failed to create RightAngle");
//! assert_eq!(result, RIGHT_ANGLE);
//!
//!
//! // Create [RightAngle] by serializing [RightAngleInput] to
//! // a json string,
//!  let json = format!(r#"{{ "rise": {}, "run": {} }}"#, A, B);
//!  let result = RightAngle::try_from(json.as_str()).expect("Failed to convert");
//!  assert_eq!(result, RIGHT_ANGLE);
//! ```

use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{ab_c, ab_r, ac_b, ac_r, bc_a, bc_r, ra_bc, rb_ac, rc_ab};

#[derive(Debug, Error)]
enum RightAngleError {
    #[error("There must be at least one side")]
    TooFewSides,
    #[error("Too many sides were provided for this method")]
    TooManySides,
    #[error("Angle is required when only one side is provided")]
    AngleRequired,
    #[error("Invalid input")]
    InvalidInput,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RightAngle {
    pub radians: f32,
    pub rise: f32,
    pub run: f32,
    pub diagonal: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RightAngleInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radians: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rise: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagonal: Option<f32>,
}

/// Given the angle (in radians) and one side, calculate the other two sides.
/// Note, this will throw if more than 1 side is provided.
fn one_side(input: &RightAngleInput) -> Result<RightAngle> {
    let radians = input.radians.ok_or(RightAngleError::AngleRequired)?;

    match (input.rise, input.run, input.diagonal) {
        (Some(a), None, None) => {
            let (b, c) = ra_bc(&radians, &a);
            Ok(RightAngle {
                radians,
                rise: a,
                run: b,
                diagonal: c,
            })
        }
        (None, Some(b), None) => {
            let (a, c) = rb_ac(&radians, &b);
            Ok(RightAngle {
                radians,
                rise: a,
                run: b,
                diagonal: c,
            })
        }
        (None, None, Some(c)) => {
            let (a, b) = rc_ab(&radians, &c);
            Ok(RightAngle {
                radians,
                rise: a,
                run: b,
                diagonal: c,
            })
        }
        (None, None, None) => Err(RightAngleError::TooFewSides.into()),
        _ => Err(RightAngleError::TooManySides.into()),
    }
}

/// Given two sides, calculate the third side.
/// An error is thrown if not enough sides are provided.
fn two_sides(input: &RightAngleInput) -> Result<RightAngle> {
    match (input.rise, input.run, input.diagonal) {
        (Some(a), Some(b), Some(c)) => Ok(RightAngle {
            rise: a,
            run: b,
            diagonal: c,
            radians: input.radians.unwrap_or_else(|| ab_r(&a, &b)),
        }),
        (Some(a), Some(b), None) => Ok(RightAngle {
            rise: a,
            run: b,
            diagonal: ab_c(&a, &b),
            radians: input.radians.unwrap_or_else(|| ab_r(&a, &b)),
        }),
        (Some(a), None, Some(c)) => Ok(RightAngle {
            rise: a,
            run: ac_b(&a, &c),
            diagonal: c,
            radians: input.radians.unwrap_or_else(|| ac_r(&a, &c)),
        }),
        (None, Some(b), Some(c)) => Ok(RightAngle {
            rise: bc_a(&b, &c),
            run: b,
            diagonal: c,
            radians: input.radians.unwrap_or_else(|| bc_r(&b, &c)),
        }),
        _ => Err(RightAngleError::TooFewSides.into()),
    }
}

fn three_sides(input: &RightAngleInput) -> Result<RightAngle> {
    let rise = input.rise.unwrap();
    let run = input.run.unwrap();
    let diagonal = input.diagonal.unwrap();
    let radians = match input.radians {
        Some(r) => r,
        None => ab_r(&rise, &run),
    };

    Ok(RightAngle {
        rise,
        run,
        diagonal,
        radians,
    })
}

impl TryFrom<&RightAngleInput> for RightAngle {
    type Error = String;
    fn try_from(input: &RightAngleInput) -> Result<Self, Self::Error> {
        RightAngle::from_input(input).map_err(|e| e.to_string())
    }
}

/// # Example
/// Create [RightAngle] by serializing [RightAngleInput] to
/// a json string,
///
/// ```rust
/// use pythagoras::right_angle::RightAngle;
///
/// // Using the standard 3,4,5 right triangle
/// const A:f32 = 3.0;
/// const B:f32 = 4.0;
/// const C:f32 = 5.0;
/// const R: f32 = 0.6435011;
///
/// const RIGHT_ANGLE: RightAngle = RightAngle {
///     rise: A,
///     run: B,
///     diagonal: C,
///     radians: R,
/// };
///  let json = format!(r#"{{ "rise": {}, "run": {} }}"#, 3.0, 4.0);
///  let result = RightAngle::try_from(json.as_str()).expect("Failed to convert");
///  assert_eq!(result, RIGHT_ANGLE);
/// ```
impl TryFrom<&str> for RightAngle {
    type Error = String;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let input = serde_json::from_str::<RightAngleInput>(input).map_err(|e| e.to_string())?;
        RightAngle::from_input(&input).map_err(|e| e.to_string())
    }
}

impl RightAngle {
    /// There are 2 ways to use this method.
    /// 1. Given 1 side and the angle, find the other 2 sides
    /// 2. Given 2 sides, find the third
    /// 3. Given 3 sides, find the angle
    pub fn from_input(input: &RightAngleInput) -> Result<Self> {
        let mut side_count = 0;
        if input.rise.is_some() {
            side_count += 1;
        }
        if input.run.is_some() {
            side_count += 1;
        }
        if input.diagonal.is_some() {
            side_count += 1;
        }

        // 1.  If only 1 side is given, then we need either one of(angle,pitch) or one of (sin,cos,tan)

        match side_count {
            1 => one_side(input),
            2 => two_sides(input),
            3 => three_sides(input),
            _ => Err(RightAngleError::InvalidInput.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RADIANS_345: f32 = 0.6435011;
    const A_345: f32 = 3.0;
    const B_345: f32 = 4.0;
    const C_345: f32 = 5.0;
    const RIGHT_ANGLE: RightAngle = RightAngle {
        rise: A_345,
        run: B_345,
        diagonal: C_345,
        radians: RADIANS_345,
    };

    #[test]
    fn test_try_from_str() {
        let json = format!(r#"{{ "rise": {}, "run": {} }}"#, A_345, B_345);
        let result = RightAngle::try_from(json.as_str()).expect("Failed to convert");
        assert_eq!(result, RIGHT_ANGLE);
    }

    #[test]
    fn test_empty_err() {
        let input = RightAngleInput {
            radians: None,
            rise: None,
            run: None,
            diagonal: None,
        };

        let result = RightAngle::try_from(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_one_err() {
        let input = RightAngleInput {
            radians: None,
            rise: None,
            run: None,
            diagonal: Some(C_345),
        };

        let result = RightAngle::try_from(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_ra() {
        let input = RightAngleInput {
            radians: Some(RADIANS_345),
            rise: Some(A_345),
            run: None,
            diagonal: None,
        };

        let result = RightAngle::try_from(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), RIGHT_ANGLE);
    }

    #[test]
    fn test_rb() {
        let input = RightAngleInput {
            radians: Some(RADIANS_345),
            rise: None,
            run: Some(B_345),
            diagonal: None,
        };

        let result = RightAngle::try_from(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), RIGHT_ANGLE);
    }

    #[test]
    fn test_rc() {
        let input = RightAngleInput {
            radians: Some(RADIANS_345),
            rise: None,
            run: None,
            diagonal: Some(C_345),
        };

        let result = RightAngle::try_from(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), RIGHT_ANGLE);
    }

    #[test]
    fn test_ab() {
        let input = RightAngleInput {
            radians: None,
            rise: Some(A_345),
            run: Some(B_345),
            diagonal: None,
        };

        let result = RightAngle::try_from(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), RIGHT_ANGLE);
    }
    #[test]
    fn test_ac() {
        let input = RightAngleInput {
            radians: Some(RADIANS_345),
            rise: Some(A_345),
            run: None,
            diagonal: Some(C_345),
        };

        let result = RightAngle::try_from(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), RIGHT_ANGLE);
    }

    #[test]
    fn test_bc() {
        let input = RightAngleInput {
            radians: None,
            rise: None,
            run: Some(B_345),
            diagonal: Some(C_345),
        };

        let result = RightAngle::try_from(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), RIGHT_ANGLE);
    }
    #[test]
    fn test_abc() {
        let input = RightAngleInput {
            radians: None,
            rise: Some(A_345),
            run: Some(B_345),
            diagonal: Some(C_345),
        };

        let result = RightAngle::try_from(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), RIGHT_ANGLE);
    }
}
