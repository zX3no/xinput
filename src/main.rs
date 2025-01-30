use xinput::*;

fn main() {
    loop {
        for controller in 0..4 {
            if let Ok(state) = get_state(controller) {
                print!("\x1B[1;1H");
                print!("\x1B[2J");
                println!("{:#?}", state);
            }
        }

        #[allow(deprecated)]
        std::thread::sleep_ms(100);
    }
}
