use SetInterpolate;
use primitive::InterpolationPrimitive;

/// Do linear interpolation.
///
/// `f(t) = p0 + (p1 - p0) * d`
/// `d = (t - t0) / (t1 - t0)`
/// `p0 = output at left keyframe`
/// `p1 = output at right keyframe`
/// `t0 = input at left keyframe`
/// `t1 = input at right keyframe`
///
/// ## Parameters:
///
/// - `input`: the input value to the function
/// - `inputs`: list of discrete input values for each keyframe
/// - `outputs`: list of output values to interpolate between, for linear interpolation this should
///              be the same size as `inputs`
/// - `normalize`: if true, normalize the interpolated value before returning it
pub struct LinearSetInterpolate;

impl<T> SetInterpolate<T> for LinearSetInterpolate where T: InterpolationPrimitive + Copy {
    fn interpolate(&self, input: f32, inputs: &Vec<f32>, outputs: &Vec<T>, normalize: bool) -> T {
        let input_index = inputs
            .binary_search_by(|v| v.partial_cmp(&input).unwrap())
            .unwrap_or_else(|index| index - 1);
        if input_index >= (inputs.len() - 1) {
            outputs[outputs.len() - 1]
        } else {
            let d = (input - inputs[input_index]) / (inputs[input_index + 1] - inputs[input_index]);
            let left = outputs[input_index];
            let right = outputs[input_index + 1];
            let v = left.add(&right.sub(&left).mul(d));
            if normalize {
                v.normalize()
            } else {
                v
            }
        }
    }
}