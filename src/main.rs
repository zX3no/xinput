use xinput::*;

fn main() {
    unsafe {
        XInputEnable(1);
        loop {
            let mut state = core::mem::zeroed();
            let _ = XInputGetState(0, &mut state);

            print!("\x1B[2J\x1B[H");
            println!("{:#?}", state);

            #[allow(deprecated)]
            std::thread::sleep_ms(100);
        }
    }
}
