use crate::Parseable; 
use crate::Result; 

use byteorder::*; 

enum Phdr_type {
    NULL,
    LOAD,
    DYNAMIC,
    INTERP,
    NOTE,
    SHLIB,
    PHDR,
    TLS,
    LOOS,
    HIOS,
    LOPROC,
    HIPROC
}

pub struct ProgramHeader {
    p_type: Phdr_type,
    flags: u32,
    offset: u64,
    vaddr: u64,
    paddr: u64,
    filesz: u64,
    memsz: u64,
    p_flags: u64,
    p_align: u64
}


impl ProgramHeader {}


impl ProgramHeader {
  
    // Parse programheaders
    pub fn parse(phdr: &[u8]) -> Result< ProgramHeader > {    
        Ok(ProgramHeader{
            p_type: parse_phdr_type(&phdr),
            flags: LittleEndian::read_u32(&phdr[0x04..0x08]),
            offset: LittleEndian::read_u64(&phdr[0x8..0x10]),
            vaddr: LittleEndian::read_u64(&phdr[0x8..0x10]),
            paddr: LittleEndian::read_u64(&phdr[0x8..0x10]),
            filesz: LittleEndian::read_u64(&phdr[0x8..0x10]),
            memsz: LittleEndian::read_u64(&phdr[0x8..0x10]),
            p_flags: 0,
            p_align: LittleEndian::read_u64(&phdr[0x8..0x10]),
        })
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