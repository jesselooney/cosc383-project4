use std::time::SystemTime;
mod bit_patterns;
use bit_patterns::transject;
use bitvec::prelude::*;

fn main() {
    let img = image::open("blahaj-in-bando.png").unwrap();

    let start = SystemTime::now();

    let mut bits = bitvec![usize, LocalBits; 0; 40];
    bits[0..10].store::<u16>(0x3A8);
    bits[10..20].store::<u16>(0x2F9);
    bits[20..30].store::<u16>(0x154);
    bits[30..40].store::<u16>(0x06D);
    println!("{:?}, {}", bits, bits.len());
    //transject(img.into(), bit_patterns::patterns::access_all, bits);
    let end = SystemTime::now();

    let duration = end.duration_since(start).unwrap();
    println!("it took {} seconds", duration.as_secs());
}
