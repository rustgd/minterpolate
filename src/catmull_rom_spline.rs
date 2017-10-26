use SetInterpolate;
use cubic_spline::spline;
use primitive::InterpolationPrimitive;

/// Catmull-Rom spline interpolation
///
/// `f(t) = (2d^3 + 3d^2 + 1)p0 + (d^3 - 2d^2 + d)m0 + (-2d^3 + 3d^2)p1 + (d^3 - d^2)m1`
/// `d = (t - t0) / (t1 - t0)`
/// `p0 = position at left keyframe`
/// `p1 = position at right keyframe`
/// `k = left keyframe index`
/// `k+1 = right keyframe index`
/// `m0 = (p_k+1 - p_k-1) / (t_k+1 - t_k-1)`
/// `m1 = (p_k+2 - p_k) / (t_k+2 - t_k)`
/// `t0 = input at left keyframe`
/// `t1 = input at right keyframe`
///
/// ## Parameters:
///
/// - `input`: the input value to the function
/// - `inputs`: list of discrete input values for each keyframe
/// - `outputs`: list of output values to interpolate between, for catmull rom spline interpolation
///             this should be the size of `inputs` + 2
///             `[ in_tangent_0, position_0, position_1, .., position_n, out_tangent_n ]`
/// - `normalize`: if true, normalize the interpolated value before returning it
pub struct CatmullRomSplineSetInterpolate;

impl<T> SetInterpolate<T> for CatmullRomSplineSetInterpolate
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
            let v = spline(
                input,
                inputs[input_index],
                t_diff,
                &outputs[input_index + 1],
                &outputs[input_index + 2],
                &catmull_tangent(input_index, inputs, outputs),
                &catmull_tangent(input_index + 1, inputs, outputs),
            );
            if normalize {
                v.normalize()
            } else {
                v
            }
        }
    }
}

fn catmull_tangent<D>(index: usize, inputs: &Vec<f32>, outputs: &Vec<D>) -> D
where
    D: InterpolationPrimitive + Copy,
{
    let output_index = index + 1;
    if index == 0 {
        outputs[0]
    } else if index == inputs.len() - 1 {
        outputs[outputs.len() - 1]
    } else {
        outputs[output_index + 1]
            .sub(&outputs[output_index - 1])
            .mul(1. / (inputs[index + 1] - inputs[index - 1]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mint::Vector3;

    #[test]
    fn test_catmull() {
        let input = vec![0., 1., 2., 3., 4.];
        let output = vec![
            Vector3::from([1., 0., 0.]),
            Vector3::from([0., 0., 0.]),
            Vector3::from([1., 0., 0.]),
            Vector3::from([0., 0., 0.]),
            Vector3::from([-1., 0., 0.]),
            Vector3::from([0., 0., 0.]),
            Vector3::from([-1., 0., 0.]),
        ];
        assert_eq!(
            Vector3::from([0.625, 0.625, 0.625]),
            CatmullRomSplineSetInterpolate.interpolate(0.5, &input, &output, false)
        );
    }
}
