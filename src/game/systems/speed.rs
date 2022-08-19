use crate::utils::{speed::Speed, IngameTime};
use bevy::prelude::*;

const SPEED_CHANGE_PER_SEC: f32 = 0.5;

pub(super) fn acceleration_system(mut q_speeds: Query<&mut Speed>, time: Res<IngameTime>) {
    let dur = time.delta_secs();
    q_speeds.for_each_mut(|mut speed| {
        let speed_diff = speed.current - speed.target;
        let is_change = (speed_diff != 0.) as u8 as f32;
        speed.current += is_change * -speed_diff.signum() * SPEED_CHANGE_PER_SEC * dur;
        speed.current = speed.current.clamp(0., speed.target.max(speed.normal));
    });
}
