use crate::SectionHeader; 

pub struct Section {
    pub section_header: SectionHeader,
    pub content: Vec<u8>
}

impl Section {
    pub fn from(bin: &Vec<u8>g) -> Section {
        return Section{
            content: bin.to_vec()
        }
    }
}


pub fn parse_sections(bin: &Vec<u8>, shdr: &Vec<SectionHeader>) -> Vec<Section> {
    let mut sections = vec![]; 

    for hdr in shdr {
        sections.push( Section::from(&bin[hdr.offset as usize..(hdr.offset+hdr.size) as usize]))     
    }

    return sections; 
}