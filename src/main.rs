#![allow(warnings)]
use std::result::*;

fn main() {

    let mut bin = elf::read_file("test/testBin").expect("Failed to open file"); 
    
    elf_editor::remove_shdrs(&mut bin); 
    elf_editor::remove_ne_phdrs(&mut bin); 

    
    bin.write_file("test/testBin.new"); 
}