use byteorder::*; 
use crate::Segment; 
use crate::Section; 
use crate::phdr; 
use crate::shdr; 
use std::cmp::Ordering;


#[derive(Clone)]
pub enum Block {
    Segment(crate::Segment),
    Section(crate::Section),
    RawDat(Vec<u8>),
    Padding(Vec<u8>),
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        
        if let (Block::Segment(s), Block::Segment(t)) = (self, other) {
            return s.phdr.offset == t.phdr.offset && s.phdr.filesz == s.phdr.filesz; 
        }

        return false; 
    }
}

impl PartialOrd for Block {
 
    // Sorting the sections/segment blocks vectors:  vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    fn partial_cmp(&self, blk: &Block) -> Option<Ordering> {
        
        let a = match self {
            Block::Segment(s) => s.phdr.offset,
            Block::Section(s) => s.hdr.offset,
            _ => return None
        };
        let b = match blk {
            Block::Segment(s) => s.phdr.offset,
            Block::Section(s) => s.hdr.offset,
            _ => return None
        };
        

        if a < b {
            return Some(Ordering::Less); 
        } else if a == b {
            return Some(Ordering::Equal); 
        } else {
            return Some(Ordering::Greater); 
        }

        None
    }
}


impl Block {
    pub fn to_bin(self) -> Vec<u8> {
        match self {
            Block::Segment(t) => t.to_le(),
            Block::Section(t) => t.to_le(),
            Block::RawDat(t) | Block::Padding(t) => t
        }
    }

    pub fn segment(&self) -> crate::Result<&crate::Segment> {
        match self {
            Block::Segment(s) => Ok(s),
            _ =>  Err(crate::ParsingError::ParsingError)
        }
    }

    pub fn raw_dat(&self) -> crate::Result<&Vec<u8>> {
        match self {
            Block::RawDat(s) => Ok(s),
            _ =>  Err(crate::ParsingError::ParsingError)
        }
    }

    pub fn raw_dat_mut(&mut self) -> crate::Result<&mut Vec<u8>> {
        match self {
            Block::RawDat(s) => Ok(s),
            _ =>  Err(crate::ParsingError::ParsingError)
        }
    }

    pub fn segment_mut(&mut self) -> crate::Result<&mut crate::Segment> {
        match self {
            Block::Segment(s) => Ok(s),
            _ =>  Err(crate::ParsingError::ParsingError)
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Block::Segment(s) => s.len(), 
            Block::Section(s) => s.len(),
            Block::RawDat(s) => s.len(),
            Block::Padding(s) => s.len(),
        }
    }

}

pub fn into_blocks(bin: Vec<u8>) -> crate::Result<Vec<Block>> {
    let shstrndx = LittleEndian::read_u16(&bin[0x3E..0x40]); 
    let program_hdrs = phdr::parse_program_header(&bin)?;
    let section_hdrs = shdr::parse_section_header(&bin, shstrndx)?; 
    
  
    let mut r_blocks:Vec<Block> = vec![]; 

    r_blocks = find_sections_narrowfit(&program_hdrs, &section_hdrs, &bin); 
    r_blocks = nest_segments(r_blocks, 0).iter().rev().cloned().collect(); 
    
    for r in &mut r_blocks {
        let mut seg = r.segment_mut().unwrap(); 

        fill_raw_data_block(seg, &bin, seg.phdr.offset as usize);
    }

    return Ok(r_blocks) 
}

