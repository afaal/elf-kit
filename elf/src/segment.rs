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
    pub shdrs: Vec<SectionHeader> 
}

impl Segment {
    pub fn from(bin: Vec<u8>,phdr: ProgramHeader, shdrs: Vec<SectionHeader>, offset: usize) -> Segment {

        return Segment{
            phdr,
            shdrs,
            raw_content: bin
        }
        
    }
}

// bin: The loaded binary file
pub fn parse_segments(bin: Vec<u8>) -> crate::Result< Vec<Segment> > {
    let shstrndx = LittleEndian::read_u16(&bin[0x3E..0x40]); 
    let program_hdrs = phdr::parse_program_header(&bin)?;
    let section_hdrs = shdr::parse_section_header(&bin, shstrndx)?; 
    let mut segments = vec![]; 
    
    // TODO: find embeded sections
    
    
    // use the program headers to parse the file 
    
    for phdr in program_hdrs {
        let mut shdrs:Vec<SectionHeader> = vec![]; 
        let mut raw_content = bin[phdr.offset as usize..(&phdr.offset+&phdr.filesz) as usize].to_vec(); 
        
        for shdr in &section_hdrs {
            
            // The section is a part of a section if it's offset is between the segments offset and filez 
            if shdr.offset >= phdr.offset && shdr.offset <= phdr.offset+phdr.filesz {
                // the offset needs to be relative to the segment start
                shdrs.push(shdr.clone()); 
            } 

        }

        segments.push(
            Segment::from(
                raw_content, //  i the sections to reference the raw content, so that if either are changed it's all changed 
                phdr,
                shdrs,
                0)
            )
    }

    return Ok(segments); 
}