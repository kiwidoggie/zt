pub struct ClvHeader {
    // 31
    pub magic: u32,

    pub unknown04: u32,
    pub unknown08: u32,
    pub unknown12: u32,

    // Some kind of count
    // // so example if there are 512 references of 0x180AA684, the unknown16 will be 256
    pub unknown16: u32,
}

impl ClvHeader {
    pub fn from_reader(reader: &mut binary_reader::BinaryReader) -> ClvHeader {
        return ClvHeader {
            magic: reader.read_u32().unwrap(),
            unknown04: reader.read_u32().unwrap(),
            unknown08: reader.read_u32().unwrap(),
            unknown12: reader.read_u32().unwrap(),
            unknown16: reader.read_u32().unwrap()
        };
    }
}

pub struct ClvEntry {
    // 0xDEADBEEF
    pub magic: u32,

    pub unknown04: u32,

    // Length in bytes of the name
    pub name_length: u32,

    // Name of this zrb reference
    //nameData: Vec<char>,

    // HELPER: NOT IN STRUCTURE
    pub name: String,
}

impl ClvEntry {
    pub fn from_reader(reader: &mut binary_reader::BinaryReader) -> ClvEntry {
        return ClvEntry {
            magic: reader.read_u32().unwrap(),
            unknown04: reader.read_u32().unwrap(),
            name_length: reader.read_u32().unwrap(),
            name: reader.read_cstr()
        };
    }
}