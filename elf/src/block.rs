use byteorder::*; 
use crate::Segment; 
use crate::Section; 
use crate::phdr; 
use crate::shdr; 


pub enum Block {
    Segment(crate::Segment),
    Section(crate::Section),
    RawDat(Vec<u8>),
    Padding(Vec<u8>),
}

impl Block {
    pub fn to_bin(self) -> Vec<u8> {
        match self {
            Block::Segment(t) => t.to_le(),
            Block::Section(t) => t.to_le(),
            Block::RawDat(t) | Block::Padding(t) => t
        }
    }
}

pub fn into_blocks(bin: Vec<u8>) -> crate::Result<Vec<Block>> {
    let shstrndx = LittleEndian::read_u16(&bin[0x3E..0x40]); 
    let program_hdrs = phdr::parse_program_header(&bin)?;
    let section_hdrs = shdr::parse_section_header(&bin, shstrndx)?; 
    
    // TODO: fix offset calculation. We are currently adding segments after eachother, but some might be nested within others. 
    // ALso there is really no need for this calculation. As we simply need to determine where the segments begin, and then 
    // subtract that from their current offset, then we get an offset relative to the beginning of the segments blob.
    
    // TODO: exec and libs differently
    
    // [Injest/inject]
    
    
    // parsed = []
    let mut r_blocks:Vec<Block> = vec![]; 

    // Loop through each programheader (segments)
    for phdr in program_hdrs {
        let mut blocks = vec![]; 
        
        // Break programheader into  section blocks and rawData blocks (if any)
        let c_start = phdr.offset; 
        let c_end = (phdr.offset+phdr.filesz); 
        

        // Create block sections
        for shdr in &section_hdrs {
            let s_start = shdr.offset;
            let s_end = (shdr.offset+shdr.size); 

            if let shdr::Shdr_type::NOBITS = shdr.sh_type {
                if s_start > c_start && s_start <= c_end {
                    
                    let mut new_shdr = shdr.clone(); 
                    new_shdr.offset = s_start - c_start; 

                    blocks.push(Block::Section(Section::from(new_shdr, &bin[s_start as usize..s_end as usize].to_vec())))           
                
                }  
            } else {
                if s_start >= c_start && s_start < c_end {
                
                    // the offset needs to be relative to the segment start
                    let mut new_shdr = shdr.clone();
                    new_shdr.offset = s_start - c_start; 

                    blocks.push(Block::Section(Section::from(new_shdr, &bin[s_start as usize..s_end as usize].to_vec())))       
                } 
            }
        }
        
        // fill in remaining raw data blocks 


        



        // blocks: blockify(&bin[phdr.offset as usize..(phdr.offset+phdr.filesz) as usize]), 



        let seg = Segment {
            phdr,
            blocks
        }; 

        r_blocks.push(Block::Segment(seg)); 
        // if already parsed programheaders blocks contains this (check offsets)
            // inject this segment block into that (placement in segment blocks is offset dependent)
        // else if already parsed programheader blocks is child of this (check offsets)
            // injest that programheader block into this  (placement in segment blocks is offset dependent)
    
        // add to parsed

    }    

    // by the end we should end up with the root array only containing a few segments probably the load segments.

    // we need to store offsets for calculating placements. However these does not have to be used when generating the resulting binary.
    

    return Ok(r_blocks) 
}
