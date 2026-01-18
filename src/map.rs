// This is for loading the resource map file.
// https://wiki.scummvm.org/index.php?title=SCI/Specifications/Resource_files/SCI0_resources
// https://github.com/scummvm/scummvm/blob/master/engines/sci/resource/resource.cpp

use std::collections::HashSet;

#[derive(Debug)]
pub struct Map {
    pub entries: Vec<Entry>,
}

#[derive(Debug)]
pub struct Entry {
    pub id: usize, // The first 2 bytes, containing the type+number, is also used as an id.
    pub resource_type: ResourceType,
    pub resource_number: usize,
    pub file: usize,
    pub offset: usize,
}

#[derive(Debug, PartialEq)]
pub enum ResourceType {
	View,
    Pic,
    Script,
    Text,
	Sound,
    Memory,
    Vocab,
    Font,
	Cursor,
    Patch,
    Bitmap,
    Palette,
	CdAudio,
    Audio,
    Sync,
    Message,
	Map,
    Heap,
    Audio36,
    Sync36,
	Translation,
    Rave,
    Unknown,
}

impl Map {
    pub fn read(path: &str) -> Map {
        let path = format!("{}/resource.map", path);
        let data = std::fs::read(path).unwrap();
        let mut ids: HashSet<usize> = HashSet::new();
        let mut entries: Vec<Entry> = Vec::new();
        for chunk in data.chunks_exact(6) {
            let Some(entry) = Entry::from_data(chunk) else { continue };
            if ids.contains(&entry.id) { continue } // Since many common resources are stored on multiple disks, only add them once.
            ids.insert(entry.id);
            entries.push(entry);
        }
        Map { entries }
    }
}

impl Entry {
    fn from_data(data: &[u8]) -> Option<Entry> {
        let id = (data[0] as usize) + ((data[1] as usize) << 8);
        let rest = (data[2] as usize) + ((data[3] as usize) << 8) +
            ((data[4] as usize) << 16) + ((data[5] as usize) << 24);
        if id == 0xffff && rest == 0xffffffff { return None }
        let resource_type = id >> 11; // High 5 bits.
        let resource_type = ResourceType::from(resource_type);
        let resource_number = id & 0b111_11111111; // Low 11 bits.
        let file = rest >> 26; // High 6 bits.
        let offset = rest & 0x3ffffff; // Low 26 bits.
        Some(Entry{
            id,
            resource_type,
            resource_number,
            file,
            offset,
        })
    }
}

impl ResourceType {
    fn from(value: usize) -> ResourceType {
        match value {
            0 => ResourceType::View,
            1 => ResourceType::Pic,
            2 => ResourceType::Script,
            3 => ResourceType::Text,
            4 => ResourceType::Sound,
            5 => ResourceType::Memory,
            6 => ResourceType::Vocab,
            7 => ResourceType::Font,
            8 => ResourceType::Cursor,
            9 => ResourceType::Patch,
            10 => ResourceType::Bitmap,
            11 => ResourceType::Palette,
            12 => ResourceType::CdAudio,
            13 => ResourceType::Audio,
            14 => ResourceType::Sync,
            15 => ResourceType::Message,
            16 => ResourceType::Map,
            17 => ResourceType::Heap,
            18 => ResourceType::Audio36,
            19 => ResourceType::Sync36,
            20 => ResourceType::Translation,
            21 => ResourceType::Rave,
            _ => ResourceType::Unknown,
        }
    }
}
