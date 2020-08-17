use elf::Elf; 

// remove all section headers from an elf object
pub fn remove_shdrs(elf: &mut Elf) {
    // find start and end of section header table and remove
    // set elf header section related content to 0
    let shdrt_start = elf.header.shdr_offset; 
    let shdrt_end = shdrt_start + (elf.header.shdr_num*elf.header.shdr_size) as u64; 
    let shdrt_size = shdrt_end-shdrt_start; 

    // overwrite bin data with 0
    elf.raw.splice(shdrt_start as usize..shdrt_end as usize, vec![0x0; shdrt_size as usize] ); 

    // rewrite elf header
    elf.header.shdr_offset = 0x0; 
    elf.header.shdr_num = 0x0; 
    elf.header.shdr_size = 0x0; 
    elf.header.shstrndx = 0x0; 
}

pub fn remove_ne_phdrs(elf: &mut Elf) {
    elf.phdrs.retain(|phdr| {
        match phdr.p_type {
            elf::phdr::Phdr_type::NOTE | elf::phdr::Phdr_type::NULL => false,
            _ => true
        }
    });

    elf.header.phdr_num = elf.phdrs.len() as u16; 
}