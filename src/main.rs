#![allow(warnings)]
use std::result::*;

fn main() {

    let bin = elf::read_file("test/testBin").expect("Failed with"); 
    
    println!("#phdrs: {}", bin.program_hdrs.len()); 
    println!("#shdrs: {}", bin.section_hdrs.len()); 

    // println!("{:02x?}", &bin.to_le()); 

    bin.write_file("./test/testBin.test"); 
}
