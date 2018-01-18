use primitive::InterpolationPrimitive;
use num::cast;

/// Do spherical linear interpolation.
///
/// `f(t) = sin((1 - d) * a) / sin (a) * p0 + sin(d * a) / sin (a) * p1`
/// `d = (t - t0) / (t1 - t0)`
/// `a = acos(p0 . p1)`
/// `p0 = output at left keyframe`
/// `p1 = output at right keyframe`
/// `t0 = input at left keyframe`
/// `t1 = input at right keyframe`
///
/// ## Parameters:
///
/// - `input`: the input value to the function
/// - `inputs`: list of discrete input values for each keyframe
/// - `outputs`: list of output values to interpolate between, for spherical
///              linear interpolation this should be the same size as `inputs`
/// - `normalize`: if true, normalize the interpolated value before returning it
pub fn spherical_linear_interpolate<T>(input: f32, inputs: &[f32], outputs: &[T], normalize: bool) -> T
where
    T: InterpolationPrimitive + Copy,
{
    if input < inputs[0] {
        return outputs[0];
    }
    let input_index = inputs
        .binary_search_by(|v| v.partial_cmp(&input).unwrap())
        .unwrap_or_else(|index| index - 1);
    if input_index >= (inputs.len() - 1) {
        outputs[outputs.len() - 1]
    } else {
        let d = (input - inputs[input_index]) / (inputs[input_index + 1] - inputs[input_index]);
        let left = outputs[input_index];
        let right = outputs[input_index + 1];

        let dot = left.dot(&right);
        let dot_threshold = cast(0.9995f32).unwrap();
        let v = if dot > dot_threshold {
            left.add(&right.sub(&left).mul(d))
        } else {
            let r_dot = if dot > 1. {
                1.
            } else if dot < -1. {
                -1.
            } else {
                dot
            };

            let theta = r_dot.acos();

            let scale1 = (theta * (1. - d)).sin();
            let scale2 = (theta * d).sin();
            left.mul(scale1).add(&right.mul(scale2)).mul(theta.sin().recip())
        };
        if normalize {
            v.normalize()
        } else {
            v
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mint::{Vector3, Quaternion};
    use std;

    #[test]
    fn test_linear_arr3() {
        let input = vec![0., 1., 2., 3., 4.];
        let output = vec![
            [0., 0., 0.],
            [1., 0., 0.],
            [0., 0., 0.],
            [-1., 0., 0.],
            [0., 0., 0.],
        ];
        assert_eq!(
            [std::f32::consts::FRAC_1_SQRT_2, 0., 0.],
            spherical_linear_interpolate(0.5, &input, &output, false)
        );
    }

    #[test]
    fn test_linear_arr4() {
        let input = vec![0., 1., 2., 3., 4.];
        let output = vec![
            [0., 0., 0., 0.],
            [1., 0., 0., 0.],
            [0., 0., 0., 0.],
            [-1., 0., 0., 0.],
            [0., 0., 0., 0.],
        ];
        assert_eq!(
            [0.99999994, 0., 0., 0.],
            spherical_linear_interpolate(0.5, &input, &output, true)
        );
    }

    #[test]
    fn test_linear_vec3() {
        let input = vec![0., 1., 2., 3., 4.];
        let output = vec![
            Vector3::from([0., 0., 0.]),
            Vector3::from([1., 0., 0.]),
            Vector3::from([0., 0., 0.]),
            Vector3::from([-1., 0., 0.]),
            Vector3::from([0., 0., 0.]),
        ];
        assert_eq!(
            Vector3::from([std::f32::consts::FRAC_1_SQRT_2, 0., 0.]),
            spherical_linear_interpolate(0.5, &input, &output, false)
        );
    }

    #[test]
    fn test_linear_quat() {
        let input = vec![0., 1., 2., 3., 4.];
        let output = vec![
            Quaternion::from([0., 0., 0., 0.]),
            Quaternion::from([1., 0., 0., 0.]),
            Quaternion::from([0., 0., 0., 0.]),
            Quaternion::from([-1., 0., 0., 0.]),
            Quaternion::from([0., 0., 0., 0.]),
        ];
        assert_eq!(
            Quaternion::from([0.99999994, 0., 0., 0.]),
            spherical_linear_interpolate(0.5, &input, &output, true)
        );
    }
}
