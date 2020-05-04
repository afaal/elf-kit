#![allow(warnings)]
use std::result::*;

fn main() {

    let bin = elf::read_file("test/testBin").expect("Failed with"); 
    
    println!("#phdrs: {}", bin.program_hdrs.len()); 
    println!("#shdrs: {}", bin.section_hdrs.len()); 
    
    for sh in &bin.section_hdrs {
        println!("{}",  sh.name); 
    }

    let dat = &bin.section_hdrs.last().unwrap().to_le();
    println!("{:x}", &bin.section_hdrs.last().unwrap().offset);
    println!("{:02x?}", dat); 
   
    // bin.write_file("./test/testBin.test"); 
}
