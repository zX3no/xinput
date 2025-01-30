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
