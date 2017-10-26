use SetInterpolate;
use primitive::InterpolationPrimitive;

/// Cubic Hermite spline interpolation
///
/// `f(t) = (2d^3 + 3d^2 + 1)p0 + (d^3 - 2d^2 + d)m0 + (-2d^3 + 3d^2)p1 + (d^3 - d^2)m1`
/// `d = (t - t0) / (t1 - t0)`
/// `p0 = position at left keyframe`
/// `p1 = position at right keyframe`
/// `m0 = out tangent at left keyframe * (t1 - t0)`
/// `m1 = in tangent at right keyframe * (t1 - t0)`
/// `t0 = input at left keyframe`
/// `t1 = input at right keyframe`
///
/// ## Parameters:
///
/// - `input`: the input value to the function
/// - `inputs`: list of discrete input values for each keyframe
/// - `outputs`: list of output values to interpolate between, for cubic spline interpolation this
///             should three times the size of `inputs` and defined as
///             `[ in_tangent_0, position_0, out_tangent_0, in_tangent_1, position_1, out_tangent_1, .. ]`
/// - `normalize`: if true, normalize the interpolated value before returning it
pub struct CubicSplineSetInterpolate;

impl<T> SetInterpolate<T> for CubicSplineSetInterpolate
where
    T: InterpolationPrimitive + Copy,
{
    fn interpolate(&self, input: f32, inputs: &Vec<f32>, outputs: &Vec<T>, normalize: bool) -> T {
        let input_index = inputs
            .binary_search_by(|v| v.partial_cmp(&input).unwrap())
            .unwrap_or_else(|index| index - 1);
        if input_index >= (inputs.len() - 1) {
            outputs[outputs.len() - 2]
        } else {
            let t_diff = inputs[input_index + 1] - inputs[input_index];
            let left_index = input_index * 3;
            let right_index = (input_index + 1) * 3;
            let v = spline(
                input,
                inputs[input_index],
                t_diff,
                &outputs[left_index + 1],
                &outputs[right_index + 1],
                &outputs[left_index + 2].mul(t_diff),
                &outputs[right_index].mul(t_diff),
            );
            if normalize {
                v.normalize()
            } else {
                v
            }
        }
    }
}

#[inline]
pub(crate) fn spline<D>(t: f32, left_t: f32, t_diff: f32, p0: &D, p1: &D, m0: &D, m1: &D) -> D
where
    D: InterpolationPrimitive,
{
    let t = (t - left_t) / t_diff;
    let t2 = t * t;
    let t3 = t2 * t;
    p0.mul(2. * t3 - 3. * t2 + 1.)
        .add(&m0.mul(t3 - 2. * t2 + t))
        .add(&p1.mul(-2. * t3 + 3. * t2))
        .add(&m1.mul(t3 - t2))
}
