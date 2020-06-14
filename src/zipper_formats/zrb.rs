pub const ZRB_FOOTER: u32 = 0xDEADBEEF;

/// .zrb file header information
/// 
pub struct ZrbHeader {
    /// This field so far has been either 4 or 6 (could this be a type)
    /// 
    pub unknown00: u32,
}

impl ZrbHeader {

    pub fn parse(&mut self, _reader: &mut binary_reader::BinaryReader) {
        // Create a new header
        let header = ZrbHeader {
            unknown00: _reader.read_u32().unwrap()
        };
    
        println!("header:");
        println!("unk00: {}", header.unknown00);
    }

    /// Parses and returns a new ZrbHeader from a binary reader
    /// 
    /// # Arguments
    /// 
    /// * `_reader` - Binary Reader opened to the correct file and position of the zrb header
    pub fn from_reader(_reader: &mut binary_reader::BinaryReader) -> ZrbHeader {
        return ZrbHeader {
            unknown00: _reader.read_u32().unwrap()
        };
    }
}

