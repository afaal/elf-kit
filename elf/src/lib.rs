#![allow(warnings)]

use std::fs;
use std::error;
use std::fmt; 
use std::io::Cursor; 
use byteorder::*; 
use std::slice::SliceIndex; 
use std::convert::TryInto; 

pub mod phdr; 
pub mod shdr; 
pub mod segment; 
pub mod section; 

use phdr::ProgramHeader; 
use shdr::SectionHeader; 
use segment::Segment;
use section::Section;

trait Parseable<T> {  
    fn parse(bin: &Vec<u8>) -> Result<T>;
}

#[derive(Debug, Clone)]
pub enum ParsingError {
    NotElf,
    ParsingError
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
#[derive(Copy, Clone)]
pub enum Elf_type {
    NONE = 0x0,
    REL = 0x1,
    EXEC = 0x2,
    DYN = 0x3,
    CORE = 0x4,
    LOOS = 0xfe00,
    HIOS = 0xfeff,
    LOPROC = 0xff00,
    HIPROC = 0xffff
}

#[derive(Copy, Clone)]
pub enum Elf_class {
    ELF32 = 1,
    ELF64 = 2
}

#[derive(Copy, Clone)]
pub enum Elf_endiannes {
    LittleEndian = 1, 
    BigEndian = 2
}

#[derive(Copy, Clone)]
pub enum Elf_arch {
    NONE    = 0x0,
    SPARC   = 0x2,
    X86     = 0x3,
    MIPS    = 0x8,
    POWERPC = 0x14,
    S390    = 0x16,
    ARM     = 0x28,
    SUPERH   = 0x2A,
    IA64     = 0x32,
    AMD64    = 0x3E,
    AARCH64  = 0xB7,
    RISCV    = 0xF3
}

#[derive(Copy, Clone)]
pub enum Elf_abi {
    NONE    = 0x0,
    HPUX    = 0x1,
    NetBSD  = 0x2,
    Linux   = 0x3,
    GNUHurd = 0x4,
    Solaris = 0x6,
    AIX     = 0x7,
    IRIX    = 0x8,
    FreeBSD = 0x9,
    Tru64   = 0xA,
    NovellModesto = 0xB,
    OpenBSD = 0xC,
    OpenVMS = 0xD,
    NonStopKernel = 0xE,
    AROS    = 0xF,
    FenixOS = 0x10,
    CloudABI = 0x11,
    OpenVOS = 0x12
}

pub struct Elf {
    e_ident: [u8;4],
    e_class: Elf_class, 
    e_endianness: Elf_endiannes,
    ei_version: u8, 
    e_abi: Elf_abi,
    e_abi_version: u8,
    e_padding: [u8;7],
    pub e_type: Elf_type,
    e_arch: Elf_arch,
    e_version: u32,
    e_entry: u64,
    e_flags: u32, 
    size: u16,
    phdr_offset: u64,
    phdr_size: u16,
    phdr_num: u16,
    shdr_offset: u64,
    shdr_size: u16,
    shdr_num: u16,
    pub program_hdrs: Vec<phdr::ProgramHeader>,
    pub section_hdrs: Vec<shdr::SectionHeader>,
    segments: Vec<Segment>,
    sections: Vec<Section>,
    shstrndx: u16
}

impl Elf {
    // return the elf as a binary file
    pub fn to_le(&self) -> Vec<u8> {
        // bin.append([1,2,3].to_vec())
        let mut bin = vec![]; 

        // do i end up owning this data, thus preventing me from using sh_type elsewhere? 
        // bin.extend_from_slice(&(self.p_type as u32).to_le_bytes()); 
        // ASSEMBLE THE ELF HEADER
        bin.extend_from_slice(&self.e_ident);  
        bin.extend_from_slice(&(self.e_class as u8).to_le_bytes()); 
        bin.extend_from_slice(&(self.e_endianness as u8).to_le_bytes()); 
        bin.extend_from_slice(&self.ei_version.to_le_bytes()); 
        bin.extend_from_slice(&(self.e_abi as u8).to_le_bytes()); 
        bin.extend_from_slice(&[self.e_abi_version]);
        bin.extend_from_slice(&self.e_padding);
        bin.extend_from_slice(&(self.e_type as u16).to_le_bytes()); 
        bin.extend_from_slice(&(self.e_arch as u16).to_le_bytes()); 
        bin.extend_from_slice(&self.e_version.to_le_bytes()); 
        bin.extend_from_slice(&self.e_entry.to_le_bytes()); 
        bin.extend_from_slice(&self.phdr_offset.to_le_bytes()); 
        bin.extend_from_slice(&self.shdr_offset.to_le_bytes()); 
        bin.extend_from_slice(&self.e_flags.to_le_bytes()); 
        bin.extend_from_slice(&self.size.to_le_bytes()); 
        bin.extend_from_slice(&self.phdr_size.to_le_bytes()); 
        bin.extend_from_slice(&self.phdr_num.to_le_bytes()); 
        bin.extend_from_slice(&self.shdr_size.to_le_bytes()); 
        bin.extend_from_slice(&self.shdr_num.to_le_bytes()); 
        bin.extend_from_slice(&self.shstrndx.to_le_bytes()); 

        // TODO: All ofsets should be calculated dynamically when recreating the
        // binary this is to accomodate changes made after parsing.
        // Offsets to change: 
        //  - ELF header (phdr, shdr, strhrdndx)
        //  - Program Header (offset, vaddr, paddr, filesz, memsz, p_align)
        //  - Section Header (shstrndx, addr, addralign, offset, size)

        // TODO: ADD necesarry padding to program header offset.
        
        // Add program headers at the program offset    
        for phdr in &self.program_hdrs {
            println!("[segment] offset: {:x} | size: {:x} ", phdr.offset, phdr.filesz); 

            bin.extend(phdr.to_le()); 
        }

        // TODO: ADD necesarry padding

        // <<< Binary SEGMENTS GOES HERE 
        for segment in &self.segments {
            println!("[adding segment] offset: {:x} | size: {:x} ", bin.len(), segment.content.len()); 
            bin.extend(&segment.content); 
        }

        // TODO: ADD necesarry padding

        // Add section headers
        for shdr in &self.section_hdrs {

            bin.extend(shdr.to_le()); 
        }
        
        return bin;
    }

