use num_complex::Complex64;
use std::fs;

pub enum Nid2 {
    Zero,
    One,
    Two,
}

pub fn read_file(filename: &str) -> Vec<Complex64> {
    let bytes = fs::read(filename).unwrap();
    let mut samples: Vec<Complex64> = Vec::new();
    for chunk in bytes.chunks(4) {
        let i = i16::from_le_bytes([chunk[0], chunk[1]]) as f64;
        let q = i16::from_le_bytes([chunk[2], chunk[3]]) as f64;
        samples.push(Complex64::new(i, q));
    }
    samples
}

pub fn generate_pss(nid: Nid2) -> Vec<Complex64> {
    let root_index: [i32; 3] = [25, 29, 34];
    let mut phase: f64;
    let mut d_u: Vec<Complex64> = Vec::new();
    let idx = match nid {
        Nid2::Zero => 0,
        Nid2::One => 1,
        Nid2::Two => 2,
    };
    for n in 0..=61 {
        let u = root_index[idx];
        if n <= 30 {
            phase = -std::f64::consts::PI * (u as f64) * (n as f64) * ((n + 1) as f64) / 63.0;
        } else {
            phase = -std::f64::consts::PI * (u as f64) * ((n + 1) as f64) * ((n + 2) as f64) / 63.0;
        }
        let d = Complex64::new(0.0, phase).exp();
        d_u.push(d);
    }
    d_u
}

pub fn correlator(pattern: &[Complex64], samples: &[Complex64]) -> (usize, f64) {
    let m = pattern.len();
    assert!(samples.len() >= m, "samples shorter than pattern");

    let correlations: Vec<Complex64> = (0..=samples.len() - m)
        .map(|k| {
            pattern
                .iter()
                .zip(samples[k..k + m].iter())
                .map(|(p, s)| p.conj() * s)
                .sum()
        })
        .collect();

    let (peak_idx, peak_val) = correlations
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.norm().partial_cmp(&b.norm()).unwrap())
        .unwrap();
    (peak_idx, peak_val.norm())
}
