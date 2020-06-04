#![allow(warnings)]
use std::result::*;

fn main() {

    let bin = elf::read_file("test/testBin").expect("Failed with"); 

    for (indx, s) in bin.segments.into_iter().enumerate() {
        println!("======== {} ======== {:X}-{:X}", indx, s.phdr.offset, s.phdr.offset+s.phdr.filesz);
        for shd in s.shdrs {
            println!("{} == {:X}-{:X}", shd.name, shd.offset, shd.offset+shd.size); 
        }
    }
    // bin.write_file("./test/testBin.test"); 
}