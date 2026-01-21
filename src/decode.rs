// This is responsible for coordinating the whole decoding process.

use crate::map;
use crate::resource_files;
use crate::resource_reader;
use crate::view;
use crate::renderer;
use crate::picture;

pub fn decode(path: &str) {
    let map = map::Map::read(path);
    let files = resource_files::Files::read(path);

    // Pictures:
    for entry in &map.entries {
        if entry.resource_number != 15 { continue }
        if entry.resource_type != map::ResourceType::Picture { continue }
        let resource = resource_reader::read(entry, &files);
        let picture = picture::Picture::parse(&resource);
        let name = format!("Output.picture.rn{}.f{}.static.png", entry.resource_number, entry.file);
        let png = renderer::png_from_picture(&picture);
        std::fs::write(name, png).unwrap();
    }

    // Views:
    // for (vi, entry) in map.entries.iter().enumerate() {
    //     if entry.resource_type != map::ResourceType::View { continue }
    //     let resource = resource_reader::read(entry, &files);
    //     let view = view::View::parse(&resource);
    //     for (li, l) in view.loops.iter().enumerate() {
    //         if renderer::is_animation(l) {
    //             // Animated.
    //             let name = format!("Output.view.rn{}.f{}.vi{}.li{}.animation.png", entry.resource_number, entry.file, vi, li);
    //             let png = renderer::apng_from_loop(l);
    //             std::fs::write(name, png).unwrap();
    //         } else {
    //             // Not animated.
    //             for (ci, c) in l.cels.iter().enumerate() {
    //                 let name = format!("Output.view.rn{}.f{}.vi{}.li{}.ci{}.static.png", entry.resource_number, entry.file, vi, li, ci);
    //                 let png = renderer::png_from_cel(c);
    //                 std::fs::write(name, png).unwrap();
    //             }
    //         }
    //     }
    // }
}
