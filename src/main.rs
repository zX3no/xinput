#![allow(deprecated)]
use xinput::*;

fn main() {
    loop {
        for controller in 0..4 {
            if let Ok(state) = get_state(controller) {
                print!("\x1B[1;1H");
                print!("\x1B[2J");
                println!("{:#?}", state);

                let gamepad = state.gamepad;
                if gamepad.buttons.contains(Button::A | Button::B) && gamepad.left_trigger > 30 {
                    println!("Pressed A, B and the left trigger.");
                }
            }
        }

        std::thread::sleep_ms(100);
    }
}
