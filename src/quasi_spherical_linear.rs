use get_input_index;
use primitive::InterpolationPrimitive;

/// Do quasi spherical linear interpolation.
///
/// This should only ever be used on quaternions, it will produce incorrect results for other data
/// types. This will produce a result that compared to real spherical linear interpolation has an
/// error around 10^-4, but runs much faster because it does no trigonometry or sqrt calls.
///
/// Algorithm was created by Jonathan Blow:
/// [Hacking Quaternions](http://number-none.com/product/Hacking%20Quaternions/)
///
/// `f(d) = d <= 0.5 => lerp(p0, p1, y'(d))`
/// `f(d) = d > 0.5 => lerp(p0, p1, 1 - y'(1 - d))`
/// `y'(d) = 2 * k * d^2 - 3 * k * d + k + 1`
/// `k = worst_case_slope * (1 - attenuation * dot(p0, p1))^2`
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
/// - `outputs`: list of output values to interpolate between, for spherical
///              linear interpolation this should be the same size as `inputs`
/// - `normalize`: if true, normalize the interpolated value before returning it
pub fn quasi_spherical_linear_interpolate<T>(
    input: f32,
    inputs: &[f32],
    outputs: &[T],
    normalize: bool,
) -> T
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
        let d = (input - inputs[input_index]) / (inputs[input_index + 1] - inputs[input_index]);
        let left = &outputs[input_index];
        let right = &outputs[input_index + 1];

        let dot = left.dot(&right);
        let d_prime = if d <= 0.5 {
            counter_warp(d, dot)
        } else {
            1. - counter_warp(1. - d, dot)
        };
        let v = left.add(&right.sub(&left).mul(d_prime));
        if normalize {
            fast_normalize(v)
        } else {
            v
        }
    }
}

const WARP_ATTENUATION: f32 = 0.82279687;
const WARP_WORST_CASE_SLOPE: f32 = 0.58549219;

// Helper function used by the quasi interpolation function above
// f(d) = 2 * k * d^2 - 3 * k * d + k + 1
fn counter_warp(d: f32, cos_alpha: f32) -> f32 {
    let factor = 1. - WARP_ATTENUATION * cos_alpha;
    let k = WARP_WORST_CASE_SLOPE * factor * factor;
    2. * k * d * d - 3. * k * d + k + 1.
}

// Normalize a value using the inverse sqrt tangent approximation below.
fn fast_normalize<T>(v: T) -> T
where
    T: InterpolationPrimitive,
{
    let s = v.magnitude2();
    let k = isqrt_approx_in_neighbourhood(s);
    let k = if s <= 0.91521198 {
        let k = k * isqrt_approx_in_neighbourhood(k * k * s);
        if s <= 0.65211970 {
            k * isqrt_approx_in_neighbourhood(k * k * s)
        } else {
            k
        }
    } else {
        k
    };
    v.mul(k)
}

const ISQRT_NEIGHBOURHOOD: f32 = 0.959066;
const ISQRT_NEIGHBOURHOOD_SQRT: f32 = 0.97931916;
const ISQRT_SCALE: f32 = 1.000311;
const ISQRT_ADDITIVE_CONSTANT: f32 = ISQRT_SCALE / ISQRT_NEIGHBOURHOOD_SQRT;
const ISQRT_FACTOR: f32 = ISQRT_SCALE * (-0.5 / (ISQRT_NEIGHBOURHOOD * ISQRT_NEIGHBOURHOOD_SQRT));

// Compute 1/sqrt(s) using a tangent line approximation.
fn isqrt_approx_in_neighbourhood(s: f32) -> f32 {
    ISQRT_ADDITIVE_CONSTANT + (s - ISQRT_NEIGHBOURHOOD) * ISQRT_FACTOR
}

#[cfg(test)]
mod tests {
    use super::*;
    use mint::Quaternion;

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
            [0.9996371, 0., 0., 0.],
            quasi_spherical_linear_interpolate(0.5, &input, &output, true)
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
            Quaternion::from([0.9996371, 0., 0., 0.]),
            quasi_spherical_linear_interpolate(0.5, &input, &output, true)
        );
    }
}
