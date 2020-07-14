#![allow(warnings)]
use std::result::*;

fn main() {

    let elf_obj = elf::read_file("/usr/bin/ls").expect("Failed with"); 
    
    elf_obj.to_bin(); 

    // bin.write_file("test/testBin.test");
}