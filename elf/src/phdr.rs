use crate::Result; 

use byteorder::*; 

#[derive(Copy, Clone)]
pub enum Phdr_type {
    NULL = 0x0,
    LOAD = 0x1,
    DYNAMIC = 0x2,
    INTERP = 0x3,
    NOTE = 0x4,
    SHLIB = 0x5,
    PHDR = 0x6,
    TLS = 0x7,
    LOOS = 0x60000000,
    HIOS = 0x6FFFFFFF,
    LOPROC = 0x70000000,
    HIPROC = 0x7FFFFFFF,
    // GNU options missing here 
    // Currently we are dropping all foreign formats
    // This might not be optimal.
}

pub struct ProgramHeader {
    pub p_type: Phdr_type,
    flags: u32,
    pub offset: u64,
    vaddr: u64,
    paddr: u64,
    pub filesz: u64,
    memsz: u64,
    p_flags: u64,
    pub p_align: u64
}

impl ProgramHeader {
  
    // Parse programheaders
    pub fn parse(phdr: &[u8]) -> Result< ProgramHeader > {    
        Ok(ProgramHeader{
            p_type: parse_phdr_type(&phdr),
            flags: LittleEndian::read_u32(&phdr[0x04..0x08]),
            offset: LittleEndian::read_u64(&phdr[0x8..0x10]),
            vaddr: LittleEndian::read_u64(&phdr[0x10..0x18]),
            paddr: LittleEndian::read_u64(&phdr[0x18..0x20]),
            filesz: LittleEndian::read_u64(&phdr[0x20..0x28]),
            memsz: LittleEndian::read_u64(&phdr[0x28..0x30]),
            p_flags: 0,
            p_align: LittleEndian::read_u64(&phdr[0x30..0x38]),
        })
    } 

    pub fn to_le(&self) -> Vec<u8> {
        self.to_le_offset(0)
    }

    pub fn to_le_offset(&self, offset:usize) -> Vec<u8> {
        // bin.append([1,2,3].to_vec())
        let mut bin = vec![]; 
        
        // do i end up owning this data, thus preventing me from using sh_type elsewhere? 
        bin.extend_from_slice(&(self.p_type as u32).to_le_bytes()); 
        bin.extend_from_slice(&self.flags.to_le_bytes()); 
        bin.extend_from_slice(&(self.offset + offset as u64).to_le_bytes()); 
        bin.extend_from_slice(&self.vaddr.to_le_bytes()); 
        bin.extend_from_slice(&self.paddr.to_le_bytes()); 
        bin.extend_from_slice(&self.filesz.to_le_bytes()); 
        bin.extend_from_slice(&self.memsz.to_le_bytes()); 
        // bin.extend_from_slice(&self.p_flags.to_le_bytes()); used in 32-bit
        bin.extend_from_slice(&self.p_align.to_le_bytes()); 
        
        // ProgramHeader::add_padding(40, &mut bin);  
        
        return bin; 
    }

    fn add_padding(target_size: u32, bin: &mut Vec<u8>) {
        while bin.len() < 40 {
            bin.push(b'\0'); 
        } 
    }
}


fn parse_phdr_type(phdr: &[u8]) -> Phdr_type {
    return match LittleEndian::read_u32(&phdr[0x0..0x4]) {
        0x0 => return Phdr_type::NULL,
        0x1 => return Phdr_type::LOAD,
        0x2 => return Phdr_type::DYNAMIC,
        0x3 => return Phdr_type::INTERP,
        0x4 => return Phdr_type::NOTE,
        0x5 => return Phdr_type::SHLIB,
        0x6 => return Phdr_type::PHDR,
        0x7 => return Phdr_type::TLS,
        0x60000000 => return Phdr_type::LOOS,
        0x6FFFFFFF => return Phdr_type::HIOS,
        0x70000000 => return Phdr_type::LOPROC,
        0x7FFFFFFF => return Phdr_type::HIPROC,
        _ => Phdr_type::NULL
    }
}


pub fn parse_program_header(bin: &Vec<u8>) -> Result<Vec<ProgramHeader>> {
    let phdr_offset = LittleEndian::read_u64(&bin[0x20..0x28]); 
    let phdr_size = LittleEndian::read_u16(&bin[0x36..0x38]); 
    let phdr_num = LittleEndian::read_u16(&bin[0x38..0x3A]);
    
    let mut phdrs:Vec<ProgramHeader> = vec![]; 

    // loop through all programheaders
    for i in 0..phdr_num {
        let start = (phdr_offset+(phdr_size as u64*i as u64) ) as usize; 
        let end = (phdr_offset+(phdr_size as u64*i as u64)+phdr_size as u64 ) as usize; 
        phdrs.push(ProgramHeader::parse(&bin[start..end])?)
    }

    return Ok(phdrs);
}