// UNFINISHED
fn fill_raw_data_block(segment: &mut Segment, bin: &Vec<u8>, segment_offset: usize) -> crate::Result<()> {
    let mut new_blocks:Vec<Block> = vec![]; 
    let mut offset = 0; 

    if segment.blocks.len() == 0 && segment.phdr.filesz != 0 {
        new_blocks.push( Block::RawDat( bin[segment_offset as usize .. segment_offset+segment.phdr.filesz as usize].to_vec() ) );            
    }

    // fill in remaining raw data blocks 
    for block in &mut segment.blocks {
        let (start, end) = match &block {
            Block::Segment(s) => (s.phdr.offset, s.phdr.offset+s.phdr.filesz),
            Block::Section(s) => (s.hdr.offset, s.hdr.offset+s.hdr.size),
            _ => return Err(crate::ParsingError::ParsingError)
        }; 

        if offset < start {
            // fill the gap up to start we might need to substract one from start to not include the beginning of the coming segment
            // according to tests the end in not inclusive. 
            new_blocks.push( Block::RawDat( bin[segment_offset+offset as usize .. segment_offset+start as usize].to_vec() ) );            
        }
        
        if let Block::Segment(s) = block {
            fill_raw_data_block(s, &bin, segment_offset+start as usize); 
        }

        new_blocks.push(block.clone()); 
        offset = end
    }

    segment.blocks = new_blocks; 

    Ok(())
}

// nest segment and place segment in the correct location relative to other blocks.
fn nest_segments(mut blocks: Vec<Block>, mut idx: usize) -> Vec<Block> {
    if idx >= blocks.len() {return blocks}; 


    let mut itmb = blocks.remove(idx); 
    let mut itm = itmb.segment_mut().unwrap(); 

    let mut is_added = false; 
    // let itm = blocks.remove(idx).segment().unwrap(); 

    for block in &mut blocks {
        let mut seg = block.segment_mut().unwrap(); 

        if seg.contains(itm) {
            is_added = true; 
            itm.phdr.offset = itm.phdr.offset - seg.phdr.offset; // relative segment offsets 
            seg.blocks.push(itmb.clone());  // we have to have clone here because we readd elements in is_added using which takes ownership
            
            // TODO: SHOULD PROBABLY BE MOVED - WE WILL END UP REORDERING THE VECTORS ALOT! 
            seg.blocks.sort_by(|a, b| a.partial_cmp(b).unwrap()); 
            
            break; // can only be contained within one segment
        }
        
        
    }

    // if we dont' add the element to another increment the index, so that we don't try to add it again
    if !is_added { 
        blocks.splice(0..0, [itmb].iter().cloned()); 
        idx+=1; 
    }


    return nest_segments(blocks, idx);
}

