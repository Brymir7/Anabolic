use shared::types::{AnimationCallbackEvent, AnimationState};

#[no_mangle]
pub fn update_enemy_animation(animation_states: &mut Vec<AnimationState>, dt: f32) -> Vec<AnimationCallbackEvent> {
    let mut res = Vec::new();
    for animation_state in animation_states {
        animation_state.current_step += 1.0 * dt;
        if (animation_state.current_step - animation_state.max_step) > 0.0 {
            animation_state.current_step = 0.0;
            if animation_state.callback != AnimationCallbackEvent::None {
                res.push(animation_state.callback);
            }
        }
    }
    res
}