// This is responsible for loading the resource.00x files into memory.

use std::collections::HashMap;

pub struct Files {
    pub files: HashMap<usize, Vec<u8>>,
}

impl Files {
    pub fn read(path: &str) -> Files {
        let mut files: HashMap<usize, Vec<u8>> = HashMap::new();
        for i in 1..999 {
            let vol_path = format!("{}/resource.{:03}", path, i);
            let exists = std::fs::exists(&vol_path).unwrap();
            if !exists { break }
            let content = std::fs::read(&vol_path).unwrap();
            files.insert(i, content);
        }
        Files { files }
    }
}
