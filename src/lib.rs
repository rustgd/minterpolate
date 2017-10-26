//! Data set interpolation for `mint` `Vector3` and `Quaternion`.
//!
//! Can be extended by users to provide their own data types to interpolate over, using the
//! `InterpolationPrimitive` trait.
//!
//! ## Examples
//!
//! ```
//! use minterpolate::{CatmullRomSplineSetInterpolate, SetInterpolate};
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
//! CatmullRomSplineSetInterpolate.interpolate(0.5, &input, &output, false);
//! ```

extern crate mint;

pub use catmull_rom_spline::CatmullRomSplineSetInterpolate;
pub use cubic_spline::CubicSplineSetInterpolate;
pub use linear::LinearSetInterpolate;
pub use primitive::InterpolationPrimitive;
pub use step::StepSetInterpolate;

mod primitive;
mod linear;
mod step;
mod cubic_spline;
mod catmull_rom_spline;

/// Interpolate between values in a data set.
///
/// Should map input against the inputs, and interpolate between the outputs for the mapped range.
///
/// ## Invariants
///
/// * inputs[0] >= 0
/// * inputs[i + 1] > inputs[i]
///
/// Note that the length of outputs is defined by the algorithm, and can be different from the
/// length of inputs. See each interpolation function for a description of the output data and
/// alignment.
pub trait SetInterpolate<T> {
    
    /// Interpolation function, `f(input) -> T`
    ///
    /// ## Parameters
    ///
    /// - `input`: the input value to the function
    /// - `inputs`: list of discrete input values for each keyframe
    /// - `outputs`: list of output values to interpolate between, note that this data set is
    ///              tied to the interpolation function, and there is no guarantee or requirement
    ///              that it is the same size as the inputs.
    /// - `normalize`: if true, normalize the interpolated value before returning it
    fn interpolate(&self, input: f32, inputs: &Vec<f32>, outputs: &Vec<T>, normalize: bool) -> T;
}
