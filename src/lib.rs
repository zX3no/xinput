#![allow(non_snake_case, non_camel_case_types)]

pub mod wrapper;
pub use wrapper::*;

#[link(name = "xinput")]
unsafe extern "system" {
    pub fn XInputGetState(user_index: u32, state: *mut XINPUT_STATE) -> u32;
    pub fn XInputSetState(user_index: u32, vibration: *mut XINPUT_VIBRATION) -> u32;
    pub fn XInputGetCapabilities(
        user_index: u32,
        flags: u32,
        capabilities: *mut XINPUT_CAPABILITIES,
    ) -> u32;
    pub fn XInputEnable(enable: i32);
    pub fn XInputGetAudioDeviceIds(
        user_index: u32,
        render_device_id: *mut u16,
        render_count: *mut u32,
        capture_device_id: *mut u16,
        capture_count: *mut u32,
    ) -> u32;
    pub fn XInputGetBatteryInformation(
        user_index: u32,
        dev_type: u8,
        battery_information: *mut XINPUT_BATTERY_INFORMATION,
    ) -> u32;
    pub fn XInputGetKeystroke(
        user_index: u32,
        reserved: u32,
        keystroke: *mut XINPUT_KEYSTROKE,
    ) -> u32;
    pub fn XInputGetDSoundAudioDeviceGuids(
        user_index: u32,
        dsound_render_guid: *mut GUID,
        dsound_capture_guid: *mut GUID,
    ) -> u32;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct XINPUT_STATE {
    pub packet_number: u32,
    pub gamepad: XINPUT_GAMEPAD,
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct Button(pub u16);

impl Button {
    pub const DPAD_UP: u16 = 0x0001;
    pub const DPAD_DOWN: u16 = 0x0002;
    pub const DPAD_LEFT: u16 = 0x0004;
    pub const DPAD_RIGHT: u16 = 0x0008;
    pub const START: u16 = 0x0010;
    pub const BACK: u16 = 0x0020;
    pub const LEFT_THUMB: u16 = 0x0040;
    pub const RIGHT_THUMB: u16 = 0x0080;
    pub const LEFT_SHOULDER: u16 = 0x0100;
    pub const RIGHT_SHOULDER: u16 = 0x0200;
    pub const A: u16 = 0x1000;
    pub const B: u16 = 0x2000;
    pub const X: u16 = 0x4000;
    pub const Y: u16 = 0x8000;
}

impl PartialEq<u16> for Button {
    fn eq(&self, other: &u16) -> bool {
        self.0.eq(other)
    }

    fn ne(&self, other: &u16) -> bool {
        !self.0.eq(other)
    }
}

impl std::ops::BitOr for Button {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::fmt::Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.0;
        let mappings = [
            (Self::DPAD_UP, "DPAD_UP"),
            (Self::DPAD_DOWN, "DPAD_DOWN"),
            (Self::DPAD_LEFT, "DPAD_LEFT"),
            (Self::DPAD_RIGHT, "DPAD_RIGHT"),
            (Self::START, "START"),
            (Self::BACK, "BACK"),
            (Self::LEFT_THUMB, "LEFT_THUMB"),
            (Self::RIGHT_THUMB, "RIGHT_THUMB"),
            (Self::LEFT_SHOULDER, "LEFT_SHOULDER"),
            (Self::RIGHT_SHOULDER, "RIGHT_SHOULDER"),
            (Self::A, "A"),
            (Self::B, "B"),
            (Self::X, "X"),
            (Self::Y, "Y"),
        ];

        let mut buttons = [""; 14];
        let mut count = 0;

        for &(mask, name) in &mappings {
            if value & mask != 0 {
                buttons[count] = name;
                count += 1;
            }
        }

        write!(f, "XInputGamepad({})", buttons[..count].join(" | "))
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
///https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad
pub struct XINPUT_GAMEPAD {
    ///Bitflags for the buttons pressed. Excludes triggers which are found bellow.
    pub buttons: Button,
    ///The value is between 0 and 255.
    pub left_trigger: u8,
    ///The value is between 0 and 255.
    pub right_trigger: u8,
    ///Each of the thumbstick axis members is a signed value between -32768 and 32767 describing the position of the thumbstick. A value of 0 is centered. Negative values signify down or to the left. Positive values signify up or to the right. The constants XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE or XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE can be used as a positive and negative value to filter a thumbstick input.
    pub left_stick_x_axis: i16,
    ///The value is between -32768 and 32767.
    pub left_stick_y_axis: i16,
    ///The value is between -32768 and 32767.
    pub right_thumb_x_axis: i16,
    ///The value is between -32768 and 32767.
    pub right_thumb_y_axis: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct XINPUT_KEYSTROKE {
    pub virtual_key: u16,
    pub unicode: u16,
    pub flags: u16,
    pub user_index: u8,
    pub hid_code: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct XINPUT_CAPABILITIES {
    pub controller_type: u8,
    pub controller_sub_type: u8,
    ///Features of the controller.
    pub flags: u16,
    pub gamepad: XINPUT_GAMEPAD,
    pub vibration: XINPUT_VIBRATION,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Set the vibration speed of each motor.
/// 0 to 65,535.
pub struct XINPUT_VIBRATION {
    pub left_motor_speed: u16,
    pub right_motor_speed: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct XINPUT_BATTERY_INFORMATION {
    pub battery_type: u8,
    pub battery_level: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GUID {
    pub Data1: u32,
    pub Data2: u16,
    pub Data3: u16,
    pub Data4: [u8; 8],
}

pub const ERROR_DEVICE_NOT_CONNECTED: u32 = 1167;
pub const ERROR_EMPTY: u32 = 4306;

pub const XINPUT_DEVTYPE_GAMEPAD: u8 = 0x01;

pub const XINPUT_DEVSUBTYPE_GAMEPAD: u8 = 0x01;
pub const XINPUT_DEVSUBTYPE_UNKNOWN: u8 = 0x00;
pub const XINPUT_DEVSUBTYPE_WHEEL: u8 = 0x02;
pub const XINPUT_DEVSUBTYPE_ARCADE_STICK: u8 = 0x03;
pub const XINPUT_DEVSUBTYPE_FLIGHT_SICK: u8 = 0x04;
pub const XINPUT_DEVSUBTYPE_DANCE_PAD: u8 = 0x05;
pub const XINPUT_DEVSUBTYPE_GUITAR: u8 = 0x06;
pub const XINPUT_DEVSUBTYPE_GUITAR_ALTERNATE: u8 = 0x07;
pub const XINPUT_DEVSUBTYPE_DRUM_KIT: u8 = 0x08;
pub const XINPUT_DEVSUBTYPE_GUITAR_BASS: u8 = 0x0B;
pub const XINPUT_DEVSUBTYPE_ARCADE_PAD: u8 = 0x13;

pub const XINPUT_CAPS_VOICE_SUPPORTED: u16 = 0x0004;
pub const XINPUT_CAPS_FFB_SUPPORTED: u16 = 0x0001;
pub const XINPUT_CAPS_WIRELESS: u16 = 0x0002;
pub const XINPUT_CAPS_PMD_SUPPORTED: u16 = 0x0008;
pub const XINPUT_CAPS_NO_NAVIGATION: u16 = 0x0010;

pub const XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE: i16 = 7849;
pub const XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE: i16 = 8689;
pub const XINPUT_GAMEPAD_TRIGGER_THRESHOLD: u8 = 30;

pub const XINPUT_FLAG_GAMEPAD: u32 = 0x00000001;

pub const BATTERY_DEVTYPE_GAMEPAD: u8 = 0x00;
pub const BATTERY_DEVTYPE_HEADSET: u8 = 0x01;

pub const BATTERY_TYPE_DISCONNECTED: u8 = 0x00;
pub const BATTERY_TYPE_WIRED: u8 = 0x01;
pub const BATTERY_TYPE_ALKALINE: u8 = 0x02;
pub const BATTERY_TYPE_NIMH: u8 = 0x03;
pub const BATTERY_TYPE_UNKNOWN: u8 = 0xFF;

pub const BATTERY_LEVEL_EMPTY: u8 = 0x00;
pub const BATTERY_LEVEL_LOW: u8 = 0x01;
pub const BATTERY_LEVEL_MEDIUM: u8 = 0x02;
pub const BATTERY_LEVEL_FULL: u8 = 0x03;

pub const XUSER_MAX_COUNT: u32 = 4;
pub const XUSER_INDEX_ANY: u32 = 0x000000FF;

pub const VK_PAD_A: u16 = 0x5800;
pub const VK_PAD_B: u16 = 0x5801;
pub const VK_PAD_X: u16 = 0x5802;
pub const VK_PAD_Y: u16 = 0x5803;
pub const VK_PAD_RSHOULDER: u16 = 0x5804;
pub const VK_PAD_LSHOULDER: u16 = 0x5805;
pub const VK_PAD_LTRIGGER: u16 = 0x5806;
pub const VK_PAD_RTRIGGER: u16 = 0x5807;
pub const VK_PAD_DPAD_UP: u16 = 0x5810;
pub const VK_PAD_DPAD_DOWN: u16 = 0x5811;
pub const VK_PAD_DPAD_LEFT: u16 = 0x5812;
pub const VK_PAD_DPAD_RIGHT: u16 = 0x5813;
pub const VK_PAD_START: u16 = 0x5814;
pub const VK_PAD_BACK: u16 = 0x5815;
pub const VK_PAD_LTHUMB_PRESS: u16 = 0x5816;
pub const VK_PAD_RTHUMB_PRESS: u16 = 0x5817;
pub const VK_PAD_LTHUMB_UP: u16 = 0x5820;
pub const VK_PAD_LTHUMB_DOWN: u16 = 0x5821;
pub const VK_PAD_LTHUMB_RIGHT: u16 = 0x5822;
pub const VK_PAD_LTHUMB_LEFT: u16 = 0x5823;
pub const VK_PAD_LTHUMB_UPLEFT: u16 = 0x5824;
pub const VK_PAD_LTHUMB_UPRIGHT: u16 = 0x5825;
pub const VK_PAD_LTHUMB_DOWNRIGHT: u16 = 0x5826;
pub const VK_PAD_LTHUMB_DOWNLEFT: u16 = 0x5827;
pub const VK_PAD_RTHUMB_UP: u16 = 0x5830;
pub const VK_PAD_RTHUMB_DOWN: u16 = 0x5831;
pub const VK_PAD_RTHUMB_RIGHT: u16 = 0x5832;
pub const VK_PAD_RTHUMB_LEFT: u16 = 0x5833;
pub const VK_PAD_RTHUMB_UPLEFT: u16 = 0x5834;
pub const VK_PAD_RTHUMB_UPRIGHT: u16 = 0x5835;
pub const VK_PAD_RTHUMB_DOWNRIGHT: u16 = 0x5836;
pub const VK_PAD_RTHUMB_DOWNLEFT: u16 = 0x5837;

pub const XINPUT_KEYSTROKE_KEYDOWN: u16 = 0x0001;
pub const XINPUT_KEYSTROKE_KEYUP: u16 = 0x0002;
pub const XINPUT_KEYSTROKE_REPEAT: u16 = 0x0004;