fn find_sections(seg: &Segment, section_hdrs: &Vec<crate::shdr::SectionHeader>, bin: &Vec<u8> ) -> Vec<Block> {
    let mut blocks = vec![]; 
    
    // Break programheader into  section blocks and rawData blocks (if any)
    let c_start = seg.phdr.offset; 
    let c_end = (seg.phdr.offset+seg.phdr.filesz); 
    
    // Create block sections
    for shdr in section_hdrs {
        let s_start = shdr.offset;
        let s_end = (shdr.offset+shdr.size); 
        
        if let shdr::Shdr_type::NOBITS = shdr.sh_type {
            if s_start > c_start && s_start <= c_end {
                
                let mut new_shdr = shdr.clone(); 
                new_shdr.offset = s_start - c_start; 

                // There is an overflow here - both /usr/bin/ls and /usr/bin/xxd fails at this point this is due to the projected size of the allocation
                // and not the actual file size. It's the .bss section causing this error and it doens't take up any space in the file thus we actually just parse an empty array
                blocks.push(Block::Section(Section::from(new_shdr, &vec![])))           
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

    blocks
}

// this can only be done when there is a sections header table
fn find_sections_narrowfit(program_hdrs: &Vec<crate::phdr::ProgramHeader>, section_hdrs: &Vec<crate::shdr::SectionHeader>, bin: &Vec<u8> ) -> Vec<Block> {
    let mut blocks = init_segments(bin).expect("Failed to parse segments"); 
    
    for shdr in section_hdrs {
        let mut best_idx = 0xFFFFFFFF; 
        let mut best_width = bin.len() as u64; 
        let s_start = shdr.offset;
        let s_end = (shdr.offset+shdr.size); 
        
        for (idx, blk) in blocks.iter().enumerate() {

            let seg = blk.segment().unwrap(); 
            let c_start = seg.phdr.offset; 
            let c_end = (seg.phdr.offset+seg.phdr.filesz); 


            let p_width = seg.phdr.offset+seg.phdr.filesz; 

              
            if let shdr::Shdr_type::NOBITS = shdr.sh_type {
                if s_start > c_start && s_start <= c_end {
                    
                    if p_width < best_width {
                        best_idx=idx; 
                        best_width=p_width; 
                    }

                }  
                
            } else {
                if s_start >= c_start && s_start < c_end {
                         
                    if p_width < best_width {
                        best_idx=idx; 
                        best_width=p_width; 
                    }  
                } 
            }

        }
        // add section
        if best_idx == 0xFFFFFFFF {continue;}

        // the offset needs to be relative to the segment start
        let mut new_shdr = shdr.clone();
        new_shdr.offset = s_start - blocks[best_idx].segment().unwrap().phdr.offset; 

        blocks[best_idx].segment_mut().unwrap().blocks.push( Block::Section(Section::from(new_shdr.clone(), &bin[shdr.offset as usize..(shdr.offset+shdr.size) as usize].to_vec())) )
    }   

    blocks
}

fn init_segments(bin: &Vec<u8>) -> crate::Result<Vec<Block>> {
    let shstrndx = LittleEndian::read_u16(&bin[0x3E..0x40]); 
    let program_hdrs = phdr::parse_program_header(&bin)?;
    let section_hdrs = shdr::parse_section_header(&bin, shstrndx)?; 

    let mut r_blocks:Vec<Block> = vec![]; 

    // Loop through each programheader (segments)
    for phdr in program_hdrs {
        let mut seg = Segment {
            phdr,
            blocks: vec![]
        };

        // Break programheader into  section blocks and rawData blocks (if any)
        let c_start = &seg.phdr.offset; 
        let c_end = (&seg.phdr.offset+&seg.phdr.filesz); 

        r_blocks.push(Block::Segment(seg)); 
    }
    
    return Ok(r_blocks) 
}

pub fn generate_section_headers(blocks: &Vec<Block>, mut offset: usize) -> Vec<crate::shdr::SectionHeader> {
    let mut sections_headers = vec![]; 
    
    for blk in blocks {
        match blk {
            Block::Segment(s) => { 
                sections_headers.extend(generate_section_headers(&s.blocks, offset)); 
                offset += s.len(); 
                // calculate the size of the segment
            }, 
            Block::Section(s) => {
                let mut shdr = s.hdr.clone(); 
                shdr.offset = offset as u64;
                sections_headers.push(shdr); 
                offset += s.content.raw_dat().unwrap().len(); 
            },
            Block::RawDat(s) => {
                offset += s.len(); 
            }
            Block::Padding(s) => {
                offset += s.len(); 
            }
        }
    }

    return sections_headers; 
}

pub fn size(blocks: &Vec<Block>) -> usize {
    let mut len = 0;

    for blk in blocks {
        len += blk.size(); 
    }

    len
}

pub fn get_phdr_inner(blocks: &mut Vec<Block>) -> crate::Result<&mut Vec<u8>> {
    for blk in blocks {
        match blk {
            Block::Segment(s) => { 

                if let crate::phdr::Phdr_type::PHDR = s.phdr.p_type {
                    if s.blocks.len() > 0 {
                        return Ok(s.blocks[0].raw_dat_mut().unwrap()); 
                    } else {
                        return Err(crate::ParsingError::ParsingError); 
                    }
                }

                if let Ok(seg) = get_phdr_inner(&mut s.blocks) {
                    if s.blocks.len() > 0 {
                        return s.blocks[0].raw_dat_mut()
                    } else {
                        return Err(crate::ParsingError::ParsingError); 
                    }
                }

            }, 
            _ => {}
        }
    }

    Err(crate::ParsingError::Missing)
} 


pub fn get_elfhdr_inner(blocks: &mut Vec<Block>) -> crate::Result<&mut Vec<u8>> {
    let firstelem = blocks[0].segment_mut().unwrap(); 
    
    for blk in &mut firstelem.blocks {
        match blk {
            Block::RawDat(s) => {
                return Ok(s); 
            },
            _ => {}
        }
    }

    Err(crate::ParsingError::Missing)
} 
