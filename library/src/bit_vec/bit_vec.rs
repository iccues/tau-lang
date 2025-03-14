use super::{ucode, AsBits};
use std::ops::Range;

pub struct BitVec {
    buf: Vec<(Range<usize>, Box<dyn AsBits>)>,
    len: usize,
}

impl BitVec {
    pub fn new(len: usize) -> BitVec {
        if len % 8 != 0 {
            panic!();
        }
        BitVec { buf: vec![], len }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn sets(&mut self, index: Range<usize>, value: &dyn AsBits) {
        self.buf.push((index, value.boxed()));
    }

    fn shift(value: ucode, shift: isize) -> u8 {
        if shift >= 0 {
            (value << shift) as u8
        } else {
            (value >> (-shift)) as u8
        }
    }

    fn write_bits(buf: &mut [u8], item: &(Range<usize>, Box<dyn AsBits>)) {
        let bits = item.1.as_bits();
        let mask = (1 << item.0.len()) - 1;

        let mut shift: isize = item.0.start as isize;
        for byte in buf.iter_mut() {
            let shift_bits = BitVec::shift(bits, shift);
            let shift_maks = BitVec::shift(mask, shift);

            *byte &= !shift_maks;
            *byte |= shift_bits & shift_maks;

            shift -= 8;
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0; self.len / 8];
        for item in self.buf.iter() {
            BitVec::write_bits(buf.as_mut_slice(), item);
        }
        buf
    }
}

#[cfg(test)]
mod tests {
    use crate::bit_vec_builder;

    #[test]
    fn test_bitvec() {
        let bv = bit_vec_builder! {
            #[32]
            [0..8] : 0x12;
            [8..16] : 0x34;
            [16..24] : 0xFF56;
            [24..32] : 0xFF78;
        };
        assert_eq!(bv.as_bytes(), vec![0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_bitvec2() {
        let bv = bit_vec_builder! {
            #[32]
            [5..9] : 0x12;
            [10..14] : 0x34;
            [15..19] : 0xFF56;
            [20..24] : 0xFF78;
        };
        dbg!(bv.as_bytes());
    }

    // #[test]
    // fn test_bitvec2() {
    //     let mut bv = BitVec::new(32);
    //     let bv2 = Rc::new(BitVec::new(32));
    //     bv.sets(0..8, &bv2);
    //     drop(bv2);
    //     dbg!(&bv.buf[0].0);
    // }
}
