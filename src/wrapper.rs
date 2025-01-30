use crate::*;

/// Available controllers are: 0, 1, 2, 3
/// XInput supports a maximum of 4 controllers.
pub fn get_state(controller: u32) -> Result<XINPUT_STATE, &'static str> {
    let mut state = unsafe { core::mem::zeroed() };
    let result = unsafe { XInputGetState(controller, &mut state) };

    match result {
        0 => Ok(state),
        ERROR_DEVICE_NOT_CONNECTED => Err("Controller not connected."),
        _ => todo!("{}", result),
    }
}

//Not sure what the point of this function even is...
pub fn get_keystroke(controller: u32) -> Result<XINPUT_KEYSTROKE, &'static str> {
    let mut keystroke = unsafe { core::mem::zeroed() };
    let result = unsafe { XInputGetKeystroke(controller, 0, &mut keystroke) };

    match result {
        0 | ERROR_EMPTY => Ok(keystroke),
        ERROR_DEVICE_NOT_CONNECTED => Err("Controller not connected."),
        _ => todo!("{}", result),
    }
}

pub fn vibrate(
    controller: u32,
    left_motor_speed: u16,
    right_motor_speed: u16,
) -> Result<XINPUT_VIBRATION, &'static str> {
    let mut vibration = XINPUT_VIBRATION {
        left_motor_speed,
        right_motor_speed,
    };
    let result = unsafe { XInputSetState(controller, &mut vibration) };

    match result {
        0 => Ok(vibration),
        ERROR_DEVICE_NOT_CONNECTED => Err("Controller not connected."),
        _ => todo!("{}", result),
    }
}
