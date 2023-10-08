
/// Handles memory allocation
#[derive(Debug, Clone)]
pub struct OS {
    pub free_list: Vec<Block>,
}

impl Default for OS {
    fn default() -> Self {
        Self { free_list: vec![Block::new(0, 14384)] }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub offset: usize,
    pub len: usize,
}

impl Block {
    pub fn new(offset: usize, len: usize) -> Self {
        Self {
            offset,
            len,
        }
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset
    }
}

impl Eq for Block {}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.offset.partial_cmp(&other.offset)
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.offset.cmp(&other.offset)
    }
}