use crate::shdr::SectionHeader; 

pub struct Section {
    pub hdr: SectionHeader,
    pub content: Vec<u8>
}

impl Section {
    pub fn from(hdr: SectionHeader, bin: &Vec<u8>) -> Section {
        return Section{
            hdr,
            content: bin.to_vec()
        }
    }
}

