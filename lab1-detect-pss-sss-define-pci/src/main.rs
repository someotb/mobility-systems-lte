mod utils;

// TODO: apply IFFT to PSS

fn main() {
    let samples = utils::read_file("data/lte_pss_sss_pbch_iq_s16le.pcm");
    let pss_zero = utils::generate_pss(utils::Nid2::Zero);
    let pss_one = utils::generate_pss(utils::Nid2::One);
    let pss_two = utils::generate_pss(utils::Nid2::Two);

    let (peak_idx_zero, peak_val_zero) = utils::correlator(&pss_zero, &samples);
    let (peak_idx_one, peak_val_one) = utils::correlator(&pss_one, &samples);
    let (peak_idx_two, peak_val_two) = utils::correlator(&pss_two, &samples);

    println!("ZERO peak at {}: {:#?}", peak_idx_zero, peak_val_zero);
    println!("ONE peak at {}: {:#?}", peak_idx_one, peak_val_one);
    println!("TWO peak at {}: {:#?}", peak_idx_two, peak_val_two);
}
