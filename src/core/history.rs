pub struct HistoryBuffer {
    pub histvec: Vec<String>,
}

impl HistoryBuffer {
    pub fn new() -> Self {
        let mut vec: Vec<String> = Vec::new();
        HistoryBuffer {
            histvec: vec,
        }
    }

    pub fn store(&mut self, input: String) {
        self.histvec.push(input);
    }

}
