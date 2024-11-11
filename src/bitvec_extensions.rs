use bitvec::prelude::*;

pub trait BitVecExt {
    fn to_bytes_le(&self) -> Vec<u8>;
    fn to_bytes_be(&self) -> Vec<u8>;
}

impl BitVecExt for BitVec {
    fn to_bytes_le(&self) -> Vec<u8> {
        self.chunks_exact(8).map(|chunk| chunk.load_le()).collect()
    }
    fn to_bytes_be(&self) -> Vec<u8> {
        self.chunks_exact(8).map(|chunk| chunk.load_be()).collect()
    }
}
