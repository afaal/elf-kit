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

    pub fn segment_mut(&mut self) -> crate::Result<&mut crate::Segment> {
        match self {
            Block::Segment(s) => Ok(s),
            _ =>  Err(crate::ParsingError::ParsingError)
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

    // TODO: Should i nest segments first? The same section is represented in two different segments .interp due to segment nesting 
    // This is  probably needed otherwise we could end up having the same section appear in multiple segments. Is there a workaround? 
    
    // parsed = []
    let mut r_blocks:Vec<Block> = vec![]; 

    // // Loop through each programheader (segments)
    // for phdr in program_hdrs {
    //     let mut seg = Segment {
    //         phdr,
    //         blocks: vec![]
    //     };
        

    //     // Break programheader into  section blocks and rawData blocks (if any)
    //     let c_start = &seg.phdr.offset; 
    //     let c_end = (&seg.phdr.offset+&seg.phdr.filesz); 

    //     seg.blocks.extend(find_sections(&seg, &section_hdrs, &bin)); 
        
    //     // fill in remaining raw data blocks 
        
    //     // fill_raw_data_block(&mut blocks, phdr.filesz); 
    //     // if already parsed programheaders blocks contains this (check offsets)
    //         // inject this segment block into that (placement in segment blocks is offset dependent)
    //     // else if already parsed programheader blocks is child of this (check offsets)
    //         // injest that programheader block into this  (placement in segment blocks is offset dependent)
        
    //     // add to parsed
    //     r_blocks.push(Block::Segment(seg)); 
    // }
    r_blocks = find_sections_narrowfit(&program_hdrs, &section_hdrs, &bin); 
    r_blocks = nest_segments(r_blocks, 0).iter().rev().cloned().collect(); 
    // by the end we should end up with the root array only containing a few segments probably the load segments.

    // we need to store offsets for calculating placements. However these does not have to be used when generating the resulting binary.

    return Ok(r_blocks) 
}

// UNFINISHED
fn fill_raw_data_block(blocks: &mut Vec<Block>, size: u64) -> Vec<Block> {

    let mut ranges:Vec<(u64,u64)> = vec![(0, size)]; 
        // fill in remaining raw data blocks 
    for block in blocks {
        if let Block::Section(sec) = block {
            for (low, high) in &ranges {
                if sec.hdr.offset >= *low && sec.hdr.offset <= *high {
                    
                }
            }
        }
    }

    vec![]
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