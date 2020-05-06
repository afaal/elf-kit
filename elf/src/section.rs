pub struct Section {
    pub content: Vec<u8>
}

impl Section {
    pub fn from(bin: &[u8]) -> Section {
        return Section{
            content: bin.to_vec()
        }
    }
}