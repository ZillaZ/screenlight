pub fn get_brightness(gpu: &str) -> i32 {
    let brightness_str = std::fs::read_to_string(format!("/sys/class/backlight/{}/brightness", gpu)).expect("Unable to read screen brightness level");
    brightness_str.trim().parse::<i32>().unwrap()
}

pub fn set_brightness(gpu: &str, level: i32) {
    std::fs::write(format!("/sys/class/backlight/{}/brightness", gpu), level.to_string()).unwrap();
}
