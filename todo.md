## IN 
* How are symbols located in a shared libary? Uses section .dynsym?

* Are some sections needed or can the entire section header table be stripped

* How do we reassemble the binary segments (nested segments into a resulting binary blob in the form of loads)

* How do the sections know where the data they are pointing to is located? 

* What is the order in which we should assemble the binary parts (elf header, section header, program header, segments and their content)? Segment -> section -> headers(elf, program, section)?


## NEXT ACTION 



## MILE STONES 

1. ELF parser
- [ ] Output segments 
  We need to group segments based on their permissions into the respective load segments 
  The load segments are already loaded into memory with their contents, so if the segment is 
  already paired with that (the contents is already loaded into the segment)
- [ ] Handle / parse segments
- [ ] Parse Dynamic libraries / imports / types 

1. Code analyser 
2. Binary patcher 
3. Memory patcher
4. Injector


## READING LIST 


## WAITING 