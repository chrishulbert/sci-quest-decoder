// This is responsible for iterating bits in a manner that LZW consumes.
// It gives you N bits at a time, happily spanning byte boundaries.
// If you pass it this data:
// Byte: AAAAAAAA BBBBBBBB CCCCCCCC DDDDDDDD
// Bits: 76543210 76543210 76543210 76543210
// It will return bits in the following order:
// A0-7 B0-7 C0-7 D0-7

pub struct BitStreamLSB<'a> {
    data: &'a [u8],
    index: usize,
    bit_buffer: usize,
    bits_in_buffer: usize,
}

impl<'a> BitStreamLSB<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        BitStreamLSB { data, index: 0, bit_buffer: 0, bits_in_buffer: 0 }
    }

    pub fn next(&mut self, bits_wanted: usize) -> usize {
        assert!(1 <= bits_wanted && bits_wanted <= 16);

        // Fetch more from data to top up the buffer.
        while bits_wanted > self.bits_in_buffer {
            // Grab another byte, add it to bit_buffer, shifted to the significant end.
            let byte = self.data[self.index];
            self.index += 1;
            self.bit_buffer += (byte as usize) << self.bits_in_buffer;
            self.bits_in_buffer += 8;
        }

        let mask_for_bits_wanted = (1 << bits_wanted) - 1; // This trick sets the first N bits to 1.
        let value = self.bit_buffer & mask_for_bits_wanted;
        self.bit_buffer >>= bits_wanted;
        self.bits_in_buffer -= bits_wanted;
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bitstream() {
        let input: Vec<u8> = vec![0b10101010, 0b00001111, 0b00110011];
        // Returns the following in order 0-n:
        // First:                      43210
        // Next:                    210              543
        // Last:                                43210       cba98765
        let mut stream= BitStreamLSB::new(&input);
        let first_5 = stream.next(5);
        let next_6 = stream.next(6);
        let last_13 = stream.next(13);
        assert_eq!(first_5, 0b01010);
        assert_eq!(next_6, 0b111101);
        assert_eq!(last_13, 0b0011001100001);
    }
}
