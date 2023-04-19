fn main() {
    match dirs::home_dir() {
        Some(home_dir) => println!("{:?}", home_dir),
        None => panic!("The path error"),
    }

    match dirs::audio_dir() {
        Some(dir) => println!("{}", dir.display()),
        None => panic!("The path error"),
    }
}
