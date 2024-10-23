use bevy::prelude::*;

use super::{ignore_incompatible, InputModifier};
use crate::action_value::{ActionValue, ActionValueDim};

/// Scales input by a set factor per axis.
///
/// Can't be applied to [`ActionValue::Bool`].
#[derive(Clone, Copy, Debug)]
pub struct Scalar {
    /// The scalar that will be applied to the input value.
    ///
    /// For example, with the scalar set to `Vec3::new(2.0, 2.0, 2.0)`, each input axis will be multiplied by 2.0.
    ///
    /// Does nothing for boolean values.
    pub scalar: Vec3,
}

impl Scalar {
    /// Creates a new scalar with all axes set to `value`.
    #[must_use]
    pub fn splat(value: f32) -> Self {
        Self::new(Vec3::splat(value))
    }

    #[must_use]
    pub fn new(scalar: Vec3) -> Self {
        Self { scalar }
    }
}

impl InputModifier for Scalar {
    fn apply(&mut self, _time: &Time<Virtual>, value: ActionValue) -> ActionValue {
        let dim = value.dim();
        if dim == ActionValueDim::Bool {
            ignore_incompatible!(value);
        }

        ActionValue::Axis3D(value.as_axis3d() * self.scalar).convert(dim)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaling() {
        let mut modifier = Scalar::splat(2.0);
        let time = Time::default();

        assert_eq!(modifier.apply(&time, true.into()), true.into());
        assert_eq!(modifier.apply(&time, 1.0.into()), 2.0.into());
        assert_eq!(modifier.apply(&time, Vec2::ONE.into()), (2.0, 2.0).into());
        assert_eq!(
            modifier.apply(&time, Vec3::ONE.into()),
            (2.0, 2.0, 2.0).into()
        );
    }
}
