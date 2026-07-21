use crate::fft::fourier_transform::ComplexValue;

pub struct BitReverser {
    size: u8,
}

impl BitReverser {
    pub fn new(size: u8) -> Self {
        Self { size }
    }

    pub fn get_reverse<T>(&self, source: &[T]) -> Vec<T>
    where
        T: Clone + Default,
    {
        let size = 2usize.pow(self.size as u32);
        let mut reversed = vec![T::default(); size];
        for (k, v) in source.iter().enumerate() {
            let k = self.reverse(k);
            reversed[k] = v.clone();
        }
        reversed
    }

    fn reverse(&self, i: usize) -> usize {
        // 1. Mask out any accidental padding bits above our length
        let clean_value = i & ((1 << self.size) - 1);
        // 2. Full 64/32-bit reverse, then shift down to the right position
        clean_value.reverse_bits() >> (usize::BITS - (self.size as u32))
    }


}

mod test {

    use super::BitReverser;
    #[test]
    fn test_reverse_array() {
        let source = vec![0, 1, 2, 3, 4, 5, 6];
        let bit_reverse_array = BitReverser::new( 3);
        assert_eq!(bit_reverse_array.get_reverse(&source), vec![0, 4, 2, 6, 1, 5, 3, 0]);
    }
}