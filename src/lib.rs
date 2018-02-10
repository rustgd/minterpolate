//! Data set interpolation for `mint` `Vector3` and `Quaternion`.
//!
//! Can be extended by users to provide their own data types to interpolate over, using the
//! `InterpolationPrimitive` trait.
//!
//! ## Examples
//!
//! ```
//! use minterpolate::catmull_rom_spline_interpolate;
//!
//! let input = vec![0., 1., 2., 3., 4.];
//! let output = vec![
//!     [1., 0., 0.],
//!     [0., 0., 0.],
//!     [1., 0., 0.],
//!     [0., 0., 0.],
//!     [-1., 0., 0.],
//!     [0., 0., 0.],
//!     [-1., 0., 0.],
//! ];
//! catmull_rom_spline_interpolate(0.5, &input, &output, false);
//! ```

extern crate mint;
extern crate num;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

pub use catmull_rom_spline::catmull_rom_spline_interpolate;
pub use cubic_spline::cubic_spline_interpolate;
pub use linear::linear_interpolate;
pub use primitive::InterpolationPrimitive;
pub use quasi_spherical_linear::quasi_spherical_linear_interpolate;
pub use spherical_linear::spherical_linear_interpolate;
pub use step::step_interpolate;

mod primitive;
mod linear;
mod spherical_linear;
mod step;
mod cubic_spline;
mod catmull_rom_spline;
mod quasi_spherical_linear;

use std::fmt;

/// Calculate the keyframe index in the input collection
///
/// ### Parameters
///
/// - `input`: the input value to the function
/// - `inputs`: list of discrete input values for each keyframe
///
/// ### Returns
///
/// The index into the `inputs`, corresponding to the given `input`.
///
pub fn get_input_index(input: f32, inputs: &[f32]) -> Option<usize> {
    if input < inputs[0] {
        None
    } else {
        Some(
            inputs
                .binary_search_by(|v| v.partial_cmp(&input).unwrap())
                .unwrap_or_else(|index| index - 1),
        )
    }
}

/// Calculate the keyframe index in the input collection, and the interpolation factor between the
/// current keyframe and the next keyframe.
///
/// ### Parameters
///
/// - `input`: the input value to the function
/// - `inputs`: list of discrete input values for each keyframe
///
/// ### Returns
///
/// The index into the `inputs`, corresponding to the given `input`, and also the interpolation
/// factor, i.e. the distance traveled between the current keyframe and the next keyframe.
///
pub fn get_interpolation_factor(input: f32, inputs: &[f32]) -> Option<(usize, f32)> {
    get_input_index(input, inputs).map(|index| {
        if index >= inputs.len() - 1 {
            (index, 0.)
        } else {
            (
                index,
                (input - inputs[index]) / (inputs[index + 1] - inputs[index]),
            )
        }
    })
}

/// Supported interpolation functions
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum InterpolationFunction<T>
where
    T: InterpolationPrimitive,
{
    /// Linear interpolation
    Linear,
    /// Spherical linear interpolation
    SphericalLinear,
    /// Quasi spherical linear interpolation
    QuasiSphericalLinear,
    /// Step interpolation
    Step,
    /// Catmull-Rom spline interpolation
    CatmullRomSpline,
    /// Cubic Hermite spline interpolation
    CubicSpline,
    /// Generic function
    #[cfg_attr(feature = "serde", serde(skip_serializing, skip_deserializing))]
    Function(fn(f32, &[f32], &[T], bool) -> T),
}

impl<T> InterpolationFunction<T>
where
    T: InterpolationPrimitive + Copy,
{
    pub fn interpolate(&self, input: f32, inputs: &[f32], outputs: &[T], normalize: bool) -> T {
        match *self {
            InterpolationFunction::Linear => linear_interpolate(input, inputs, outputs, normalize),
            InterpolationFunction::SphericalLinear => {
                spherical_linear_interpolate(input, inputs, outputs, normalize)
            }
            InterpolationFunction::QuasiSphericalLinear => {
                quasi_spherical_linear_interpolate(input, inputs, outputs, normalize)
            }
            InterpolationFunction::Step => step_interpolate(input, inputs, outputs, normalize),
            InterpolationFunction::CubicSpline => {
                cubic_spline_interpolate(input, inputs, outputs, normalize)
            }
            InterpolationFunction::CatmullRomSpline => {
                catmull_rom_spline_interpolate(input, inputs, outputs, normalize)
            }
            InterpolationFunction::Function(ref f) => f(input, inputs, outputs, normalize),
        }
    }
}

impl<T> fmt::Debug for InterpolationFunction<T>
where
    T: InterpolationPrimitive,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InterpolationFunction::Linear => write!(f, "Linear"),
            InterpolationFunction::SphericalLinear => write!(f, "SphericalLinear"),
            InterpolationFunction::QuasiSphericalLinear => write!(f, "QuasiSphericalLinear"),
            InterpolationFunction::Step => write!(f, "Step"),
            InterpolationFunction::CatmullRomSpline => write!(f, "CatmullRomSpline"),
            InterpolationFunction::CubicSpline => write!(f, "CubicSpline"),
            InterpolationFunction::Function(_) => write!(f, "Function"),
        }
    }
}

impl<T> PartialEq for InterpolationFunction<T>
where
    T: InterpolationPrimitive,
{
    fn eq(&self, other: &InterpolationFunction<T>) -> bool {
        use self::InterpolationFunction::*;
        match (self, other) {
            (&Linear, &Linear) => true,
            (&SphericalLinear, &SphericalLinear) => true,
            (&QuasiSphericalLinear, &QuasiSphericalLinear) => true,
            (&Step, &Step) => true,
            (&CatmullRomSpline, &CatmullRomSpline) => true,
            (&CubicSpline, &CubicSpline) => true,
            _ => false, // Functions should never be equal
        }
    }
}