    pub fn write_file(&self, path: &str) -> Result<()> {
        let bin = self.to_le(); 
        
        match fs::write(path, bin) {
            Ok(res) => Ok(res),
            Err(_) => return Err(ParsingError::ParsingError)
        }
    }    
}

fn pad(size: u32) -> Vec<u8> {
    return vec![0; size as usize]; 
}

fn parse_segments(bin: &Vec<u8>, phdr: &Vec<ProgramHeader>) -> Vec<Segment> {
    let mut segments = vec![]; 

    for hdr in phdr {
        segments.push( Segment::from(&bin[hdr.offset as usize..(hdr.offset+hdr.filesz) as usize]))     
    }

    return segments; 
}

fn parse_sections(bin: &Vec<u8>, shdr: &Vec<SectionHeader>) -> Vec<Section> {
    let mut sections = vec![]; 

    for hdr in shdr {
        sections.push( Section::from(&bin[hdr.offset as usize..(hdr.offset+hdr.size) as usize]))     
    }

    return sections; 
}

impl Parseable<Elf> for Elf {
    fn parse(bin: &Vec<u8>) -> Result<Elf> {
        
        if !is_elf(&bin) {
            return Err(ParsingError::NotElf)
        }

        // TODO: ADD lengths checks to ensure it is an ELF of prober length
        
        let shstrndx = LittleEndian::read_u16(&bin[0x3E..0x40]); 
        let e_flags = LittleEndian::read_u32(&bin[0x30..0x34]);
        let size = LittleEndian::read_u16(&bin[0x34..0x36]);
        let phdr_offset = LittleEndian::read_u64(&bin[0x20..0x28]);
        let phdr_size = LittleEndian::read_u16(&bin[0x36..0x38]);
        let phdr_num = LittleEndian::read_u16(&bin[0x38..0x3A]);
        let shdr_offset = LittleEndian::read_u64(&bin[0x28..0x30]);
        let shdr_size = LittleEndian::read_u16(&bin[0x3A..0x3C]);
        let shdr_num = LittleEndian::read_u16(&bin[0x3C..0x3E]);
        let program_hdrs = parse_program_header(&bin)?;
        let section_hdrs = parse_section_header(&bin, shstrndx)?;
        let segments = parse_segments(bin, &program_hdrs); 
        let sections = parse_sections(bin,&section_hdrs); 
        
        return Ok(Elf{
            e_ident:        [0x7F, 0x45, 0x4C, 0x46],
            e_endianness:   parse_endianness(&bin),
            e_class:        parse_class(&bin),
            ei_version:     bin[0x06],
            e_abi_version:  bin[0x08],
            e_padding:      [bin[0x9],bin[0xA],bin[0xB],bin[0xC],bin[0xD],bin[0xE],bin[0xF]],
            e_abi:          parse_abi(&bin),
            e_version:      LittleEndian::read_u32(&bin[0x14..0x18]),
            e_arch:         parse_arch(&bin),
            e_type:         parse_type(&bin),
            e_entry:        parse_entry64(&bin),
            e_flags,
            size,
            phdr_offset,
            phdr_size,
            phdr_num,
            shdr_offset,
            shdr_size,
            shdr_num,
            program_hdrs,
            section_hdrs,
            shstrndx,
            segments,
            sections
            // Add sections to           
        });   
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
