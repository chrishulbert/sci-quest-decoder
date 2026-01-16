mod map;
mod decode;
mod resource_files;

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
