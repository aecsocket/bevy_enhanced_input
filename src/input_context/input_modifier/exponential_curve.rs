use bevy::prelude::*;

use super::{ignore_incompatible, InputModifier};
use crate::action_value::{ActionValue, ActionValueDim};

/// Response curve exponential.
///
/// Apply a simple exponential response curve to input values, per axis.
///
/// Can't be applied to [`ActionValue::Bool`].
#[derive(Clone, Copy, Debug)]
pub struct ExponentialCurve {
    /// Curve exponent.
    pub exponent: Vec3,
}

impl ExponentialCurve {
    /// Creates a new exponential curve with all axes set to `value`
    #[must_use]
    pub fn splat(value: f32) -> Self {
        Self::new(Vec3::splat(value))
    }

    #[must_use]
    pub fn new(exponent: Vec3) -> Self {
        Self { exponent }
    }
}

impl InputModifier for ExponentialCurve {
    fn apply(&mut self, _time: &Time<Virtual>, value: ActionValue) -> ActionValue {
        let dim = value.dim();
        if dim == ActionValueDim::Bool {
            ignore_incompatible!(value);
        }

        let mut value = value.as_axis3d();
        value.x = value.x.signum() * value.x.abs().powf(self.exponent.x);
        value.y = value.y.signum() * value.y.abs().powf(self.exponent.y);
        value.z = value.z.signum() * value.z.abs().powf(self.exponent.z);
        ActionValue::Axis3D(value).convert(dim)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exp() {
        let time = Time::default();
        let mut modifier = ExponentialCurve::splat(2.0);

        assert_eq!(modifier.apply(&time, true.into()), true.into());
        assert_eq!(modifier.apply(&time, (-0.5).into()), (-0.25).into());
        assert_eq!(modifier.apply(&time, 0.5.into()), 0.25.into());
        assert_eq!(
            modifier.apply(&time, (Vec2::ONE * 2.0).into()),
            (Vec2::ONE * 4.0).into()
        );
        assert_eq!(
            modifier.apply(&time, (Vec3::ONE * 2.0).into()),
            (Vec3::ONE * 4.0).into()
        );
    }
}
