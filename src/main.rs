use xinput::*;

fn main() {
    loop {
        let state = get_state(0).unwrap();

        print!("\x1B[1;1H");
        print!("\x1B[2J");

        println!("{:#?}", state);

        let keystroke = get_keystroke(0).unwrap();
        println!("{:#?}", keystroke);

        #[allow(deprecated)]
        std::thread::sleep_ms(100);
    }
}
