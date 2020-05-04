#![allow(warnings)]
use std::result::*;

fn main() {

    let bin = elf::read_file("test/testBin").expect("Failed with"); 
    
    println!("#phdrs: {}", bin.program_hdrs.len()); 
    println!("#shdrs: {}", bin.section_hdrs.len()); 

    let dat = &bin.program_hdrs.first().unwrap().to_le();
    println!("{:x}", &bin.program_hdrs.first().unwrap().offset);
    println!("{:02x?}", dat); 
   
    // bin.write_file("./test/testBin.test"); 
}
