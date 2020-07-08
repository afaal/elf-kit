pub enum Block {
    Segment(crate::Segment),
    Section(crate::Section),
    RawDat(Vec<u8>),
    Padding(Vec<u8>)
}

impl Block {
    pub fn to_bin(self) -> Vec<u8> {
        match self {
            Block::Segment(t) => t.to_le(),
            Block::Section(t) => t.to_le(),
            Block::RawDat(t) | Block::Padding(t) => t
        }
    }
}