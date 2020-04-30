use crate::Parseable; 
use crate::Result;
 
use byteorder::*; 


enum Shdr_type {
    NULL,
    PROGBITS,
    SYMTAB,
    STRTAB,
    RELA,
    HASH,
    DYNAMIC,
    NOTE,
    NOBITS,
    REL,
    SHLIB,
    DYNSYM,
    INIT_ARRAY,
    FINI_ARRAY,
    PRE_INIT_ARRAY,
    GROUP,
    SYMTAB_SHNDX,
    NUM,
    LOOS,
    GNU_VERDEF,
    GNU_VERNEED,
    GNU_VERSYM,
}

enum Shdr_flags {
    WRITE,
    ALLOC,
    EXECINSTR,
    MERGE,
    STRINGS,
    INFO_LINK,
    LINK_ORDER,
    OS_NONCONFORMING,
    GROUP,
    TLS,
    MASKOS,
    MASKPROC,
    ORDERED,
    EXCLUDE
}

pub struct SectionHeader {
    name: String,
    sh_type: Shdr_type,
    flags: Shdr_flags,
    addr: u64,
    offset: u64,
    size: u64,
    link: u32,
    info: u32,
    addralign: u64,
    entsize: u64,
}

fn parse_shdr_type(phdr: &[u8]) -> Shdr_type {
    return match LittleEndian::read_u32(&phdr[0x04..0x08]) {
        0x0 => return Shdr_type::NULL,
        0x1 => return Shdr_type::PROGBITS,
        0x2 => return Shdr_type::SYMTAB,
        0x3 => return Shdr_type::STRTAB,
        0x4 => return Shdr_type::RELA,
        0x5 => return Shdr_type::HASH,
        0x6 => return Shdr_type::DYNAMIC,
        0x7 => return Shdr_type::NOTE,
        0x8 => return Shdr_type::NOBITS,
        0x9 => return Shdr_type::REL,
        0xA => return Shdr_type::SHLIB,
        0xB => return Shdr_type::DYNSYM,
        0xE => return Shdr_type::INIT_ARRAY,
        0xF => return Shdr_type::FINI_ARRAY,
        0x10 => return Shdr_type::PRE_INIT_ARRAY,
        0x11 => return Shdr_type::GROUP,
        0x12 => return Shdr_type::SYMTAB_SHNDX,
        0x13 => return Shdr_type::NUM,
        0x60000000 => return Shdr_type::LOOS,
        // 0xXXXXX => return Shdr_type::GNU_VERDEF,
        // 0xXXXXX => return Shdr_type::GNU_VERNEED,
        // 0xXXXXX => return Shdr_type::GNU_VERSYM,
        _ => return Shdr_type::NULL
    }
}


fn parse_shdr_flags(phdr: &[u8]) -> Shdr_flags {
    return match LittleEndian::read_u32(&phdr[0x08..0x10]) {
        0x1 => return Shdr_flags::WRITE,
        0x2 => return Shdr_flags::ALLOC,
        0x4 => return Shdr_flags::EXECINSTR,
        0x10 => return Shdr_flags::MERGE,
        0x20 => return Shdr_flags::STRINGS,
        0x40 => return Shdr_flags::INFO_LINK,
        0x80 => return Shdr_flags::LINK_ORDER,
        0x100 => return Shdr_flags::OS_NONCONFORMING,
        0x200 => return Shdr_flags::GROUP,
        0x400 => return Shdr_flags::TLS,
        0x0ff00000 => return Shdr_flags::MASKOS,
        0xf0000000 => return Shdr_flags::MASKPROC,
        0x4000000 => return Shdr_flags::ORDERED,
        0x8000000 => return Shdr_flags::EXCLUDE,

        // TODO: Throw error here instead -> and return Result
        _ => return Shdr_flags::ALLOC
    }
}

impl SectionHeader{
  
    // Parse programheaders
    pub fn parse(shdr: &[u8]) -> Result< SectionHeader > {
        Ok(SectionHeader{
            name: String::from("hello"),
            sh_type: parse_shdr_type(&shdr),
            flags: parse_shdr_flags(&shdr),
            addr: LittleEndian::read_u64(&shdr[0x10..0x18]),
            offset: LittleEndian::read_u64(&shdr[0x18..0x20]),
            size: LittleEndian::read_u64(&shdr[0x20..0x28]),
            link: LittleEndian::read_u32(&shdr[0x28..0x2C]),
            info: LittleEndian::read_u32(&shdr[0x2C..0x30]),
            addralign: LittleEndian::read_u64(&shdr[0x30..0x38]),
            entsize: LittleEndian::read_u64(&shdr[0x38..0x40]),
        })
    } 

}