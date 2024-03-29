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

use segment::Segment;
use section::Section;

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

pub struct Elf_header {
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
    pub phdr_offset: u64,
    pub phdr_size: u16,
    pub phdr_num: u16,
    pub shdr_offset: u64,
    pub shdr_size: u16,
    pub shdr_num: u16,
    pub shstrndx: u16
}

impl Elf_header {

    pub fn to_le(self) -> Vec<u8> {
        let mut bin = vec![]; 

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

        return bin; 
    }

}


pub struct Elf {
    pub header: Elf_header,    // pub program_hdrs: Vec<phdr::ProgramHeader>,
    pub phdrs: Vec<phdr::ProgramHeader>,
    pub shdrs: Vec<shdr::SectionHeader>,
    pub raw: Vec<u8>
    // pub segments: Vec<Segment>,
    // pub section_hdrs: Vec<shdr::SectionHeader>,
}

impl Elf {
    // return the elf as a binary file
    pub fn to_le(mut self) -> Vec<u8> {
        // let mut bin = vec![];

        // // bin.resize(segment::get_segments_size(&self.segments) as usize, 0);

        
        // // We need to create a new shstrndx using the segments 


        // // get segment blob
        // // TODO: We need to take nested segments into account.
        // let segment_blob = segment::get_segments_blob(&self.segments);  
        
        // // TODO: calculate the elf header size, program header and section headers.
        // let ehdr_offset = 0x0; 
        // let phdr_offset = 0x40; 
        // let segment_offset = phdr_offset+segment::phdrs_size(&self.segments);
        // let shdr_offset = segment_offset+segment_blob.len(); 

        // // TODO: Set the offsets to be file offsets instead of local offsets
        // self.header.phdr_offset = phdr_offset as u64; 
        // self.header.phdr_num = self.segments.len() as u16; 
        
        // self.header.shdr_offset = shdr_offset as u64; 
        // self.header.shdr_num = segment::shdrs_len(&self.segments) as u16; 


        // // - we need to have dynamic program header offsets as well -- implemented?

        
        
        // // alterations to the elf_headers offsets of section headers and program headers should be made before getting the blob 
        // // - change phdrs offset -- implemented
        // // - change shdrs offset -- implemented
        // let phdrs_blob = segment::get_phdrs_blob(&self.segments, segment_offset);         
        // let shdrs_blob = segment::get_shdrs_blob(&self.segments);         
        // let ehdr_blob = self.header.to_le(); 

        

        // bin.extend(ehdr_blob); 
        // bin.extend(phdrs_blob); 
        // bin.extend(segment_blob); 
        // bin.extend(shdrs_blob); 
        let phdrt_start = self.header.phdr_offset; 
        let phdrt_end = phdrt_start + (self.header.phdr_num*self.header.phdr_size) as u64; 
        let phdrt_size = phdrt_end-phdrt_start; 
    
        // add elf header
        self.raw.splice(0..0x40, self.header.to_le()); 

        // add program headers 
        self.raw.splice(phdrt_start as usize..phdrt_end as usize, phdr::to_le(self.phdrs) ); 

        return self.raw;
    }

    pub fn write_file(self, path: &str) -> Result<()> {
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

impl Elf {
    fn parse(bin: Vec<u8>) -> Result<Elf> {
        let shstrndx = LittleEndian::read_u16(&bin[0x3E..0x40]); 
        let program_hdrs = phdr::parse_program_header(&bin)?;
        let section_hdrs = shdr::parse_section_header(&bin, shstrndx)?; 

        return Ok(Elf {
            header: Elf_header::parse(&bin)?,
            phdrs:program_hdrs,
            shdrs: section_hdrs,
            raw: bin
        })
    }
}



impl Elf_header {
    fn parse(bin: &Vec<u8>) -> Result<Elf_header> {
        
        if !is_elf(&bin) {
            return Err(ParsingError::NotElf)
        }

        // TODO: ADD lengths checks to ensure it is an ELF of prober length
        let e_ident = [0x7F, 0x45, 0x4C, 0x46];
        let e_endianness = parse_endianness(&bin);
        let e_class = parse_class(&bin);
        let ei_version = bin[0x06];
        let e_abi_version = bin[0x08];
        let e_padding = [bin[0x9],bin[0xA],bin[0xB],bin[0xC],bin[0xD],bin[0xE],bin[0xF]];
        let e_abi = parse_abi(&bin);
        let e_version = LittleEndian::read_u32(&bin[0x14..0x18]);
        let e_arch = parse_arch(&bin);
        let e_type = parse_type(&bin);
        let e_entry = parse_entry64(&bin);
        let shstrndx = LittleEndian::read_u16(&bin[0x3E..0x40]); 
        let e_flags = LittleEndian::read_u32(&bin[0x30..0x34]);
        let size = LittleEndian::read_u16(&bin[0x34..0x36]);
        let phdr_offset = LittleEndian::read_u64(&bin[0x20..0x28]);
        let phdr_size = LittleEndian::read_u16(&bin[0x36..0x38]);
        let phdr_num = LittleEndian::read_u16(&bin[0x38..0x3A]);
        let shdr_offset = LittleEndian::read_u64(&bin[0x28..0x30]);
        let shdr_size = LittleEndian::read_u16(&bin[0x3A..0x3C]);
        let shdr_num = LittleEndian::read_u16(&bin[0x3C..0x3E]);
        let section_hdrs = shdr::parse_section_header(&bin, shstrndx)?;
        // let sections = section::parse_sections(bin,&section_hdrs); 
        
        return Ok(Elf_header{
            e_ident,
            e_endianness,
            e_class,
            ei_version,
            e_abi_version,
            e_padding,
            e_abi,
            e_version,
            e_arch,
            e_type,
            e_entry,
            e_flags,
            size,
            phdr_offset,
            phdr_size,
            phdr_num,
            shdr_offset,
            shdr_size,
            shdr_num,
            shstrndx
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

pub fn from_file(path: &str) -> Result<Elf> {
    let bin = fs::read(path).expect("Failed to read path"); 
    Elf::parse(bin) 
}