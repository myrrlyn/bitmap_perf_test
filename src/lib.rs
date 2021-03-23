use std::convert::TryInto;
use bitvec::prelude::*;
const BIT_MASK: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];

/// Sets bit at position `i` in `byte` according to LSB
// least-significant bit (LSB) numbering (also known as bit-endianness)
#[inline]
fn set(byte: u8, i: usize) -> u8 {
    byte | BIT_MASK[i]
}

/// Returns whether bit at position `i` in `byte` is set or not
#[inline]
fn is_set(byte: u8, i: usize) -> bool {
    (byte & BIT_MASK[i]) != 0
}

pub fn scalar_eq_bitmap1(lhs: &[i32], rhs: i32) -> Vec<u8> {
    let length = lhs.len();
    let chunks = length / 8;

    let mut buffer = vec![0u8; (length + 7) / 8];

    let chunks_iter = lhs.chunks_exact(8);
    let remainder = chunks_iter.remainder();

    buffer[..chunks]
        .iter_mut()
        .zip(chunks_iter)
        .for_each(|(byte, chunk)| {
            (0..8).for_each(|i| {
                if chunk[i] == rhs {
                    *byte = set(*byte, i)
                }
            });
        });

    if !remainder.is_empty() {
        let last = &mut buffer[chunks];
        remainder.iter().enumerate().for_each(|(i, value)| {
            if *value == rhs {
                *last = set(*last, i)
            }
        });
    };
    buffer
}

pub fn scalar_eq_bitmap(lhs: &[i32], rhs: i32) -> Vec<u8> {
    let length = lhs.len();
    let mut iterator = lhs.iter();

    let chunks = length / 8;
    let reminder = length % 8;

    let mut buffer = vec![0u8; (length + 7) / 8];

    buffer[..chunks].iter_mut().for_each(|byte| {
        (0..8).for_each(|i| {
            if *iterator.next().unwrap() == rhs {
                *byte = set(*byte, i)
            }
        })
    });

    if reminder != 0 {
        let last = &mut buffer[chunks];
        iterator.enumerate().for_each(|(i, value)| {
            if *value == rhs {
                *last = set(*last, i)
            }
        });
    };
    buffer
}

pub fn scalar_eq_bool(lhs: &[i32], rhs: i32) -> Vec<bool> {
    lhs.iter().map(|x| *x == rhs).collect()
}

pub fn bv_scalar_eq_loop(lhs: &[i32], rhs: i32) -> BitVec<Lsb0, u8> {
    let len = lhs.len();
    let mut buf = BitVec::with_capacity(len);
    for (mut bit, &val) in buf.iter_mut().zip(lhs.iter()) {
        *bit = val == rhs;
    }
    buf
}

pub fn bv_scalar_eq_collect(lhs: &[i32], rhs: i32) -> BitVec<Lsb0, u8> {
    lhs.iter().map(|&val| val == rhs).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitmap_basics() {
        let vec = (0..2049).map(|x| (x * x + x) % 10).collect::<Vec<_>>();
        let result = scalar_eq_bitmap(&vec, 0);

        (0..2049).for_each(|i| {
            let byte = result[i / 8];

            let result = is_set(byte, i % 8);
            let expected = (i * i + i) % 10 == 0;
            assert_eq!(result, expected);
        });
    }
}
