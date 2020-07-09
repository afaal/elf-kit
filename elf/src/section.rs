use crate::shdr::SectionHeader; 
use crate::block::Block; 

#[derive(Clone)]
pub struct Section {
    pub hdr: SectionHeader,
    pub content: Box<Block>,
}

impl Section {
    // this is where we 
    pub fn from(hdr: SectionHeader, bin: &Vec<u8>) -> Section {
        Section { 
            hdr,
            content: Box::new(Block::RawDat(bin.clone())) 
        }
    }

    pub fn to_le(self) -> Vec<u8> {
        return self.content.to_bin(); 
    }
}

