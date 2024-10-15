use bevy::prelude::*;

use super::InputModifier;
use crate::{action_value::ActionValue, ActionValueDim};

/// Multiplies the input value by delta time for this frame.
///
/// Can't be applied to [`ActionValue::Bool`].
#[derive(Clone, Copy, Debug)]
pub struct ScaleByDelta;

impl InputModifier for ScaleByDelta {
    fn apply(&mut self, _world: &World, delta: f32, value: ActionValue) -> ActionValue {
        let dim = value.dim();
        if dim == ActionValueDim::Bool {
            super::ignore_incompatible!(value);
        }

        ActionValue::Axis3D(value.as_axis3d() * delta).convert(dim)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaling() {
        let world = World::new();

        for delta in [0.0, 0.5, 1.0] {
            assert_eq!(ScaleByDelta.apply(&world, delta, true.into()), true.into());
            assert_eq!(
                ScaleByDelta.apply(&world, delta, 0.5.into()),
                (0.5 * delta).into()
            );
            assert_eq!(
                ScaleByDelta.apply(&world, delta, Vec2::ONE.into()),
                (Vec2::ONE * delta).into()
            );
            assert_eq!(
                ScaleByDelta.apply(&world, delta, Vec3::ONE.into()),
                (Vec3::ONE * delta).into()
            );
        }
    }
}
