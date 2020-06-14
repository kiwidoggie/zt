use std::fs::File;

pub const ZRB_FOOTER: u32 = 0xDEADBEEF;

/// .zrb file header information
/// 
pub struct ZrbHeader {
    pub unknown00: u32,
}

impl ZrbHeader {
    pub fn parse(&mut self, _options: &zipper_tool::Options) {
        // Open up the zrb file
        let mut file = File::open(&_options.input_file).unwrap();
    
        // Create a new reader
        let mut reader = binary_reader::BinaryReader::from_file(&mut file);
    
        // Create a new header
        let header = ZrbHeader {
            unknown00: reader.read_u32().unwrap()
        };
    
        println!("header:");
        println!("unk00: {}", header.unknown00);
    }
}