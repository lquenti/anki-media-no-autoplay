const AUDIO_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/Bruh.mp3");
const VIDEO_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/aaaa.mp4");

fn main() {
    println!("{}", AUDIO_PATH);
    println!("{}", VIDEO_PATH);
}
