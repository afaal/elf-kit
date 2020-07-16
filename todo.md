This repo will undergo three stages of development. All described in the coming section.

# ELF.parser

ELF parser is the first stage in ELF.inject. It's goal is to tokenize an ELF and be able to dynamically put it back together. This will make the addition of new segments/sections/raw binary seamless.

- [ ] Are we padding the end of a segment with rawdat? Alignment isn't taken into account. 
- [x] Add sections to the end of the file 
- [ ] Rewrite program headers / elf header to reflect changes to locations of sections and segments
- [ ] Replace blocks where header and program header would be located with the updated structures. 
- [ ] Relocate entry point
- [ ] find_sections_narrowfit() only works with binaries having a sections table, a backup should be made

# ELF.patcher

This is a tool to modify existing binaries. The main purpose of this tool is trojanize binaries with payloads (other binaries). But the tool will also be able to menial tasks such as removing section headers from executables and program headers from libraries, as well as all non essential segments/sections (NOTE, COMMENT...)


# ELF.inject

This is a tool designed to modify processes.



# Resources 
1. https://en.wikipedia.org/wiki/Executable_and_Linkable_Format