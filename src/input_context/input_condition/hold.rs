use bevy::prelude::*;

use super::{
    primitives::{Actuation, HeldTimer},
    InputCondition,
};
use crate::{
    action_value::ActionValue,
    input_context::input_action::{ActionState, ActionsData},
};

/// Returns [`ActionState::Ongoing`] when the input becomes actuated and
/// [`ActionState::Fired`] when input remained actuated for [`Self::hold_time`] seconds.
///
/// Returns [`ActionState::None`] when the input stops being actuated earlier than [`Self::hold_time`] seconds.
/// May optionally fire once, or repeatedly fire.
#[derive(Debug)]
pub struct Hold {
    // How long does the input have to be held to cause trigger.
    pub hold_time: f32,

    // Should this trigger fire only once, or fire every frame once the hold time threshold is met?
    pub one_shot: bool,

    /// Trigger threshold.
    pub actuation: Actuation,

    held_timer: HeldTimer,

    fired: bool,
}

impl Hold {
    #[must_use]
    pub fn new(hold_time: f32) -> Self {
        Self {
            hold_time,
            one_shot: false,
            actuation: Default::default(),
            held_timer: Default::default(),
            fired: false,
        }
    }

    #[must_use]
    pub fn one_shot(mut self, one_shot: bool) -> Self {
        self.one_shot = one_shot;
        self
    }

    #[must_use]
    pub fn with_actuation(mut self, actuation: impl Into<Actuation>) -> Self {
        self.actuation = actuation.into();
        self
    }

    #[must_use]
    pub fn with_held_timer(mut self, held_timer: HeldTimer) -> Self {
        self.held_timer = held_timer;
        self
    }
}

impl InputCondition for Hold {
    fn evaluate(
        &mut self,
        world: &World,
        _actions_data: &ActionsData,
        delta: f32,
        value: ActionValue,
    ) -> ActionState {
        let actuated = self.actuation.is_actuated(value);
        if actuated {
            self.held_timer.update(world, delta);
        } else {
            self.held_timer.reset();
        }

        let is_first_trigger = !self.fired;
        self.fired = self.held_timer.duration() >= self.hold_time;

        if self.fired {
            if is_first_trigger || !self.one_shot {
                ActionState::Fired
            } else {
                ActionState::None
            }
        } else if actuated {
            ActionState::Ongoing
        } else {
            ActionState::None
        }
    }
}