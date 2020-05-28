use crate::shdr::SectionHeader; 

pub struct Section {
    pub hdr: SectionHeader,
    pub content: Vec<u8>
}

impl Section {
    // this is where we 
    pub fn from(hdr: SectionHeader, bin: &Vec<u8>) -> Section {
        Section { 
            hdr,
            content: vec![]
        }
    }
}

