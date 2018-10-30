use get_input_index;
use primitive::InterpolationPrimitive;

/// Do step interpolation.
///
/// Step interpolation will remain on a keyframe until the next keyframe is reached, and then
/// step to that keyframe.
///
/// ## Parameters:
///
/// - `input`: the input value to the function
/// - `inputs`: list of discrete input values for each keyframe
/// - `outputs`: list of output values to interpolate between, for step interpolation this should
///              be the same size as `inputs`
/// - `normalize`: if true, normalize the interpolated value before returning it
pub fn step_interpolate<T>(input: f32, inputs: &[f32], outputs: &[T], _: bool) -> T
where
    T: InterpolationPrimitive + Clone,
{
    let input_index = match get_input_index(input, inputs) {
        Some(index) => index,
        None => return outputs[0].clone(),
    };
    if input_index >= (inputs.len() - 1) {
        outputs[outputs.len() - 1].clone()
    } else {
        outputs[input_index].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mint::{Quaternion, Vector3};

    #[test]
    fn test_step_arr3() {
        let input = vec![0., 1., 2., 3., 4.];
        let output = vec![
            [0., 0., 0.],
            [1., 0., 0.],
            [0., 0., 0.],
            [-1., 0., 0.],
            [0., 0., 0.],
        ];
        assert_eq!([0., 0., 0.], step_interpolate(0.5, &input, &output, false));
    }

    #[test]
    fn test_step_arr4() {
        let input = vec![0., 1., 2., 3., 4.];
        let output = vec![
            [0., 0., 0., 0.],
            [1., 0., 0., 0.],
            [0., 0., 0., 0.],
            [-1., 0., 0., 0.],
            [0., 0., 0., 0.],
        ];
        assert_eq!(
            [0., 0., 0., 0.],
            step_interpolate(0.5, &input, &output, true)
        );
    }

    #[test]
    fn test_step_vec3() {
        let input = vec![0., 1., 2., 3., 4.];
        let output = vec![
            Vector3::from([0., 0., 0.]),
            Vector3::from([1., 0., 0.]),
            Vector3::from([0., 0., 0.]),
            Vector3::from([-1., 0., 0.]),
            Vector3::from([0., 0., 0.]),
        ];
        assert_eq!(
            Vector3::from([0., 0., 0.]),
            step_interpolate(0.5, &input, &output, false)
        );
    }

    #[test]
    fn test_step_quat() {
        let input = vec![0., 1., 2., 3., 4.];
        let output = vec![
            Quaternion::from([0., 0., 0., 0.]),
            Quaternion::from([1., 0., 0., 0.]),
            Quaternion::from([0., 0., 0., 0.]),
            Quaternion::from([-1., 0., 0., 0.]),
            Quaternion::from([0., 0., 0., 0.]),
        ];
        assert_eq!(
            Quaternion::from([0., 0., 0., 0.]),
            step_interpolate(0.5, &input, &output, true)
        );
    }
}
