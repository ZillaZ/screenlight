use evdev_rs::{*, enums::{EventCode, EV_KEY}};

mod env;
mod brightness;
use env::get_args;

use brightness::*;

enum Which {
    UP,
    DOWN
}

fn main() {
    let args = get_args();
    let device_path = &args[1];
    let device_file = std::fs::File::open(device_path).expect("Unable to read file with given path.");
    let device = Device::new_from_file(device_file).expect("Unable to create virtual device from given file.");
    let gpu = &args[2];

    loop {
        let (_, event) = device.next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING).expect("Unable to read events from device");
        let which = match event.event_code {
            EventCode::EV_KEY(EV_KEY::KEY_F3) => Some(Which::UP),
            EventCode::EV_KEY(EV_KEY::KEY_F4) => Some(Which::DOWN),
            _ => None
        };

        if let Some(data) = which {
            let mut actual_brightness = get_brightness(gpu);
            let _ = match data {
                Which::UP => actual_brightness += 10,
                _ => actual_brightness -= 10,
            };
            set_brightness(gpu, actual_brightness);
        }
    }
}
