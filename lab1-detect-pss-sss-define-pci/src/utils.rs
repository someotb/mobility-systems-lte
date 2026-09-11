use std::fs;

pub fn read_file(filename: &str) -> Vec<i16> {
    let bytes = fs::read(filename).unwrap();
    let mut samples = Vec::new();
    for chunk in bytes.chunks(2) {
        samples.push(i16::from_le_bytes([chunk[0], chunk[1]]));
    }
    samples
}
