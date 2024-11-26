//! Demonstrates all available input conditions.
//! Press keys from the number row on the keyboard to trigger actions and observe the output in console.

use bevy::{log::LogPlugin, prelude::*};
use bevy_enhanced_input::prelude::*;

fn main() {
    // Setup logging to display triggered events.
    let mut log_plugin = LogPlugin::default();
    log_plugin.filter += ",bevy_enhanced_input::input_context::input_action=trace";

    App::new()
        .add_plugins((
            DefaultPlugins.set(log_plugin),
            EnhancedInputPlugin,
            GamePlugin,
        ))
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_input_context::<DummyContext>()
            .add_systems(Startup, Self::spawn);
    }
}

impl GamePlugin {
    fn spawn(mut commands: Commands) {
        commands.spawn(DummyContext);
    }
}

#[derive(Component, InputContext)]
#[input_context(ctor = new_dummy_context)]
struct DummyContext;

fn new_dummy_context(In(_): In<Entity>) -> ContextInstance {
    let mut ctx = ContextInstance::default();

    ctx.bind::<PressAction>()
        .with(PressAction::KEY)
        .with_condition(Press::default());
    ctx.bind::<JustPressAction>()
        .with(JustPressAction::KEY)
        .with_condition(JustPress::default());
    ctx.bind::<HoldAction>()
        .with(HoldAction::KEY)
        .with_condition(Hold::new(1.0));
    ctx.bind::<HoldAndReleaseAction>()
        .with(HoldAndReleaseAction::KEY)
        .with_condition(HoldAndRelease::new(1.0));
    ctx.bind::<PulseAction>()
        .with(PulseAction::KEY)
        .with_condition(Pulse::new(1.0));
    ctx.bind::<ReleaseAction>()
        .with(ReleaseAction::KEY)
        .with_condition(Release::default());
    ctx.bind::<TapAction>()
        .with(TapAction::KEY)
        .with_condition(Tap::new(0.5));
    ctx.bind::<ChordMember1>()
        .with(ChordMember1::KEY)
        .with_condition(BlockBy::<ChordAction>::events_only()); // Don't trigger the action when the chord is active.
    ctx.bind::<ChordMember2>()
        .with(ChordMember2::KEY)
        .with_condition(BlockBy::<ChordAction>::events_only());
    ctx.bind::<ChordAction>()
        .with_condition(Chord::<ChordMember1>::default())
        .with_condition(Chord::<ChordMember2>::default());
    ctx.bind::<BlockerAction>().with(BlockerAction::KEY);
    ctx.bind::<BlockByAction>()
        .with(BlockByAction::KEY)
        .with_condition(BlockBy::<BlockerAction>::default());

    ctx
}

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct PressAction;

impl PressAction {
    const KEY: KeyCode = KeyCode::Digit1;
}

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct JustPressAction;

impl JustPressAction {
    const KEY: KeyCode = KeyCode::Digit2;
}

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct HoldAction;

impl HoldAction {
    const KEY: KeyCode = KeyCode::Digit3;
}

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct HoldAndReleaseAction;

impl HoldAndReleaseAction {
    const KEY: KeyCode = KeyCode::Digit4;
}

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct PulseAction;

impl PulseAction {
    const KEY: KeyCode = KeyCode::Digit5;
}

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct ReleaseAction;

impl ReleaseAction {
    const KEY: KeyCode = KeyCode::Digit6;
}

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct TapAction;

impl TapAction {
    const KEY: KeyCode = KeyCode::Digit7;
}

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct ChordMember1;

impl ChordMember1 {
    const KEY: KeyCode = KeyCode::Digit8;
}

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct ChordMember2;

impl ChordMember2 {
    const KEY: KeyCode = KeyCode::Digit9;
}

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct BlockerAction;

impl BlockerAction {
    const KEY: KeyCode = KeyCode::Digit0;
}

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct ChordAction;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct BlockByAction;

impl BlockByAction {
    const KEY: KeyCode = KeyCode::Minus;
}
