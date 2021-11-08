#![allow(warnings)]
use std::result::*;

fn main() {

    let mut elf = elf::from_file("test/testBin").expect("Failed to open file"); 

    // API 
    // elf.inject( ELF )
    // elf.strip()
    // elf.obfuscate()
    // elf.encrypt( key )
    // elf.pack(Packer::UPX)
    // elf.save( path )

}