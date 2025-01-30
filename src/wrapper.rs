use crate::*;

// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// pub enum Error {
//     ControllerNotConnected,
// }

pub fn get_state(user_index: u32) -> Result<XINPUT_STATE, &'static str> {
    unsafe {
        let mut state = core::mem::zeroed();
        let result = XInputGetState(user_index, &mut state);

        match result {
            0 => Ok(state),
            ERROR_DEVICE_NOT_CONNECTED => Err("Controller not connected."),
            _ => todo!("{}", result),
        }
    }
}

//Not sure what the point of this function even is...
pub fn get_keystroke(user_index: u32) -> Result<XINPUT_KEYSTROKE, &'static str> {
    unsafe {
        let mut keystroke = core::mem::zeroed();
        let result = XInputGetKeystroke(user_index, 0, &mut keystroke);

        match result {
            0 | ERROR_EMPTY => Ok(keystroke),
            ERROR_DEVICE_NOT_CONNECTED => Err("Controller not connected."),
            _ => todo!("{}", result),
        }
    }
}
