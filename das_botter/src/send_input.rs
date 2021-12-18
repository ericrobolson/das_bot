use crate::{key::*, toggle::Toggle};
use windows::{
    Win32::UI::Input::KeyboardAndMouse::SendInput,
    Win32::UI::Input::KeyboardAndMouse::{INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, *},
};

#[derive(Clone, Copy, Debug)]
pub enum InputError {}

pub fn send_keyboard(key: Key, toggle: Toggle) -> Result<(), InputError> {
    // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendinput
    // https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-keybdinput

    let virtual_key = key.to_virtual_key();

    let hardware_scan_code = 0;

    const NUM_INPUTS: u32 = 1;

    let mut dw_flags = 0;
    dw_flags |= match toggle {
        Toggle::On => 0,
        Toggle::Off => KEYEVENTF_KEYUP,
    };

    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: virtual_key,
                wScan: hardware_scan_code,
                dwFlags: dw_flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    let result = unsafe {
        SendInput(
            NUM_INPUTS,
            &input as *const _,
            std::mem::size_of::<INPUT>() as i32,
        )
    };
    match result {
        NUM_INPUTS => Ok(()),
        r => {
            todo!("how to handle result for SendInput - {:?}", r);
        }
    }
}
