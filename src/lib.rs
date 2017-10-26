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

pub use catmull_rom_spline::catmull_rom_spline_interpolate;
pub use cubic_spline::cubic_spline_interpolate;
pub use linear::linear_interpolate;
pub use primitive::InterpolationPrimitive;
pub use step::step_interpolate;

mod primitive;
mod linear;
mod step;
mod cubic_spline;
mod catmull_rom_spline;
