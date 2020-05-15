use crate::phdr;
use crate::shdr;
use byteorder::*; 
use crate::phdr::ProgramHeader;
use crate::shdr::SectionHeader; 
use crate::Section;

pub struct Segment {
    // Either should be able not to be set
    pub phdr: ProgramHeader,
    pub raw_content: Vec<u8>,
    pub sections: Vec<Section> 
}

impl Segment {
    pub fn from(phdr: ProgramHeader, bin: &[u8], shdrs: &Vec<Section>, offset: usize) -> Segment {
        
        let sections  = vec![]; 

        return Segment{
            phdr,
            sections,
            raw_content: bin.to_vec()
        }
        
    }
}

// bin: The loaded binary file
pub fn parse_segments(bin: &Vec<u8>) -> crate::Result<Vec<Segment>> {
    let shstrndx = LittleEndian::read_u16(&bin[0x3E..0x40]); 
    let program_hdrs = phdr::parse_program_header(&bin)?;
    let section_hdrs = shdr::parse_section_header(bin, shstrndx); 
    let mut segments = vec![]; 
    
    // TODO: find embeded sections
    let sections:Vec<Section> = vec![]; 

    // use the program headers to parse the file 
    
    for hdr in program_hdrs {
    
        segments.push(
            Segment::from(
                hdr,
                &bin[hdr.offset as usize..(hdr.offset+hdr.filesz) as usize],
                &sections,
                0)
            )
    }

    return Ok(segments); 
}