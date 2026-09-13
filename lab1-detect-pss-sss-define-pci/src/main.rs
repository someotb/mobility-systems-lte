mod utils;

// TODO: apply IFFT to PSS

fn main() {
    let samples = utils::read_file("data/lte_pss_sss_pbch_iq_s16le.pcm");
    println!("Samples: \n{:#?}", samples);
    let pss = utils::generate_pss(utils::Nid2::Two);
    println!("PSS: \n{:#?}", pss)
}
