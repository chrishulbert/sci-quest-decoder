// This is responsible for decompressing LZW data.
// https://github.com/scummvm/scummvm/blob/master/engines/sci/resource/decompressor.cpp

use crate::bitstream;

pub fn decompress(src: &[u8], decompressed_size: usize) -> Vec<u8> {
    const INITIAL_CODE_BIT_LENGTH: usize = 9;
    const INITIAL_TABLE_SIZE: usize = 258;
    const INITIAL_CODE_LIMIT: usize = 512; // SCI0.
    const MAX_TABLE_SIZE: usize = 4096;

    let mut code_bit_length = INITIAL_CODE_BIT_LENGTH;
	let mut table_size = INITIAL_TABLE_SIZE;
    let mut code_limit = INITIAL_CODE_LIMIT;
    let mut string_offsets: Vec<usize> = vec![0; MAX_TABLE_SIZE]; // 0-257: unused
    let mut string_lengths: Vec<usize> = vec![0; MAX_TABLE_SIZE];

    let mut stream = bitstream::BitStream::new(src);
    let mut out: Vec<u8> = Vec::new();

	while out.len() < decompressed_size {
        let code = stream.next(code_bit_length);

		if code >= table_size {
            panic!("LZW code {} exceeds table size {}", code, table_size);
		}

		if code == 257 { break } // Terminator.

		if code == 256 { // Reset.
			code_bit_length = INITIAL_CODE_BIT_LENGTH;
			table_size = INITIAL_TABLE_SIZE;
			code_limit = 512;
			continue;
		}

		let new_string_offset = out.len();
		if code <= 255 {
            out.push(code as u8);
		} else {
			// Code is a table index.
            let len = string_lengths[code];
            let offset = string_offsets[code];
            for i in 0..len {
                out.push(out[offset + i]);

                // Boundary check included because the previous decompressor had a
                // comment saying it's "a normal situation" for a string to attempt
                // to write beyond the destination. I have not seen this occur.
                if out.len() >= decompressed_size { break }
            }
		}

		// Stop adding to the table once it is full.
		if table_size >= MAX_TABLE_SIZE { continue }

		// Increase code size once a bit limit has been reached.
		if table_size == code_limit && code_bit_length < 12 {
			code_bit_length += 1;
			code_limit = 1 << code_bit_length;
		}

		// Append code to table.
		string_offsets[table_size] = new_string_offset;
		string_lengths[table_size] = out.len() - new_string_offset + 1;
		table_size += 1;
    }

    out
}
