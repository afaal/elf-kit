pub struct Segment {
    pub content: Vec<u8>
}

impl Segment {
    pub fn from(bin: &[u8]) -> Segment {
        return Segment{
            content: bin.to_vec()
        }
    }
}