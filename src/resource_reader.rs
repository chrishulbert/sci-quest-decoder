// This is responsible for reading the resources out of the files.
// https://wiki.scummvm.org/index.php?title=SCI/Specifications/Resource_files/SCI0_resources
// https://wiki.scummvm.org/index.php?title=SCI/Specifications/Resource_files/Decompression_algorithms
// Note this won't support SCI0.1 games because method 2 = COMP3: https://sciwiki.sierrahelp.com/index.php/Sierra_SCI_Release_List#SCI0.1

use crate::resource_files::Files;
use crate::map::Entry;
use crate::lzw;
use crate::huffman;

const METHOD_UNCOMPRESSED: usize = 0;
const METHOD_LZW: usize = 1;
const METHOD_HUFFMAN: usize = 2;

pub fn read(entry: &Entry, files: &Files) -> Vec<u8> {
    // Get the data from the appropriate file:
    let file = files.files.get(&entry.file).unwrap();
    let header_onwards = &file[entry.offset..];
    let content_onwards = &header_onwards[8..];

    // Parse the header:
    let id = (header_onwards[0] as usize) + ((header_onwards[1] as usize) << 8);
    let compressed_size  = (header_onwards[2] as usize) + ((header_onwards[3] as usize) << 8);
    let decompressed_size = (header_onwards[4] as usize) + ((header_onwards[5] as usize) << 8);
    let method = (header_onwards[6] as usize) + ((header_onwards[7] as usize) << 8);

    // Use the header to get the maybe-compressed content of correct length:
    if id != entry.id {
        panic!("Id for resource data doesn't match!");
    }
    let actual_compressed_size = compressed_size - 4; // Compressed size actually starts counting at the record position of decompressed_size.
    let content = &content_onwards[..actual_compressed_size]; // Maybe-compressed content.

    // Decompress it if necessary:
    decompress(content, decompressed_size, method)
}

fn decompress(content: &[u8], decompressed_size: usize, method: usize) -> Vec<u8> {
    if method == METHOD_UNCOMPRESSED {
        assert!(content.len() == decompressed_size, "Sizes must be the same when uncompressed!");
        content.to_vec()
    } else if method == METHOD_LZW {
        lzw::decompress(content, decompressed_size)
    } else if method == METHOD_HUFFMAN {
        huffman::decompress(content, decompressed_size)
    } else {
        panic!("Unknown compression method: {}", method);
    }
}
