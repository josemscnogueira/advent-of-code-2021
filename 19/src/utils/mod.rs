use itertools::Itertools;
use nalgebra::{Matrix3, Rotation3};

pub fn rot90() -> Vec<Matrix3<i32>> {
    (0..4)
        .cartesian_product(0..4)
        .cartesian_product(0..4)
        .map(|((roll, pitch), yaw)| {
            Matrix3::<i32>::from_iterator(
                Rotation3::from_euler_angles(
                    roll as f64 * std::f64::consts::FRAC_PI_2,
                    pitch as f64 * std::f64::consts::FRAC_PI_2,
                    yaw as f64 * std::f64::consts::FRAC_PI_2,
                )
                .into_inner()
                .iter()
                .map(|v| v.round() as i32),
            )
        })
        .unique()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rot90_len() {
        assert_eq!(rot90().into_iter().unique().count(), 24);
    }

    #[test]
    fn test_rot90_is_rotation() {
        for r in rot90() {
            assert_eq!(r * r.transpose(), Matrix3::<i32>::identity());
            assert!(
                (Matrix3::<f64>::from_iterator(r.iter().map(|v| *v as f64))
                    .determinant()
                    - 1.0)
                    .abs()
                    <= f64::EPSILON,
            );
        }
    }
}
