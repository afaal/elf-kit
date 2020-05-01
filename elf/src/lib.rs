#![allow(warnings)]

use std::fs;
use std::error;
use std::fmt; 
use std::io::Cursor; 
use byteorder::*; 
use std::slice::SliceIndex; 

pub mod phdr; 
pub mod shrd; 

use phdr::ProgramHeader; 
use shrd::SectionHeader; 

trait Parseable<T> {  
    fn parse(bin: &Vec<u8>) -> Result<T>;
}

#[derive(Debug, Clone)]
pub enum ParsingError {
    NotElf
}

type Result<T> = std::result::Result<T, ParsingError>; 
// This is important for other errors to wrap this one.

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for ParsingError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
pub enum Elf_type {
    NONE,
    REL,
    EXEC,
    DYN,
    CORE,
    LOOS,
    HIOS,
    LOPROC,
    HIPROC
}

pub enum Elf_class {
    ELF64,
    ELF32
}

pub enum Elf_endiannes {
    LittleEndian, 
    BigEndian
}

pub enum Elf_arch {
    NONE,
    SPARC,
    X86,
    MIPS,
    POWERPC,
    S390,
    ARM,
    SUPERH,
    IA64,
    AMD64,
    AARCH64,
    RISCV
}

pub enum Elf_abi {
    NONE,
    HPUX,
    NetBSD,
    Linux,
    GNUHurd,
    Solaris,
    AIX,
    IRIX,
    FreeBSD,
    Tru64,
    NovellModesto,
    OpenBSD,
    OpenVMS,
    NonStopKernel,
    AROS,
    FenixOS,
    CloudABI,
    OpenVOS
}

pub struct Elf {
    e_ident: String,
    pub e_type: Elf_type,
    e_abi: Elf_abi,
    e_arch: Elf_arch,
    e_endianness: Elf_endiannes,
    e_version: Elf_class, 
    e_entry: u64,
    e_flags: u32, 
    pub program_hdrs: Vec<phdr::ProgramHeader>,
    pub section_hdrs: Vec<shrd::SectionHeader>,
    shstrndx: u16
}

impl Parseable<Elf> for Elf {
    fn parse(bin: &Vec<u8>) -> Result<Elf> {
        
        if !is_elf(&bin) {
            return Err(ParsingError::NotElf)
        }
 
        
        let shstrndx = LittleEndian::read_u16(&bin[0x3E..0x40]); 

        return Ok(Elf{
            e_ident:        String::from("ELF"),
            e_endianness:   parse_endianness(&bin),
            e_version:      parse_class(&bin),
            e_abi:          parse_abi(&bin),
            e_arch:         parse_arch(&bin),
            e_type:         parse_type(&bin),
            e_entry:        parse_entry64(&bin),
            e_flags:        0x100,
            program_hdrs:   parse_program_header(&bin)?,
            section_hdrs:   parse_section_header(&bin, shstrndx)?,
            shstrndx,
        });   
    }
}

impl Elf {
    pub fn write(path: &str) -> Result<()> {
        Ok(())
    }
}


fn is_elf(bin: &Vec<u8>) -> bool {
    if bin.len() < 4 {
        return false
    }
    
    if bin[0] == 0x7F && 
       bin[1] == 0x45 &&
       bin[2] == 0x4c &&
       bin[3] == 0x46 { return true }
        
    return false; 
}

fn parse_program_header(bin: &Vec<u8>) -> Result<Vec<ProgramHeader>> {
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

fn parse_entry64(bin: &Vec<u8>) -> u64 {
    return LittleEndian::read_u64(&bin[0x18..0x20])
}

fn parse_type(bin: &Vec<u8>) -> Elf_type {
    return match LittleEndian::read_u16(&bin[0x10..0x12]) {
        0x0 => return Elf_type::NONE,
        0x1 => return Elf_type::REL,
        0x2 => return Elf_type::EXEC,
        0x3 => return Elf_type::DYN,
        0x4 => return Elf_type::CORE,
        0xFE00 => return Elf_type::LOOS,
        0xFEFF => return Elf_type::HIOS,
        0xFF00 => return Elf_type::LOPROC,
        0xFFFF => return Elf_type::HIPROC,
        
        _ => return Elf_type::NONE,
    }
}

fn parse_section_header(bin: &Vec<u8>, shstrndx: u16) -> Result<Vec<SectionHeader>> {
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

fn parse_arch(bin: &Vec<u8>) -> Elf_arch {
    return match LittleEndian::read_u16(&bin[0x12..0x14]) {
        0x0 => return Elf_arch::NONE,
        0x2 => return Elf_arch::SPARC,
        0x3 => return Elf_arch::X86,
        0x8 => return Elf_arch::MIPS,
        0x14 => return Elf_arch::POWERPC,
        0x16 => return Elf_arch::S390,
        0x28 => return Elf_arch::ARM,
        0x2A => return Elf_arch::SUPERH,
        0x32 => return Elf_arch::IA64,
        0x3E => return Elf_arch::AMD64,
        0xB7 => return Elf_arch::AARCH64,
        0xF3 => return Elf_arch::RISCV,
        _ => return Elf_arch::NONE,
    }
}

fn parse_class(bin: &Vec<u8>) -> Elf_class {
    return match bin[4] {
        1 => return Elf_class::ELF32,
        _ => return Elf_class::ELF64
    }
}

fn parse_abi(bin: &Vec<u8>) -> Elf_abi {
    return match bin[7] {
        0x0 => return Elf_abi::NONE,
        0x1 => return Elf_abi::HPUX,
        0x2 => return Elf_abi::NetBSD,
        0x3 => return Elf_abi::Linux,
        0x4 => return Elf_abi::GNUHurd,
        0x6 => return Elf_abi::Solaris,
        0x7 => return Elf_abi::AIX,
        0x8 => return Elf_abi::IRIX,
        0x9 => return Elf_abi::FreeBSD,
        0x0A => return Elf_abi::Tru64,
        0x0B => return Elf_abi::NovellModesto,
        0x0C => return Elf_abi::OpenBSD,
        0x0D => return Elf_abi::OpenVMS,
        0x0E => return Elf_abi::NonStopKernel,
        0x0F => return Elf_abi::AROS,
        0x10 => return Elf_abi::FenixOS,
        0x11 => return Elf_abi::CloudABI,
        0x12 => return Elf_abi::OpenVOS,
        _ => return Elf_abi::NONE
    }
}

fn parse_endianness(bin: &Vec<u8>) -> Elf_endiannes {
    return match bin[5] {
        1 => return Elf_endiannes::LittleEndian,
        _ => return Elf_endiannes::BigEndian
    }
}

pub fn read_file(path: &str) -> Result<Elf> {
    let bin = fs::read(path).expect("Failed to read path"); 
    Elf::parse(&bin) 
}
