mod automatic;
mod decode;
mod extensions;
mod extract;
mod helpers;
mod iteration_order;
use crate::iteration_order::{IterationOrder, Order::Forward};
use std::fs;
use std::str;

use crate::automatic::*;
use crate::extensions::*;
use crate::extract::*;
use crate::helpers::*;
use anyhow::Result;
use bitvec::field::BitField;
use bitvec::prelude::*;
use image::*;
use itertools::iproduct;
use itertools::Itertools;

use num::integer::gcd;
use num::BigUint;

fn totient(n: u64) -> u64 {
    let mut count = 0;
    for i in 1..n {
        print!("Trying i={}              \r", i);
        if gcd(i, n) == 1 {
            count += 1;
        }
    }
    count
}

fn crack_rsa(e: u64, phi_n: u64) -> u64 {
    for d in 1..phi_n {
        print!("Trying d={}               \r", d);
        if (e * d) % phi_n == 1 {
            return d;
        }
    }
    0
}

// https://rob.co.bb/posts/2019-02-10-modular-exponentiation-in-rust/
fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

fn main() -> Result<()> {
    /*
    if let Err(err) = write_extractions_dir("assets/sources/", "assets/extractions3/") {
        println!("{}", err);
    }

    if let Err(err) = write_extractions_dir("assets/extractions3/", "assets/extractions4/") {
        println!("{}", err);
    }*/

    //write_amplified_dir("tests/sources/", "tests/amplified").expect("failed to amplify all");

    /*
    write_extracted_bytes(
        "tests/sources/BombAnswers-extract.png",
        &IterationOrder::left_to_right_top_to_bottom([0, 1, 2], [0]),
    )
    .expect("should work");*/

    let bytes = fs::read("BombAnswers-extract-extract.bin")?;
    let bits: BitVec<u8> = bytes[0..bytes.len() - 1].iter().map(|x| *x != 48).collect();
    let bytes2: &[u8] = bits.as_raw_slice();
    let mut bits2: BitVec<u8> = bytes2.iter().map(|x| *x != 12).collect();
    bits2.chunks_exact_mut(8).for_each(|bs| bs.reverse());
    let bytes3 = bits2.as_raw_slice();
    println!("{:?}", str::from_utf8(bytes3)?);

    /*
        let im1: RgbImage = image::open("assets/extractions/Ideal-extract_FF_0_0_0123.png")?.into();
        let im2: RgbImage = image::open("assets/extractions/Dance-extract_FF_1_0_0123.png")?.into();

        let bits1 = extract_bits(&im1, &IterationOrder::all());
        let bits2 = extract_bits(&im2, &IterationOrder::all());

        let bits3 = bits1 ^ bits2;

        RgbImage::from_bitvec(im1.width(), im1.height(), bits3)?.save("xor-out.png")?;
    */
    /*
    let message = [
        139986, 145697, 197229, 465230, 391098, 278252, 197229, 317115, 465230, 513217, 285941,
        38287, 145697, 197229, 345398, 513217, 465230, 260043, 285941, 513217, 14960, 213315,
        197229, 82525, 265974, 396661, 285941, 197229, 278252, 38287, 513217, 139944, 285941,
        26770, 37756, 197229, 26770, 345398, 465230, 37756, 197229, 144595, 26770, 197229, 513217,
        144595, 145552, 396661, 37756, 197229, 133784, 145697, 278252, 285941, 513217, 197229,
        37756, 396661, 285941, 197229, 391098, 38287, 148799, 345398, 434094, 82525, 197229,
        495641, 465230, 148799, 285941, 37756, 144595, 148799, 285941, 26770, 197229, 37756,
        396661, 285941, 197229, 215248, 513217, 342500, 231482, 38287, 334938, 342500, 206007,
        197229, 144595, 26770, 197229, 513217, 144595, 145552, 396661, 37756, 197229, 133784,
        145697, 278252, 285941, 513217, 197229, 226600, 465230, 133784, 513217, 197229, 145697,
        465230, 26770, 285941, 434094, 197229, 52022, 285941, 197229, 148799, 38287, 226600,
        197229, 465230, 103367, 37756, 285941, 145697, 197229, 278252, 144595, 26770, 513217,
        285941, 145552, 38287, 513217, 278252, 197229, 26770, 465230, 148799, 285941, 37756,
        396661, 144595, 145697, 145552, 197229, 37756, 396661, 38287, 37756, 197229, 26770, 285941,
        285941, 148799, 26770, 197229, 465230, 14960, 260043, 144595, 465230, 133784, 26770,
        197229, 14960, 285941, 13908, 38287, 133784, 26770, 285941, 197229, 144595, 37756, 177906,
        26770, 197229, 117115, 144595, 145697, 197229, 37756, 396661, 285941, 197229, 391098,
        144595, 145552, 396661, 37756, 177906, 434094, 434094, 434094, 197229, 395435, 260043,
        285941, 145697, 197229, 314860, 396661, 285941, 145697, 197229, 37756, 396661, 285941,
        197229, 495641, 465230, 226600, 465230, 145697, 197229, 144595, 26770, 197229, 144595,
        145697, 197229, 345398, 391098, 38287, 144595, 145697, 197229, 26770, 144595, 145552,
        396661, 37756, 434094,
    ];

    let n = 524357;
    let e = 7;
    let phi_n = totient(n);
    let d = crack_rsa(e, phi_n);
    println!("{d}");

    let decrypted = message.map(|x| mod_pow(x, d, n) as u8);
    println!("{:?}", decrypted);
    let s = str::from_utf8(&decrypted)?;
    println!("{:?}", s);*/
    /*for double_byte in decrypted {
        bits.extend(double_byte.view_bits::<Lsb0>());
    }*/

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_totient() {
        assert_eq!(totient(9), 6);
        assert_eq!(totient(10), 4);
        assert_eq!(totient(19), 18);
        assert_eq!(totient(100), 40);
    }
}
