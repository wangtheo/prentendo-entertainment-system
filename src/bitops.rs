/// Helper trait for defining bit operations
pub trait BitOps {
    fn is_bit_set(&self, val: usize) -> bool;
    fn set_bit(&mut self, val: usize);
    fn clear_bit(&mut self, val: usize);
    fn assign_bit(&mut self, bit: usize, val: bool);
    fn replace_bits(&self, mask: Self, new_bits: Self) -> Self;
}

macro_rules! bitops_impl {
    ($t:ty) => {
        impl BitOps for $t {
            #[inline]
            fn is_bit_set(&self, val: usize) -> bool {
                (self & 1 << val) != 0
            }

            #[inline]
            fn set_bit(&mut self, val: usize) {
                *self |= 1 << val;
            }

            #[inline]
            fn clear_bit(&mut self, val: usize) {
                *self &= !(1 << val);
            }

            #[inline]
            fn assign_bit(&mut self, bit: usize, val: bool) {
                if val {
                    self.set_bit(bit);
                } else {
                    self.clear_bit(bit);
                }
            }

            #[inline]
            fn replace_bits(&self, mask: Self, new_bits: Self) -> Self {
                (self & !mask) | (mask & new_bits)
            }
        }
    };
}

bitops_impl!(u8);
bitops_impl!(u16);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_bit_set() {
        let byte: u8 = 0b0010_1100;
        assert!(byte.is_bit_set(2));
        assert!(!byte.is_bit_set(4));
    }

    #[test]
    fn test_set_bit() {
        let mut byte: u8 = 0b0010_1100;
        byte.set_bit(0);
        byte.set_bit(3);
        assert_eq!(byte, 0b0010_1101);
    }

    #[test]
    fn test_clear_bit() {
        let mut byte: u8 = 0b0010_1100;
        byte.clear_bit(5);
        byte.clear_bit(0);
        assert_eq!(byte, 0b0000_1100);
    }

    #[test]
    fn test_replace_bits() {
        let byte: u16 = 0b00000_00100;
        assert_eq!(byte.replace_bits(0b11111_00000, 0b10010_11111), 0b10010_00100);
    }
}
