mod utils;

fn main() {
    let samples = utils::read_file("data/lte_pss_sss_pbch_iq_s16le.pcm");
    println!("{:?}", samples);
}
