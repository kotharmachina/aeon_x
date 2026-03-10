pub struct Memory {
    data: Vec<u8>,
}

impl Memory {

    pub fn new() -> Self {
        Self{
            data: vec![0;2_147_483_648], //2 GB of unified memory.
        }
    }

    pub fn read(&self, address:u64) -> u8 {
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u64, value: u8) {
        self.data[address as usize] = value;
    }
}


