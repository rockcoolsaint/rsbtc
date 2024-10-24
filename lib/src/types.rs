pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { blocks: vec![] }
    }
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}

pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Block {
            header,
            transactions,
        }
    }
    pub fn hash(&self) -> ! {
        unimplemented!()
    }
}
pub struct BlockHeader;
pub struct Transaction;
