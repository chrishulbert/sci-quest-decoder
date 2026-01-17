// This is responsible for coordinating the whole decoding process.

use crate::map;
use crate::resource_files;
use crate::resource_reader;
use crate::view;
use crate::renderer;

pub fn decode(path: &str) {
    let map = map::Map::read(path);
    let files = resource_files::Files::read(path);

    for (vi, entry) in map.entries.iter().enumerate() {
        if entry.resource_type != map::ResourceType::View { continue }
        let resource = resource_reader::read(entry, &files);
        let view = view::View::parse(&resource);
        for (li, l) in view.loops.iter().enumerate() {
            if renderer::is_animation(l) {
                // Animated.
                let name = format!("Output.view.{}.{}.animation.png", vi, li);
                let png = renderer::apng_from_loop(l);
                std::fs::write(name, png).unwrap();
            } else {
                // Not animated.
                for (ci, c) in l.cels.iter().enumerate() {
                    let name = format!("Output.view.{}.{}.{}.static.png", vi, li, ci);
                    let png = renderer::png_from_cel(c);
                    std::fs::write(name, png).unwrap();
                }
            }
        }
    }

    // for (pi, resource) in resources.pictures.iter().enumerate() {
    //     if resource.is_empty() { continue }
    //     let picture = picture::Picture::parse(&resource);
    //     let name = format!("Output.picture.{}.static.png", pi);
    //     let png = renderer::png_from_picture(&picture);
    //     std::fs::write(name, png)?;
    // }
}
