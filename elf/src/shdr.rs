use crate::Parseable; 
use crate::Result;
 
use byteorder::*; 


#[repr(u32)]
#[derive(Copy, Clone)]
enum Shdr_type {
    NULL = 0x0,
    PROGBITS = 0x1,
    SYMTAB = 0x2,
    STRTAB = 0x3,
    RELA = 0x4,
    HASH = 0x5,
    DYNAMIC = 0x6,
    NOTE = 0x7,
    NOBITS = 0x8,
    REL = 0x9,
    SHLIB = 0x0A,
    DYNSYM = 0x0B,
    INIT_ARRAY = 0x0E,
    FINI_ARRAY = 0x0F,
    PRE_INIT_ARRAY = 0x10,
    GROUP = 0x11,
    SYMTAB_SHNDX = 0x12,
    NUM = 0x13,
    LOOS = 0x60000000,
    GNU_VERDEF,
    GNU_VERNEED,
    GNU_VERSYM,
}
#[repr(u64)]
#[derive(Copy, Clone)]
enum Shdr_flags {
    NONE = 0x0,
    WRITE = 0x1,
    ALLOC = 0x2,
    EXECINSTR = 0x4,
    MERGE = 0x10,
    STRINGS = 0x20,
    INFO_LINK = 0x40,
    LINK_ORDER = 0x80,
    OS_NONCONFORMING = 0x100,
    GROUP = 0x200,
    TLS = 0x400,
    MASKOS = 0x0ff00000,
    MASKPROC = 0xf0000000,
    ORDERED = 0x4000000,
    EXCLUDE = 0x8000000
}
#[derive(Clone)]
pub struct SectionHeader {
    pub name: String,
    shstrndx_offset: u32,
    sh_type: Shdr_type,
    flags: Shdr_flags,
    addr: u64,
    pub offset: u64,
    pub size: u64,
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
        0x0 => return Shdr_flags::NONE,
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
        _ => return Shdr_flags::NONE
    }
}

impl SectionHeader{
  
    // Parse programheaders
    pub fn parse(shdr: &[u8], name: &str) -> Result< SectionHeader > {
        Ok(SectionHeader{
            name: String::from(name),
            shstrndx_offset: LittleEndian::read_u32(&shdr[0x0..0x4]),
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
    
    
    // print the section header as a LittleEndian formatted object
    // should this come with/or without padding??? 
    pub fn to_le(&self) -> Vec<u8> {
        // bin.append([1,2,3].to_vec())
        let mut bin = vec![]; 

        bin.extend_from_slice(&self.shstrndx_offset.to_le_bytes()); 
        
        // do i end up owning this data, thus preventing me from using sh_type elsewhere? 
        bin.extend_from_slice(&(self.sh_type as u32).to_le_bytes()); 
        bin.extend_from_slice(&(self.flags as u64).to_le_bytes()); 
        bin.extend_from_slice(&self.addr.to_le_bytes()); 
        bin.extend_from_slice(&self.offset.to_le_bytes()); 
        bin.extend_from_slice(&self.size.to_le_bytes()); 
        bin.extend_from_slice(&self.link.to_le_bytes()); 
        bin.extend_from_slice(&self.info.to_le_bytes()); 
        bin.extend_from_slice(&self.addralign.to_le_bytes()); 
        bin.extend_from_slice(&self.entsize.to_le_bytes()); 
        
        SectionHeader::add_padding(40, &mut bin);  
        
        return bin; 
    }

    fn add_padding(target_size: u32, bin: &mut Vec<u8>) {
        while bin.len() < 40 {
            bin.push(b'\0'); 
        } 
    }

}


pub fn parse_section_header(bin: &Vec<u8>, shstrndx: u16) -> Result<Vec<SectionHeader>> {
    let shdr_offset = LittleEndian::read_u64(&bin[0x28..0x30]); 
    let shdr_size = LittleEndian::read_u16(&bin[0x3A..0x3C]); 
    let shdr_num = LittleEndian::read_u16(&bin[0x3C..0x3E]); 
    let shstr_table_offset: usize = (shdr_offset + (shdr_size * shstrndx) as u64 ) as usize;
    let str_table_offset = LittleEndian::read_u64(&bin[shstr_table_offset+0x18..shstr_table_offset+0x20]) as usize;
    let str_table_size = LittleEndian::read_u64(&bin[shstr_table_offset+0x20..shstr_table_offset+0x28]) as usize; 

    let mut shdrs:Vec<SectionHeader> = vec![]; 

    // loop through all section headers
    for i in 0..shdr_num {
        let start = (shdr_offset+(shdr_size as u64*i as u64) ) as usize; 
        let end = (shdr_offset+(shdr_size as u64*i as u64)+shdr_size as u64 ) as usize; 
        let name_offset = str_table_offset + LittleEndian::read_u32(&bin[start..start+0x4]) as usize; 
        
        let name = str_from_u8_nul_utf8(&bin[name_offset..str_table_offset+str_table_size])?;
        let section = SectionHeader::parse(&bin[start..end], name)?; 
        // println!("{}", String::from_utf8_lossy(&bin[name_offset..name_offset+0x4])); 
        // add the section to the table of sections 
        shdrs.push(section); 
    }

    // loop through them again populating their names
    // we do this because we haven't mapped the shstrndx yet. 
    return Ok(shdrs); 
} 


pub fn str_from_u8_nul_utf8(utf8_src: &[u8]) -> Result<&str> {
    let nul_range_end = utf8_src.iter()
        .position(|&c| c == b'\0')
        .unwrap_or(utf8_src.len()); // default to length if no `\0` present
    return Ok(::std::str::from_utf8(&utf8_src[0..nul_range_end]).unwrap())
}