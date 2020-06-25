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
    pub shdrs: Vec<SectionHeader>,
    // store nested segments -- ideally we would have all load segments at the
    // top level, and their children nested
}

impl Segment {
    pub fn from(bin: Vec<u8>,phdr: ProgramHeader, shdrs: Vec<SectionHeader>) -> Segment {

        return Segment{
            phdr,
            shdrs,
            raw_content: bin,
        }
        
    }

    pub fn offset(&mut self, offset: usize) {
        self.phdr.offset = offset as u64; 
    }
}

// bin: The loaded binary file
pub fn parse_segments(bin: Vec<u8>) -> crate::Result< Vec<Segment> > {
    let shstrndx = LittleEndian::read_u16(&bin[0x3E..0x40]); 
    let program_hdrs = phdr::parse_program_header(&bin)?;
    let section_hdrs = shdr::parse_section_header(&bin, shstrndx)?; 
    let mut segments = vec![]; 
    
    // use the program headers to parse the file 
    
    for mut phdr in program_hdrs {
        let mut shdrs:Vec<SectionHeader> = vec![]; 
        let mut raw_content = bin[phdr.offset as usize..(&phdr.offset+&phdr.filesz) as usize].to_vec(); 

        let mut offset:usize = 0; 

        // We need to include the end, but exclude the beginning?
        for shdr in &section_hdrs {
            match shdr.sh_type {
                
                // We are not finding the bss section. This is due to the bss
                // section being loaded on program init, and thus being placed
                // at the absolute end of segment 5 (load) thus failing the if
                // statement because it overflows the file size As a result we
                // make a check for specifically NOBITS sections and have them
                // include the end aswell 
        
                // TODO: Refactor this into an if let

                shdr::Shdr_type::NOBITS => {
                    if shdr.offset > phdr.offset && shdr.offset <= phdr.offset+phdr.filesz {
                        // the offset needs to be relative to the segment start
                        let mut t_shdr = shdr.clone();
                        t_shdr.offset = t_shdr.offset - phdr.offset; 

                        shdrs.push(t_shdr); 
                    }                                  
                },
                _ => {
                    if shdr.offset >= phdr.offset && shdr.offset < phdr.offset+phdr.filesz {
                        // the offset needs to be relative to the segment start
                        let mut t_shdr = shdr.clone();
                        t_shdr.offset = t_shdr.offset - phdr.offset; 

                        shdrs.push(t_shdr); 
                    } 
                }
            }          
            // The section is a part of a section if it's offset is between the segments offset and filez 
            
        }


        // TODO: pad the segment according to allignment 
        // TODO: make the phdr offset relative 
        phdr.offset = offset as u64; 

        pad(&mut raw_content, &phdr); 

        offset += raw_content.len(); 


        segments.push(
            Segment::from(
                raw_content, //  i the sections to reference the raw content, so that if either are changed it's all changed 
                phdr,
                shdrs)
        ); 

    }

    return Ok(segments); 
}

fn pad(buf: &mut Vec<u8>, phdr: &phdr::ProgramHeader) {
    let missing_bytes = phdr.p_align - (buf.len() as u64 % phdr.p_align); 
    buf.extend_from_slice(&vec![0; missing_bytes as usize]);
}


pub fn get_segments_size(segments: &Vec<Segment>) -> u64 {
    let mut t = 0; 
    
    for seg in segments {
        t += seg.phdr.filesz + (seg.phdr.p_align - (seg.phdr.filesz % seg.phdr.p_align))
    }
    return t;
}

pub fn get_segments_blob(segments: &Vec<Segment>) -> Vec<u8> {
    let mut blob = vec![]; 
    
    for segment in segments {
        // we want to make sure everything is properly aligned at this point.
        blob.extend_from_slice(&segment.raw_content); 
    }

    return blob; 
}

pub fn get_phdrs_blob(segments: &Vec<Segment>) -> Vec<u8> {
    let mut blob = vec![]; 
    
    for segment in segments {
        blob.extend_from_slice(&segment.phdr.to_le()); 
    }

    return blob; 
}

pub fn get_shdrs_blob(segments: &Vec<Segment>) -> Vec<u8> {
    let mut blob = vec![]; 
    
    // TODO: We need to construct the strings table.

    // We need to update the section offsets to global offsets and not local ones

    for segment in segments {
        for shdr in &segment.shdrs {
            blob.extend_from_slice(&shdr.to_le_offset(segment.phdr.offset as usize)); 
        }
    }

    return blob; 
}

pub fn phdrs_size(segments: &Vec<Segment>) -> usize {
    // 32-bit = 0x20
    // 64-bit = 0x38
    return 0x38*segments.len(); 
}

pub fn shdrs_size(segments: &Vec<Segment>) -> usize {
    // 32-bit = 0x28
    // 64-bit = 0x40
    let mut size = 0;  
    
    // TODO: We need to construct the strings table.

    for segment in segments {
        for shdr in &segment.shdrs {
            size += 0x40; 
        }
    }

    return size; 
}

pub fn shdrs_len(segments: &Vec<Segment>) -> usize {
        // 32-bit = 0x28
    // 64-bit = 0x40
    let mut size = 0;  
    
    // TODO: We need to construct the strings table.

    for segment in segments {
        for shdr in &segment.shdrs {
            size += 1; 
        }
    }

    return size; 
}