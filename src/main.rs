#![allow(warnings)]
use std::result::*;

fn main() {

    let bin = elf::read_file("test/testBin").expect("Failed with"); 
    
    
    bin.write_file("test/testBin.test");
}