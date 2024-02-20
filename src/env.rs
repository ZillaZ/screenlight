pub fn get_args() -> Vec<String> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        panic!("Not enough arguments!");
    }

    args
}
