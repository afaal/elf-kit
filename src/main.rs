#![allow(warnings)]
use std::result::*;

fn main() {

    let bin = elf::read_file("test/testBin").expect("Failed with"); 

    for (indx, s) in bin.segments.into_iter().enumerate() {
        println!("======== {} ========", indx);
        for shd in s.shdrs {
            println!("{}", shd.name); 
        }
    }
    // bin.write_file("./test/testBin.test"); 
}
