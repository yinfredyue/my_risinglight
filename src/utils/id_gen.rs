pub struct IntIdGen {
    next_id: u32,
}

impl IntIdGen {
    pub fn new() -> Self {
        IntIdGen { next_id: 1 }
    }

    pub fn next_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}
