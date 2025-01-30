#![allow(deprecated)]
use xinput::*;

fn main() {
    vibrate(0, 20000, 20000).unwrap();
    std::thread::sleep_ms(500);
    vibrate(0, 0, 0).unwrap();

    loop {
        for controller in 0..4 {
            if let Ok(state) = get_state(controller) {
                print!("\x1B[1;1H");
                print!("\x1B[2J");
                println!("{:#?}", state);
            }
        }

        std::thread::sleep_ms(100);
    }
}
