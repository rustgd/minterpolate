use SetInterpolate;
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
pub struct StepSetInterpolate;

impl<T> SetInterpolate<T> for StepSetInterpolate
where
    T: InterpolationPrimitive + Copy,
{
    fn interpolate(&self, input: f32, inputs: &Vec<f32>, outputs: &Vec<T>, _: bool) -> T {
        let input_index = inputs
            .binary_search_by(|v| v.partial_cmp(&input).unwrap())
            .unwrap_or_else(|index| index - 1);
        if input_index >= (inputs.len() - 1) {
            outputs[outputs.len() - 1]
        } else {
            outputs[input_index]
        }
    }
}
