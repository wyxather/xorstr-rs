use std::{
    fs::File,
    io::Write,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

fn time() {
    let total_seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let hours = (total_seconds / 3600) % 24;
    let minutes = (total_seconds / 60) % 60;
    let seconds = total_seconds % 60;
    let output = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
    let path: &Path = Path::new("src/xorstr/time.rs");
    let mut file = File::create(path).unwrap();
    let _ = writeln!(file, "pub const __TIME__: &str = {:?};", output);
}

fn main() {
    time();
}
