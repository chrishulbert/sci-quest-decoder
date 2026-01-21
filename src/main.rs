mod bitstream_lsb;
mod bitstream_msb;
mod decode;
mod huffman;
mod lzw;
mod map;
mod palette;
mod picture_splitter;
mod picture;
mod png;
mod renderer;
mod resource_files;
mod resource_reader;
mod view;
mod xbrz;

fn main() {
    println!("-=[ SCI Quest Decoder ]=-");

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage:");
        println!("sci-quest-decoder /Path/To/SQ3");
    } else {
        decode::decode(&args[1]);
    }
}
