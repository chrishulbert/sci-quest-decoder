// This is responsible for decompressing Huffman-packed data.
// https://github.com/scummvm/scummvm/blob/master/engines/sci/resource/decompressor.cpp
// https://sciwiki.sierrahelp.com/index.php/SCI_Specifications:_Chapter_2_-_Resource_files#Decompression_algorithm_HUFFMAN
// https://github.com/icefallgames/SCICompanion/blob/master/SCICompanionLib/Src/Util/Codec.cpp

use crate::bitstream_msb::{self, BitStreamMSB};

pub fn decompress(src: &[u8], decompressed_size: usize) -> Vec<u8> {
    // Get the header info from the data:
    let node_count = src[0] as usize;
    let terminator = src[1];
    let node_size = node_count * 2;
    let nodes_data = &src[2..(2 + node_size)];
    let data = &src[(2 + node_size)..];

    // Parse it into nodes / prep it into a bitstream:
    let nodes: Vec<Node> = nodes_data.chunks_exact(2).map(Node::from).collect();
    let mut bitstream = bitstream_msb::BitStreamMSB::new(data);
    let mut out: Vec<u8> = Vec::with_capacity(decompressed_size);

    // Loop pulling a byte at a time until we hit a terminator or decompress enough bytes:
    loop {
        let (byte, is_bitstream_literal) = get_next_byte(&mut bitstream, &nodes);
        if is_bitstream_literal && byte == terminator { break }
        if out.len() >= decompressed_size { break } // In case there's no terminator.
        out.push(byte);
    }

    if out.len() != decompressed_size {
        panic!("Huffman decoding incorrect length: {}, expected {}", out.len(), decompressed_size);
    }

    out
}

struct Node {
    value: u8,
    siblings: u8,
}

impl Node {
    fn from(data: &[u8]) -> Self {
        Self { value: data[0], siblings: data[1] }
    }
}

// Bool true = from bitstream literal (right bit but 0 sibling).
// Bool false = from node value (no sibling left nor right).
fn get_next_byte(bitstream: &mut BitStreamMSB, nodes: &[Node]) -> (u8, bool) {
    let node = &nodes[0];
    if node.siblings == 0 { return (node.value, false) }
    let is_low_nibble = bitstream.next(1) != 0;
    let sibling = if is_low_nibble { node.siblings & 0x0f } else { node.siblings >> 4 };
    if sibling == 0 {
        let literal_token = bitstream.next(8) as u8;
        return (literal_token, true);
    } 
    get_next_byte(bitstream, &nodes[(sibling as usize)..])
}
