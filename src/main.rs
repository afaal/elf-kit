#![allow(warnings)]
use std::result::*;

fn main() {

    let elf_obj = elf::read_file("./test/testBin").expect("Failed with"); 
    
    elf_obj.to_bin(); 
    // elf_obj.write_file("test/testBin.test");
}