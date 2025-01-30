use crate::*;

/// Available controllers are: 0, 1, 2, 3
/// XInput supports a maximum of 4 controllers.
pub fn get_state(controller: u32) -> Result<XINPUT_STATE, &'static str> {
    unsafe {
        let mut state = core::mem::zeroed();
        let result = XInputGetState(controller, &mut state);

        match result {
            0 => Ok(state),
            ERROR_DEVICE_NOT_CONNECTED => Err("Controller not connected."),
            _ => todo!("{}", result),
        }
    }
}

//Not sure what the point of this function even is...
pub fn get_keystroke(controller: u32) -> Result<XINPUT_KEYSTROKE, &'static str> {
    unsafe {
        let mut keystroke = core::mem::zeroed();
        let result = XInputGetKeystroke(controller, 0, &mut keystroke);

        match result {
            0 | ERROR_EMPTY => Ok(keystroke),
            ERROR_DEVICE_NOT_CONNECTED => Err("Controller not connected."),
            _ => todo!("{}", result),
        }
    }
}
